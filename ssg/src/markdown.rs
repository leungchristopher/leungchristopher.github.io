use std::collections::HashMap;

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(c),
        }
    }
    out
}

/// Renders inline markdown (emphasis, code, links, math spans) to HTML.
fn inline(s: &str, refs: &HashMap<String, String>) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut out = String::new();
    let mut i = 0;
    while i < n {
        let c = chars[i];
        // Math spans ($$...$$ or $...$) pass through verbatim for MathJax;
        // emphasis/code are not parsed inside them.
        if c == '$' {
            let display = i + 1 < n && chars[i + 1] == '$';
            let delim_len = if display { 2 } else { 1 };
            let delim: String = chars[i..i + delim_len].iter().collect();
            if let Some(rel_end) = find_seq(&chars, i + delim_len, &delim) {
                let inner: String = chars[i..rel_end + delim_len].iter().collect();
                out.push_str(&escape(&inner));
                i = rel_end + delim_len;
                continue;
            }
        }
        if c == '`' {
            if let Some(end) = find_char(&chars, i + 1, '`') {
                let inner: String = chars[i + 1..end].iter().collect();
                out.push_str("<code>");
                out.push_str(&escape(&inner));
                out.push_str("</code>");
                i = end + 1;
                continue;
            }
        }
        if c == '[' {
            if let Some(close) = find_char(&chars, i + 1, ']') {
                let label: String = chars[i + 1..close].iter().collect();
                // Explicit [text](url)
                if close + 1 < n && chars[close + 1] == '(' {
                    if let Some(paren_close) = find_char(&chars, close + 2, ')') {
                        let url: String = chars[close + 2..paren_close].iter().collect();
                        out.push_str("<a href=\"");
                        out.push_str(&escape(&url));
                        out.push_str("\">");
                        out.push_str(&inline(&label, refs));
                        out.push_str("</a>");
                        i = paren_close + 1;
                        continue;
                    }
                }
                // Shortcut reference [label] matching a [label]: url definition
                if let Some(url) = refs.get(&label) {
                    out.push_str("<a href=\"");
                    out.push_str(&escape(url));
                    out.push_str("\">");
                    out.push_str(&inline(&label, refs));
                    out.push_str("</a>");
                    i = close + 1;
                    continue;
                }
            }
        }
        if c == '*' {
            let is_bold = i + 1 < n && chars[i + 1] == '*';
            let marker = if is_bold { "**" } else { "*" };
            let start = i + marker.len();
            if let Some(end) = find_seq(&chars, start, marker) {
                let inner: String = chars[start..end].iter().collect();
                let tag = if is_bold { "strong" } else { "em" };
                out.push_str(&format!("<{}>", tag));
                out.push_str(&inline(&inner, refs));
                out.push_str(&format!("</{}>", tag));
                i = end + marker.len();
                continue;
            }
        }
        out.push_str(&escape(&c.to_string()));
        i += 1;
    }
    out
}

fn find_char(chars: &[char], from: usize, target: char) -> Option<usize> {
    (from..chars.len()).find(|&j| chars[j] == target)
}

fn find_seq(chars: &[char], from: usize, seq: &str) -> Option<usize> {
    let seq: Vec<char> = seq.chars().collect();
    let n = chars.len();
    let m = seq.len();
    if m == 0 || from + m > n {
        return None;
    }
    (from..=n - m).find(|&j| chars[j..j + m] == seq[..])
}

fn ial_class(line: &str) -> Option<String> {
    let line = line.trim();
    let inner = line.strip_prefix("{:")?.strip_suffix('}')?;
    let inner = inner.trim();
    inner.strip_prefix('.').map(|c| c.trim().to_string())
}

