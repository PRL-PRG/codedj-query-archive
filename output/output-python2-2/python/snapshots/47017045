#!/usr/bin/env python

import os
import tempfile
import unittest
import shutil

import dejumble.test.base
from dejumble.test.base import *
import dejumble.filters.completedirectory
from dejumble.filters.completedirectory import *

class CompleteDirectoryFileListFilterTestCase(BaseFileListFilterTestCase):
    def testfilelist(self):
        original_file1 = tempfile.mkstemp('', '', self.original_dir)
        original_file2 = tempfile.mkstemp('', '', self.original_dir)
        original_subdir = tempfile.mkdtemp('', '', self.original_dir)
        original_file3 = tempfile.mkstemp('', '', original_subdir)

        filter = CompleteDirectoryFileListFilter('', self.original_dir)
        filelist = list(filter.filelist())

        self.assertEqual(len(filelist), 4)
        self.assertTrue(original_file1[1] in filelist)
        self.assertTrue(original_file2[1] in filelist)
        self.assertTrue(original_subdir in filelist)
        self.assertTrue(original_file3[1] in filelist)

