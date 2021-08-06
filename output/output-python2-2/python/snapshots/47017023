#!/usr/bin/env python

import os
import os.path

import dejumble.filter
from dejumble.filter import *

logger = logging.getLogger('dejumble.Filter')


class CompleteDirectoryFileListFilter(FileListFilter):
    def filelist(self):
        return list(self._generatefilelist(self.root))

    def _generatefilelist(self, dir):
        for path in os.listdir(dir):
            path = os.path.join(dir, path)
            if os.path.isdir(path) and not os.path.islink(path):
                for realpath in self._generatefilelist(path):
                    yield realpath 
            else:
                yield path

