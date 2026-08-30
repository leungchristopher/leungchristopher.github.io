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

pub fn log_entry_article(
    title: &str,
    date_long: &str,
    date_iso: &str,
    goal: &str,
    summary: &str,
    sessions_table: &str,
    body_html: &str,
    tags_html: &str,
) -> String {
    let goal_html = if goal.is_empty() {
        String::new()
    } else {
        format!("<p><strong>Goal:</strong> {}</p>\n", goal)
    };
    let summary_html = if summary.is_empty() {
        String::new()
    } else {
        format!("<p><strong>Summary:</strong> {}</p>\n", summary)
    };
    format!(
        r#"<article class="log-entry">
  <header class="entry-header">
    <h1 class="entry-title">{title}</h1>
    <time class="entry-date" datetime="{date_iso}">{date_long}</time>
  </header>

  <div class="entry-body">
    {goal}{summary}{sessions}
{body}
    {tags}
  </div>

  <p class="entry-back"><a href="/">Back to log</a></p>
</article>
"#,
        title = title,
        date_iso = date_iso,
        date_long = date_long,
        goal = goal_html,
        summary = summary_html,
        sessions = sessions_table,
        body = body_html,
        tags = tags_html,
    )
}

pub fn sessions_table(sessions: &[crate::frontmatter::Session]) -> String {
    if sessions.is_empty() {
        return String::new();
    }
    let rows: String = sessions
        .iter()
        .map(|s| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                s.time_in, s.time_out, s.task, s.tags.join(", ")
            )
        })
        .collect();
    format!(
        "<table class=\"sessions\">\n<thead><tr><th>In</th><th>Out</th><th>Task</th><th>Tag</th></tr></thead>\n<tbody>\n{}\n</tbody>\n</table>",
        rows
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

fn hm(mins: i64) -> String {
    format!("{} hr {} min", mins / 60, mins % 60)
}

pub fn hours_chart(daily: &[(crate::dates::Date, i64)]) -> String {
    let n = daily.len();
    let max_mins = daily.iter().map(|(_, m)| *m).max().unwrap_or(0).max(1) as f64;

    let window = daily.len().min(10);
    let recent = &daily[daily.len() - window..];
    let weekdays: Vec<i64> = recent.iter().filter(|(d, _)| d.is_weekday()).map(|(_, m)| *m).collect();
    let avg_weekday_mins = if !weekdays.is_empty() {
        weekdays.iter().sum::<i64>() / weekdays.len() as i64
    } else {
        0
    };

    let cols: String = daily
        .iter()
        .map(|(date, mins)| {
            let pct = (*mins as f64 / max_mins) * 100.0;
            format!(
                "<div class=\"hours-col\"><div class=\"hours-bar-fill\" data-tooltip=\"{day}: {hm}\" style=\"height: {pct:.2}%\"></div></div>",
                day = date.d, hm = hm(*mins), pct = pct
            )
        })
        .collect::<Vec<_>>()
        .join("\n    ");

    let labels: String = daily
        .iter()
        .map(|(date, _)| format!("<div class=\"hours-label\">{day}</div>", day = date.d))
        .collect::<Vec<_>>()
        .join("\n    ");

    format!(
        r#"<div class="hours-chart" role="img" aria-label="Hours worked per day">
  <p class="hours-chart-avg">Last 10 days, weekdays: {avg_hm}/day avg</p>
  <div class="hours-chart-scroll">
    <div class="hours-tracks" style="width: {content_pct}%; --n: {n}">
    {cols}
    </div>
    <div class="hours-labels" style="width: {content_pct}%; --n: {n}">
    {labels}
    </div>
  </div>
</div>
<script>
document.addEventListener('DOMContentLoaded', function () {{
  var scroll = document.querySelector('.hours-chart-scroll');
  if (!scroll) return;
  scroll.scrollLeft = scroll.scrollWidth;
}});
</script>"#,
        n = n, cols = cols, labels = labels, content_pct = n * 10, avg_hm = hm(avg_weekday_mins)
    )
}

pub fn tag_cloud(tags: &[(String, i64, f64, &'static str)]) -> String {
    let buttons: String = tags
        .iter()
        .map(|(tag, mins, size, color)| {
            format!(
                "<button type=\"button\" class=\"tag-cloud-item\" data-tag=\"{tag}\" style=\"font-size: {size}em; color: {color}\" data-tooltip=\"{tag}: {hm}\">{tag}</button>",
                tag = tag, size = size, color = color, hm = hm(*mins)
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    format!(
        r#"<div class="tag-cloud" role="group" aria-label="Filter by tag">
  {buttons}
</div>

<script>
document.addEventListener('DOMContentLoaded', function () {{
  var cloud = document.querySelector('.tag-cloud');
  var list = document.querySelector('.entry-list');
  if (!cloud || !list) return;
  var buttons = cloud.querySelectorAll('.tag-cloud-item');
  var items = list.querySelectorAll('.entry-item');
  var active = null;

  function apply() {{
    buttons.forEach(function (b) {{
      b.classList.toggle('is-active', b.dataset.tag === active);
    }});
    items.forEach(function (li) {{
      var tags = JSON.parse(li.dataset.tags || '[]');
      li.hidden = active !== null && tags.indexOf(active) === -1;
    }});
  }}

  buttons.forEach(function (b) {{
    b.addEventListener('click', function () {{
      active = active === b.dataset.tag ? null : b.dataset.tag;
      apply();
    }});
  }});
}});
</script>"#,
        buttons = buttons
    )
}

/// Shared custom tooltip for [data-tooltip] elements (tag cloud + hours chart).
pub fn tooltip_script() -> &'static str {
    r#"<script>
document.addEventListener('DOMContentLoaded', function () {
  var tip = document.createElement('div');
  tip.className = 'chart-tooltip';
  tip.hidden = true;
  document.body.appendChild(tip);

  function show(e) {
    var el = e.target.closest('[data-tooltip]');
    if (!el) return;
    tip.textContent = el.dataset.tooltip;
    tip.hidden = false;
    move(e);
  }
  function move(e) {
    if (tip.hidden) return;
    tip.style.left = (e.clientX + 12) + 'px';
    tip.style.top = (e.clientY + 12) + 'px';
  }
  function hide(e) {
    if (!e.target.closest('[data-tooltip]')) return;
    tip.hidden = true;
  }

  document.addEventListener('mouseover', show);
  document.addEventListener('mousemove', move);
  document.addEventListener('mouseout', hide);
});
</script>"#
}
