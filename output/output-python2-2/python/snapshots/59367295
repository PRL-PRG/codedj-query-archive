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

import logging
from sqlite3 import connect, OperationalError
import exception
from conf import config

logger = logging.getLogger('storage')

def adjustSchema():
    c = connect(config['storage']['path'], isolation_level=None).cursor()
    c.execute('''CREATE TABLE IF NOT EXISTS
                        connections(id integer PRIMARY KEY AUTOINCREMENT,
                                    name text,
                                    host text,
                                    port integer)''')

    # To prevent a windows bug on 'IF NOT EXISTS' clause of CREATE TRIGGER
    try:
        c.execute('''DROP TRIGGER connection_delete_trg''')
    except OperationalError:
        pass

    c.execute('''CREATE TRIGGER connection_delete_trg
                        AFTER DELETE ON connections
                        BEGIN
                            DELETE FROM aliases WHERE id_conn=old.id;
                            DELETE FROM macros WHERE id_conn=old.id;
                            DELETE FROM accounts WHERE id_conn=old.id;
                            DELETE FROM options WHERE id_conn=old.id;
                        END''')


    c.execute('''CREATE TABLE IF NOT EXISTS
                        aliases(id_conn integer,
                                label text,
                                body text)''')

    c.execute('''CREATE INDEX IF NOT EXISTS aliases_conn_idx ON
                    aliases(id_conn)''')

    c.execute('''CREATE TABLE IF NOT EXISTS
                        macros(id_conn integer,
                                command text,
                                shift integer,
                                alt integer,
                                ctrl integer,
                                keycode integer)''')

    c.execute('''CREATE INDEX IF NOT EXISTS macros_conn_idx ON
                        macros(id_conn)''')

    c.execute('''CREATE TABLE IF NOT EXISTS
                        preferences(echo_text integer,
                                    echo_color text,
                                    keep_text integer,
                                    save_log integer)''')

    c.execute('''CREATE TABLE IF NOT EXISTS
                        accounts(id integer PRIMARY KEY AUTOINCREMENT,
                                    id_conn integer,
                                    username text,
                                    UNIQUE (id_conn, username))''')

    try:
        c.execute('''DROP TRIGGER account_delete_trg''')
    except OperationalError:
        pass

    c.execute('''CREATE TRIGGER account_delete_trg AFTER DELETE ON accounts
                        BEGIN
                            DELETE FROM accounts_cmd WHERE id_account=old.id;
                            DELETE FROM accounts_prompt WHERE id_account=old.id;
                        END''')

    c.execute('''CREATE TABLE IF NOT EXISTS
                        accounts_cmd(id_account integer,
                                        num integer,
                                        command text)''')

    c.execute('''CREATE INDEX IF NOT EXISTS accounts_cmd_idx ON
                        accounts_cmd(id_account)''')

    c.execute('''CREATE TABLE IF NOT EXISTS
                        options(param_name text,
                                param_value text,
                                id_conn int,
                                PRIMARY KEY (param_name, id_conn))''')

    c.execute('''CREATE TABLE IF NOT EXISTS
                        accounts_prompt(id_account integer PRIMARY KEY,
                                        normal text,
                                        fight text)''')


class Option(object):
    SAVE_ACCOUNT = 'save_account'
    DEFAULT_ACCOUNT = 'default_account'
    DEFAULT_CONNECTION = 'default_connection'


