#!/usr/bin/env python

import time

from PyDbLite import Base

import dejumble.util
from dejumble.util import *
import dejumble.cacheable
from dejumble.cacheable import *

DB_FILES = './.dejumbledb'


class Cache(Cacheable):
    """
    This is the base class for the caching system
    """

    def __init__(self, filter_):
        self.filter = filter_
        self.files = Base(DB_FILES)
        self.reset()

    def reset(self):
        self.files.create('realpath', mode = 'override')
        self.files.create_index('filename')
        Cacheable.reset(self)

    def updatecache(self):
        for realpath in self.filter.filelist():
            self.files.insert(realpath)

    def filelist(self):
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
        logger.debug('rename(%s)' % realpath)
        dirname = os.path.dirname(realpath)
        dirnamedest = os.path.dirname(pathdest)
        if dirname == dirnamedest:
            filenamedest = os.path.basename(pathdest)
            realpath = self.organizer.path(realpath)
            dirname = os.path.dirname(path)
            os.rename(path, os.path.join(dirname, filenamedest))
        else:
            return -errno.ENOENT
        self.organizer.expirecache()

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
            global server
            realpath = server.organizer.realpath(path)
            if not os.path.exists(realpath):
                server.organizer.expirecache()
            f = os.open(realpath, flags, *mode)
            self.file = os.fdopen(f, flags2mode(flags))
            self.fd = self.file.fileno()

        def read(self, length, offset):
            self.file.seek(offset)
            return self.file.read(length)

        def write(self, buf, offset):
            self.file.seek(offset)
            self.file.write(buf)
            return len(buf)

        def release(self, flags):
            self.file.close()

        def _fflush(self):
            if 'w' in self.file.mode or 'a' in self.file.mode:
                self.file.flush()

        def fsync(self, isfsyncfile):
            self._fflush()
            if isfsyncfile and hasattr(os, 'fdatasync'):
                os.fdatasync(self.fd)
            else:
                os.fsync(self.fd)

        def flush(self):
            self._fflush()
            os.close(os.dup(self.fd))

        def fgetattr(self):
            return os.fstat(self.fd)

        def ftruncate(self, len):
            self.file.truncate(len)

