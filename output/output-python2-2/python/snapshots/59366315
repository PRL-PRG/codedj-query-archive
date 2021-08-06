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
import glob
import os.path
import unittest

os.chdir(os.path.join(os.getcwd(), os.path.dirname(sys.argv[0])))
test_files = glob.glob('test_*.py')
test_files.remove('test_all.py')
imported = {}
for filename in test_files:
    execfile(filename, imported)

alltests = unittest.TestSuite([unittest.makeSuite(ref) for name, ref
                                in imported.iteritems()
                                if name.startswith('Test')])

if __name__ == '__main__':
    unittest.TextTestRunner(verbosity=2).run(alltests)
