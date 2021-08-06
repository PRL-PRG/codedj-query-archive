#!/usr/bin/env python

import logging
import os.path
import time

import dejumble.providers
from dejumble.providers import *
import dejumble.util
from dejumble.util import *

logger = logging.getLogger('dejumble')

def getorganizer(name, provider, query):
    logger.info('provider = %sFileListProvider(%s)' % (provider, query))

    provider = {
        'Null': NullFileListProvider,
        'Shell': ShellFileListProvider,
        'Beagle': BeagleFileListProvider,
        'OriginalDirectory': OriginalDirectoryFileListProvider
    }[provider](query)

    logger.info('organizer = %sOrganizer' % name)

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


class TagOrganizer(Organizer):
    def __init__(self, provider, category):
        Organizer.__init__(self, provider)
        self.category = category

    def _filelist(self, path):
        if path == '/':
            return self.provider.storage.taglist(self.category)
        else:
            return self.provider.storage.filelistbytags(self.category, path[1:])

    def _isdir(self, path):
        return len(pathparts(path)) == 1


class DocumentsOrganizer(TagOrganizer):
    def __init__(self, provider):
        TagOrganizer.__init__(self, provider, 'filetype')
        self.filetypes = readconfig('filetypes')
        for filetype, extensions in self.filetypes.iteritems():
            self.filetypes[filetype] = map(extensionregex, extensions.split(','))

    def _refreshcache(self):
        for filename in filter(ignoretag, self.provider.filelist()):
            hastag = False
            for filetype, extensions in self.filetypes.iteritems():
                for extension in extensions:
                    if not extension.search(filename) == None:
                        self.provider.storage.tag(filename, self.category, _(filetype))
                        hastag = True
            if not hastag:
                self.provider.storage.tag(filename, self.category, _('Other'))



class DateOrganizer(TagOrganizer):
    def __init__(self, provider):
        TagOrganizer.__init__(self, provider, 'date')

    def _refreshcache(self):
        for filename in filter(ignoretag, self.provider.filelist()):
            stats = os.stat(self.provider.realpath(addtrailingslash(filename)))
            lastmod = time.localtime(stats[8])
            today = time.localtime()
            self.provider.storage.tag(filename, self.category, time.strftime('%Y %B', lastmod))
            if time.strftime('%x', today) == time.strftime('%x', lastmod):
                self.provider.storage.tag(filename, self.category, _('Today'))
            if time.strftime('%Y%W', today) == time.strftime('%Y%W', lastmod):
                self.provider.storage.tag(filename, self.category, _('This Week'))
            lastweek = time.localtime(time.time() - 7 * 24 * 60 * 60)
            if time.strftime('%Y%W', lastweek) == time.strftime('%Y%W', lastmod):
                self.provider.storage.tag(filename, self.category, _('Last Week'))

