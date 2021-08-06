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

    def testConnections(self):
        conn = [('name','host', 111, 1)]
        self.storage.saveConnections(conn)
        self.assert_(self.storage.connections() == conn)

    def testConnections2(self):
        conn = [('name','host', 111, 1)]
        self.storage.saveConnections(conn)

        conn.extend([('name2','host2', 222, 1)])
        self.storage.saveConnections(conn)
        self.assert_(self.storage.connections() == conn)

    def testConnections3(self):
        conn = [('name','host', 111, 1)]
        self.storage.saveConnections(conn)

        conn = [('test','host', 111, 1)]
        self.storage.saveConnections(conn)
        self.assert_(self.storage.connections() == conn)

    def testConnections4(self):
        conn = [('name1','host1', 111, 0),
                ('name2','host2', 112, 1),
                ('name3','host3', 113, 0)]
        self.storage.saveConnections(conn)
        self.assert_(self.storage.connections() == conn)

    def testConnections5(self):
        conn = [('name1','host1', 111, 0),
                ('name2','host2', 112, 1),
                ('name3','host3', 113, 1)]
        self.storage.saveConnections(conn)

        del conn[1]
        self.storage.saveConnections(conn)
        self.assert_(self.storage.connections() == conn)

class TestStorage2(TestBase):

    def testMultiConnections(self):
        conn = [('name','host', 111, 0)]
        Storage().saveConnections(conn)
        self.assert_(Storage().connections() == conn)

if __name__ == '__main__':
    unittest.main()
