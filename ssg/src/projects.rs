pub struct Project {
    pub title: String,
    pub affil: String,
    pub desc: String,
    pub img: String,
    pub link: Option<String>,
    pub link_text: String,
    pub current: bool,
}

fn unquote(v: &str) -> String {
    let v = v.trim();
    if v.len() >= 2 {
        let first = v.chars().next().unwrap();
        let last = v.chars().last().unwrap();
        if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
            return v[1..v.len() - 1].to_string();
        }
    }
    v.to_string()
}

/// Parses the hand-rolled `- key: value` / `  key: value` record list format
/// used by data/projects.yml. Each record starts with `- title: ...` and
/// continues with indented `key: value` lines until the next `- `.
pub fn parse(src: &str) -> Vec<Project> {
    let mut projects = Vec::new();
    let mut cur: Option<Project> = None;

    for line in src.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let (is_new, body) = if let Some(rest) = trimmed.strip_prefix("- ") {
            (true, rest)
        } else {
            (false, trimmed)
        };
        if is_new {
            if let Some(p) = cur.take() {
                projects.push(p);
            }
            cur = Some(Project {
                title: String::new(),
                affil: String::new(),
                desc: String::new(),
                img: String::new(),
                link: None,
                link_text: String::new(),
                current: false,
            });
        }
        let Some((key, val)) = body.split_once(':') else {
            continue;
        };
        let Some(p) = cur.as_mut() else { continue };
        let val = unquote(val);
        match key.trim() {
            "title" => p.title = val,
            "affil" => p.affil = val,
            "desc" => p.desc = val,
            "img" => p.img = val,
            "link" => p.link = Some(val),
            "link_text" => p.link_text = val,
            "current" => p.current = val == "true",
            _ => {}
        }
    }
    if let Some(p) = cur.take() {
        projects.push(p);
    }
    projects
}
