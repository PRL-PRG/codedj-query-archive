import logging
import os
import os.path
import re

from PyDbLite import Base
import fuse

from . import util
from .util import Cacheable


DB_TRANSFORMED = './.dejumblefs_transformed.pydblite'
DB_FILE_TAGS = './.dejumblefs_tags.pydblite'

_INCREASE_REGEX = re.compile('^(.*)\((\d+)\)$')

logger = logging.getLogger('dejumblefs.Organizer')


class Organizer(Cacheable):
    """
    This is the base class for organizers
    """

    def __init__(self, cache, recursive=True):
        Cacheable.__init__(self)
        self.cache = cache
        self.recursive = recursive
        self.transformed = None
        # Do not call reset here, it is called from fs.py when the fs is
        # already started

    def reset(self):
        if not self.transformed:
            self.transformed = Base(DB_TRANSFORMED)
        self.transformed.create('realpath', 'path', 'dirname', mode='override')
        self.transformed.create_index('realpath')
        self.transformed.create_index('path')
        self.transformed.create_index('dirname')
        self.cache.reset()
        Cacheable.reset(self)

    def updatecache(self):
        self.generateallpaths()

    def deletefromcache(self, path):
        realpath = self.realpath(path)
        logger.debug("deletefromcache(%s)" % realpath)
        self.cache.deletefromcache(realpath)
        items = self.transformed.get_index('realpath')[realpath]
        self.transformed.delete(items)

    def addtocache(self, path):
        if not self.transformed.get_index('path')[path]:
            realpath = self.realpath(path)
            self.cache.addtocache(realpath)
            self.addfile(realpath)

    ############################################
    # Overwritable functions

    def dirlist(self, path): #IGNORE:W0613
        """
        Returns a list of (non-existent, generated, virtual) directories for a
        given path. Default implementation.
        """
        return []

    def generatepaths(self, realpath):
        """
        Generates paths for a given real path. A file can have more than one
        transformed path. Default implementation.
        """
        yield util.addtrailingslash(util.removeroot(realpath,
                                                    self.cache.filter.root))

    def generaterealpath(self, path):
        """
        Generates a real path for a inexistent path. Default implementation.
        """
        return os.path.join(self.cache.filter.root, path[1:])

    ############################################
    # General functions

    def generateallpaths(self):
        """
        Generates paths for all the files given by the cache and stores them
        in self.transformed
        """
        for realpath in self.cache.filelist():
            if self.recursive:
                # Add all sub-directories first
                currentpath = self.cache.filter.root

                for pathpart in util.pathparts(util.removeroot(realpath,
                                                  self.cache.filter.root)):
                    currentpath = os.path.join(currentpath, pathpart)
                    self.addfile(currentpath)
            else:
                self.addfile(realpath)

    def addfile(self, realpath):
        """
        Stores a file in self.transformed if not there already and returns the
        paths for that file in the proxy file system
        """
        logger.debug('addfile(%s)' % realpath)
        if not util.ignoretag(util.removeroot(realpath,
                                              self.cache.filter.root)):
            return []

        self.refreshcache()
        transformed = self.transformed.get_index('realpath')[realpath]

        if transformed:
            return (record['path'] for record in transformed)
        else:
            paths = []

            for path in self.paths(realpath):
                while self.transformed.get_index('path')[path]:
                    path = self.increasefilename(path)

                dirname = os.path.dirname(path)
                logger.debug('addfile(%s, %s, %s)' % (realpath, path, dirname))
                self.transformed.insert(realpath=realpath, path=path,
                                        dirname=dirname)
                paths.append(path)

            return paths

    def increasefilename(self, filename):
        """
        Returns a new filename in sequence. Called if the current filename
        already exists. This default implementation adds a "(1)" to the end if
        not present or increases that number by one.
        """
        root, ext = os.path.splitext(filename)

        num = 1
        matches = _INCREASE_REGEX.match(root)

        if not matches is None:
            num = int(matches.group(2)) + 1
            filename = matches.group(1)

        return '%s(%i)%s' % (root, num, ext)

    ############################################
    # General functions that read the cache

    def filelist(self, path):
        """
        Returns a list of directories and filenames in a list from cache
        """
        logger.debug('filelist(%s)' % path)
        self.refreshcache()

        for dirname in self.dirlist(path):
            yield dirname

        for record in self.transformed.get_index('dirname')[path]:
            yield os.path.basename(record['path'])

    def paths(self, realpath):
        """
        Generates or returns paths from cache for a given real path
        """
        self.refreshcache()
        paths = self.transformed.get_index('realpath')[realpath]

        if paths:
            return (path['path'] for path in paths)
        else:
            return (path for path in self.generatepaths(realpath))

    def realpath(self, path):
        """
        Returns the real path for a file given the path in the file system.
        """
        logger.debug('realpath(%s)' % path)
        self.refreshcache()
        realpaths = [r['realpath']
                     for r in self.transformed.get_index('path')[path]]

        realpath = None

        if realpaths:
            realpath = realpaths[0]
        elif path == '/':
            realpath = self.cache.filter.root
        elif path == util.addtrailingslash(util.ORIGINAL_DIR):
            realpath = '.'
        elif util.pathparts(path)[0] == util.ORIGINAL_DIR:
            realpath = os.path.join('.', os.sep.join(util.pathparts(path)[1:]))
        else:
            realpath = self.generaterealpath(path)

        logger.debug('realpath(%s) = %s' % (path, realpath))
        return realpath

    ############################################
    # File system functions

    def getattr(self, path):
        dirname = os.path.dirname(path)
        if util.removeroot(path, os.sep) in self.dirlist(dirname):
            return self.cache.getattr(self.realpath(dirname))
        else:
            return self.cache.getattr(self.realpath(path))

    def readdir(self, path, offset): #IGNORE:W0613
        for filename in util.getbasefilelist():
            yield fuse.Direntry(filename)

        for filename in self._filelist(path):
            yield fuse.Direntry(filename)

    def _filelist(self, path):
        filelist = []
        if path == util.addtrailingslash(util.ORIGINAL_DIR):
            filelist = os.listdir('.')
        elif util.pathparts(path)[0] == util.ORIGINAL_DIR:
            filelist = os.listdir(self.realpath(path))
        else:
            filelist = self.filelist(path)

        for filename in filelist:
            yield filename


