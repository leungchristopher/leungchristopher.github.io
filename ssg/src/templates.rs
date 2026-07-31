use crate::config::*;

pub struct Page {
    pub title: String,
    pub path: String,
    pub body_class: Option<String>,
    pub math: bool,
    pub content: String,
}

const TOC_SCRIPT: &str = r#"<nav class="toc" aria-label="On this page" hidden>
  <ul class="toc-list"></ul>
</nav>
<script>
(function () {
  var body = document.querySelector('.essay-body, .home, .links-page');
  var nav  = document.querySelector('.toc');
  if (!body || !nav) return;

  var heads = Array.prototype.filter.call(
    body.querySelectorAll('h1, h2, h3'),
    function (h) { return h.textContent.trim().length; }
  );
  if (heads.length < 2) return;

  var list = nav.querySelector('.toc-list');
  var links = heads.map(function (h, i) {
    if (!h.id) h.id = 'section-' + i;
    var li = document.createElement('li');
    li.className = 'toc-item toc-' + h.tagName.toLowerCase();
    var a = document.createElement('a');
    a.href = '#' + h.id;
    a.innerHTML = '<span class="toc-dash" aria-hidden="true"></span>' +
                  '<span class="toc-labelwrap"><span class="toc-label"></span></span>';
    a.querySelector('.toc-label').textContent = h.textContent.trim();
    li.appendChild(a);
    list.appendChild(li);
    return a;
  });
  nav.hidden = false;

  var reduced = window.matchMedia('(prefers-reduced-motion: reduce)');

  var animating = 0;
  function glideTo(top) {
    cancelAnimationFrame(animating);
    var start = window.pageYOffset;
    var dist  = top - start;
    if (!dist) return;
    var dur = Math.min(1100, 320 + Math.sqrt(Math.abs(dist)) * 26);
    var t0 = null;
    function step(now) {
      if (t0 === null) t0 = now;
      var p = Math.min(1, (now - t0) / dur);
      var e = 1 - Math.pow(1 - p, 4);
      window.scrollTo(0, start + dist * e);
      if (p < 1) animating = requestAnimationFrame(step);
    }
    animating = requestAnimationFrame(step);
  }

  links.forEach(function (a, i) {
    a.addEventListener('click', function (ev) {
      ev.preventDefault();
      var top = heads[i].getBoundingClientRect().top + window.pageYOffset - 90;
      top = Math.max(0, Math.min(top, document.documentElement.scrollHeight - window.innerHeight));
      if (reduced.matches) window.scrollTo(0, top);
      else glideTo(top);
      history.replaceState(null, '', '#' + heads[i].id);
      setActive(i);
    });
  });

  var current = -1;
  function setActive(i) {
    if (i === current) return;
    if (links[current]) links[current].classList.remove('is-active');
    if (links[i]) links[i].classList.add('is-active');
    current = i;
  }

  var ticking = false;
  function sync() {
    ticking = false;
    var line = window.pageYOffset + window.innerHeight * 0.28;
    var i = 0;
    for (var k = 0; k < heads.length; k++) {
      if (heads[k].getBoundingClientRect().top + window.pageYOffset <= line) i = k;
    }
    setActive(i);
  }
  window.addEventListener('scroll', function () {
    if (!ticking) { ticking = true; requestAnimationFrame(sync); }
  }, { passive: true });
  window.addEventListener('resize', sync, { passive: true });
  sync();
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
    let year = crate::dates::Date::today().y;

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{title_tag}</title>
  <meta name="description" content="{description}">
  <link rel="canonical" href="{url}{path}">
  <link rel="stylesheet" href="/assets/css/style.css">
  <meta name="view-transition" content="same-origin">
  <link type="application/atom+xml" rel="alternate" href="/feed/essays.xml" title="{site_title} — Essays">
  {mathjax}</head>
<body{body_class}>
  <div class="wrap">
    <header class="site-header">
      <a class="site-name" href="/">{name}</a>
      <nav class="site-nav">
        <a href="/about/">About</a>
        <a href="/essays/">Essays</a>
        <a href="/dlog/">Log</a>
        <a href="/links/">Links</a>
      </nav>
    </header>

    <main class="content">
      {content}
    </main>

    <footer class="site-footer">
      <div class="social">
        {social}
      </div>
      <div class="copyright">
        <span class="mark">© {year} {name}</span>
        <a class="answer" href="https://mailtoll.app/chris" rel="noopener" hidden>Don't Panic — but it'll cost you →</a>
      </div>
    </footer>
  </div>

  <script>
    (function () {{
      var mark = document.querySelector('.copyright .mark');
      var answer = document.querySelector('.copyright .answer');
      if (!mark || !answer) return;
      var clicks = 0, idle;
      mark.addEventListener('click', function () {{
        clicks++;
        clearTimeout(idle);
        idle = setTimeout(function () {{ clicks = 0; mark.style.opacity = ''; }}, 3000);
        if (clicks > 32) mark.style.opacity = Math.max(0, 1 - (clicks - 32) / 10);
        if (clicks === 42) {{
          mark.hidden = true;
          answer.hidden = false;
        }}
      }});
    }})();
  </script>
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
        name = NAME,
        social = social,
        year = year,
        content = p.content,
    )
}

