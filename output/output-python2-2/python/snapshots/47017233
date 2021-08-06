#!/usr/bin/env python

from pysqlite2 import dbapi2 as sqlite

import dejumble.util
from dejumble.util import *

#DB_FILE = '/tmp/dejumblefsdb'
DB_FILE = ':memory:'

class Storage:
    def __init__(self):
        self.reset()

    def reset(self):
        self._reset()
        for filename in getbasefilelist():
            self.savefile(filename, filename)

    def savefile(self, filename, realpath):
        while not self._realpath(filename) == None:
            filename = increasefilename(filename)
        self._savefile(filename, realpath)

    def realpath(self, filename):
        return self._realpath(filename)

    def filelist(self):
        return self._filelist()

class MemoryStorage(Storage):
    def _reset(self):
        self.files = {}

    def _savefile(self, filename, realpath):
        self.files[filename] = realpath

    def _realpath(self, filename):
        if filename in self.files:
            return self.files[filename]
        else:
            return None

    def _filelist(self):
        return self.files.keys()


class DBStorage(Storage):
    def __init__(self):
        self._initdb()
        Storage.init(self)

    def _initdb(self):
        logger.debug('_initdb')
        if os.path.exists(DB_FILE):
            os.remove(DB_FILE)
        cur = DejumbleCursor()
        cur.execute(pkg_resources.resource_string('dejumble', 'conf/schema.sql'))
        cur.close()

    def _reset(self):
        logger.debug('_resetdb')
        cur = DejumbleCursor()
        cur.execute('DELETE FROM files')
        cur.close()

    def _savefile(self, filename, realpath):
        cur = DejumbleCursor()
        cur.execute('INSERT INTO files (filename, realpath) VALUES (?, ?)', (filename, realpath))
        cur.close()

    def _realpath(self, filename):
        cur = DejumbleCursor()
        cur.execute('SELECT realpath FROM files WHERE filename=?', (filename,))
        result = cur.fetchone()
        cur.close()
        return extract(result)

    def _filelist(self):
        cur = DejumbleCursor()
        cur.execute('SELECT filename FROM files')
        results = cur.fetchall()
        cur.close()
        return map(extract, results)


class DejumbleCursor(sqlite.Cursor):
    def __init__(self):
        self.db = sqlite.connect(DB_FILE, isolation_level=None)
        sqlite.Cursor.__init__(self, self.db)

    def execute(self, *args, **kwargs):
        logging.debug('execute ' + str(args) + " :: " + str(kwargs))
        sqlite.Cursor.execute(self, *args, **kwargs)

    def close(self):
        sqlite.Cursor.close(self)
        self.db.close()

