#!/usr/bin/env python

import dejumble.filters
from dejumble.filters import *


class NullFileListFilter(FileListFilter):
    def __init__(self, query, root):
        FileListFilter.__init__(self, query, '/dev/')
        
    def filelist(self):
        yield '/dev/null'