class TagOrganizer(Organizer):

    def __init__(self, cache, category=None):
        self.tags = None
        self.category = category
        Organizer.__init__(self, cache, False)

    def reset(self):
        if not self.tags:
            self.tags = Base(DB_FILE_TAGS)
        self.tags.create('realpath', 'category', 'tag', mode = 'override')
        self.tags.create_index('realpath')
        self.tags.create_index('category')
        Organizer.reset(self)

    def updatecache(self):
        self._generatetags()
        Organizer.updatecache(self)

    def _deletefromcache(self, path):
        realpath = self.realpath(path)
        logger.debug("_deletefromcache(%s)" % realpath)
        for record in self.tags.get_index('realpath')[realpath]:
            del self.tags[record['__id__']]
        Organizer.deletefromcache(self, path)

    def deletefromcache(self, path):
        self._deletefromcache(path)
        Organizer.deletefromcache(self, path)

    def addtocache(self, path):
        self._deletefromcache(path)
        self.generatetags(self.realpath(path))
        Organizer.addtocache(self, path)

    def generatepaths(self, realpath):
        for record in self.tags.get_index('realpath')[realpath]:
            yield os.path.join(os.sep, record['tag'],
                               os.path.basename(realpath))

    def dirlist(self, path):
        if path == '/':
            return self.taglist(self.category)
        else:
            return []

    ############################################
    # Tag functions

    def _generatetags(self):
        for filename in filter(util.ignoretag, #IGNORE:W0141
                               self.cache.filelist()):
            self.generatetags(filename)

    def generatetags(self, filename):
        pass

    def tag(self, realpath, category, tag):
        logger.debug('tag(%s, %s, %s)' % (realpath, category, tag))
        if not tag == None and not tag == '':
            self.tags.insert(realpath, category, tag)

    def filelistbytags(self, category, tags):
        self.refreshcache()
        for record in self.tags.get_index('category')[category]:
            if record['tag'] in tags:
                yield os.path.basename(record['realpath'])

    def taglist(self, category):
        self.refreshcache()
        return util.unique([record['tag'] for record in
                            self.tags.get_index('category')[category]])