/// Renders a markdown document body to HTML. `directives` maps a bare
/// `{{name}}` line to pre-rendered HTML to splice in verbatim (used for
/// project-card loops driven by data outside the markdown file).
pub fn render(src: &str, directives: &HashMap<&str, String>) -> String {
    // Pass 1: pull out `[label]: url` reference definitions.
    let mut refs: HashMap<String, String> = HashMap::new();
    let mut lines: Vec<&str> = Vec::new();
    for line in src.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix('[') {
            if let Some((label, tail)) = rest.split_once("]:") {
                refs.insert(label.to_string(), tail.trim().to_string());
                continue;
            }
        }
        lines.push(line);
    }

    let mut out = String::new();
    let mut i = 0;
    let n = lines.len();
    while i < n {
        let line = lines[i];
        let trimmed = line.trim();

        if trimmed.is_empty() {
            i += 1;
            continue;
        }

        // Directive line: {{name}}
        if trimmed.starts_with("{{") && trimmed.ends_with("}}") {
            let name = &trimmed[2..trimmed.len() - 2];
            if let Some(html) = directives.get(name) {
                out.push_str(html);
                out.push('\n');
            }
            i += 1;
            continue;
        }

        // Fenced code block
        if let Some(lang) = trimmed.strip_prefix("```") {
            let mut body = Vec::new();
            i += 1;
            while i < n && lines[i].trim() != "```" {
                body.push(lines[i]);
                i += 1;
            }
            i += 1; // skip closing fence
            let class = if lang.is_empty() {
                String::new()
            } else {
                format!(" class=\"language-{}\"", escape(lang))
            };
            out.push_str(&format!("<pre><code{}>", class));
            out.push_str(&escape(&body.join("\n")));
            out.push_str("</code></pre>\n");
            continue;
        }

        // Raw HTML block: starts with '<', passed through verbatim until
        // a blank line.
        if trimmed.starts_with('<') {
            let mut body = Vec::new();
            while i < n && !lines[i].trim().is_empty() {
                body.push(lines[i]);
                i += 1;
            }
            out.push_str(&body.join("\n"));
            out.push('\n');
            continue;
        }

        // ATX heading
        if let Some(rest) = trimmed.strip_prefix('#') {
            let mut level = 1;
            let mut rest = rest;
            while let Some(r) = rest.strip_prefix('#') {
                level += 1;
                rest = r;
            }
            let text = rest.trim();
            out.push_str(&format!(
                "<h{}>{}</h{}>\n",
                level,
                inline(text, &refs),
                level
            ));
            i += 1;
            continue;
        }

        // Blockquote
        if let Some(_) = trimmed.strip_prefix("> ") {
            let mut body = Vec::new();
            while i < n && lines[i].trim_start().starts_with('>') {
                let t = lines[i].trim_start();
                body.push(t.strip_prefix("> ").or(t.strip_prefix('>')).unwrap_or(t));
                i += 1;
            }
            out.push_str("<blockquote><p>");
            out.push_str(&inline(&body.join(" "), &refs));
            out.push_str("</p></blockquote>\n");
            continue;
        }

        // Unordered list
        if trimmed.starts_with("- ") {
            out.push_str("<ul>\n");
            while i < n && lines[i].trim().starts_with("- ") {
                let item = &lines[i].trim()[2..];
                out.push_str(&format!("<li>{}</li>\n", inline(item, &refs)));
                i += 1;
            }
            out.push_str("</ul>\n");
            continue;
        }

        // Ordered list
        if is_ordered_item(trimmed) {
            out.push_str("<ol>\n");
            while i < n && is_ordered_item(lines[i].trim()) {
                let item = lines[i].trim().splitn(2, ". ").nth(1).unwrap_or("");
                out.push_str(&format!("<li>{}</li>\n", inline(item, &refs)));
                i += 1;
            }
            out.push_str("</ol>\n");
            continue;
        }

        // Paragraph: gather consecutive plain lines.
        let mut body = Vec::new();
        while i < n {
            let t = lines[i].trim();
            if t.is_empty()
                || t.starts_with('#')
                || t.starts_with("```")
                || t.starts_with('<')
                || t.starts_with("- ")
                || is_ordered_item(t)
                || t.starts_with('>')
                || ial_class(t).is_some()
            {
                break;
            }
            body.push(lines[i].trim());
            i += 1;
        }
        let class = if i < n {
            ial_class(lines[i].trim())
        } else {
            None
        };
        if class.is_some() {
            i += 1;
        }
        let class_attr = class.map(|c| format!(" class=\"{}\"", c)).unwrap_or_default();
        out.push_str(&format!(
            "<p{}>{}</p>\n",
            class_attr,
            inline(&body.join(" "), &refs)
        ));
    }
    out
}

fn is_ordered_item(t: &str) -> bool {
    match t.split_once(". ") {
        Some((num, _)) => !num.is_empty() && num.chars().all(|c| c.is_ascii_digit()),
        None => false,
    }
}
