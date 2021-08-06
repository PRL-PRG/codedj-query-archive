#!/usr/bin/env python

import commands
import logging
import errno

logger = logging.getLogger('searchfs')

def getHandler(name):
    return {
        'Search': SearchHandler
    }[name]()

def filenamepart(path):
    return path.rsplit('/', 1)[-1]


class Handler:
    def realpath(self, path):
        return self.files[path]

    def filelist(self, path):
        return self.files.iteritems()

    def executequery(self, query):
        return -errno.ENOENT


class SearchHandler(Handler):
    def executequery(self, query):
        logger.debug('executequery(' + query + ')')
        self.files = { '..': '..', '.': '.' }
        filenames = commands.getoutput(query).splitlines()
        logger.debug('result (first line): ' + filenamepart(filenames[0]));
        for r in filenames:
    	    # TODO: Watch out for duplicates
            self.files['/' + filenamepart(r)] = r
        logger.debug('result (last line): ' + filenamepart(r));



