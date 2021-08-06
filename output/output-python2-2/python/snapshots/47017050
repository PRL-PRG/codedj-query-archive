#!/usr/bin/env python

import logging
import os
import os.path
import time

from PyDbLite import Base
import fuse
from fuse import Fuse

import dejumble.filter
from dejumble.filter import *
import dejumble.util
from dejumble.util import *

DB_TRANSFORMED = './.dejumbledb_transformed'
DB_FILE_TAGS = './.dejumbledb_tags'

increase_regex = re.compile('^(.*)\((\d+)\)$')

class Organizer(Cacheable):
    """
    This is the base class for organizers
    """

    def __init__(self, cache, recursive=True):
        self.cache = cache
        self.recursive = recursive
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
        self.generateallpaths()

	############################################
	# Overwritable functions

    def dirlist(self, path):
        """
        Returns a list of (non-existent, generated, virtual) directories for a given path. Default implementation.
        """
        return [ ]

    def generatepaths(self, realpath):
        """
        Generates paths for a given real path. A file can have more than one transformed path. Default implementation.
        """
        yield addtrailingslash(removeroot(realpath, self.cache.filter.root))

    def generaterealpath(self, path):
        """
        Generates a real path for a inexistent path. Default implementation.
        """
        return os.path.join(self.cache.filter.root, os.path.basename(path))

	############################################
	# General functions

    def generateallpaths(self):
        """
        Generates paths for all the files given by the cache and stores them in self.transformed
        """
        for realpath in self.cache.filelist():
            if self.recursive:
                # Add all sub-directories first
                currentpath = self.cache.filter.root
                
                for pathpart in pathparts(removeroot(realpath, self.cache.filter.root)):
                    currentpath = os.path.join(currentpath, pathpart)
                    self.addfile(currentpath)
            else:
                self.addfile(realpath)

    def addfile(self, realpath):
        """
        Stores a file in self.transformed if not there already and returns the paths for that
        file in the proxy file system 
        """
        logger.debug('addfile(%s)' % realpath)
        if not ignoretag(removeroot(realpath, self.cache.filter.root)):
            return [ ]

        self.refreshcache()
        transformed = self.transformed._realpath[realpath]

        if transformed:
            return [ r['path'] for r in transformed ]
        else:
            paths = [ ]

            for path in self.paths(realpath):
                while self.transformed._path[path]:
                    path = self.increasefilename(path)

                dir = os.path.dirname(path)
                logger.debug('addfile(%s, %s, %s)' % (realpath, path, dir))
                self.transformed.insert(realpath=realpath, path=path, dir=dir)
                paths.append(path)

            return paths

    def increasefilename(self, filename):
        """
        Returns a new filename in sequence. Called if the current filename already exists.
        This default implementation adds a "(1)" to the end if not present or increases that
        number by one.
        """
        root, ext = os.path.splitext(filename)
    
        num = 1
        m = increase_regex.match(root)
    
        if not m is None:
            num = int(m.group(2)) + 1
            filename = m.group(1)
    
        return '%s(%i)%s' % (root, num, ext)        

    ############################################
    # General functions that read the cache

    def filelist(self, path):
        """
        Returns a list of filenames in a list from cache
        """
        self.refreshcache()
        [ (yield os.path.basename(r['path'])) for r in self.transformed._dir[path]  ]

    def paths(self, realpath):
        """
        Generates or returns paths from cache for a given real path
        """
        self.refreshcache()
        paths = self.transformed._realpath[realpath]

        if paths:
            [ (yield path['path']) for path in paths ]
        else:
            [ (yield path) for path in self.generatepaths(realpath) ]

    def realpath(self, path):
        """
        Returns the real path for a file given the path in the file system.
        """
        logger.debug('realpath(%s)' % path)
        self.refreshcache()
        realpaths = [ r['realpath'] for r in self.transformed._path[path] ]

        if realpaths:
            return realpaths[0]

        if path == '/':
            return self.cache.filter.root
        elif path == addtrailingslash(ORIGINAL_DIR):
            return '.'
        elif pathparts(path)[0] == ORIGINAL_DIR:
            return os.path.join('.', '/'.join(pathparts(path)[1:]))
        else:
            return self.generaterealpath(path)

	############################################
	# File system functions

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
    def __init__(self, cache, category=None):
        self.tags = Base(DB_FILE_TAGS)
        self.category = category
        Organizer.__init__(self, cache, False)

    def reset(self):
        self.tags.create('realpath', 'category', 'tag', mode = 'override')
        self.tags.create_index('category')
        Organizer.reset(self)

    def updatecache(self):
        self.generatetags()
        Organizer.updatecache(self)

    def generatepaths(self, realpath):
        [ (yield os.path.join(tag, os.path.basename(realpath))) for tag in [ r['tag'] for r in self.tags._realpath[realpath] ] ]

    def dirlist(self, path):
        if path == '/':
            return self.taglist(self.category)
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
        self.refreshcache()
        [ (yield os.path.basename(r['realpath'])) for r in self.tags._category[category] if r['tag'] in tags ]

    def taglist(self, category):
        self.refreshcache()
        return unique([ r['tag'] for r in self.tags._category[category] ])

