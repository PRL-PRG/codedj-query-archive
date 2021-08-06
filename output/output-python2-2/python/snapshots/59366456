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

import sys
import os.path
import unittest

sys.path.append('..')

import conf
from modules.storage import Storage

class TestBase(unittest.TestCase):

    def setUp(self):
        abspath = os.path.abspath('../../data/storage/dbtest.sqlite')
        conf.config['storage'] = {'path': abspath}

        if os.path.exists(conf.config['storage']['path']):
            os.unlink(conf.config['storage']['path'])

    def tearDown(self):
        if os.path.exists(conf.config['storage']['path']):
            os.unlink(conf.config['storage']['path'])

class TestStorage(TestBase):

    def setUp(self):
        super(TestStorage, self).setUp()
        self.storage = Storage()

    def testEmptyConnections(self):
        self.assert_(self.storage.connections() == [])

    def addConnection(self):
        conn = (0, 'name','host', 111, 1)
        self.storage.addConnection(list(conn))
        self.assert_(self.storage.connections()[0][1:] == conn[1:])

    def addConnection2(self):
        conn = [0, 'name','host', 111, 1]
        self.storage.addConnection(conn)
        self.assert_(self.storage.connections()[0] == conn)

    def addConnection3(self):
        conn = [0, 'name','host', 111, 1]
        self.storage.addConnection(conn)

        conn2 = [0, 'name2','host2', 222, 1]
        self.storage.addConnection(conn2)
        connections = [tuple(conn), tuple(conn2)]
        self.assert_(self.storage.connections() == connections)

    def updateConnection(self):
        conn = [0, 'name','host', 111, 1]
        self.storage.addConnection(conn)

        conn[1] = 'new_name'
        self.storage.updateConnection(conn)
        self.assert_(self.storage.connections()[0] == conn)

    def deleteConnection(self):
        conn = [0, 'name','host', 111, 1]
        self.storage.addConnection(conn)

        conn2 = [0, 'name2','host2', 222, 1]
        self.storage.addConnection(conn2)

        self.storage.deleteConnection(conn2)
        self.assert_(self.storage.connections()[0] == conn)

class TestStorage2(TestBase):

    def testMultiConnection(self):
        conn = [0, 'name','host', 111, 0]
        Storage().addConnection(conn)
        self.assert_(Storage().connections()[0] == tuple(conn))

if __name__ == '__main__':
    unittest.main()
