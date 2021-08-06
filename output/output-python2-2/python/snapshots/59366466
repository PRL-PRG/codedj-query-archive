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

__version__ = "$Revision:$"[11:-2]
__docformat__ = 'restructuredtext'

import os.path
import sqlite3

from conf import config

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
            c.execute('''create table connections(name text,
                                                  host text,
                                                  port int,
                                                  def int)''')

    def connections(self):
        """
        Load the list of connections.

        :return: a list of tuples (name, host, port, default)
        """

        c = self.conn.cursor()
        c.execute('select * from connections')
        data =  [row for row in c]
        return data

    def saveConnections(self, new_conn):
        """
        Replace the actual list of connections with the new list.

        :Parameters:
          new_conn : list
            the list of tuples (name, host, port, default)
        """

        c = self.conn.cursor()
        c.execute('delete from connections')

        for row in new_conn:
            c.execute('insert into connections values(?, ?, ?, ?)', row)