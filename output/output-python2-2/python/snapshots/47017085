#!/usr/bin/env python

import os
import tempfile
import unittest
import shutil

import dejumble.filters.completedirectory
from dejumble.filters.completedirectory import *

class CompleteDirectoryFileListFilterTestCase(unittest.TestCase):
    def setUp(self):
        self.original_dir = tempfile.mkdtemp()
        self.mount_dir = tempfile.mkdtemp()
        os.chdir(self.mount_dir)
        
    def testfilelist(self):
        original_file = tempfile.mkstemp('', '', self.original_dir)
        filter = CompleteDirectoryFileListFilter('', self.original_dir)
        filelist = list(filter.filelist())
        self.assertEqual(len(filelist), 1)
        self.assertEqual(filelist[0], original_file[1])
        
    def tearDown(self):
        shutil.rmtree(self.original_dir)
        shutil.rmtree(self.mount_dir)
        