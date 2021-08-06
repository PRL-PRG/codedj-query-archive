#!/usr/bin/env python

import unittest

import dejumble.test.base
from dejumble.test.base import *
import dejumble.filters.null
from dejumble.filters.null import *


class NullFileListFilterTestCase(BaseFileListFilterTestCase):
    def testfilelist(self):
        filelist = list(NullFileListFilter().filelist())
        self.assertEqual(len(filelist), 1)
        self.assertEqual(filelist[0], '/dev/null')
