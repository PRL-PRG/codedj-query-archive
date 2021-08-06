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

import dejumble.organizers
from dejumble.organizers import *
import dejumble.util
from dejumble.util import *

fuse.fuse_python_api = (0, 2)

logger = logging.getLogger('dejumblefs.main')


class DejumbleFS(Fuse):
    def main(self, *a, **kw):
        global server
        logger.info(_('Initializing dejumblefs'))
        server = self 
        self.file_class = self.DejumbleFile
        self.organizer = getorganizer(self.organizer, self.provider, self.query)
        self.originaldir = os.open(self.fuse_args.mountpoint, os.O_RDONLY)
        try:
            result = Fuse.main(self, *a, **kw)
        except fuse.FuseError:
            result = -errno.ENOENT 
            logger.warn(_('Finalizing dejumblefs'))
        os.close(self.originaldir)
        return result

    def fsinit(self):
        os.fchdir(self.originaldir)

    def getattr(self, path):
        if path == '/':
            return os.lstat('.')
        else:
            logger.debug('getattr(' + path + ')')
            logger.debug('getattr ' + self.organizer.realpath(path))
            return os.lstat(self.organizer.realpath(path))

    def readdir(self, path, offset):
        logger.debug('readdir(' + path + ')')
        for filename in self.organizer.filelist(path):
            yield fuse.Direntry(filename)

    def readlink(self, path):
        return os.readlink(self.organizer.realpath(path))

    def unlink(self, path):
        os.unlink(self.organizer.realpath(path))
        self.organizer.expirecache()


    def rename(self, path, pathdest):
        dirname = os.path.dirname(path)
        dirnamedest = os.path.dirname(pathdest)
        if dirname == dirnamedest:
            filenamedest = os.path.basename(pathdest)
            realpath = self.organizer.realpath(path)
            realdirname = os.path.dirname(realpath)
            os.rename(realpath, os.path.join(realdirname, filenamedest))
        else:
            return -errno.ENOENT
        self.organizer.expirecache()

    def chmod(self, path, mode):
        os.chmod(self.organizer.realpath(path), mode)

    def chown(self, path, user, group):
        os.chown(self.organizer.realpath(path), user, group)

    def truncate(self, path, len):
        f = open(self.organizer.realpath(path), 'a')
        f.truncate(len)
        f.close()

    def utime(self, path, times):
        os.utime(self.organizer.realpath(path), times)

    def access(self, path, mode):
        if not os.access(self.organizer.realpath(path), mode):
            return -errno.EACCES

    class DejumbleFile(object):
        def __init__(self, path, flags, *mode):
            f = os.open(server.organizer.realpath(path), flags, *mode)
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


