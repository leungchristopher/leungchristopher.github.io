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

/// Parses a `!progress[Title](X/Y)` line into (title, filled, total).
fn parse_progress(line: &str) -> Option<(String, u32, u32)> {
    let rest = line.strip_prefix("!progress[")?;
    let (title, rest) = rest.split_once("](")?;
    let rest = rest.strip_suffix(')')?;
    let (filled, total) = rest.split_once('/')?;
    let filled: u32 = filled.trim().parse().ok()?;
    let total: u32 = total.trim().parse().ok()?;
    Some((title.to_string(), filled, total))
}

/// Renders a reusable progress bar: a title line above a continuous
/// black fill bar, with an "X/Y" count.
fn render_progress(title: &str, filled: u32, total: u32, refs: &HashMap<String, String>) -> String {
    let pct = if total > 0 { (filled as f64 / total as f64) * 100.0 } else { 0.0 };
    format!(
        "<div class=\"progress-bar\">\n<div class=\"progress-bar-title\">{}</div>\n<div class=\"progress-bar-track\"><div class=\"progress-bar-fill\" style=\"width: {:.2}%\"></div></div>\n<div class=\"progress-bar-count\">{}/{}</div>\n</div>\n",
        inline(title, refs),
        pct,
        filled,
        total
    )
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

        // Segmented progress bar: !progress[Title](X/Y)
        if let Some((title, filled, total)) = parse_progress(trimmed) {
            out.push_str(&render_progress(&title, filled, total, &refs));
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

/// Strips an optional trailing `[[slug]]` writeup marker off a reading-page
/// list item, returning the remaining text and the essay slug if present.
fn strip_writeup(item: &str) -> (&str, Option<&str>) {
    let t = item.trim_end();
    if let Some(inner) = t.strip_suffix("]]") {
        if let Some(start) = inner.rfind("[[") {
            let slug = &inner[start + 2..];
            let rest = t[..start].trim_end();
            return (rest, Some(slug));
        }
    }
    (item, None)
}

/// Renders the reading page: `#` is a plain title, `##` opens a collapsible
/// top-level group, `###` opens a collapsible subgroup within it, and `-`
/// items are gathered into that subgroup's list. Plain lines become
/// paragraphs (with `{: .class}` IAL support), same as `render`.
pub fn render_reading(src: &str) -> String {
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
    let mut in_wrapper = false;
    let mut in_h1 = false;
    let mut in_h2 = false;
    let mut in_ul = false;

    let close_ul = |out: &mut String, in_ul: &mut bool| {
        if *in_ul {
            out.push_str("</ul>\n");
            *in_ul = false;
        }
    };
    let close_h2 = |out: &mut String, in_ul: &mut bool, in_h2: &mut bool| {
        close_ul(out, in_ul);
        if *in_h2 {
            out.push_str("</details>\n");
            *in_h2 = false;
        }
    };
    let close_h1 = |out: &mut String, in_ul: &mut bool, in_h2: &mut bool, in_h1: &mut bool| {
        close_h2(out, in_ul, in_h2);
        if *in_h1 {
            out.push_str("</div>\n</details>\n");
            *in_h1 = false;
        }
    };

    let mut i = 0;
    let n = lines.len();
    while i < n {
        let line = lines[i];
        let trimmed = line.trim();

        if trimmed.is_empty() {
            i += 1;
            continue;
        }

        if let Some(text) = trimmed.strip_prefix("### ") {
            close_h2(&mut out, &mut in_ul, &mut in_h2);
            out.push_str(&format!(
                "<details class=\"link-h2\" open>\n<summary><span class=\"link-arrow\"></span><h3 class=\"link-h2-title\">{}</h3></summary>\n",
                inline(text.trim(), &refs)
            ));
            in_h2 = true;
            i += 1;
            continue;
        }

        if let Some(text) = trimmed.strip_prefix("## ") {
            close_h1(&mut out, &mut in_ul, &mut in_h2, &mut in_h1);
            if !in_wrapper {
                out.push_str("<div class=\"reading-groups\">\n");
                in_wrapper = true;
            }
            out.push_str(&format!(
                "<details class=\"link-h1\" open>\n<summary><span class=\"link-arrow\"></span><h2 class=\"link-h1-title\">{}</h2></summary>\n<div class=\"link-h1-body\">\n",
                inline(text.trim(), &refs)
            ));
            in_h1 = true;
            i += 1;
            continue;
        }

        if let Some(text) = trimmed.strip_prefix("# ") {
            close_h1(&mut out, &mut in_ul, &mut in_h2, &mut in_h1);
            out.push_str(&format!("<h1>{}</h1>\n", inline(text.trim(), &refs)));
            i += 1;
            continue;
        }

        if trimmed.starts_with("- ") {
            if !in_ul {
                out.push_str("<ul>\n");
                in_ul = true;
            }
            let (item, writeup) = strip_writeup(&trimmed[2..]);
            let mut li = inline(item, &refs);
            if let Some(slug) = writeup {
                li.push_str(&format!(
                    " <a class=\"link-writeup\" href=\"/writeups/{}/\">[writeup]</a>",
                    slug
                ));
            }
            out.push_str(&format!("<li>{}</li>\n", li));
            i += 1;
            continue;
        }

        // Paragraph: gather consecutive plain lines, honoring a trailing IAL.
        let mut body = Vec::new();
        while i < n {
            let t = lines[i].trim();
            if t.is_empty()
                || t.starts_with('#')
                || t.starts_with("- ")
                || ial_class(t).is_some()
            {
                break;
            }
            body.push(lines[i].trim());
            i += 1;
        }
        let class = if i < n { ial_class(lines[i].trim()) } else { None };
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

    close_h1(&mut out, &mut in_ul, &mut in_h2, &mut in_h1);
    if in_wrapper {
        out.push_str("</div>\n");
    }
    out
}

fn is_ordered_item(t: &str) -> bool {
    match t.split_once(". ") {
        Some((num, _)) => !num.is_empty() && num.chars().all(|c| c.is_ascii_digit()),
        None => false,
    }
}
