use crate::config::*;

pub struct Page {
    pub title: String,
    pub path: String,
    pub body_class: Option<String>,
    pub math: bool,
    pub content: String,
}

const ESSAY_TOC: &str = r#"<nav class="essay-toc" aria-label="On this page" hidden>
  <p class="essay-toc-title">Contents</p>
  <ol class="essay-toc-list"></ol>
</nav>
<script>
(function () {
  var body = document.querySelector('.essay-body');
  var nav = document.querySelector('.essay-toc');
  if (!body || !nav) return;
  var headings = body.querySelectorAll('h2, h3');
  if (headings.length < 2) return;
  var list = nav.querySelector('.essay-toc-list');
  headings.forEach(function (heading, index) {
    if (!heading.id) heading.id = 'section-' + index;
    var li = document.createElement('li');
    li.className = 'essay-toc-' + heading.tagName.toLowerCase();
    var a = document.createElement('a');
    a.href = '#' + heading.id;
    a.textContent = heading.textContent.trim();
    li.appendChild(a);
    list.appendChild(li);
  });
  nav.hidden = false;
})();
</script>
"#;

const MATHJAX: &str = r#"<script>
  window.MathJax = {
    tex: {
      inlineMath: [['$', '$'], ['\\(', '\\)']],
      displayMath: [['$$', '$$'], ['\\[', '\\]']],
      processEscapes: true,
      processEnvironments: true,
      tags: 'ams'
    },
    options: {
      skipHtmlTags: ['script', 'noscript', 'style', 'textarea', 'pre', 'code']
    },
    svg: { fontCache: 'global' }
  };
</script>
<script src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-svg.js" async></script>
"#;

pub fn page(p: &Page) -> String {
    let title_tag = if p.title.is_empty() || p.title == TITLE {
        TITLE.to_string()
    } else {
        format!("{} · {}", p.title, TITLE)
    };
    let body_class = p
        .body_class
        .as_ref()
        .map(|c| format!(" class=\"{}\"", c))
        .unwrap_or_default();
    let mathjax = if p.math { MATHJAX } else { "" };
    let social = SOCIAL
        .iter()
        .map(|(label, href)| format!("<a href=\"{}\">{}</a>", href, label))
        .collect::<Vec<_>>()
        .join("\n        ");
    let nav_link = |path: &str, label: &str, active: bool| {
        if active {
            format!("<a class=\"is-active\" href=\"{}\" aria-current=\"page\">{}</a>", path, label)
        } else {
            format!("<a href=\"{}\">{}</a>", path, label)
        }
    };
    let nav = [
        nav_link("/", "Now", p.path == "/"),
        nav_link("/projects/", "Projects", p.path.starts_with("/projects/")),
        nav_link("/essays/", "Essays", p.path.starts_with("/essays/")),
        nav_link("/links/", "Links", p.path.starts_with("/links/")),
    ]
    .join("\n        ");

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{title_tag}</title>
  <meta name="description" content="{description}">
  <link rel="canonical" href="{url}{path}">
  <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Lato:ital,wght@0,400;0,500;0,700;1,400;1,500;1,700&amp;display=swap">
  <link rel="stylesheet" href="/assets/css/style.css">
  <meta name="view-transition" content="same-origin">
  <link type="application/atom+xml" rel="alternate" href="/feed/essays.xml" title="{site_title} — Essays">
  {mathjax}</head>
<body{body_class}>
  <div class="wrap">
    <header class="site-header">
      <nav class="site-nav">
        {nav}
      </nav>
    </header>

    <main class="content">
      {content}
    </main>

    <footer class="site-footer">
      <div class="social">
        {social}
      </div>
    </footer>
  </div>

</body>
</html>
"#,
        title_tag = title_tag,
        description = DESCRIPTION,
        url = URL,
        path = p.path,
        site_title = TITLE,
        mathjax = mathjax,
        body_class = body_class,
        social = social,
        nav = nav,
        content = p.content,
    )
}

pub fn home_wrap(body_html: &str) -> String {
    format!("<div class=\"home\">\n{}</div>\n", body_html)
}

pub fn essay_article(title: &str, date_long: &str, date_iso: &str, body_html: &str) -> String {
    format!(
        r#"<article class="essay">
  <header class="essay-header">
    <h1 class="essay-title">{title}</h1>
    <time class="essay-date" datetime="{date_iso}">{date_long}</time>
  </header>

  {toc}

  <div class="essay-body">
{body}
  </div>

  <p class="essay-back"><a href="/essays/">All essays</a></p>
</article>"#,
        title = title,
        date_iso = date_iso,
        date_long = date_long,
        body = body_html,
        toc = ESSAY_TOC,
    )
}

pub fn writeup_article(title: &str, date_long: &str, date_iso: &str, body_html: &str) -> String {
    format!(
        r#"<article class="essay">
  <header class="essay-header">
    <h1 class="essay-title">{title}</h1>
    <time class="essay-date" datetime="{date_iso}">{date_long}</time>
  </header>

  <div class="essay-body">
{body}
  </div>

  <p class="essay-back"><a href="/">Back to reading</a></p>
</article>"#,
        title = title,
        date_iso = date_iso,
        date_long = date_long,
        body = body_html,
    )
}

pub fn project_card(
    title: &str,
    affil: &str,
    desc: &str,
    link: &Option<String>,
    link_text: &str,
) -> String {
    let link_html = match link {
        Some(l) => format!(
            "<p class=\"project-link\"><a class=\"btn\" href=\"{}\">{}</a></p>",
            l, link_text
        ),
        None => String::new(),
    };
    format!(
        r#"<div class="project">
  <div class="project-body">
    <h3>{title}</h3>
    <p class="affil">{affil}</p>
    <p class="desc">{desc}</p>
    {link}
  </div>
</div>"#,
        title = title,
        affil = affil,
        desc = desc,
        link = link_html,
    )
}
