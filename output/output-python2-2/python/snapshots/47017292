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

def getFileListProvider(name, query):
    logger.debug('getFileListProvider(' + name + ', ' + query + ')')
    return {
        'Null': NullFileListProvider,
        'Shell': ShellFileListProvider,
        'Beagle': BeagleFileListProvider,
        'OriginalDirectory': OriginalDirectoryFileListProvider
    }[name](query)


class FileListProvider:
    def __init__(self, query):
        self.query = query
        self.expirefilelist()

    def realpath(self, path):
        logger.debug('realpath ' + path)
        self.refreshfilelist()
        if path == '/':
            return '.' + ORIGINAL_DIR
        elif path == ORIGINAL_DIR:
            return '.'
        elif path in self.files:
            return self.files[path]
        elif pathparts(path)[0] == ORIGINAL_DIR[1:]:
            return os.path.join('.', '/'.join(pathparts(path)[1:]))
        else:
            self.expirefilelist()
            return './' + path

    def filelist(self, path):
        self.refreshfilelist()
        if path == '/':
            return self.files.iterkeys()
        elif path == ORIGINAL_DIR:
            return getbasefilelist() + map(addtrailingslash, os.listdir('.'))
        else:
            return getbasefilelist() + map(addtrailingslash, os.listdir(self.realpath(path)))

    def expirefilelist(self):
        self.expiretime = time.time()

    def refreshfilelist(self):
        if self.expiretime < time.time():
            logger.debug('Executing query ' + self.query);
            self.expiretime = time.time() + 60
            self._refreshfilelist()

    def _refreshfilelist(self):
        return -errno.ENOENT


class NullFileListProvider(FileListProvider):
    def _refreshfilelist(self):
        self.files = getbasefilemap()
        self.files['/null'] = '/dev/null'


class ShellFileListProvider(FileListProvider):
    def _refreshfilelist(self):
        self.files = getbasefilemap()
        status, output = commands.getstatusoutput(self.query)
        if status == -1:
            return -errno.ENOENT
        filenames = output.splitlines()
        for path in filenames:
            filename = '/' + os.path.basename(path)
            while filename in self.files:
                filename = increasefilename(filename)
            self.files[filename] = path


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
                filename = addtrailingslash(os.path.basename(path))
                while filename in self.files:
                    filename = increasefilename(filename)
                self.files[filename] = os.path.join(currentpath, path)


