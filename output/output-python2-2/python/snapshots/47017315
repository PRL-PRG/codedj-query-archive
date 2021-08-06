#!/usr/bin/env python

import commands
import logging
import errno
import time

logger = logging.getLogger('searchfs')

def getHandler(name, query):
    logger.debug('getHandler(' + name + ', ' + query + ')')
    return {
        'Null': NullHandler,
        'Search': SearchHandler
    }[name](query)

def filenamepart(path):
    return path.rsplit('/', 1)[-1]


class Handler:
    def __init__(self, query):
        self.query = query
        self.expirequery()

    def realpath(self, path):
        self.executequery()
        return self.files[path]

    def filelist(self, path):
        self.executequery()
        # TODO: handle subdirectories
        if path == '/':
            return self.files.iteritems()
        else:
            return os.listdir(self.realpath(path))

    def expirequery(self):
        self.expiretime = time.time()

    def executequery(self):
        if self.expiretime < time.time():
            logger.info('Executing query ' + self.query);
            self.expiretime = time.time() + 60
            self._executequery()

    def _executequery(self):
        return -errno.ENOENT


class NullHandler(Handler):
    def _executequery(self):
        self.files = { '/..': '..', '/.': '.', '/null': '/dev/null' }


class SearchHandler(Handler):
    def _executequery(self):
        self.files = { '/..': '..', '/.': '.' }
        filenames = commands.getoutput(self.query).splitlines()
        # TODO: Check response code from command
        for r in filenames:
    	    # TODO: Watch out for duplicates
            self.files['/' + filenamepart(r)] = r