pub fn links_wrap(body_html: &str) -> String {
    format!("{}\n\n{}", body_html, TOC_SCRIPT)
}

pub fn home_wrap(body_html: &str, with_toc: bool) -> String {
    if with_toc {
        format!("<div class=\"home\">\n{}</div>\n\n{}", body_html, TOC_SCRIPT)
    } else {
        format!("<div class=\"home\">\n{}</div>\n", body_html)
    }
}

pub fn essay_article(title: &str, date_long: &str, date_iso: &str, body_html: &str) -> String {
    format!(
        r#"<article class="essay">
  <header class="essay-header">
    <h1 class="essay-title">{title}</h1>
    <time class="essay-date" datetime="{date_iso}">{date_long}</time>
  </header>

  <div class="essay-body">
{body}
  </div>

  <p class="essay-back"><a href="/essays/">All essays</a></p>
</article>

{toc}"#,
        title = title,
        date_iso = date_iso,
        date_long = date_long,
        body = body_html,
        toc = TOC_SCRIPT,
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

  <p class="essay-back"><a href="/links/">Back to links</a></p>
</article>

{toc}"#,
        title = title,
        date_iso = date_iso,
        date_long = date_long,
        body = body_html,
        toc = TOC_SCRIPT,
    )
}

pub fn dlog_entry_article(
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
        r#"<article class="dlog-entry">
  <header class="entry-header">
    <h1 class="entry-title">{title}</h1>
    <time class="entry-date" datetime="{date_iso}">{date_long}</time>
  </header>

  <div class="entry-body">
    {goal}{summary}{sessions}
{body}
    {tags}
  </div>

  <p class="entry-back"><a href="/dlog/">Back to log</a></p>
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
    img: &str,
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
    // SVGs are already tiny vectors; only raster (webp) images get an AVIF
    // source with the webp kept as the <img> fallback for browsers without
    // AVIF support (older Safari/iOS, dead browsers) — zero compatibility
    // loss, since those browsers just skip straight to the <img>.
    let figure = if let Some(base) = img.strip_suffix(".webp") {
        format!(
            r#"<picture><source srcset="{base}.avif" type="image/avif"><img src="{img}" alt="{title}" width="380" height="380" loading="lazy" decoding="async"></picture>"#,
            base = base,
            img = img,
            title = title,
        )
    } else {
        format!(
            r#"<img src="{img}" alt="{title}" width="380" height="380" loading="lazy" decoding="async">"#,
            img = img,
            title = title,
        )
    };
    format!(
        r#"<div class="project">
  <div class="project-body">
    <h3>{title}</h3>
    <p class="affil">{affil}</p>
    <p class="desc">{desc}</p>
    {link}
  </div>
  <div class="project-figure">{figure}</div>
</div>"#,
        title = title,
        affil = affil,
        desc = desc,
        link = link_html,
        figure = figure,
    )
}

fn hm(mins: i64) -> String {
    format!("{} hr {} min", mins / 60, mins % 60)
}

pub fn hours_chart(daily: &[(crate::dates::Date, i64)]) -> String {
    let n = daily.len();
    let max_mins = daily.iter().map(|(_, m)| *m).max().unwrap_or(0).max(1) as f64;

    let window = daily.len().min(7);
    let avg_7_mins = if window > 0 {
        daily[daily.len() - window..].iter().map(|(_, m)| *m).sum::<i64>() / window as i64
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
  <p class="hours-chart-avg">Last 7 days: {avg_hm}/day avg</p>
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
        n = n, cols = cols, labels = labels, content_pct = n * 10, avg_hm = hm(avg_7_mins)
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
