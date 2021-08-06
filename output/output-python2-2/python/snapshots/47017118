#!/usr/bin/env python

import commands
import errno

import dejumble.filter
from dejumble.filter import *


class ShellFileListFilter(FileListFilter):
    def filelist(self):
        status, output = commands.getstatusoutput(self.query)

        if status == -1:
            return -errno.ENOENT

        filenames = output.splitlines()
        
        # TODO: convert files inside the mount directory to relative paths.

        return filenames

