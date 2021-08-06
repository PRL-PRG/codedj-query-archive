import unittest
import tempfile
import shutil
import os


class BaseFileListFilterTestCase(unittest.TestCase):

    def setUp(self):
        self.original_dir = tempfile.mkdtemp()
        self.mount_dir = tempfile.mkdtemp()
        os.chdir(self.mount_dir)

    def tearDown(self):
        shutil.rmtree(self.original_dir)
        shutil.rmtree(self.mount_dir)
