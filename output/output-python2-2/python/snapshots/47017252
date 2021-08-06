#!/usr/bin/env python

import errno
import logging
import os.path

import dejumble.providers
from dejumble.providers import *
import dejumble.util
from dejumble.util import *

logger = logging.getLogger('dejumblefs.organizer')

def getorganizer(name, provider, query):
    logger.info('provider = ' + provider + 'FileListProvider(' + query + ')')

    provider = {
        'Null': NullFileListProvider,
        'Shell': ShellFileListProvider,
        'Beagle': BeagleFileListProvider,
        'OriginalDirectory': OriginalDirectoryFileListProvider
    }[provider](query)

    logger.info('organizer = ' + name + "Organizer")

    organizer = {
        'Flat': FlatOrganizer,
        'Extension': ExtensionOrganizer,
        'Date': DateOrganizer
    }[name](provider)

    return organizer


class Organizer:
    def __init__(self, provider):
        self.provider = provider
        self.expirecache()

    def realpath(self, path):
        if path == '/':
            return '.' + ORIGINAL_DIR
        elif path == ORIGINAL_DIR:
            return '.'
        elif pathparts(path)[0] == ORIGINAL_DIR[1:]:
            return os.path.join('.', '/'.join(pathparts(path)[1:]))
        else:
            filename = os.path.basename(path)
            return self.provider.realpath(addtrailingslash(filename))

    def filelist(self, path):
        if path == '/':
            return self._filelist(path)
        elif path == ORIGINAL_DIR:
            return getbasefilelist() + os.listdir('.')
        else:
            return getbasefilelist() + os.listdir(self.realpath(path))

    def _filelist(self, path):
        return -errno.ENOENT

    def expirecache(self):
        self.expiretime = time.time()

    def refreshcache(self):
        if self.expiretime < time.time():
            self.expiretime = time.time() + 60
            self.provider.refreshfilelist()
            self._refreshcache()

    def _refreshcache(self):
        None


class FlatOrganizer(Organizer):
    def _filelist(self, path):
        return self.provider.filelist(path)


class ExtensionOrganizer(Organizer):
    def _filelist(self, path):
        return self.provider.filelist()

    def _refreshcache(self):
        None


class DateOrganizer(Organizer):
    def _filelist(self, path):
        return self.provider.filelist()

    def _refreshcache(self):
        None



