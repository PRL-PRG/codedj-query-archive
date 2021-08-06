#!/usr/bin/env python

import unittest

import dejumble.test.base
from dejumble.test.base import *
import dejumble.filters.shell
from dejumble.filters.shell import *


class ShellFileListFilterTestCase(BaseFileListFilterTestCase):
    def testfilelist(self):
        filelist = list(ShellFileListFilter('echo /dev/null', '/').filelist())
        self.assertEqual(len(filelist), 1)
        self.assertEqual(filelist[0], '/dev/null')
