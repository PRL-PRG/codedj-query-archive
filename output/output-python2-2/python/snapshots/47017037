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

dejumble_obj = None

def setserver(server):
    global dejumble_obj
    dejumble_obj = server

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
        os.close(self.originaldir)
        return result

    def setup_organizer(self):
        # TODO: remove trailing slash from root if present
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

    def fsinit(self):
        os.fchdir(self.originaldir)
        self.organizer.reset()
        logger.info(_('dejumblefs initialized!'))

    def getattr(self, path):
        logger.debug('getattr(%s)' % path)
        return self.organizer.getattr(path)

    def readdir(self, path, offset):
        logger.debug('readdir(%s, %s)' % (path, offset))
        return self.organizer.readdir(path, offset)

    def readlink(self, path):
        logger.debug('readlink(%s)' % path)
        return self.organizer.cache.readlink(self.organizer.realpath(path))

    def unlink(self, path):
        logger.debug('unlink(%s)' % path)
        self.organizer.cache.unlink(self.organizer.realpath(path))
        self.organizer.expirecache()

    def rename(self, path, pathdest):
        logger.debug('rename(%s, %s)' % (path, pathdest))
        self.organizer.cache.rename(self.organizer.realpath(path), self.organizer.realpath(pathdest))
        self.organizer.expirecache()

    def chmod(self, path, mode):
        logger.debug('chmod(%s, %s)' % (path, mode))
        self.organizer.cache.chmod(self.organizer.realpath(path), mode)

    def chown(self, path, user, group):
        logger.debug('chown(%s, %s, %s)' % (path, user, group))
        self.organizer.cache.chown(self.organizer.realpath(path), user, group)

    def truncate(self, path, len):
        logger.debug('truncate(%s, %s)' % (path, len))
        self.organizer.cache.truncate(self.organizer.realpath(path), len)

    def utime(self, path, times):
        logger.debug('utime(%s, %s)' % (path, times))
        self.organizer.cache.utime(self.organizer.realpath(path), times)

    def access(self, path, mode):
        logger.debug('access(%s, %s)' % (path, mode))
        self.organizer.cache.access(self.organizer.realpath(path), mode)