class Storage(object):
    """
    Store dynamic data for all the client modules
    """

    def __init__(self):
        self.conn = connect(config['storage']['path'], isolation_level=None)
        c = self.conn.cursor()

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

        :return: a list of tuples (id, name, host, port)
        """

        data = [row for row in self._execQuery('SELECT id, name, host, port ' +
                                               'FROM connections')]
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
        self._execQuery('INSERT INTO connections (name, host, port) ' +
                        'VALUES(?, ?, ?)', conn[1:], c)

        conn[0] = self.getIdConnection(conn[1], c)
        logger.debug('id connection obtained: %d' % conn[0])

    def deleteConnection(self, conn):
        c = self.conn.cursor()
        self._execQuery('DELETE FROM connections WHERE id = ?', (conn[0],), c)

    def updateConnection(self, conn):
        params = conn[1:]
        params.append(conn[0])
        self._execQuery('UPDATE connections SET name = ?, host = ?, port = ? ' +
                        'WHERE id = ?', params)

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

    def preferences(self):
        """
        Return the list of preferences.

        :return: a tuple (echo_text, echo_color, keep_text, save_log)
        """

        c = self._execQuery('SELECT echo_text, echo_color, keep_text, ' +
                            'save_log FROM preferences')
        row = c.fetchone()
        return row if row else ()

    def savePreferences(self, preferences):
        self._execQuery('DELETE FROM preferences')
        self._execQuery('INSERT INTO preferences VALUES(?, ?, ?, ?)',
                        preferences)

    def saveAccount(self, commands, id_conn, cmd_user):
        username = commands[cmd_user - 1]

        c = self._execQuery('SELECT id FROM accounts WHERE id_conn=? ' +
                            'AND username=?', (id_conn, username))
        r = c.fetchone()
        if r: # update the account replacing old command
            id_account = r[0]
            self._execQuery('DELETE FROM accounts_cmd WHERE id_account=?',
                            (id_account,))
        else:
            self._execQuery('INSERT INTO accounts(id_conn, username) ' +
                            'VALUES(?, ?)', (id_conn, username), c)
            id_account = c.lastrowid

        for num, cmd in enumerate(commands):
            self._execQuery('INSERT INTO accounts_cmd VALUES (?, ?, ?)',
                            (id_account, num, cmd), c)

    def accounts(self, id_conn):
        """
        Return the list of (username of) account for a connection.

        :Parameters:
          id_conn : int
            the id of connection.
        """

        c = self._execQuery('SELECT username FROM accounts WHERE id_conn = ? ',
                            (id_conn,))

        return [row[0] for row in c]

    def accountDetail(self, id_conn, username):
        c = self._execQuery('SELECT command FROM accounts AS a JOIN ' +
                            'accounts_cmd AS c ON a.id=c.id_account WHERE ' +
                            'id_conn = ? AND username = ? ORDER BY num',
                            (id_conn, username))

        return [row[0] for row in c]

    def deleteAccount(self, id_conn, username):
        self._execQuery('DELETE FROM accounts WHERE id_conn=? AND username=?',
                        (id_conn, username))

    def option(self, name, default, id_conn=0):
        """
        Return the value of an option.

        :Parameters:
          name : str
            the name of the option.

          default : mix
            the default value of the option.

          id_conn : int
            the id of connection.
        """

        c = self._execQuery('SELECT param_value FROM options WHERE ' +
                            'id_conn = ? AND param_name = ?', (id_conn, name))

        row = c.fetchone()
        if row:
            if type(default) == int:
                return int(row[0])
            else:
                return row[0]
        return default

    def setOption(self, name, value, id_conn=0):
        self._execQuery('REPLACE INTO options VALUES(?, ?, ?)',
                        (name, value, id_conn))

    def deleteAccount(self, id_conn, username):
        self._execQuery('DELETE FROM accounts WHERE id_conn=? AND username=?',
                        (id_conn, username))

    def savePrompt(self, id_conn, username, normal, fight):
        c = self._execQuery('SELECT id FROM accounts WHERE id_conn=? ' +
                            'AND username=?', (id_conn, username))

        id_account = c.fetchone()[0]
        self._execQuery('REPLACE INTO accounts_prompt VALUES(?, ?, ?)',
                        (id_account, normal, fight), c)

    def prompt(self, id_conn, username):
        if not username:
            return ('', '')

        c = self._execQuery('SELECT id FROM accounts WHERE id_conn=? ' +
                            'AND username=?', (id_conn, username))

        id_account = c.fetchone()[0]
        self._execQuery('SELECT normal, fight FROM accounts_prompt WHERE ' +
                        'id_account = ?', (id_account, ), c)

        r = self._execQuery('SELECT normal, fight FROM accounts_prompt WHERE ' +
                            'id_account = ?', (id_account, ), c).fetchone()

        return (r[0], r[1]) if r else ('', '')