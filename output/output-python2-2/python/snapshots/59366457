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

from conf import config

logger = logging.getLogger('storage')

class Storage(object):
    """
    Store dynamic data for all the client modules
    """

    def __init__(self):
        build = not os.path.exists(config['storage']['path'])

        self.conn = sqlite3.connect(config['storage']['path'],
                                    isolation_level=None)

        if build:
            c = self.conn.cursor()
            c.execute('''create table connections(id integer primary key
                                                             autoincrement,
                                                  name text,
                                                  host text,
                                                  port integer,
                                                  def integer)''')

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

        logger.debug('sql: ' + sql)
        logger.debug('params:' + str(params))
        cursor.execute(sql, params)
        return cursor

    def connections(self):
        """
        Load the list of connections.

        :return: a list of tuples (id, name, host, port, default)
        """

        data = [row for row in self._execQuery('select * from connections')]
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
        self._execQuery('''insert into connections (name, host, port, def)
                           values(?, ?, ?, ?)''', conn[1:], c)

        conn[0] = self._execQuery('select id from connections where name = ?',
                                  (conn[1],), c).fetchone()[0]

        logger.debug('id connection obtained: ' + str(conn[0]))

    def deleteConnection(self, conn):
        self._execQuery('delete from connections where id = ?', (conn[0],))

    def updateConnection(self, conn):
        params = conn[1:]
        params.append(conn[0])
        self._execQuery('''update connections set name = ?, host = ?, port = ?,
                           def = ? where id = ?''', params)

