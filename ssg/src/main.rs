mod config;
mod dates;
mod frontmatter;
mod markdown;
mod minify;
mod projects;
mod templates;

use dates::Date;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

fn write(out_dir: &Path, rel_path: &str, content: &str) {
    let path = out_dir.join(rel_path.trim_start_matches('/'));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

fn read_dir_sorted(dir: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|e| e == "md").unwrap_or(false))
        .collect();
    paths.sort();
    paths
}

struct Essay {
    slug: String,
    title: String,
    date: Date,
    draft: bool,
    html: String,
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let content = root.join("content");
    let out = root.join("_site");

    if out.exists() {
        fs::remove_dir_all(&out).unwrap();
    }
    fs::create_dir_all(&out).unwrap();
    let today = Date::today();

    // ---- Projects data, shared by the home page and /projects/ ----
    let projects_src = fs::read_to_string(content.join("data/projects.yml")).unwrap();
    let all_projects = projects::parse(&projects_src);
    let render_cards = |ps: &[&projects::Project]| -> String {
        ps.iter()
            .map(|p| {
                templates::project_card(&p.title, &p.affil, &p.desc, &p.link, &p.link_text)
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let all_cards = render_cards(&all_projects.iter().collect::<Vec<_>>());

    // ---- Home page ----
    {
        let src = fs::read_to_string(content.join("pages/index.md")).unwrap();
        let (fm, body) = frontmatter::parse(&src);
        let body_html = markdown::render(body, &HashMap::new());
        let page = templates::Page {
            title: fm.get("title").unwrap_or("").to_string(),
            path: "/".to_string(),
            body_class: None,
            math: fm.flag("math"),
            content: templates::home_wrap(&body_html),
        };
        write(&out, "/index.html", &templates::page(&page));
    }

    // ---- Projects page ----
    {
        let src = fs::read_to_string(content.join("pages/projects.md")).unwrap();
        let (fm, body) = frontmatter::parse(&src);
        let mut directives = HashMap::new();
        directives.insert("projects:all", all_cards.clone());
        let body_html = markdown::render(body, &directives);
        let page = templates::Page {
            title: fm.get("title").unwrap_or("Projects").to_string(),
            path: "/projects/".to_string(),
            body_class: None,
            math: fm.flag("math"),
            content: templates::home_wrap(&body_html),
        };
        write(&out, "/projects/index.html", &templates::page(&page));
    }

    // ---- Links page ----
    {
        let src = fs::read_to_string(content.join("pages/links.md")).unwrap();
        let (fm, body) = frontmatter::parse(&src);
        let body_html = markdown::render_reading(body);
        let page = templates::Page {
            title: fm.get("title").unwrap_or("Links").to_string(),
            path: "/links/".to_string(),
            body_class: None,
            math: false,
            content: templates::home_wrap(&body_html),
        };
        write(&out, "/links/index.html", &templates::page(&page));
    }

    // ---- Writeups: intentionally unlisted, no index/feed/sitemap entry ----
    {
        let dir = content.join("writeups");
        if dir.exists() {
            for path in read_dir_sorted(&dir) {
                let slug = path.file_stem().unwrap().to_string_lossy().to_string();
                let src = fs::read_to_string(&path).unwrap();
                let (fm, body) = frontmatter::parse(&src);
                let title = fm.get("title").unwrap_or(&slug).to_string();
                let date = Date::parse(fm.get("date").unwrap_or("1970-01-01")).unwrap();
                let math = fm.flag("math");
                let body_html = markdown::render(body, &HashMap::new());
                let article =
                    templates::writeup_article(&title, &date.long(), &date.iso(), &body_html);
                let page = templates::Page {
                    title,
                    path: format!("/writeups/{}/", slug),
                    body_class: None,
                    math,
                    content: article,
                };
                write(&out, &format!("/writeups/{}/index.html", slug), &templates::page(&page));
            }
        }
    }

    // ---- 404 page ----
    {
        let content_html = r#"<h1>Page not found</h1>
<p>Sorry, that page doesn't exist. Head back to the <a href="/">home page</a>.</p>"#;
        let page = templates::Page {
            title: "Page not found".to_string(),
            path: "/404.html".to_string(),
            body_class: None,
            math: false,
            content: content_html.to_string(),
        };
        write(&out, "/404.html", &templates::page(&page));
    }

    // ---- Essays ----
    let mut essays: Vec<Essay> = Vec::new();
    for path in read_dir_sorted(&content.join("essays")) {
        let slug = path.file_stem().unwrap().to_string_lossy().to_string();
        let src = fs::read_to_string(&path).unwrap();
        let (fm, body) = frontmatter::parse(&src);
        let title = fm.get("title").unwrap_or(&slug).to_string();
        let date = Date::parse(fm.get("date").unwrap_or("1970-01-01")).unwrap();
        let math = fm.flag("math");
        let draft = fm.flag("draft");
        let body_html = markdown::render(body, &HashMap::new());
        let article =
            templates::essay_article(&title, &date.long(), &date.iso(), &body_html);
        let page = templates::Page {
            title: title.clone(),
            path: format!("/essays/{}/", slug),
            body_class: None,
            math,
            content: article,
        };
        write(&out, &format!("/essays/{}/index.html", slug), &templates::page(&page));
        essays.push(Essay { slug, title, date, draft, html: body_html });
    }
    essays.sort_by(|a, b| b.date.cmp(&a.date));

    // ---- Essays index ----
    {
        let items: String = essays
            .iter()
            .filter(|e| !e.draft)
            .map(|e| {
                format!(
                    "<li class=\"essay-item\">\n<a class=\"essay-item-title\" href=\"/essays/{slug}/\">{title}</a>\n<span class=\"essay-item-rule\" aria-hidden=\"true\"></span>\n<time class=\"essay-item-date\" datetime=\"{iso}\">{date}</time>\n</li>",
                    slug = e.slug, title = e.title, iso = e.date.iso(), date = e.date.dotted()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let content_html = format!(
            r#"<h1>Essays</h1>

<p class="lede"><a href="/feed/essays.xml">Subscribe with RSS</a></p>

<ul class="essay-list">
{items}
</ul>"#,
            items = items
        );
        let page = templates::Page {
            title: "Essays".to_string(),
            path: "/essays/".to_string(),
            body_class: None,
            math: false,
            content: content_html,
        };
        write(&out, "/essays/index.html", &templates::page(&page));
    }

    // ---- Essays feed (Atom) ----
    {
        let updated = essays.first().map(|e| e.date.iso()).unwrap_or_else(|| today.iso());
        let entries: String = essays
            .iter()
            .filter(|e| !e.draft)
            .map(|e| {
                format!(
                    r#"  <entry>
    <title>{title}</title>
    <link href="{url}/essays/{slug}/"/>
    <id>{url}/essays/{slug}/</id>
    <published>{iso}</published>
    <updated>{iso}</updated>
    <author><name>{name}</name></author>
    <content type="html">{content}</content>
  </entry>"#,
                    title = xml_escape(&e.title),
                    url = config::URL,
                    slug = e.slug,
                    iso = e.date.iso(),
                    name = config::NAME,
                    content = xml_escape(&e.html),
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let feed = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>{title} — Essays</title>
  <link href="{url}/feed/essays.xml" rel="self"/>
  <link href="{url}/"/>
  <updated>{updated}</updated>
  <id>{url}/</id>
  <author><name>{name}</name></author>
{entries}
</feed>
"#,
            title = config::TITLE,
            url = config::URL,
            updated = updated,
            name = config::NAME,
            entries = entries,
        );
        write(&out, "/feed/essays.xml", &feed);
    }


    // ---- Sitemap ----
    {
        let today_iso = today.iso();
        let mut urls: Vec<(String, String)> = vec![
            ("/".to_string(), today_iso.clone()),
            ("/projects/".to_string(), today_iso.clone()),
            ("/essays/".to_string(), today_iso.clone()),
            ("/links/".to_string(), today_iso.clone()),
        ];
        for e in essays.iter().filter(|e| !e.draft) {
            urls.push((format!("/essays/{}/", e.slug), e.date.iso()));
        }
        let body: String = urls
            .iter()
            .map(|(u, lastmod)| format!("  <url><loc>{}{}</loc><lastmod>{}</lastmod></url>", config::URL, u, lastmod))
            .collect::<Vec<_>>()
            .join("\n");
        let sitemap = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n{}\n</urlset>\n",
            body
        );
        write(&out, "/sitemap.xml", &sitemap);
    }

    // ---- Static assets ----
    let css_src = fs::read_to_string(root.join("assets/css/style.css")).unwrap();
    write(&out, "/assets/css/style.css", &minify::css(&css_src));

    // GitHub Pages custom domain: needed in the published output on every
    // build, since the Actions deploy replaces the whole artifact each time.
    write(&out, "/CNAME", "leungchristopher.com\n");

    println!("Built {} essays.", essays.len());
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
