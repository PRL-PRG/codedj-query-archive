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

import os
import sys
import shutil
import os.path
import unittest

# FIX
sys.path.append('..')
sys.path.append('../configobj')

import devclient.storage as storage
import devclient.exception as exception
from devclient.conf import config


class TestBase(unittest.TestCase):

    def __init__(self, methodName='runTest'):
        super(TestBase, self).__init__(methodName)
        self.test_dir = '../../data/storage/test_dir'

    def setUp(self):
        if os.path.exists(self.test_dir):
            shutil.rmtree(self.test_dir)
        os.mkdir(self.test_dir)
        config['storage'] = {'path': os.path.abspath(self.test_dir)}
        storage.loadStorage()

    def tearDown(self):
        if os.path.exists(self.test_dir):
            shutil.rmtree(self.test_dir)


class TestStorage(TestBase):

    def testEmptyConnections(self):
        self.assert_(storage.connections() == [])

    def testAddConnection(self):
        conn = (0, 'name', 'host', 111)
        storage.addConnection(list(conn))
        self.assert_(storage.connections()[0][1:] == conn[1:])

    def testAddConnection2(self):
        conn = [0, 'name', 'host', 111]
        storage.addConnection(conn)
        self.assert_(storage.connections()[0] == tuple(conn))

    def testAddConnection3(self):
        conn = [0, 'name', 'host', 111]
        storage.addConnection(conn)

        conn2 = [0, 'name2', 'host2', 222]
        storage.addConnection(conn2)
        connections = [tuple(conn), tuple(conn2)]
        self.assert_(storage.connections() == connections)

    def testUpdateConnection(self):
        conn = [0, 'name', 'host', 111]
        storage.addConnection(conn)

        conn[1] = 'new_name'
        storage.updateConnection(conn)
        self.assert_(storage.connections()[0] == tuple(conn))

    def testUpdateConnection2(self):
        conn = [0, 'name', 'host', 111]
        storage.addConnection(conn)
        storage.saveAccount(['john', 'john'], 1, 1)

        conn[1] = 'new_name'
        storage.updateConnection(conn)
        self.assert_(storage.accountDetail(1, 'john') == ['john', 'john'])

    def testDeleteConnection(self):
        conn = [0, 'name', 'host', 111]
        storage.addConnection(conn)

        conn2 = [0, 'name2', 'host2', 222]
        storage.addConnection(conn2)

        storage.deleteConnection(conn2)
        self.assert_(storage.connections()[0] == tuple(conn))

    def testEmptyAliases(self):
        self.assertRaises(exception.ConnectionNotFound,
                          storage.aliases,
                          'conn_name')

    def testEmptyAliases2(self):
        storage.addConnection([0, 'name', 'host', 111])
        self.assert_(storage.aliases('name') == [])

    def testSaveAliases(self):
        conn_name = 'conn'
        aliases = [('label', 'body')]

        self.assertRaises(exception.ConnectionNotFound,
                          storage.saveAliases,
                          conn_name, aliases)

    def testSaveAliases2(self):
        conn_name = 'conn'
        conn = [0, conn_name, 'host', 111]
        storage.addConnection(conn)

        aliases = [('label', 'body')]
        storage.saveAliases(conn_name, aliases)
        self.assert_(storage.aliases(conn_name) == aliases)

    def testSaveAliases3(self):
        conn_name = 'conn'
        conn = [0, conn_name, 'host', 111]
        storage.addConnection(conn)

        aliases = [('label1', 'body1'), ('label2', 'body2')]
        storage.saveAliases(conn_name, aliases)
        self.assert_(storage.aliases(conn_name) == aliases)

    def testSaveAliases4(self):
        conn_name = 'conn'
        conn = [0, conn_name, 'host', 111]
        storage.addConnection(conn)

        aliases = [('label1', 'body1')]
        storage.saveAliases(conn_name, aliases)

        aliases.append(('label2', 'body2'))
        storage.saveAliases(conn_name, aliases)
        self.assert_(storage.aliases(conn_name) == aliases)

    def testSaveAliases5(self):
        conn_name = 'conn'
        conn = [0, conn_name, 'host', 111]
        storage.addConnection(conn)

        aliases = [('label1', 'body1')]
        storage.saveAliases(conn_name, aliases)

        aliases[0] = ('label2', 'body2')
        storage.saveAliases(conn_name, aliases)
        self.assert_(storage.aliases(conn_name) == aliases)

    def testSaveAliases6(self):
        conn_name = 'conn'
        conn = [0, conn_name, 'host', 111]
        storage.addConnection(conn)

        aliases = [('label1', 'body1'), ('label2', 'body2')]
        storage.saveAliases(conn_name, aliases)

        del aliases[0]
        storage.saveAliases(conn_name, aliases)
        self.assert_(storage.aliases(conn_name) == aliases)

    def testEmptyMacros(self):
        self.assertRaises(exception.ConnectionNotFound,
                          storage.macros,
                          'conn_name')

    def testEmptyMacros2(self):
        storage.addConnection([0, 'name', 'host', 111])
        self.assert_(storage.macros('name') == [])

    def testSaveMacros(self):
        conn_name = 'conn'
        macros = [('command', '1', '0', '0', '65')]

        self.assertRaises(exception.ConnectionNotFound,
                          storage.saveMacros,
                          conn_name, macros)

    def testSaveMacros2(self):
        conn_name = 'conn'
        conn = [0, conn_name, 'host', 111]
        storage.addConnection(conn)

        macros = [('command', 1, 0, 0, 65)]
        storage.saveMacros(conn_name, macros)
        self.assert_(storage.macros(conn_name) == macros)

    def testSaveMacros3(self):
        conn_name = 'conn'
        conn = [0, conn_name, 'host', 111]
        storage.addConnection(conn)

        macros = [('command', 1, 0, 0, 65), ('command', 0, 0, 0, 73)]
        storage.saveMacros(conn_name, macros)
        self.assert_(storage.macros(conn_name) == macros)

    def testSaveMacros4(self):
        conn_name = 'conn'
        conn = [0, conn_name, 'host', 111]
        storage.addConnection(conn)

        macros = [('command', 1, 0, 0, 65)]
        storage.saveMacros(conn_name, macros)

        macros.append(('command', 0, 0, 0, 73))
        storage.saveMacros(conn_name, macros)
        self.assert_(storage.macros(conn_name) == macros)

    def testSaveMacros5(self):
        conn_name = 'conn'
        conn = [0, conn_name, 'host', 111]
        storage.addConnection(conn)

        macros = [('command', 1, 0, 0, 65)]
        storage.saveMacros(conn_name, macros)

        macros[0] = ('command', 0, 0, 0, 73)
        storage.saveMacros(conn_name, macros)
        self.assert_(storage.macros(conn_name) == macros)

    def testSaveMacros6(self):
        conn_name = 'conn'
        conn = [0, conn_name, 'host', 111]
        storage.addConnection(conn)

        macros = [('command', 1, 0, 0, 65), ('command', 0, 0, 0, 73)]
        storage.saveMacros(conn_name, macros)

        del macros[0]
        storage.saveMacros(conn_name, macros)
        self.assert_(storage.macros(conn_name) == macros)

    def testEmptyPreferences(self):
        self.assert_(storage.preferences() == ('', 0, 0, ';'))

    def testSavePreferences(self):
        preferences = ('#FF0000', 0, 1, '#')
        storage.savePreferences(preferences)
        self.assert_(storage.preferences() == preferences)

    def testEmptyAccounts(self):
        self.assertRaises(exception.ConnectionNotFound,
                          storage.accounts, 1)

    def testEmptyAccounts2(self):
        storage.addConnection([0, 'name', 'host', 111])
        self.assert_(storage.accounts(1) == [])

    def testSaveAccounts(self):
        storage.addConnection([0, 'name', 'host', 111])
        storage.saveAccount(['john', 'john'], 1, 1)
        self.assert_(storage.accounts(1) == ['john'])

    def testSaveAccounts2(self):
        storage.addConnection([0, 'name', 'host', 111])
        storage.saveAccount(['john', 'johnpwd'], 1, 1)
        storage.saveAccount(['sarah', 'sarahpwd'], 1, 1)
        self.assert_(storage.accounts(1) == ['john', 'sarah'])

    def testSaveAccounts3(self):
        storage.addConnection([0, 'name', 'host', 111])
        storage.saveAccount(['john', 'pwd'], 1, 1)
        self.assert_(storage.accountDetail(1, 'john') == ['john', 'pwd'])

    def testSaveAccounts4(self):
        storage.addConnection([0, 'name', 'host', 111])
        storage.saveAccount(['john', 'pwd'], 1, 1)
        storage.saveAccount(['john', 'ola'], 1, 1)
        self.assert_(storage.accountDetail(1, 'john') == ['john', 'ola'])

    def testDeleteAccount(self):
        storage.addConnection([0, 'name', 'host', 111])
        storage.saveAccount(['john', 'johnpwd'], 1, 1)
        storage.saveAccount(['sarah', 'sarahpwd'], 1, 1)
        storage.deleteAccount(1, 'john')
        self.assert_(storage.accounts(1) == ['sarah'])

    def testGetOption1(self):
        self.assert_(storage.option('save_account') == 0)

    def testSetOption1(self):
        storage.setOption('save_account', 1)
        self.assert_(storage.option('save_account') == 1)

    def testSetOption2(self):
        storage.addConnection([0, 'name', 'host', 111])
        storage.setOption('default_account', 1, 1)
        self.assert_(storage.option('default_account', 1) == 1)

    def testSetOption3(self):
        storage.addConnection([0, 'name', 'host', 111])
        storage.setOption('default_connection', 1)
        self.assert_(storage.option('default_connection') == 1)

    def testSetOption4(self):
        storage.addConnection([0, 'name', 'host', 111])
        storage.setOption('default_account', 1, 1)
        storage.setOption('default_account', 2, 1)
        self.assert_(storage.option('default_account', 1) == 2)

    def testSetOption5(self):
        storage.addConnection([0, 'name', 'host', 111])
        self.assertRaises(exception.ConnectionNotFound,
                          storage.setOption,
                          'default_account', 20, 2)

    def testSetOption6(self):
        storage.addConnection([0, 'name', 'host', 111])
        storage.addConnection([0, 'name2', 'host2', 222])
        storage.setOption('default_account', 10, 1)
        storage.setOption('default_account', 20, 2)
        self.assert_(storage.option('default_account', 1) == 10)
        self.assert_(storage.option('default_account', 2) == 20)

    def testEmptyTriggers(self):
        self.assertRaises(exception.ConnectionNotFound,
                          storage.triggers,
                          'conn_name')

    def testEmptyTriggers2(self):
        storage.addConnection([0, 'name', 'host', 111])
        self.assert_(storage.triggers('name') == [])

    def testSaveTriggers(self):
        conn_name = 'conn'
        triggers = [('* dwarf *', 0, 'bow dwarf', '', '')]

        self.assertRaises(exception.ConnectionNotFound,
                          storage.saveTriggers,
                          conn_name, triggers)

    def testSaveTriggers2(self):
        conn_name = 'conn'
        conn = [0, conn_name, 'host', 111]
        storage.addConnection(conn)

        triggers = [('* dwarf *', 0, 'bow dwarf','', '')]
        storage.saveTriggers(conn_name, triggers)
        self.assert_(storage.triggers(conn_name) == triggers)

    def testSaveTriggers3(self):
        conn_name = 'conn'
        conn = [0, conn_name, 'host', 111]
        storage.addConnection(conn)

        triggers = [('* (White Aura) %w', 0, '', '#FF0000', '#000000')]
        storage.saveTriggers(conn_name, triggers)
        self.assert_(storage.triggers(conn_name) == triggers)


class TestStorage2(TestBase):

    def testMultiConnection(self):
        conn = [0, 'name','host', 111]
        storage.addConnection(conn)
        self.assert_(storage.connections()[0] == tuple(conn))


if __name__ == '__main__':
    unittest.main()
