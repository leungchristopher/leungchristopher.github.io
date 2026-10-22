# Browser authoring

From the repository root, run:

```sh
./edit
```

Your browser opens **http://127.0.0.1:8765/notes/** automatically. If port 8765 is occupied, a free port is chosen and the actual URL is printed and opened. Click **Add / edit notes**.
Python 3.9+ and Rust/Cargo are required; there are no extra packages to install.

- Click **New folder** to create a folder, then click its name to enter it. Empty folders are kept in Git.
- Click **New note**, give it a title, and write. Formatting buttons insert headings, lists, links, code blocks and inline/display TeX at your cursor. Select text first to wrap it.
- The live preview uses the site's Rust Markdown renderer and its existing MathJax setup. MathJax needs internet access.
- Click **Save note** or press **⌘/Ctrl+S**. Files are written under `content/notes/`, and the local site rebuilds immediately. **View note** opens the result.
- Existing notes can be opened from the editor's file list or **Edit this note** on their local page. Unsaved changes trigger a warning before leaving; a stale tab cannot overwrite a newer saved revision.
- The full Markdown file is editable, including optional existing front matter. New notes use a `# Title` heading. Folder names are displayed as entered; note filenames are derived from titles. Repeated titles in a folder are rejected on save to protect existing files: open the existing note or choose a different title.

Visitors see `/notes/`, with one directory level per page. Subfolders must be opened to see their contents. All notes in this directory are public when published.

Commit the Markdown files and push as usual to publish through the existing GitHub Pages workflow. Saving locally does not commit, push, or deploy. The editor is served only by this loopback server and is not included in the published site. Stop the server with Ctrl-C.

This is a Markdown editor with a rendered preview, not a rich-text editor. It uses the site's existing Markdown subset (headings, emphasis, links, lists, quotes, fenced code, raw HTML and math). Move or rename files in your filesystem when needed; the next save or server restart rebuilds the site.

Checks:

```sh
cargo test --manifest-path ssg/Cargo.toml
python3 -m unittest discover -s tools -p 'test_*.py'
```
