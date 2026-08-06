# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

Personal site at https://leungchristopher.com — a static site built by a hand-written Rust static site generator in `ssg/`. Content is markdown + data files in `content/`; output goes to `_site/` (gitignored). No Node, no npm, no template engine, no markdown crate.

## Commands

```sh
cd ssg && cargo run              # build the site into ../_site
cd ssg && cargo run --release    # what CI runs
cd ssg && cargo check            # fast type check
python3 -m http.server -d _site 8000   # preview (paths are root-absolute, so serve _site as root)
```

There are no tests. The build is the check: it panics loudly (every `fs` call `.unwrap()`s) if content is malformed or a file is missing.

Deploy: push to `master`. `.github/workflows/deploy.yml` runs `cargo run --release` and publishes `_site` to GitHub Pages. The build wipes and recreates `_site` each time, and re-emits `CNAME` — anything not written by `main.rs` does not survive a deploy.

Photo hook: `git config core.hooksPath .githooks` (already set locally). The pre-commit hook strips EXIF/GPS from staged files under `assets/photos/` and requires `exiftool`.

## Architecture

`ssg/` has **zero dependencies**, deliberately (see the comment in `Cargo.toml`). Front matter, YAML-ish data, markdown, dates, and CSS minification are all hand-rolled. Don't add a crate to solve a parsing problem — extend the relevant module instead.

`ssg/src/main.rs` is the whole pipeline, top to bottom, one block per output: home, about, links, writeups, 404, essays + index + Atom feed, dlog entries + index, sitemap, assets. Adding a page type means adding a block here. Root is resolved from `env!("CARGO_MANIFEST_DIR")/..`, so the build works from any cwd.

Module split:
- `frontmatter.rs` — `---` header parsing. Scalar `key: value` fields plus one special case: a `sessions:` block of `- {in: ..., out: ..., task: ..., tag: ...}` flow maps (dlog only).
- `markdown.rs` — two renderers. `render()` for essays/writeups/pages; `render_links()` for the links page only, where `##`/`###` become nested `<details>` groups instead of headings. Block-level parsing is line-oriented; `inline()` recurses.
- `templates.rs` — HTML as Rust `format!` strings. `page()` wraps everything (head, header, nav, footer, social from `config::SOCIAL`). Inlined JS lives here as `const` string literals: `TOC_SCRIPT` (auto table of contents), `MATHJAX` (CDN, only when a page sets `math: true`), `tooltip_script()` (dlog charts).
- `dlog.rs` — stats over dlog entries: total minutes, streak, % of days documented, minutes-per-tag, daily series. Session length = `out - in`, minute arithmetic only.
- `dates.rs` — `Date` (Y/M/D) with Howard Hinnant's civil↔days conversions. Sorting and diffing are done in days-since-epoch.
- `projects.rs` — parses `content/data/projects.yml` (a hand-rolled `- key: value` record list, not real YAML).
- `minify.rs` — conservative CSS minifier; intentionally never touches spacing that isn't adjacent to `{ } : ; ,` so `calc()` survives.
- `config.rs` — site title, name, description, URL, `DLOG_START`, social links. Change site-wide constants here.

## Markdown dialect

The renderer supports a deliberate subset. Things it does **not** support (silently render wrong): nested lists, tables, inline HTML mid-paragraph, setext headings, images via `![]()`, hard line breaks. Blocks are separated by blank lines; a paragraph run ends at a blank line or a block-starting token.

Custom syntax beyond CommonMark basics:
- `{{name}}` on its own line — a directive, replaced by pre-rendered HTML passed in from `main.rs`. Currently `{{projects:current}}` (index) and `{{projects:all}}` (about). Unknown names render as nothing.
- `!progress[Title](X/Y)` — progress bar block (used in `content/pages/dlog.md`).
- `{: .classname}` on the line after a paragraph — Kramdown-style IAL, sets the `<p>` class (e.g. `{: .lede}`).
- `[[slug]]` at the end of a links-page list item — appends a `[writeup]` link to `/writeups/<slug>/`.
- `$...$` / `$$...$$` pass through untouched for MathJax; the page's front matter must set `math: true` or MathJax won't load.
- Raw HTML lines starting with `<` pass through verbatim until a blank line.

## Content model

- `content/essays/<slug>.md` → `/essays/<slug>/`. Front matter: `title`, `date` (YYYY-MM-DD), optional `math: true`, optional `draft: true`. Drafts still build a page but are excluded from the index, feed, and sitemap.
- `content/writeups/<slug>.md` → `/writeups/<slug>/`. Same front matter, but intentionally unlisted: no index, no feed, no sitemap entry — reachable only via `[[slug]]` on the links page.
- `content/dlog/YYYY-MM-DD.md` → `/dlog/<date>/`. Filename is the slug and the date fallback. Front matter: `title`, `goal`, `summary`, `sessions`. Tags come only from session `tag:` fields — they drive the tag cloud, per-tag minutes, and the `data-tags` filter attribute on the index.
- `content/pages/{index,about,links,dlog}.md` — each is wired to a specific block in `main.rs`; adding a file here does nothing on its own.
- `content/data/projects.yml` — project cards. `current: true` puts a card on the home page; all cards appear on `/about/`.
- `assets/images/projects/` — a `.webp` card image is auto-paired with a same-named `.avif` in a `<picture>`; keep both files. `.svg` is used as-is.

`assets/css/style.css` is the single stylesheet (~770 lines, light theme, CSS custom properties in `:root`). Class names are produced by `templates.rs` and `markdown.rs` — changing a class means changing both.
