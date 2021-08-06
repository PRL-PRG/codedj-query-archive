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

from devclient.conf import config
from devclient.storage import Storage
from devclient.alias import Alias

class TestAlias(unittest.TestCase):
    def setUp(self):
        abspath = os.path.abspath('../../data/storage/dbtest.sqlite')
        config['storage'] = {'path': abspath}

        if os.path.exists(config['storage']['path']):
            os.unlink(config['storage']['path'])

        self.conn_name = 'conn'
        conn = [0, self.conn_name, 'host', 111, 1]
        self.storage = Storage()
        self.storage.addConnection(conn)

    def tearDown(self):
        if os.path.exists(config['storage']['path']):
            os.unlink(config['storage']['path'])

    def testCheck1(self):
        aliases = [('h', 'hello world')]
        self.storage.saveAliases(self.conn_name, aliases)
        self.assert_(Alias(self.conn_name).check("h, I'm Gianni") ==
                     "hello world, I'm Gianni")

    def testCheck2(self):
        aliases = [('h', 'hello world')]
        self.storage.saveAliases(self.conn_name, aliases)
        self.assert_(Alias(self.conn_name).check("I'm Gianni, h") ==
                     "I'm Gianni, h")

    def testCheck3(self):
        aliases = [('h', 'hello %s')]
        self.storage.saveAliases(self.conn_name, aliases)
        self.assert_(Alias(self.conn_name).check("h world") ==
                     "hello world")

    def testCheck4(self):
        aliases = [('h', "hello %s, nice to meet you")]
        self.storage.saveAliases(self.conn_name, aliases)
        self.assert_(Alias(self.conn_name).check("h Gianni") ==
                     "hello Gianni, nice to meet you")

    def testCheck5(self):
        aliases = [('h', "hello %s, nice to %s you")]
        self.storage.saveAliases(self.conn_name, aliases)
        self.assert_(Alias(self.conn_name).check("h Gianni meet") ==
                     "hello Gianni, nice to meet you")

    def testCheck6(self):
        aliases = [('h', "hello %s, nice to %s you")]
        self.storage.saveAliases(self.conn_name, aliases)
        self.assert_(Alias(self.conn_name).check("h Gianni") ==
                     "hello Gianni, nice to  you")

if __name__ == '__main__':
    unittest.main()
