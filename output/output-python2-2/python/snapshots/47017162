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

logger = logging.getLogger('dejumble')


class DejumbleFS(Fuse):
    def main(self, *a, **kw):
        logger.info(_('Initializing dejumblefs'))
        self.setup_organizer()
        self.file_class = self.organizer.cache.DejumbleFile
        self.originaldir = os.open(self.fuse_args.mountpoint, os.O_RDONLY)
        try:
            result = self.main(*a, **kw)
        except fuse.FuseError:
            result = -errno.ENOENT
            logger.warn(_('Finalizing dejumblefs'))
        os.close(self.originaldir)
        return result

    def setup_organizer(self):
        what = 'dejumble.filters.%s.%sFileListFilter' % (self.filter.lower(), self.filter)
        logger.info('Loading filter %s(%s)' % (what, self.query))
        filter_ = __import__(what)(self.query, self.root)

        what = 'dejumble.caches.%s.%sCache' % (self.cache.lower(), self.cache)
        logger.info('Loading cache %s' % what)
        cache = __import__(what)(filter_)

        what = 'dejumble.organizers.%s.%sOrganizer' % (self.organizer.lower(), self.organizer)
        logger.info('Loading organizer %s' % what)
        self.organizer = __import__(what)(cache)

    def fsinit(self):
        os.fchdir(self.originaldir)
        self.organizer.reset()

    def getattr(self, path):
        return self.organizer.getattr(path)

    def readdir(self, path, offset):
        return self.organizer.readdir(path, offset)

    def readlink(self, path):
        return self.organizer.cache.readlink(self.organizer.realpath(path))

    def unlink(self, path):
        self.organizer.expirecache()
        self.organizer.cache.unlink(self.organizer.realpath(path))

    def rename(self, path, pathdest):
        self.organizer.expirecache()
        self.organizer.cache.rename(self.organizer.realpath(path), self.organizer.realpath(pathdest))

    def chmod(self, path, mode):
        self.organizer.cache.chmod(self.organizer.realpath(path), mode)

    def chown(self, path, user, group):
        self.organizer.cache.chown(self.organizer.realpath(path), user, group)

    def truncate(self, path, len):
        self.organizer.cache.truncate(self.organizer.realpath(path), len)

    def utime(self, path, times):
        self.organizer.cache.utime(self.organizer.realpath(path), times)

    def access(self, path, mode):
        self.organizer.cache.access(self.organizer.realpath(path), mode)
