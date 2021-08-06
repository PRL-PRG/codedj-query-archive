#!/usr/bin/env python

import logging
import os
import os.path
import time

import dejumble.filters
from dejumble.filters import *
import dejumble.util
from dejumble.util import *

DB_TRANSFORMED = './.dejumbledb_transformed'
DB_FILE_TAGS = './.dejumbledb_tags'

increase_regex = re.compile('^(.*)\((\d+)\)$')

logger = logging.getLogger('dejumble')


class Organizer(Cacheable):
    """
    This is the base class for organizers
    """

    def __init__(self, cache):
        self.cache = cache
        self.transformed = Base(DB_TRANSFORMED)
        # Do not call reset here, it is called from fs.py when the fs is already started

    def reset(self):
        self.transformed.create('realpath', 'path', 'dir', mode='override')
        self.transformed.create_index('realpath')
        self.transformed.create_index('path')
        self.transformed.create_index('dir')
    	self.cache.reset()
    	Cacheable.reset(self)

    def updatecache(self):
        self.generatepaths()

	############################################
	# Overwritable functions

    def paths(self, realpath):
        """
        Generates paths for a given realpath (a file can have more than one transformed path)
        """
        # TODO: substract root from this path
        return realpath

    def dirlist(self, path):
        """
        Returns a list of (non-existant) directories for a given path
        """
        return [ ]

	############################################
	# General functions

    def generatepaths(self):
        """
        Generates paths for all the files given by the filter and stores them in self.transformed
        """
        for realpath in self.filter.filelist():
            addfile(realpath)

    def addfile(self, realpath):
        """
        Stores a file in self.transformed if not there already
        """
        if len(self.transformed._realpath[realpath]) > 0:
            return
            
        for path in self.paths(realpath):
            while len(self.transformed._path[path]) > 0:
                path = self.increasefilename(path)

            self.transformed.insert(realpath=realpath, path=path, dir=os.path.dirname(path))

    def increasefilename(self, filename):
        """
        Returns a new filename in sequence. Called if the current filename already exists.
        This default implementation adds a (1) to the end if not present or increases that
        number by one.
        """
        root, ext = os.path.splitextension(filename)
    
        num = 1
        m = increase_regex.match(root)
    
        if not m is None:
            num = int(m.group(2)) + 1
            filename = m.group(1)
    
        return '%s(%i)%s' % (root, num, ext)        

    def realpath(self, path):
        """
        Returns the real path for a file given the path in the filesystem.
        """
        realpaths = [ r['realpath'] for r in self.transformed._path[path] ]

        if len(realpaths) > 0:
            return realpaths[0]

        if path == '/':
            return self.cache.filter.root
        elif path == addtrailingslash(ORIGINAL_DIR):
            return '.'
        elif pathparts(path)[0] == ORIGINAL_DIR:
            return os.path.join('.', '/'.join(pathparts(path)[1:]))

    def filelist(self, path):
        return [ os.basename(r['path']) for r in self.cache.transformed._dir[path]  ]

	############################################
	# Filesystem functions

    def getattr(self, path):
        if self.realpath(path) is not None:
            return self.cache.getattr(self.realpath(path))
        else:
            return self.cache.getattr('.')

    def readdir(self, path, offset):
        logger.debug('readdir(%s)' % path)
        for filename in self._filelist(path):
            yield fuse.Direntry(filename)

    def _filelist(self, path):
        if path == addtrailingslash(ORIGINAL_DIR):
            return getbasefilelist() + os.listdir('.')
        elif pathparts(path)[0] == ORIGINAL_DIR:
            return getbasefilelist() + os.listdir(self.realpath(path))
        elif self.filelist != Organizer.filelist:
            return self.filelist(path)


class TagOrganizer(Organizer):
    def __init__(self, cache, category):
        self.tags = Base(DB_FILE_TAGS)
        self.category = category
        Organizer.__init__(self, cache)

    def reset(self):
        self.tags.create('realpath', 'category', 'tag', mode = 'override')
        self.tags.create_index('category')
        Organizer.resetcache(self)

    def updatecache(self):
        self.generatetags() 
        Organizer.updatecache(self)

    def paths(self, realpath):
        for tag in [ r['tag'] for r in self.tags._realpath[realpath] ]:
            yield os.path.join(tag, os.path.basename(realpath))

    def dirlist(self, path):
        if path == '/':
            return self.cache.taglist(self.category)
        else:
            return [ ]

	############################################
	# Tag functions

    def generatetags(self):
        None

    def tag(self, realpath, category, tag):
        if not tag == None and not tag == '':
            self.tags.insert(realpath, category, tag)

    def filelistbytags(self, category, tags):
        return [ os.path.basename(r['realpath']) for r in self.tags._category[category] if r['tag'] in tags ]

    def taglist(self, category):
        return unique([ r['tag'] for r in self.tags._category[category] ])

