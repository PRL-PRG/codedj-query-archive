#!/usr/bin/env python

import time
import errno
import logging
import os

from PyDbLite import Base

import dejumble.util
from dejumble.util import *
import dejumble.fs
from dejumble.fs import *

DB_FILES = './.dejumbledb'

logger = logging.getLogger('dejumble.Cache')


class Cache(Cacheable):
    """
    This is the base class for the caching system
    """

    def __init__(self, filter):
        self.filter = filter
        self.files = Base(DB_FILES)
        self.reset()

    def reset(self):
        self.files.create('realpath', mode = 'override')
        self.files.create_index('realpath')
        Cacheable.reset(self)

    def updatecache(self):
        for realpath in self.filter.filelist():
            self.files.insert(realpath)

    def deletefromcache(self, realpath):
        for r in self.files._realpath[realpath]:
            del self.files[r['__id__']]

    def addtocache(self, realpath):
        self.files.insert(realpath)

    def filelist(self):
        self.refreshcache()
        [ (yield r['realpath']) for r in self.files ]

    ############################################
    # Original filesystem functions

    def getattr(self, realpath):
        logger.debug('getattr(%s)' % realpath)
        return os.lstat(realpath)

    def readlink(self, realpath):
        logger.debug('readlink(%s)' % realpath)
        return os.readlink(realpath)

    def unlink(self, realpath):
        logger.debug('unlink(%s)' % realpath)
        os.unlink(realpath)
        self.expirecache()

    def rename(self, realpath, pathdest):
        logger.debug('rename(%s, %s)' % (realpath, pathdest))
        os.rename(realpath, pathdest)
        self.expirecache()

    def chmod(self, realpath, mode):
        logger.debug('chmod(%s, %s)' % (realpath, mode))
        os.chmod(realpath, mode)

    def chown(self, realpath, user, group):
        logger.debug('chown(%s, %s, %s)' % (realpath, user, group))
        os.chown(realpath, user, group)

    def truncate(self, realpath, len):
        logger.debug('truncate(%s, %s)' % (realpath, len))
        f = open(realpath, 'a')
        f.truncate(len)
        f.close()

    def utime(self, realpath, times):
        logger.debug('utime(%s, %s)' % (realpath, times))
        os.utime(realpath, times)

    def access(self, realpath, mode):
        logger.debug('access(%s, %s)' % (realpath, mode))
        if not os.access(realpath, mode):
            return -errno.EACCES

    ############################################
    # File functions embedded in a class

    class DejumbleFile(object):
        """
        This is the base class to manage a File on the caching system.
        """

        def __init__(self, path, flags, *mode):
            logger.debug('DejumbleFile.__init__(%s, %s)' % (path, flags))
            if flags & os.O_CREAT:
                getserver().organizer.addtocache(path)
            realpath = getserver().organizer.realpath(path)
            self.fd = os.open(realpath, flags, *mode)
            self.file = os.fdopen(self.fd, flags2mode(flags))

        def read(self, length, offset):
            logger.debug('DejumbleFile.read(%s, %s)' % (length, offset))
            self.file.seek(offset)
            return self.file.read(length)

        def write(self, buf, offset):
            logger.debug('DejumbleFile.write(%s, %s)' % (len(buf), offset))
            self.file.seek(offset)
            self.file.write(buf)
            return len(buf)

        def release(self, flags):
            logger.debug('DejumbleFile.release(%s)' % flags)
            self.file.close()

        def _fflush(self):
            if 'w' in self.file.mode or 'a' in self.file.mode:
                self.file.flush()

        def fsync(self, isfsyncfile):
            logger.debug('DejumbleFile.fsync(%s)' % isfsyncfile)
            self._fflush()
            if isfsyncfile and hasattr(os, 'fdatasync'):
                os.fdatasync(self.fd)
            else:
                os.fsync(self.fd)

        def flush(self):
            logger.debug('DejumbleFile.flush()')
            self._fflush()
            os.close(os.dup(self.fd))

        def fgetattr(self):
            logger.debug('DejumbleFile.fgetattr()')
            return os.fstat(self.fd)

        def ftruncate(self, len):
            logger.debug('DejumbleFile.ftruncate()')
            self.file.truncate(len)

