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

import sys
from sqlite3 import connect
from base64 import b64encode
from os.path import dirname, abspath

from PyQt4.QtCore import Qt

sys.path.append('../configobj')

from configobj import ConfigObj

def getConnections(curs):
    return [r for r in curs.execute('select id, name, host, port ' +
                                    'FROM connections')]

def getAliases(curs, conn_name):
    return [r for r in curs.execute('SELECT label, body FROM aliases AS a ' +
                                    'JOIN connections AS c ON a.id_conn = c.id ' +
                                    'WHERE c.name = ?', (conn_name,))]


def getMacros(curs, conn_name):
    return [r for r in curs.execute('SELECT command, shift, alt, ctrl, keycode ' +
                                    'FROM macros AS m JOIN connections AS c ' +
                                    'ON m.id_conn = c.id WHERE c.name = ?',
                                    (conn_name,))]

def getAccounts(curs, id_conn):
    return [r for r in curs.execute('SELECT id, username FROM accounts WHERE ' +
                                    'id_conn = ? ', (id_conn,))]

def getAccountDetail(curs, id_conn, username):
    return [r[0] for r in curs.execute('SELECT command FROM accounts AS a JOIN ' +
                                    'accounts_cmd AS c ON a.id=c.id_account WHERE ' +
                                    'id_conn = ? AND username = ? ORDER BY num',
                                    (id_conn, username))]

def getPrompt(curs, id_account):
    return curs.execute('SELECT normal, fight FROM accounts_prompt WHERE ' +
                        'id_account = ?', (id_account, )).fetchone()

def getPreferences(curs):
    return curs.execute('SELECT echo_text, echo_color, keep_text, ' +
                        'save_log FROM preferences').fetchone()

def option(curs, name, default, id_conn=0):

    curs.execute('SELECT param_value FROM options WHERE ' +
                 'id_conn = ? AND param_name = ?', (id_conn, name))

    row = curs.fetchone()
    if row:
        if type(default) == int:
            return int(row[0])
        else:
            return row[0]
    return default


def convert(db_file):
    conn = connect(db_file)
    curs = conn.cursor()

    connections = getConnections(curs)

    for c in connections:
        config = ConfigObj(None, options={'indent_type': '  '})
        config['id'] = c[0]
        config['name'] = c[1]
        config['host'] = c[2]
        config['port'] = c[3]
        config['default_account'] = option(curs, 'default_account', '', c[0])
        config.filename = dirname(db_file) + '/' + c[1] + '.save'
        print 'connessione:', c
        aliases = getAliases(curs, c[1])
        if aliases:
            config['aliases'] = {}
            for a in aliases:
                config['aliases'][a[0]] = a[1]

        macros = getMacros(curs, c[1])
        if macros:
            config['macros'] = {}
            i = 1
            for m in macros:
                config['macros']['%d' % i] = {}
                config['macros']['%d' % i]['keycode'] = m[4]
                config['macros']['%d' % i]['shift'] = m[1]
                config['macros']['%d' % i]['alt'] = m[2]
                config['macros']['%d' % i]['ctrl'] = m[3]
                config['macros']['%d' % i]['command'] = m[0]
                i += 1

        accounts = getAccounts(curs, c[0])
        if accounts:
            for a in accounts:
                if not a[1]:
                    continue
                if 'accounts' not in config:
                    config['accounts'] = {}
                config['accounts']['%s' % a[1]] = {}
                account_cmds = getAccountDetail(curs, c[0], a[1])
                pwd_idx = account_cmds.index(a[1]) + 1
                account_cmds[pwd_idx] = b64encode(account_cmds[pwd_idx])
                i = 1
                for cmd in account_cmds:
                    config['accounts']['%s' % a[1]]['cmd-%d' % i] = cmd
                    i += 1

                prompt = getPrompt(curs, a[0])
                if prompt:
                    config['accounts']['%s' % a[1]]['normal_prompt'] = prompt[0]
                    config['accounts']['%s' % a[1]]['fight_prompt'] = prompt[1]

        config.write()

        config = ConfigObj(options={'indent_type': '  '})
        config.filename = dirname(db_file) + '/general.save'
        p = getPreferences(curs)
        config['echo_text'] = p[0]
        config['echo_color'] = p[1]
        config['keep_text'] = p[2]
        config['save_log'] = p[3]

        config['save_account'] = option(curs, 'save_account', 0)
        config['default_connection'] = option(curs, 'default_connection', 0)

        config.write()


if __name__ == '__main__':
    convert(abspath('../../data/storage/db.sqlite'))
