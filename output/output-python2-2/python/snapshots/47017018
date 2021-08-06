#!/usr/bin/env python

#    Copyright (C) 2006  Cesar Izurieta  <cesar@ecuarock.net>
#
#    This program can be distributed under the terms of the GNU LGPL.
#    See the file COPYING.
#

import os
import os.path
import stat
import errno
import logging
import time

import fuse
from fuse import Fuse

import dejumble.organizer
from dejumble.organizer import *
import dejumble.util
from dejumble.util import *

fuse.fuse_python_api = (0, 2)

logger = logging.getLogger('dejumble.DejumbleFS')

_server = None

def setserver(server):
    global _server
    _server = server

def getserver():
    global _server
    return _server

class DejumbleFS(Fuse):
    def main(self, *a, **kw):
        logger.info(_('Initializing dejumblefs'))
        self.setup_organizer()
        self.file_class = self.organizer.cache.DejumbleFile
        self.originaldir = os.open(self.fuse_args.mountpoint, os.O_RDONLY)
        try:
            result = Fuse.main(self, *a, **kw)
        except fuse.FuseError:
            result = -errno.ENOENT
            logger.warn(_('Finalizing dejumblefs'))
        logger.debug('done with main() =============================')
        return result

    def setoptions(self):
        self.conf = self.root = self.filter = self.query = self.cache = self.organizer = None     

        self.parser.add_option(mountopt="conf",
                               metavar="CONF",
                               default='~/.dejumblefs/default.xml',
                               help=_("read configuration from CONF file [default: %default]"))        
        self.parser.add_option(mountopt="root",
                               metavar="ROOT",
                               default='.',
                               help=_("root for all file operations (can be absolute or relative " +
                                      "to the mountpoint) [default: %default]"))
        self.parser.add_option(mountopt="filter",
                               metavar="FILTER",
                               default='OriginalDirectory',
                               help=_("use FILTER to handle QUERY [default: %default]"))
        self.parser.add_option(mountopt="query",
                               metavar="QUERY",
                               default='',
                               help=_("execute QUERY [default: %default]"))
        self.parser.add_option(mountopt="cache",
                               metavar="CACHE",
                               default='PassThrough',
                               help=_("use CACHE to handle caching [default: %default]"))
        self.parser.add_option(mountopt="organizer",
                               metavar="ORGANIZER",
                               default='Original',
                               help=_("use ORGANIZER [default: %default]"))

    def setup_organizer(self):
        # HACK: set defaults since fuse is not doing that
        defaults = self.parser.get_default_values()
        
        if not self.conf: self.conf = defaults.conf
        if not self.root: self.root = defaults.root
        if not self.filter: self.filter = defaults.filter
        if not self.query: self.query = defaults.query
        if not self.cache: self.cache = defaults.cache
        if not self.organizer: self.organizer = defaults.organizer
        # end HACK
        
        if self.root.endswith('/'):
            self.root = self.root[:-1]
        
        filter_ = self._loadclass('filters', 'FileListFilter', self.filter)(self.query, self.root)
        cache = self._loadclass('caches', 'Cache', self.cache)(filter_)
        self.organizer = self._loadclass('organizers', 'Organizer', self.organizer)(cache)
        logger.info(_('Done loading modules'))

    def _loadclass(self, moduleprefix, classsuffix, name):
        modulename = 'dejumble.%s.%s' % (moduleprefix, name.lower())
        classname = '%s%s' % (name, classsuffix)
        logger.info('Loading %s.%s' % (modulename, classname))
        return getattr(self._import(modulename), classname)

    def _import(self, name):
        mod = __import__(name)
        components = name.split('.')
        for comp in components[1:]:
            mod = getattr(mod, comp)
        return mod

    ############################################
    # Filesystem functions

    def fsinit(self):
        if self.filter == 'OriginalDirectory':
            os.fchdir(self.originaldir)
        os.close(self.originaldir)
        self.organizer.reset()
        logger.info(_('dejumblefs initialized!'))

    def fsdestroy(self):
        logger.debug('fsdestroy() =============================')

    def getattr(self, path):
        logger.debug('getattr(%s) =============================' % path)
        return self.organizer.getattr(path)

    def readdir(self, path, offset):
        logger.debug('readdir(%s, %s) =============================' % (path, offset))
        # FIXME: convert to list from generator to bring up errors
        return list(self.organizer.readdir(path, offset))

    def readlink(self, path):
        logger.debug('readlink(%s) =============================' % path)
        return self.organizer.cache.readlink(self.organizer.realpath(path))

    def unlink(self, path):
        logger.debug('unlink(%s) =============================' % path)
        self.organizer.cache.unlink(self.organizer.realpath(path))
        self.organizer.deletefromcache(path)

    def rename(self, path, pathdest):
        logger.debug('rename(%s, %s) =============================' % (path, pathdest))
        self.organizer.cache.rename(self.organizer.realpath(path), self.organizer.realpath(pathdest))
        self.organizer.deletefromcache(path)
        self.organizer.addtocache(pathdest)

    def chmod(self, path, mode):
        logger.debug('chmod(%s, %s) =============================' % (path, mode))
        self.organizer.cache.chmod(self.organizer.realpath(path), mode)

    def chown(self, path, user, group):
        logger.debug('chown(%s, %s, %s) =============================' % (path, user, group))
        self.organizer.cache.chown(self.organizer.realpath(path), user, group)

    def truncate(self, path, len):
        logger.debug('truncate(%s, %s) =============================' % (path, len))
        self.organizer.cache.truncate(self.organizer.realpath(path), len)

    def utime(self, path, times):
        logger.debug('utime(%s, %s) =============================' % (path, times))
        self.organizer.cache.utime(self.organizer.realpath(path), times)

    def access(self, path, mode):
        logger.debug('access(%s, %s) =============================' % (path, mode))
        self.organizer.cache.access(self.organizer.realpath(path), mode)
