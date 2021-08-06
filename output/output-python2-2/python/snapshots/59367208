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
from devclient.conf import config
from devclient.alias import Alias

class TestAlias(unittest.TestCase):
    def __init__(self, methodName='runTest'):
        super(TestAlias, self).__init__(methodName)
        self.test_dir = '../../data/storage/test_dir'

    def setUp(self):
        if os.path.exists(self.test_dir):
            shutil.rmtree(self.test_dir)
        os.mkdir(self.test_dir)
        config['storage'] = {'path': os.path.abspath(self.test_dir)}
        storage.loadStorage()

        self.conn_name = 'conn'
        conn = [0, self.conn_name, 'host', 111]
        storage.addConnection(conn)

    def tearDown(self):
        if os.path.exists(self.test_dir):
            shutil.rmtree(self.test_dir)

    def testCheck1(self):
        aliases = [('h', 'hello world')]
        storage.saveAliases(self.conn_name, aliases)
        self.assert_(Alias(self.conn_name).check("h, I'm Gianni") ==
                     "hello world, I'm Gianni")

    def testCheck2(self):
        aliases = [('h', 'hello world')]
        storage.saveAliases(self.conn_name, aliases)
        self.assert_(Alias(self.conn_name).check("I'm Gianni, h") ==
                     "I'm Gianni, h")

    def testCheck3(self):
        aliases = [('h', 'hello %s')]
        storage.saveAliases(self.conn_name, aliases)
        self.assert_(Alias(self.conn_name).check("h world") ==
                     "hello world")

    def testCheck4(self):
        aliases = [('h', "hello %s, nice to meet you")]
        storage.saveAliases(self.conn_name, aliases)
        self.assert_(Alias(self.conn_name).check("h Gianni") ==
                     "hello Gianni, nice to meet you")

    def testCheck5(self):
        aliases = [('h', "hello %s, nice to %s you")]
        storage.saveAliases(self.conn_name, aliases)
        self.assert_(Alias(self.conn_name).check("h Gianni meet") ==
                     "hello Gianni, nice to meet you")

    def testCheck6(self):
        aliases = [('h', "hello %s, nice to %s you")]
        storage.saveAliases(self.conn_name, aliases)
        self.assert_(Alias(self.conn_name).check("h Gianni") ==
                     "hello Gianni, nice to  you")

if __name__ == '__main__':
    unittest.main()
