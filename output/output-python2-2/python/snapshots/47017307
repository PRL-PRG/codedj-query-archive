#!/usr/bin/env python

import os
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
        self.refreshfilelist()
        return self.files[path]

    def filelist(self, path):
        self.refreshfilelist()
        # TODO: handle subdirectories
        if path == '/':
            return self.files.iteritems()
        else:
            return os.listdir(self.realpath(path))

    def expirefilelist(self):
        self.expiretime = time.time()

    def refreshfilelist(self):
        if self.expiretime < time.time():
            logger.info('Executing query ' + self.query);
            self.expiretime = time.time() + 60
            self._refreshfilelist()

    def _refreshfilelist(self):
        return -errno.ENOENT


class NullFileListProvider(FileListProvider):
    def _refreshfilelist(self):
        self.files = { '/..': '..', '/.': '.', '/null': '/dev/null' }


class ShellFileListProvider(FileListProvider):
    def _refreshfilelist(self):
        self.files = { '/..': '..', '/.': '.' }
        status, output = commands.getstatusoutput(self.query)
        if status == -1:
            return -errno.ENOENT
        filenames = output.splitlines()
        for path in filenames:
            filename = '/' + filenamepart(path)
            while filename in self.files:
                filename = increasefilename(filename)
            self.files[filename] = path


class BeagleFileListProvider(FileListProvider):
    def _refreshfilelist(self):
        self.files = { '/..': '..', '/.': '.' }
    

class OriginalDirectoryFileListProvider(FileListProvider):
    def _refreshfilelist(self):
        self.files = { '/..': '..', '/.': '.' }
        for path in os.listdir('.'):
            # TODO recurse into subdirectories
            filename = '/' + filenamepart(path)
            self.files[filename] = path
            logger.debug(filename + ' = ' + path)
