mod config;
mod dates;
mod dlog;
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

fn copy_dir(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.file_name().unwrap().to_string_lossy().starts_with('.') {
            continue;
        }
        let dest = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir(&path, &dest);
        } else {
            fs::copy(&path, &dest).unwrap();
        }
    }
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
    let dlog_start = Date::parse(config::DLOG_START).unwrap();

    // ---- Projects data, shared by the home page and /about/ ----
    let projects_src = fs::read_to_string(content.join("data/projects.yml")).unwrap();
    let all_projects = projects::parse(&projects_src);
    let render_cards = |ps: &[&projects::Project]| -> String {
        ps.iter()
            .map(|p| {
                templates::project_card(&p.title, &p.affil, &p.desc, &p.img, &p.link, &p.link_text)
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let current_cards = render_cards(
        &all_projects.iter().filter(|p| p.current).collect::<Vec<_>>(),
    );
    let all_cards = render_cards(&all_projects.iter().collect::<Vec<_>>());

    // ---- Home page ----
    {
        let src = fs::read_to_string(content.join("pages/index.md")).unwrap();
        let (fm, body) = frontmatter::parse(&src);
        let mut directives = HashMap::new();
        directives.insert("projects:current", current_cards.clone());
        let body_html = markdown::render(body, &directives);
        let page = templates::Page {
            title: fm.get("title").unwrap_or("").to_string(),
            path: "/".to_string(),
            body_class: None,
            math: fm.flag("math"),
            content: templates::home_wrap(&body_html, true),
        };
        write(&out, "/index.html", &templates::page(&page));
    }

    // ---- About page ----
    {
        let src = fs::read_to_string(content.join("pages/about.md")).unwrap();
        let (fm, body) = frontmatter::parse(&src);
        let mut directives = HashMap::new();
        directives.insert("projects:all", all_cards.clone());
        let body_html = markdown::render(body, &directives);
        let page = templates::Page {
            title: fm.get("title").unwrap_or("About").to_string(),
            path: "/about/".to_string(),
            body_class: None,
            math: fm.flag("math"),
            content: templates::home_wrap(&body_html, true),
        };
        write(&out, "/about/index.html", &templates::page(&page));
    }

    // ---- Links page ----
    {
        let src = fs::read_to_string(content.join("pages/links.md")).unwrap();
        let (fm, body) = frontmatter::parse(&src);
        let body_html = markdown::render_links(body);
        let page = templates::Page {
            title: fm.get("title").unwrap_or("Links").to_string(),
            path: "/links/".to_string(),
            body_class: Some("links-page".to_string()),
            math: false,
            content: templates::links_wrap(&body_html),
        };
        write(&out, "/links/index.html", &templates::page(&page));
    }

    // ---- Writeups: linked only from /links/, not essays, no index/feed/sitemap entry ----
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
<p>Sorry, that page doesn't exist. Head back to the <a href="/">home page</a> or browse the <a href="/dlog/">log</a>.</p>"#;
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
                    "<li class=\"essay-item\">\n<a class=\"essay-item-title\" href=\"/essays/{slug}/\">{title}</a>\n<time class=\"essay-item-date\" datetime=\"{iso}\">{long}</time>\n</li>",
                    slug = e.slug, title = e.title, iso = e.date.iso(), long = e.date.long()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let content_html = format!(
            r#"<h1>Essays</h1>

<p class="lede">Notes and essays on machine learning, neuroscience, genetics, and the occasional curiosity. <a href="/feed/essays.xml">Subscribe with RSS</a></p>

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

    // ---- Dlog entries ----
    let mut dlog_entries: Vec<dlog::DlogEntry> = Vec::new();
    for path in read_dir_sorted(&content.join("dlog")) {
        let slug = path.file_stem().unwrap().to_string_lossy().to_string();
        let src = fs::read_to_string(&path).unwrap();
        let (fm, body) = frontmatter::parse(&src);
        let date = Date::parse(fm.get("date").unwrap_or(&slug)).unwrap();
        let entry = dlog::DlogEntry {
            date,
            title: fm.get("title").unwrap_or("").to_string(),
            goal: fm.get("goal").unwrap_or("").to_string(),
            summary: fm.get("summary").unwrap_or("").to_string(),
            sessions: fm.sessions.clone(),
            body_html: markdown::render(body, &HashMap::new()),
            slug,
        };
        dlog_entries.push(entry);
    }
    dlog_entries.sort_by(|a, b| b.date.cmp(&a.date));

    for e in &dlog_entries {
        let tags = e.tags();
        let tags_html = if tags.is_empty() {
            String::new()
        } else {
            format!(
                "<p class=\"tags\">{}</p>",
                tags.iter()
                    .map(|t| format!("<span class=\"tag\">{}</span>", t))
                    .collect::<Vec<_>>()
                    .join("")
            )
        };
        let article = templates::dlog_entry_article(
            e.display_title(),
            &e.date.long(),
            &e.date.iso(),
            &e.goal,
            &e.summary,
            &templates::sessions_table(&e.sessions),
            &e.body_html,
            &tags_html,
        );
        let page = templates::Page {
            title: e.display_title().to_string(),
            path: format!("/dlog/{}/", e.slug),
            body_class: Some("dlog-page".to_string()),
            math: false,
            content: article,
        };
        write(&out, &format!("/dlog/{}/index.html", e.slug), &templates::page(&page));
    }

    // ---- Dlog index ----
    {
        let stats = dlog::compute(&dlog_entries, dlog_start, today);
        let intro_src = fs::read_to_string(content.join("pages/dlog.md")).unwrap();
        let (_, intro_body) = frontmatter::parse(&intro_src);
        let intro_html = markdown::render(intro_body, &HashMap::new());

        let items: String = dlog_entries
            .iter()
            .map(|e| {
                let tags = json_tags(&e.tags());
                format!(
                    "<li class=\"entry-item\" data-tags=\"{tags}\">\n<a class=\"entry-item-title\" href=\"/dlog/{slug}/\">{title}</a>\n<time class=\"entry-item-date\" datetime=\"{iso}\">{long}</time>\n</li>",
                    tags = tags, slug = e.slug, title = e.display_title(), iso = e.date.iso(), long = e.date.long()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        let content_html = format!(
            r#"<h1>Log</h1>

<p class="lede">
Day {num_days}, {num_documented} logged ({pct}%), {streak} day streak, {hrs} hr {mins} min total.
</p>
{intro}
<div class="stats-row">
<div class="stats-panel">
{tag_cloud}
</div>
<div class="stats-panel">
{hours_chart}
</div>
</div>
{tooltip_script}

<h2>Entries</h2>

<ul class="entry-list">
{items}
</ul>"#,
            num_days = stats.num_days,
            num_documented = stats.num_documented_days,
            pct = stats.percentage_documented,
            streak = stats.streak,
            hrs = stats.total_hrs,
            mins = stats.total_mins,
            intro = intro_html,
            tooltip_script = templates::tooltip_script(),
            tag_cloud = templates::tag_cloud(&stats.tags),
            hours_chart = templates::hours_chart(&stats.daily),
            items = items,
        );
        let page = templates::Page {
            title: "Log".to_string(),
            path: "/dlog/".to_string(),
            body_class: Some("dlog-page".to_string()),
            math: false,
            content: content_html,
        };
        write(&out, "/dlog/index.html", &templates::page(&page));
    }

    // ---- Sitemap ----
    {
        let today_iso = today.iso();
        let mut urls: Vec<(String, String)> = vec![
            ("/".to_string(), today_iso.clone()),
            ("/about/".to_string(), today_iso.clone()),
            ("/essays/".to_string(), today_iso.clone()),
            ("/dlog/".to_string(), today_iso.clone()),
            ("/links/".to_string(), today_iso.clone()),
        ];
        for e in essays.iter().filter(|e| !e.draft) {
            urls.push((format!("/essays/{}/", e.slug), e.date.iso()));
        }
        for e in &dlog_entries {
            urls.push((format!("/dlog/{}/", e.slug), e.date.iso()));
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
    copy_dir(&root.join("assets/images"), &out.join("assets/images"));

    // GitHub Pages custom domain: needed in the published output on every
    // build, since the Actions deploy replaces the whole artifact each time.
    write(&out, "/CNAME", "leungchristopher.com\n");

    println!("Built {} essays, {} dlog entries.", essays.len(), dlog_entries.len());
}

/// Encodes tags as a JSON array for the `data-tags` attribute, so a
/// multi-word tag like "world models" can't be confused with a
/// space- or comma-delimited list when the client-side filter reads it.
fn json_tags(tags: &[String]) -> String {
    let items: Vec<String> = tags
        .iter()
        .map(|t| format!("\"{}\"", t.replace('\\', "\\\\").replace('"', "\\\"")))
        .collect();
    let json = format!("[{}]", items.join(","));
    // HTML-attribute-escape (the value sits inside a double-quoted attribute).
    json.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
