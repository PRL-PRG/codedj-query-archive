#!/usr/bin/env python

import os
import os.path
import commands
import logging
import errno
import time
import pkg_resources

import dejumble.util
from dejumble.util import *
import dejumble.storage
from dejumble.storage import *

logger = logging.getLogger('dejumble')


class FileListProvider:
    def __init__(self, query):
        self.query = query
        self.storage = MemoryStorage()

    def _reset(self):
        self.storage.reset()

    def _addfilename(self, realpath):
        filename = os.path.basename(realpath)
        self.storage.savefile(filename, realpath)

    def realpath(self, filename):
        return self.storage.realpath(filename)
     
    def realpath(self, path):
        realpath = self.storage.realpath(path[1:])
        if not realpath == None:
            return realpath
        else:
            return '.' + path

    def filelist(self):
        return self.storage.filelist()

    def refreshfilelist(self):
        self._refreshfilelist()


class NullFileListProvider(FileListProvider):
    def _refreshfilelist(self):
        self._reset()
        self._addfilename('/dev/null')


class ShellFileListProvider(FileListProvider):
    def _refreshfilelist(self):
        self._reset()
        status, output = commands.getstatusoutput(self.query)
        if status == -1:
            return -errno.ENOENT
        filenames = output.splitlines()
        for path in filenames:
            self._addfilename(path)


class BeagleFileListProvider(FileListProvider):
    def _refreshfilelist(self):
        self._reset()
    

class OriginalDirectoryFileListProvider(FileListProvider):
    def _refreshfilelist(self):
        self._reset()
        self._getdirlist('.', '.')

    def _getdirlist(self, dir, currentpath):
        for path in os.listdir(dir):
            if os.path.isdir(path) and not os.path.islink(path):
                self._getdirlist(path, os.path.join(path, dir))
            else:
                self._addfilename(os.path.join(currentpath, path))


