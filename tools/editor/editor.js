const $ = id => document.getElementById(id);
const params = new URLSearchParams(location.search);
let file = params.get('file');
let folder = file ? file.split('/').slice(0, -1).join('/') : (params.get('folder') || '').replace(/\/$/, '');
let revision = null;
let dirty = false;
let timer;
let previewRequest = 0;
const source = $('source');
const status = message => { $('status').textContent = message; };
const urlPath = path => path.split('/').map(encodeURIComponent).join('/');
async function api(path, data) {
  const response = await fetch(path, data === undefined ? {} : {
    method: 'POST', headers: {'Content-Type': 'application/json', 'X-Editor-Token': document.querySelector('[name="editor-token"]').content}, body: JSON.stringify(data)
  });
  const result = await response.json();
  if (!response.ok) throw new Error(result.error || 'Request failed');
  return result;
}
async function browse() {
  $('location').textContent = 'content/notes/' + (folder ? folder + '/' : '');
  const entries = await api('/api/folder?path=' + encodeURIComponent(folder));
  $('files').replaceChildren();
  function entry(name, href) {
    const li = document.createElement('li');
    const a = document.createElement('a'); a.textContent = name; a.href = href;
    li.append(a); $('files').append(li);
  }
  if (folder) entry('← Parent folder', '/edit/?folder=' + encodeURIComponent(folder.split('/').slice(0, -1).join('/')));
  for (const item of entries) {
    const path = (folder ? folder + '/' : '') + item.name;
    entry(item.name + (item.folder ? '/' : ''), '/edit/?' + (item.folder ? 'folder=' : 'file=') + encodeURIComponent(path));
  }
}
async function preview() {
  const request = ++previewRequest;
  try {
    const result = await api('/api/preview', {source: source.value});
    if (request === previewRequest) $('preview').srcdoc = result.html;
  } catch (error) { if (request === previewRequest) status('Preview failed: ' + error.message); }
}
function changed() {
  dirty = true; status('Unsaved changes');
  clearTimeout(timer); timer = setTimeout(preview, 400);
}
function showEditor() {
  $('workspace').hidden = false;
  $('filename').textContent = file;
  $('view').href = '/notes/' + urlPath(file.replace(/\.md$/, '.html'));
  $('view').hidden = revision === null;
  preview();
}
function insert(before, after, block = false, placeholder = 'text') {
  const start = source.selectionStart, end = source.selectionEnd;
  const selected = source.value.slice(start, end) || placeholder;
  const prefix = block && start > 0 ? '\n\n' : '';
  const suffix = block ? '\n\n' : '';
  source.setRangeText(prefix + before + selected + after + suffix, start, end, 'end');
  source.focus();
  source.setSelectionRange(start + prefix.length + before.length, start + prefix.length + before.length + selected.length);
  changed();
}
for (const button of document.querySelectorAll('[data-before]')) {
  button.onclick = () => insert(button.dataset.before, button.dataset.after, !!button.dataset.block, button.textContent.includes('TeX') ? 'x^2' : 'text');
}
$('code').onclick = () => {
  const language = prompt('Code language (optional):', 'python');
  if (language !== null) insert('```' + language.replace(/[^a-zA-Z0-9_+-]/g, '') + '\n', '\n```', true, 'print("Hello")');
};
$('link').onclick = () => {
  const url = prompt('Link URL:', 'https://');
  if (url) insert('[', '](' + url.replace(/[\n\r)]/g, '') + ')', false, 'link text');
};
$('new-note').onclick = () => {
  if (dirty && !confirm('Discard your unsaved changes?')) return;
  const title = prompt('Note title:');
  if (!title || !title.trim()) return;
  const slug = title.trim().toLowerCase().normalize('NFKD').replace(/[\u0300-\u036f]/g, '').replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '') || 'note';
  file = (folder ? folder + '/' : '') + slug + '.md';
  revision = null; source.value = '# ' + title.replace(/[\r\n]/g, ' ') + '\n\n';
  history.replaceState(null, '', '/edit/?file=' + encodeURIComponent(file));
  showEditor(); changed(); source.focus();
};
$('new-folder').onclick = async () => {
  const name = prompt('Folder name:');
  if (!name || !name.trim()) return;
  if (/[\/\\]/.test(name) || name.trim().startsWith('.')) return status('Use a folder name without slashes or a leading dot.');
  try {
    const result = await api('/api/folder', {path: (folder ? folder + '/' : '') + name.trim()});
    await browse(); status(result.warning || 'Folder created. Click its name to open it.');
  } catch (error) { status(error.message); }
};
async function save() {
  if (!file || $('save').disabled) return;
  $('save').disabled = true; $('new-note').disabled = true; status('Saving…');
  const savedSource = source.value;
  try {
    const result = await api('/api/save', {path: file, source: savedSource, revision});
    revision = result.revision; dirty = source.value !== savedSource;
    $('view').hidden = false;
    await browse();
    status(result.warning || (dirty ? 'Saved. Newer changes are still unsaved.' : 'Saved to content/notes/' + file));
  } catch (error) { status(error.message); }
  finally { $('save').disabled = false; $('new-note').disabled = false; }
}
$('save').onclick = save;
source.addEventListener('input', changed);
window.addEventListener('beforeunload', event => { if (dirty) { event.preventDefault(); event.returnValue = ''; } });
window.addEventListener('keydown', event => { if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === 's') { event.preventDefault(); save(); } });
(async () => {
  try {
    await browse();
    if (file) {
      const result = await api('/api/note?path=' + encodeURIComponent(file));
      source.value = result.source; revision = result.revision; showEditor();
    }
  } catch (error) { status(error.message); }
})();
