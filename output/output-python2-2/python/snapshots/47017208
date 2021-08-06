#!/usr/bin/env python

import logging
import os.path

import dejumble.providers
from dejumble.providers import *
import dejumble.util
from dejumble.util import *

logger = logging.getLogger('dejumble')

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
        'Documents': DocumentsOrganizer,
        'Date': DateOrganizer
    }[name](provider)

    return organizer


class Organizer:
    def __init__(self, provider):
        self.provider = provider
        self.expirecache()

    def isdir(self, path):
        return self._isdir(path)

    def realpath(self, path):
        self.refreshcache()
        if path == '/':
            return ORIGINAL_DIR
        elif path == addtrailingslash(ORIGINAL_DIR):
            return '.'
        elif pathparts(path)[0] == ORIGINAL_DIR:
            return os.path.join('.', '/'.join(pathparts(path)[1:]))
        else:
            filename = os.path.basename(path)
            return self.provider.realpath(addtrailingslash(filename))

    def filelist(self, path):
        self.refreshcache()
        if path == addtrailingslash(ORIGINAL_DIR):
            return getbasefilelist() + os.listdir('.')
        elif pathparts(path)[0] == ORIGINAL_DIR:
            return getbasefilelist() + os.listdir(self.realpath(path))
        else:
            return self._filelist(path)

    def expirecache(self):
        self.expiretime = time.time()

    def refreshcache(self):
        if self.expiretime < time.time():
            self.expiretime = time.time() + 60
            self.provider.refreshfilelist()
            self._refreshcache()

    def _isdir(self, path):
        None

    def _refreshcache(self):
        None


class FlatOrganizer(Organizer):
    def _filelist(self, path):
        return self.provider.filelist()


class DocumentsOrganizer(Organizer):
    def __init__(self, provider):
        logger.debug('1')
        Organizer.__init__(self, provider)
        logger.debug('2')
        self.filetypes = readconfig('filetypes')
        logger.debug('3')

    def _filelist(self, path):
        if path == '/':
            return self.provider.storage.taglist('extension')
        else:
            return self.provider.storage.filelistbytag('extension', path[1:])

    def _isdir(self, path):
        return len(pathparts(path)) == 1

    def _refreshcache(self):
        for filetype in self.filetypes.keys():
            extensions = self.filetypes[filetype]
            for extension in extensions.split(','):
                reg = re.compile('%s$' % extension);
                for filename in self.provider.filelist():
                    if not reg.search(filename) == None:
                        self.provider.storage.tag(filename, 'extension', filetype)


class DateOrganizer(Organizer):
    def _filelist(self, path):
        return self.provider.filelist()

    def _isdir(self, path):
        None

    def _refreshcache(self):
        None



