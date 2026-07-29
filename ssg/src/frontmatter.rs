use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Session {
    pub time_in: String,
    pub time_out: String,
    pub task: String,
    /// From a `tag: "a, b"` field — one or more comma-separated tags.
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct FrontMatter {
    pub fields: HashMap<String, String>,
    pub sessions: Vec<Session>,
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

/// Splits a `key: value, key: value` flow-map body (no braces) on commas
/// that aren't inside quotes.
fn split_flow_fields(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for c in body.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                cur.push(c);
            }
            ',' if !in_quotes => {
                out.push(cur.trim().to_string());
                cur = String::new();
            }
            _ => cur.push(c),
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

fn parse_session_line(line: &str) -> Option<Session> {
    let line = line.trim();
    let line = line.strip_prefix("- {")?;
    let line = line.strip_suffix('}')?;
    let mut s = Session::default();
    for field in split_flow_fields(line) {
        let (k, v) = field.split_once(':')?;
        let v = unquote(v);
        match k.trim() {
            "in" => s.time_in = v,
            "out" => s.time_out = v,
            "task" => s.task = v,
            "tag" => {
                s.tags = v
                    .split(',')
                    .map(|t| t.trim().to_string())
                    .filter(|t| !t.is_empty())
                    .collect()
            }
            _ => {}
        }
    }
    Some(s)
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
        if line.trim() == "sessions:" {
            while let Some(next) = lines.peek() {
                if next.trim_start().starts_with("- {") {
                    if let Some(s) = parse_session_line(next) {
                        fm.sessions.push(s);
                    }
                    lines.next();
                } else {
                    break;
                }
            }
            continue;
        }
        if let Some((k, v)) = line.split_once(':') {
            fm.fields.insert(k.trim().to_string(), unquote(v));
        }
    }

    (fm, body)
}
