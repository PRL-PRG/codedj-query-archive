#!/usr/bin/env python

import unittest

import dejumble.filters.shell
from dejumble.filters.shell import *

class ShellFileListFilterTestCase(unittest.TestCase):
    def testfilelist(self):
        filelist = list(ShellFileListFilter('echo /dev/null', '/').filelist())
        self.assertEqual(len(filelist), 1)
        self.assertEqual(filelist[0], '/dev/null')
