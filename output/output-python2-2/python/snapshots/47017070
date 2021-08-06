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
        original_file = tempfile.mkstemp('', '', self.original_dir)
        filter = CompleteDirectoryFileListFilter('', self.original_dir)
        filelist = list(filter.filelist())
        self.assertEqual(len(filelist), 1)
        self.assertEqual(filelist[0], original_file[1])

