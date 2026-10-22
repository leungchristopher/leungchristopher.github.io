use crate::{frontmatter, markdown, templates, write};
use std::{collections::HashMap, fs, path::Path};

pub fn escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

fn url(s: &str) -> String {
    s.bytes().map(|b| match b {
        b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'/' => (b as char).to_string(),
        _ => format!("%{b:02X}"),
    }).collect()
}

pub fn article(src: &str, path: &str) -> String {
    let (fm, body) = frontmatter::parse(src);
    let heading = fm.get("title").map(|t| format!("<h1>{}</h1>", escape(t))).unwrap_or_default();
    let parent = path.rsplit_once('/').map(|(p, _)| format!("{p}/")).unwrap_or_default();
    templates::page(&templates::Page {
        title: fm.get("title").unwrap_or("Note").into(),
        path: format!("/notes/{}.html", url(path.trim_end_matches(".md"))),
        body_class: None,
        math: true,
        content: format!("<article class=\"essay\" data-note-path=\"{}\">{}<div class=\"essay-body\">{}</div><p><a href=\"/notes/{}\">← Back to folder</a></p></article>", escape(path), heading, markdown::render(body, &HashMap::new()), url(&parent)),
    })
}

pub fn build(dir: &Path, out: &Path, relative: &str) -> Vec<String> {
    let mut entries = if dir.exists() { fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect::<Vec<_>>() } else { vec![] };
    entries.retain(|p| !p.is_symlink() && !p.file_name().unwrap().to_string_lossy().starts_with('.'));
    entries.sort_by_key(|p| (!p.is_dir(), p.file_name().unwrap().to_os_string()));
    let here = format!("/notes/{relative}");
    let mut urls = vec![url(&here)];
    let mut items = String::new();
    for path in entries {
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let rel = format!("{relative}{name}");
        if path.is_dir() {
            items.push_str(&format!("<li><a href=\"/notes/{}\">{}/</a> <small>Folder</small></li>", url(&format!("{rel}/")), escape(&name)));
            urls.extend(build(&path, out, &format!("{rel}/")));
        } else if path.extension().is_some_and(|e| e == "md") {
            let src = fs::read_to_string(&path).unwrap();
            let (fm, body) = frontmatter::parse(&src);
            let title = fm.get("title").or_else(|| body.lines().find_map(|l| l.strip_prefix("# "))).unwrap_or(name.trim_end_matches(".md"));
            let page_path = format!("/notes/{}.html", rel.trim_end_matches(".md"));
            items.push_str(&format!("<li><a href=\"{}\">{}</a></li>", url(&page_path), escape(title)));
            write(out, &page_path, &article(&src, &rel));
            urls.push(url(&page_path));
        }
    }
    let back = relative.trim_end_matches('/').rsplit_once('/').map(|(p, _)| format!("{p}/")).unwrap_or_default();
    let title = if relative.is_empty() { "Notes" } else { relative.trim_end_matches('/').rsplit('/').next().unwrap() };
    let parent = if relative.is_empty() { String::new() } else { format!("<p><a href=\"/notes/{}\">← Parent folder</a></p>", url(&back)) };
    if items.is_empty() { items = "<li>No notes here yet.</li>".into(); }
    write(out, &format!("{here}index.html"), &templates::page(&templates::Page {
        title: title.into(), path: url(&here), body_class: None, math: false,
        content: format!("<section data-folder-path=\"{}\"><h1>{}</h1>{parent}<ul class=\"notes-list\">{items}</ul></section>", escape(relative), escape(title)),
    }));
    urls
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nested_notes_and_code_math() {
        let root = std::env::temp_dir().join(format!("site-notes-test-{}", std::process::id()));
        let input = root.join("input/linear algebra");
        fs::create_dir_all(&input).unwrap();
        fs::write(input.join("vectors.md"), "# Vectors\n\n$x_i$\n\n```python\nprint('<hi>')\n```\n").unwrap();
        let out = root.join("out");
        let urls = build(&root.join("input"), &out, "");
        assert!(urls.contains(&"/notes/linear%20algebra/vectors.html".into()));
        let index = fs::read_to_string(out.join("notes/index.html")).unwrap();
        assert!(index.contains("linear%20algebra/"));
        assert!(!index.contains("Vectors"));
        let note = fs::read_to_string(out.join("notes/linear algebra/vectors.html")).unwrap();
        assert!(note.contains("$x_i$"));
        assert!(note.contains("&lt;hi&gt;"));
        assert!(note.contains("mathjax"));
        fs::remove_dir_all(root).unwrap();
    }
}
