#!/usr/bin/env python

import os
import os.path

import dejumble.filter
from dejumble.filter import *


class CompleteDirectoryFileListFilter(FileListFilter):
    def filelist(self):
        return self._filelist(self.root, self.root)

    def _filelist(self, dir, currentpath):
        for path in os.listdir(dir):
            if os.path.isdir(path) and not os.path.islink(path):
                for realpath in self._filelist(path, os.path.join(path, dir)):
                    yield realpath 
            else:
                yield os.path.join(currentpath, path)

