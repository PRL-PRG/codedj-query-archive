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
import unittest
import os
import time

sys.path.append('..')

from devclient.socket import Socket

class TestSocket(unittest.TestCase):
    def setUp(self):
        i, o = os.popen2("python server_echo.py")
        time.sleep(1)

    def testData(self):
        s = Socket()
        s.connect("localhost", 6666)
        s.write("prova")
        time.sleep(1)
        self.assert_(s.read() == "prova\n")

if __name__ == '__main__':
    unittest.main()
