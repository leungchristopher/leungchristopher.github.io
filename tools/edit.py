#!/usr/bin/env python3
"""Local browser authoring. Run: python3 tools/edit.py"""
import errno
import hashlib
import json
import os
from pathlib import Path
import secrets
import subprocess
import tempfile
import webbrowser
from http.server import HTTPServer, SimpleHTTPRequestHandler
from urllib.parse import parse_qs, unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]
NOTES = ROOT / 'content/notes'
BINARY = ROOT / 'ssg/target/release/ssg'
EDITOR = ROOT / 'tools/editor'
TOKEN = secrets.token_urlsafe(32)
LIMIT = 1024 * 1024


def note_path(value):
    if not isinstance(value, str) or '\\' in value:
        raise ValueError('Invalid path')
    parts = value.split('/') if value else []
    if any(not part or part.startswith('.') or any(ord(c) < 32 for c in part) for part in parts):
        raise ValueError('Invalid path')
    path = NOTES.joinpath(*parts)
    if not path.resolve().is_relative_to(NOTES.resolve()):
        raise ValueError('Path must stay inside content/notes')
    if any(NOTES.joinpath(*parts[:i]).is_symlink() for i in range(len(parts) + 1)):
        raise ValueError('Symlinks cannot be edited')
    return path


def revision(path):
    return hashlib.sha256(path.read_bytes()).hexdigest() if path.is_file() else None


def build():
    subprocess.run([str(BINARY)], cwd=ROOT, check=True, capture_output=True, timeout=30)


class Handler(SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(ROOT / '_site'), **kwargs)

    def respond(self, body, status=200, kind='application/json'):
        data = json.dumps(body).encode() if kind == 'application/json' else body.encode()
        self.send_response(status)
        self.send_header('Content-Type', kind + '; charset=utf-8')
        self.send_header('Content-Length', str(len(data)))
        self.send_header('Cache-Control', 'no-store')
        self.send_header('X-Content-Type-Options', 'nosniff')
        self.end_headers()
        self.wfile.write(data)

    def local_request(self):
        expected = f'127.0.0.1:{self.server.server_port}'
        return self.headers.get('Host') == expected and self.headers.get('Origin', f'http://{expected}') == f'http://{expected}'

    def do_GET(self):
        if not self.local_request():
            return self.respond({'error': 'Use the local URL printed in the terminal.'}, 403)
        url = urlsplit(self.path)
        try:
            if url.path in ('/edit', '/edit/'):
                return self.respond((EDITOR / 'index.html').read_text().replace('__TOKEN__', TOKEN), kind='text/html')
            if url.path in ('/edit/editor.js', '/edit/editor.css'):
                file = EDITOR / url.path.rsplit('/', 1)[1]
                return self.respond(file.read_text(), kind='text/javascript' if file.suffix == '.js' else 'text/css')
            if url.path == '/api/note':
                path = note_path(parse_qs(url.query).get('path', [''])[0])
                if path.suffix != '.md' or not path.is_file():
                    raise ValueError('Note not found')
                return self.respond({'source': path.read_text(), 'revision': revision(path)})
            if url.path == '/api/folder':
                path = note_path(parse_qs(url.query).get('path', [''])[0].rstrip('/'))
                entries = [{'name': p.name, 'folder': p.is_dir()} for p in sorted(path.iterdir(), key=lambda p: (not p.is_dir(), p.name)) if not p.name.startswith('.') and not p.is_symlink() and (p.is_dir() or p.suffix == '.md')]
                return self.respond(entries)
            # The authoring link exists only in responses from this local server.
            target = (ROOT / '_site' / unquote(url.path).lstrip('/')).resolve()
            if not target.is_relative_to((ROOT / '_site').resolve()):
                return self.respond({'error': 'Invalid path'}, 403)
            if target.is_dir():
                target /= 'index.html'
            if target.suffix == '.html' and target.is_file():
                link = '''<script>
const note = document.querySelector('[data-note-path]');
const folder = document.querySelector('[data-folder-path]');
const edit = document.createElement('a');
edit.href = '/edit/?' + (note ? 'file=' + encodeURIComponent(note.dataset.notePath) : 'folder=' + encodeURIComponent(folder ? folder.dataset.folderPath : ''));
edit.textContent = note ? 'Edit this note' : 'Add / edit notes';
const bar = document.createElement('p'); bar.append(edit); document.querySelector('main').prepend(bar);
</script>'''
                return self.respond(target.read_text().replace('</body>', link + '</body>'), kind='text/html')
            return super().do_GET()
        except (ValueError, OSError) as error:
            self.respond({'error': str(error)}, 400)

    def do_POST(self):
        if not self.local_request() or self.headers.get('X-Editor-Token') != TOKEN:
            return self.respond({'error': 'Invalid editor session. Reload the editor.'}, 403)
        try:
            size = int(self.headers.get('Content-Length', '0'))
            if not 0 < size <= LIMIT:
                raise ValueError('Notes must be smaller than 1 MB')
            data = json.loads(self.rfile.read(size))
            if not isinstance(data, dict):
                raise ValueError('Expected an object')
            route = urlsplit(self.path).path
            if route == '/api/preview':
                source = data.get('source')
                if not isinstance(source, str):
                    raise ValueError('Missing Markdown')
                preview = subprocess.run([str(BINARY), '--preview'], input=source, text=True, capture_output=True, check=True, timeout=10)
                return self.respond({'html': preview.stdout})
            path = note_path(data.get('path'))
            if route == '/api/folder':
                if path == NOTES or not path.parent.is_dir():
                    raise ValueError('Choose a name inside an existing folder')
                path.mkdir()
                (path / '.gitkeep').touch()
            elif route == '/api/save':
                if path.suffix != '.md' or not path.parent.is_dir():
                    raise ValueError('Choose a .md file inside an existing folder')
                if revision(path) != data.get('revision'):
                    return self.respond({'error': 'This note changed on disk or in another tab. Copy your text before reloading; your changes have not overwritten it.'}, 409)
                source = data.get('source')
                if not isinstance(source, str):
                    raise ValueError('Missing Markdown')
                # Replace atomically so a failed write cannot leave a partial note.
                fd, temporary = tempfile.mkstemp(dir=path.parent, prefix='.note-')
                try:
                    with os.fdopen(fd, 'w') as file:
                        file.write(source)
                        file.flush()
                        os.fsync(file.fileno())
                    os.replace(temporary, path)
                finally:
                    if os.path.exists(temporary):
                        os.unlink(temporary)
            else:
                return self.respond({'error': 'Unknown endpoint'}, 404)
            result = {'revision': revision(path)}
            try:
                build()
            except (subprocess.SubprocessError, OSError):
                result['warning'] = 'Saved to disk, but the site build failed. Check the content and run cargo run --manifest-path ssg/Cargo.toml.'
            self.respond(result)
        except (ValueError, OSError, subprocess.SubprocessError) as error:
            self.respond({'error': str(error)}, 400)


def create_server():
    try:
        return HTTPServer(('127.0.0.1', 8765), Handler)
    except OSError as error:
        if error.errno != errno.EADDRINUSE:
            raise
        return HTTPServer(('127.0.0.1', 0), Handler)


if __name__ == '__main__':
    subprocess.run(['cargo', 'build', '--release', '--manifest-path', str(ROOT / 'ssg/Cargo.toml')], check=True)
    NOTES.mkdir(parents=True, exist_ok=True)
    build()
    server = create_server()
    url = f'http://127.0.0.1:{server.server_port}/notes/'
    print(f'Open {url} — Ctrl-C to stop.', flush=True)
    webbrowser.open(url)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        server.server_close()
