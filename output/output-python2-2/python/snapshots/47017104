#!/usr/bin/env python

import os
import os.path

import dejumble.filters.completedirectory
from dejumble.filters.completedirectory import *


class OriginalDirectoryFileListFilter(CompleteDirectoryFileListFilter):
    def __init__(self, query, root):
        CompleteDirectoryFileListFilter.__init__(self, query, './')

    def filelist(self):
        return _filelist('.', '.')

