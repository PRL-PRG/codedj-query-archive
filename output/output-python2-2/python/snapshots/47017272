#!/usr/bin/env python

import os
import os.path
import commands
import logging
import errno
import time

import SearchFS.util
from SearchFS.util import *

logger = logging.getLogger('searchfs')


class FileListProvider:
    def __init__(self, query):
        self.query = query

    def realpath(self, path):
        self.refreshfilelist()
        if path[1:] in self.files:
            return self.files[path[1:]]
        else:
            return '.' + path

    def filelist(self, path):
        self.refreshfilelist()
        return self.files.iterkeys()

    def refreshfilelist(self):
        logger.debug('Executing query ' + self.query)
        self._refreshfilelist()

    def _refreshfilelist(self):
        return -errno.ENOENT

    def _addfilename(self, path):
        filename = os.path.basename(path)
        while filename in self.files:
            filename = increasefilename(filename)
        self.files[filename] = path


class NullFileListProvider(FileListProvider):
    def _refreshfilelist(self):
        self.files = getbasefilemap()
        self.files['null'] = '/dev/null'


class ShellFileListProvider(FileListProvider):
    def _refreshfilelist(self):
        self.files = getbasefilemap()
        status, output = commands.getstatusoutput(self.query)
        if status == -1:
            return -errno.ENOENT
        filenames = output.splitlines()
        for path in filenames:
            self._addfilename(path)


class BeagleFileListProvider(FileListProvider):
    def _refreshfilelist(self):
        self.files = getbasefilemap()
    

class OriginalDirectoryFileListProvider(FileListProvider):
    def _refreshfilelist(self):
        self.files = getbasefilemap()
        self._getdirlist(self.files, '.', '.')

    def _getdirlist(self, files, dir, currentpath):
        for path in os.listdir(dir):
            if os.path.isdir(path) and not os.path.islink(path):
                self._getdirlist(files, path, os.path.join(path, dir))
            else:
                self._addfilename(os.path.join(currentpath, path))


