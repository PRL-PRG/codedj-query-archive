#!/usr/bin/env python

from PyDbLite import Base

import dejumble.util
from dejumble.util import *

DB_FILE = './.dejumbledb'
DB_FILE_TAGS = './.dejumbledb_tags'


class Storage:
    def __init__(self):
        self.db = Base(DB_FILE)
        self.tags = Base(DB_FILE_TAGS)
        self.reset()

    def reset(self):
        self.db.create('filename', 'realpath', mode = 'override')
        self.db.create_index('filename')

        self.tags.create('filename', 'category', 'tag', mode = 'override')
        self.tags.create_index('category')

        for filename in getbasefilelist():
            self.savefile(filename, filename)

    def savefile(self, filename, realpath):
        while not self.realpath(filename) == None:
            filename = increasefilename(filename)

        self.db.insert(filename, realpath)

    def tag(self, filename, category, tag):
        if not tag == None and not tag == '':
            self.tags.insert(filename, category, tag)

    def realpath(self, filename):
        realpaths = [ r['realpath'] for r in self.db._filename[filename] ]
        if len(realpaths) == 0:
            return None
        else:
            return realpaths[0]

    def filelist(self):
        return [ r['filename'] for r in self.db ] 

    def filelistbytags(self, category, tags):
        return [ r['filename'] for r in self.tags._category[category] if r['tag'] in tags ]

    def taglist(self, category):
        return unique([ r['tag'] for r in self.tags._category[category] ])

