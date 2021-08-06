#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright (C) 2007 Gianni Valdambrini, Develer S.r.l (http://www.develer.com)
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.
#
# Author: Gianni Valdambrini gvaldambrini@develer.com

__version__ = "$Revision$"[11:-2]
__docformat__ = 'restructuredtext'

import os.path
import sqlite3
import logging

import exception
from conf import config

logger = logging.getLogger('storage')

class Storage(object):
    """
    Store dynamic data for all the client modules
    """

    def __init__(self):

        self.conn = sqlite3.connect(config['storage']['path'],
                                    isolation_level=None)

        c = self.conn.cursor()
        c.execute('''CREATE TABLE IF NOT EXISTS
                            connections(id integer primary key autoincrement,
                                        name text,
                                        host text,
                                        port integer,
                                        def integer)''')

        c.execute('''CREATE TABLE IF NOT EXISTS
                            aliases(id_conn integer,
                                    label text,
                                    body text)''')

        c.execute('''CREATE TABLE IF NOT EXISTS
                            macros(id_conn integer,
                                   command text,
                                   shift integer,
                                   alt integer,
                                   ctrl integer,
                                   keycode integer)''')

    def _execQuery(self, sql, params=(), cursor=None):
        """
        Execute a query.

        :Parameters:
          sql : str
            the string of query
          params : tuple
            the tuple of params
          cursor : object
            the cursor object

        :return: a cursor object.
        """

        if not cursor:
            cursor = self.conn.cursor()

        cursor.execute(sql, params)

        for p in params:
            sql = sql.replace('?', "'%s'" % p, 1)
        logger.debug('sql: ' + sql)

        return cursor

    def connections(self):
        """
        Load the list of connections.

        :return: a list of tuples (id, name, host, port, default)
        """

        data = [row for row in self._execQuery('SELECT * FROM connections')]
        return data

    def addConnection(self, conn):
        """
        Add a new connection at list of connections.

        :Parameters:
          conn : list
            the params of connection to add. The id param should be return
            valued.
        """

        c = self.conn.cursor()
        self._execQuery('INSERT INTO connections (name, host, port, def)' +
                        'VALUES(?, ?, ?, ?)', conn[1:], c)

        conn[0] = self.getIdConnection(conn[1], c)
        logger.debug('id connection obtained: %d' % conn[0])

    def deleteConnection(self, conn):
        c = self.conn.cursor()
        self._execQuery('DELETE FROM connections WHERE id = ?', (conn[0],), c)
        self._execQuery('DELETE FROM aliases WHERE id_conn = ?', (conn[0],), c)

    def updateConnection(self, conn):
        params = conn[1:]
        params.append(conn[0])
        self._execQuery('UPDATE connections SET name = ?, host = ?, port = ?,' +
                        'def = ? WHERE id = ?', params)

    def getIdConnection(self, conn_name, cursor=None):
        row = self._execQuery('SELECT id FROM connections WHERE name = ?',
                              (conn_name,), cursor).fetchone()

        if not row:
            raise exception.ConnectionNotFound

        return row[0]

    def aliases(self, conn_name):
        """
        Load the list of alias for a connection.

        :Parameters:
          conn_name : str
            the name of connection.

        :return: a list of tuples (label, body)
        """

        c = self._execQuery('SELECT label, body FROM aliases AS a ' +
                            'JOIN connections AS c ON a.id_conn = c.id ' +
                            'WHERE c.name = ?', (conn_name,))
        return [row for row in c]

    def saveAliases(self, conn_name, aliases):

        c = self.conn.cursor()
        id_conn = self.getIdConnection(conn_name, c)
        self._execQuery('DELETE FROM aliases WHERE id_conn = ?', (id_conn,))

        for alias in aliases:
            self._execQuery('INSERT INTO aliases VALUES(?, ?, ?)',
                            (id_conn, alias[0], alias[1]), c)

    def macros(self, conn_name):
        """
        Load the list of macro for a connection.

        :Parameters:
          conn_name : str
            the name of connection.

        :return: a list of tuples (command, shift, alt, ctrl, keycode)
        """

        c = self._execQuery('SELECT command, shift, alt, ctrl, keycode ' +
                            'FROM macros AS m JOIN connections AS c ' +
                            'ON m.id_conn = c.id WHERE c.name = ?',
                            (conn_name,))

        return [row for row in c]

    def saveMacros(self, conn_name, macros):
        c = self.conn.cursor()
        id_conn = self.getIdConnection(conn_name, c)

        self._execQuery('DELETE FROM macros WHERE id_conn = ?', (id_conn,))

        for m in macros:
            p = list(m)
            p.insert(0, id_conn)
            self._execQuery('INSERT INTO macros VALUES(?, ?, ?, ?, ?, ?)',
                            p, c)