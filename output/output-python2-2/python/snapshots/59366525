#!/usr/bin/env python
#-*- coding: utf-8 -*-

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