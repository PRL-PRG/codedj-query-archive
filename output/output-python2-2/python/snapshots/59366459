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
import unittest

sys.path.append('..')

import conf
import modules.exception as exception
from modules.loader import Loader

class TestLoader(unittest.TestCase):
    def setUp(self):
        conf.config['modules'] = {'path': '../modules'}
        self.loader = Loader()

    def testFindModules(self):
        self.loader._findModules()
        self.assert_('thread' in self.loader.modules)

    def testLoadClasses(self):
        self.loader._findModules()
        classes = self.loader._loadClasses(['Thread'])
        self.assert_('Thread' in classes)

    def testLoadClassesFailure(self):
        self.loader._findModules()
        self.assertRaises(exception.ClassNotFound,
                          self.loader._loadClasses,
                          ['Fake'])

if __name__ == '__main__':
    unittest.main()
