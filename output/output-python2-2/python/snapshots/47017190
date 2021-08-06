#!/usr/bin/env python

import commands
import errno

import dejumble.filters
from dejumble.filters import *


class ShellFileListFilter(FileListFilter):
    def filelist(self):
        status, output = commands.getstatusoutput(self.query)

        if status == -1:
            return -errno.ENOENT

        filenames = output.splitlines()

        return filenames

