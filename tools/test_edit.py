import errno
import json
from pathlib import Path
import tempfile
import threading
import unittest
from urllib.error import HTTPError
from urllib.request import Request, urlopen
from unittest.mock import patch
import edit


class PortTest(unittest.TestCase):
    def test_default_and_occupied_port(self):
        with patch.object(edit, 'HTTPServer') as server:
            self.assertIs(edit.create_server(), server.return_value)
            server.assert_called_once_with(('127.0.0.1', 8765), edit.Handler)
        fallback = object()
        with patch.object(edit, 'HTTPServer', side_effect=[OSError(errno.EADDRINUSE, 'Busy'), fallback]) as server:
            self.assertIs(edit.create_server(), fallback)
            self.assertEqual(server.call_args_list[0].args[0], ('127.0.0.1', 8765))
            self.assertEqual(server.call_args_list[1].args[0], ('127.0.0.1', 0))
        with patch.object(edit, 'HTTPServer', side_effect=PermissionError(errno.EACCES, 'Denied')) as server:
            with self.assertRaises(PermissionError):
                edit.create_server()
            self.assertEqual(server.call_count, 1)


class EditorTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.notes = patch.object(edit, 'NOTES', Path(self.temp.name))
        self.notes.start()
        self.build = patch.object(edit, 'build')
        self.build.start()
        self.server = edit.HTTPServer(('127.0.0.1', 0), edit.Handler)
        self.thread = threading.Thread(target=self.server.serve_forever)
        self.thread.start()
        self.base = f'http://127.0.0.1:{self.server.server_port}'

    def tearDown(self):
        self.server.shutdown()
        self.thread.join()
        self.server.server_close()
        self.build.stop()
        self.notes.stop()
        self.temp.cleanup()

    def post(self, route, data, token=edit.TOKEN, origin=None):
        headers = {'Content-Type': 'application/json', 'X-Editor-Token': token}
        if origin:
            headers['Origin'] = origin
        request = Request(self.base + route, json.dumps(data).encode(), headers)
        try:
            with urlopen(request) as response:
                return response.status, json.load(response)
        except HTTPError as error:
            return error.code, json.load(error)

    def test_save_folders_conflicts_and_boundaries(self):
        self.assertEqual(self.post('/api/folder', {'path': 'Math'})[0], 200)
        self.assertTrue((edit.NOTES / 'Math/.gitkeep').exists())
        data = {'path': 'Math/test.md', 'source': '# Test\n\n$x^2$\n', 'revision': None}
        code, saved = self.post('/api/save', data)
        self.assertEqual(code, 200)
        self.assertEqual((edit.NOTES / data['path']).read_text(), data['source'])
        self.assertEqual(self.post('/api/save', data)[0], 409)
        data['revision'] = saved['revision']
        data['source'] += 'Updated'
        self.assertEqual(self.post('/api/save', data)[0], 200)
        self.assertEqual(self.post('/api/save', data, token='wrong')[0], 403)
        self.assertEqual(self.post('/api/save', data, origin='https://example.com')[0], 403)
        for path in ['../escape.md', '/tmp/escape.md', '.hidden.md', 'Math/../../escape.md']:
            self.assertEqual(self.post('/api/save', {**data, 'path': path})[0], 400)
        (edit.NOTES / 'link').symlink_to('/tmp', target_is_directory=True)
        self.assertEqual(self.post('/api/save', {**data, 'path': 'link/escape.md'})[0], 400)

    def test_saved_file_survives_build_failure(self):
        with patch.object(edit, 'build', side_effect=OSError('build failed')):
            code, result = self.post('/api/save', {'path': 'saved.md', 'source': 'Kept', 'revision': None})
        self.assertEqual(code, 200)
        self.assertIn('warning', result)
        self.assertEqual((edit.NOTES / 'saved.md').read_text(), 'Kept')


if __name__ == '__main__':
    unittest.main()
