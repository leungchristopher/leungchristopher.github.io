use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct FrontMatter {
    pub fields: HashMap<String, String>,
}

impl FrontMatter {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.fields.get(key).map(|s| s.as_str())
    }

    pub fn flag(&self, key: &str) -> bool {
        self.get(key) == Some("true")
    }
}

fn unquote(v: &str) -> String {
    let v = v.trim();
    if v.len() >= 2 {
        let first = v.chars().next().unwrap();
        let last = v.chars().last().unwrap();
        if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
            return v[1..v.len() - 1].replace("\\\"", "\"");
        }
    }
    v.to_string()
}

/// Splits leading `---\n ... \n---\n` front matter from a source file and
/// parses it. Returns (front_matter, remaining_body).
pub fn parse(src: &str) -> (FrontMatter, &str) {
    let mut fm = FrontMatter::default();
    let src = src.strip_prefix('\n').unwrap_or(src);
    if !src.starts_with("---") {
        return (fm, src);
    }
    let after_open = &src[3..];
    let after_open = after_open.strip_prefix('\n').unwrap_or(after_open);
    let Some(close_idx) = after_open.find("\n---") else {
        return (fm, src);
    };
    let header = &after_open[..close_idx];
    let rest = &after_open[close_idx + 4..];
    let body = rest.strip_prefix('\n').unwrap_or(rest);

    let mut lines = header.lines().peekable();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some((k, v)) = line.split_once(':') {
            fm.fields.insert(k.trim().to_string(), unquote(v));
        }
    }

    (fm, body)
}
