#!/usr/bin/env python

import os
import os.path

import dejumble.filters
from dejumble.filters import *


class OriginalDirectoryFileListFilter(FileListFilter):
    def filelist(self):
        return _getdirlist('.', '.')

    def _getdirlist(self, dir, currentpath):
        filenames = [ ]

        for path in os.listdir(dir):
            if os.path.isdir(path) and not os.path.islink(path):
                filenames.extend(self._getdirlist(path, os.path.join(path, dir)))
            else:
                filenames.append(os.path.join(currentpath, path))

        return filenames

