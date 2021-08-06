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
import modules.exception as exception
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

    def testAddConnection(self):
        conn = (0, 'name', 'host', 111, 1)
        self.storage.addConnection(list(conn))
        self.assert_(self.storage.connections()[0][1:] == conn[1:])

    def testAddConnection2(self):
        conn = [0, 'name', 'host', 111, 1]
        self.storage.addConnection(conn)
        self.assert_(self.storage.connections()[0] == tuple(conn))

    def testAddConnection3(self):
        conn = [0, 'name', 'host', 111, 1]
        self.storage.addConnection(conn)

        conn2 = [0, 'name2', 'host2', 222, 1]
        self.storage.addConnection(conn2)
        connections = [tuple(conn), tuple(conn2)]
        self.assert_(self.storage.connections() == connections)

    def testUpdateConnection(self):
        conn = [0, 'name', 'host', 111, 1]
        self.storage.addConnection(conn)

        conn[1] = 'new_name'
        self.storage.updateConnection(conn)
        self.assert_(self.storage.connections()[0] == tuple(conn))

    def testDeleteConnection(self):
        conn = [0, 'name', 'host', 111, 1]
        self.storage.addConnection(conn)

        conn2 = [0, 'name2', 'host2', 222, 1]
        self.storage.addConnection(conn2)

        self.storage.deleteConnection(conn2)
        self.assert_(self.storage.connections()[0] == tuple(conn))

    def testEmptyAliases(self):
        self.assert_(self.storage.aliases('conn_name') == [])

    def testSaveAliases(self):
        conn_name = 'conn'
        aliases = [('label', 'body')]

        self.assertRaises(exception.ConnectionNotFound,
                          self.storage.saveAliases,
                          conn_name, aliases)

    def testSaveAliases2(self):
        conn_name = 'conn'

        conn = [0, conn_name, 'host', 111, 1]
        self.storage.addConnection(conn)

        aliases = [('label', 'body')]
        self.storage.saveAliases(conn_name, aliases)
        self.assert_(self.storage.aliases(conn_name) == aliases)

    def testSaveAliases3(self):
        conn_name = 'conn'

        conn = [0, conn_name, 'host', 111, 1]
        self.storage.addConnection(conn)

        aliases = [('label1', 'body1'), ('label2', 'body2')]
        self.storage.saveAliases(conn_name, aliases)
        self.assert_(self.storage.aliases(conn_name) == aliases)

    def testSaveAliases4(self):
        conn_name = 'conn'

        conn = [0, conn_name, 'host', 111, 1]
        self.storage.addConnection(conn)

        aliases = [('label1', 'body1')]
        self.storage.saveAliases(conn_name, aliases)

        aliases.append(('label2', 'body2'))
        self.storage.saveAliases(conn_name, aliases)
        self.assert_(self.storage.aliases(conn_name) == aliases)

    def testSaveAliases5(self):
        conn_name = 'conn'

        conn = [0, conn_name, 'host', 111, 1]
        self.storage.addConnection(conn)

        aliases = [('label1', 'body1')]
        self.storage.saveAliases(conn_name, aliases)

        aliases[0] = ('label2', 'body2')
        self.storage.saveAliases(conn_name, aliases)
        self.assert_(self.storage.aliases(conn_name) == aliases)

    def testSaveAliases6(self):
        conn_name = 'conn'

        conn = [0, conn_name, 'host', 111, 1]
        self.storage.addConnection(conn)

        aliases = [('label1', 'body1'), ('label2', 'body2')]
        self.storage.saveAliases(conn_name, aliases)

        del aliases[0]
        self.storage.saveAliases(conn_name, aliases)
        self.assert_(self.storage.aliases(conn_name) == aliases)


class TestStorage2(TestBase):

    def testMultiConnection(self):
        conn = [0, 'name','host', 111, 0]
        Storage().addConnection(conn)
        self.assert_(Storage().connections()[0] == tuple(conn))

if __name__ == '__main__':
    unittest.main()
