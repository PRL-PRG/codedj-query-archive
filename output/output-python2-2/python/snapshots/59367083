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
from devclient.trigger import Trigger

class TestTrigger(unittest.TestCase):
    def __init__(self, methodName='runTest'):
        super(TestTrigger, self).__init__(methodName)
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

    def testCheckAction1(self):
        triggers = [('* dwarf', 0, 'bow dwarf')]
        storage.saveTriggers(self.conn_name, triggers)
        text = "A dwarf with a long red beard mindlessly bumps into people."
        self.assert_(Trigger(self.conn_name).checkActions(text) == ['bow dwarf'])

    def testCheckAction2(self):
        triggers = [('dwarf', 0, 'bow dwarf')]
        storage.saveTriggers(self.conn_name, triggers)
        text = "A dwarf with a long red beard mindlessly bumps into people."
        self.assert_(Trigger(self.conn_name).checkActions(text) == [])

    def testCheckAction3(self):
        triggers = [('* (White Aura) %w', 0, 'look %2')]
        storage.saveTriggers(self.conn_name, triggers)
        text = "(Translucent) (White Aura) Ravi, granter of deeds, sits " + \
               "calmly waiting for the next adventurer."
        self.assert_(Trigger(self.conn_name).checkActions(text) == ['look Ravi'])

if __name__ == '__main__':
    unittest.main()
