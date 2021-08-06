#!/usr/bin/env python

import dejumble.filter
from dejumble.filter import *


class NullFileListFilter(FileListFilter):
    def __init__(self, query='', root='/dev/'):
        FileListFilter.__init__(self, query, '/dev/')
        
    def filelist(self):
        yield '/dev/null'

