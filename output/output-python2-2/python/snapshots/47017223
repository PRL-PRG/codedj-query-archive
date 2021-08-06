#!/usr/bin/env python

from PyDbLite import Base

import dejumble.util
from dejumble.util import *

DB_FILE = './.dejumbledb'
DB_FILE_METADATA = './.dejumbledb_metadata'


class Storage:
    def __init__(self):
        self.db = Base(DB_FILE)
        self.metadata = Base(DB_FILE_METADATA)
        self.reset()

    def reset(self):
        self.db.create('filename', 'realpath', mode = 'override')
        self.metadata.create('filename', 'type', 'value', 'type_value', mode = 'override')
        self.db.create_index('filename')
        self.metadata.create_index('type_value')

        for filename in getbasefilelist():
            self.savefile(filename, filename)

    def savefile(self, filename, realpath):
        while not self.realpath(filename) == None:
            filename = increasefilename(filename)

        self.db.insert(filename, realpath)

        f, extension = filenameextension(filename)
        self.setmetadata(filename, 'extension', extension)

    def realpath(self, filename):
        realpaths = [ r['realpath'] for r in self.db._filename[filename] ]
        if len(realpaths) == 0:
            return None
        else:
            return realpaths[0]

    def filelist(self):
        return [ r['filename'] for r in self.db ] 

    def metadatafilelist(self, type, *values):
        list = []

        for value in values:
            list += self.metadata._type_value[type + '=' + value]

        return [ r['filename'] for r in list ]

    def setmetadata(self, filename, type, value):
        if not value == None and not value == '':
            self.metadata.insert(filename, type, value, type  + '=' + value)

    def typelist(self, type):
        return unique([ r['value'] for r in self.metadata if r['type'] == type ])

