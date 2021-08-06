#!/usr/bin/env python

import logging
import os.path
import time

import dejumble.filters
from dejumble.filters import *
import dejumble.util
from dejumble.util import *

DB_TRANSFORMED = './.dejumbledb_transformed'
logger = logging.getLogger('dejumble')


class Organizer:
    def __init__(self, cache):
        self.cache = cache
        self.transformed = Base(DB_TRANSFORMED)

    def reset(self):
        self.transformed.create('realpath', 'path', mode = 'override')
        self.transformed.create_index('realpath')
        self.transformed.create_index('path')

    	self.cache.reset()

    def realpath(self, path):
        if path == '/':
            return ORIGINAL_DIR
        elif path == addtrailingslash(ORIGINAL_DIR):
            return '.'
        elif pathparts(path)[0] == ORIGINAL_DIR:
            return os.path.join('.', '/'.join(pathparts(path)[1:]))
        else:
            filename = os.path.basename(path)
            realpaths = [ r['realpath'] for r in self.cache.transformed._path[_path] ]
            if len(realpaths) == 0:
                return None
            else:
                return realpaths[0]

    def filelist(self, path):
        if path == addtrailingslash(ORIGINAL_DIR):
            return getbasefilelist() + os.listdir('.')
        elif pathparts(path)[0] == ORIGINAL_DIR:
            return getbasefilelist() + os.listdir(self.realpath(path))
        else:
            return self._filelist(path)

    def isdir(self, path):
        None

	############################################
	# Filesystem functions

    def getattr(self, path):
        if path == '/' or self.isdir(path):
            return self.cache.getattr('.')
        else:
            return self.cache.getattr(self.realpath(path))

    def readdir(self, path, offset):
        logger.debug('readdir(%s)' % path)
        for filename in self.filelist(path):
            yield fuse.Direntry(filename)


class TagOrganizer(Organizer):
    def __init__(self, cache, category):
        Organizer.__init__(self, cache)
        self.category = category

    def isdir(self, path):
        return len(pathparts(path)) == 1

    def _filelist(self, path):
        if path == '/':
            return self.cache.storage.taglist(self.category)
        else:
            return self.cache.storage.filelistbytags(self.category, path[1:])
