#!/usr/bin/env python

import os
import tempfile
import unittest
import shutil

import dejumble.util
from dejumble.util import *
import dejumble.test.base
from dejumble.test.base import *
import dejumble.filters.originaldirectory
from dejumble.filters.originaldirectory import *

class OriginalDirectoryFileListFilterTestCase(BaseFileListFilterTestCase):
    def testfilelist(self):
        original_file1 = tempfile.mkstemp('', '', self.mount_dir)
        original_file2 = tempfile.mkstemp('', '', self.mount_dir)
        original_subdir = tempfile.mkdtemp('', '', self.mount_dir)
        original_file3 = tempfile.mkstemp('', '', original_subdir)

        filter = OriginalDirectoryFileListFilter()
        filelist = list(filter.filelist())

        self.assertEqual(len(filelist), 3)
        self.assertTrue('.%s' % removeroot(original_file1[1], self.mount_dir) in filelist)
        self.assertTrue('.%s' % removeroot(original_file2[1], self.mount_dir) in filelist)
        self.assertTrue('.%s' % removeroot(original_file3[1], self.mount_dir) in filelist)
