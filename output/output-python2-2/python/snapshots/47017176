#!/usr/bin/env python

import time

from PyDbLite import Base

import dejumble.util
from dejumble.util import *

DB_FILES = './.dejumbledb'
DB_FILE_TAGS = './.dejumbledb_tags'


class Cache:
    def __init__(self, filter_):
        self.filter = filter_
        self.files = Base(DB_FILES)
        self.tags = Base(DB_FILE_TAGS)
        self.reset()

    def reset(self):
        self.files.create('realpath', mode = 'override')
        self.files.create_index('filename')

        self.tags.create('realpath', 'category', 'tag', mode = 'override')
        self.tags.create_index('category')

        self.expirecache()
        self.refreshcache()

    def expirecache(self):
        self.expiretime = time.time()

    def refreshcache(self):
        if self.expiretime < time.time():
            self.expiretime = time.time() + 60
            self.refreshfilelist()

    def refreshfilelist(self):
        for realpath in self.filter.filelist():
            self.addfile(realpath)

    def addfile(self, realpath):
        self.files.insert(realpath)

    def filelist(self):
        return [ r['realpath'] for r in self.files ]

    ############################################
    # Tag management

    def tag(self, realpath, category, tag):
        if not tag == None and not tag == '':
            self.tags.insert(realpath, category, tag)

    def pathlistbytags(self, category, tags):
        return [ r['realpath'] for r in self.tags._category[category] if r['tag'] in tags ]

    def taglist(self, category):
        return unique([ r['tag'] for r in self.tags._category[category] ])

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
        os.chmod(realpath, mode)

    def chown(self, path, user, group):
        os.chown(realpath, user, group)

    def truncate(self, realpath, len):
        f = open(realpath, 'a')
        f.truncate(len)
        f.close()

    def utime(self, realpath, times):
        os.utime(realpath, times)

    def access(self, realpath, mode):
        if not os.access(realpath, mode):
            return -errno.EACCES


    class DejumbleFile(object):
        def __init__(self, path, flags, *mode):
            global server
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

