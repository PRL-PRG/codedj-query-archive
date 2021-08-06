import errno
import logging
import os

from PyDbLite import Base

from . import util
from .util import Cacheable
from .fs import getserver, CommandHandler

DB_FILES = './.dejumblefs_cache.pydblite'

logger = logging.getLogger('dejumblefs.Cache')


class Cache(Cacheable):
    """
    This is the base class for the caching system
    """

    def __init__(self, filter_):
        self.filter = filter_
        self.files = None
        Cacheable.__init__(self)

    def reset(self):
        if not self.files:
            self.files = Base(DB_FILES)
        self.files.create('realpath', mode = 'override')
        self.files.create_index('realpath')
        Cacheable.reset(self)

    def updatecache(self):
        for realpath in self.filter.filelist():
            self.files.insert(realpath)

    def deletefromcache(self, realpath):
        for file in self.files.get_index('realpath')[realpath]:
            self.files.delete(file)

    def addtocache(self, realpath):
        self.files.insert(realpath)

    def filelist(self):
        self.refreshcache()
        for record in self.files:
            yield record['realpath']

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

    def truncate(self, realpath, length):
        logger.debug('truncate(%s, %s)' % (realpath, length))
        ofile = open(realpath, 'a')
        ofile.truncate(length)
        ofile.close()

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
            self.keep_cache = False
            self.direct_io = False

            if util.iscommand(path):
                self.file = CommandHandler(path, *mode)
            else:
                self.file = self.getfile(path, flags, *mode)

            if flags & os.O_CREAT:
                getserver().organizer.addtocache(path)

        def getfile(self, path, flags, *mode):
            realpath = getserver().organizer.realpath(path)
            file = open(realpath, util.flags2mode(flags))
            return file

        def read(self, length, offset):
            logger.debug('DejumbleFile.read(%s, %s)' % (length, offset))
            self.file.seek(offset)
            return self.file.read(length)

        def write(self, data, offset):
            logger.debug('DejumbleFile.write(%s, %s)' % (len(data), offset))
            self.file.seek(offset)
            self.file.write(data)
            return len(data)

        def release(self, flags):
            logger.debug('DejumbleFile.release(%s)' % flags)
            self.file.close()

        def _fflush(self):
            if 'w' in self.file.mode or 'a' in self.file.mode:
                self.file.flush()

        def fsync(self, isfsyncfile):
            logger.debug('DejumbleFile.fsync(%s)' % isfsyncfile)
            if hasattr(self.file, 'fileno'):
                self._fflush()
                if isfsyncfile and hasattr(os, 'fdatasync'):
                    os.fdatasync(self.file.fileno())
                else:
                    os.fsync(self.file.fileno())

        def flush(self):
            logger.debug('DejumbleFile.flush()')
            self._fflush()
            if hasattr(self.file, 'fileno'):
                os.close(os.dup(self.file.fileno()))

        def fgetattr(self):
            logger.debug('DejumbleFile.fgetattr()')
            if hasattr(self.file, 'fileno'):
                return os.fstat(self.file.fileno())

        def ftruncate(self, length):
            logger.debug('DejumbleFile.ftruncate()')
            self.file.truncate(length)
