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

import modules.exception as exception
from modules.model import CircularList

class TestCircularList(unittest.TestCase):
    def setUp(self):
        self.c = CircularList(10)

    def testAppend(self):
        self.c.append('hello')
        self.assert_('hello' == self.c._data[0])

    def testAppend2(self):
        for i in xrange(5):
            self.c.append('hello')
        self.assert_(5 == len(self.c._data))

    def testAppend3(self):
        for i in xrange(15):
            self.c.append('hello')
        self.assert_(10 == len(self.c._data))

    def testAppend4(self):
        for i in xrange(11):
            self.c.append('hello%d' % i)
        self.assert_(self.c._data[0] == 'hello10')

    def testAppend5(self):
        for i in xrange(13):
            self.c.append('hello%d' % i)
        self.assert_(self.c._data[2] == 'hello12')

    def testGet(self):
        data = ['hello', 'world']
        for x in data:
            self.c.append(x)
        self.assert_(self.c.get() == data)

    def testGet2(self):
        data = ['hello', 'world']
        for x in data:
            self.c.append(x)

        self.assert_(self.c.get(0) == data[1:])

    def testGet3(self):
        for i in xrange(13):
            self.c.append('hello%d' % i)

        self.assert_(self.c.get(10) == ['hello11', 'hello12'])

    def testGet4(self):
        for i in xrange(12):
            self.c.append('hello%d' % i)

        self.assert_(self.c.get(8) == ['hello9', 'hello10', 'hello11'])

    def testGet5(self):
        for i in xrange(12):
            self.c.append('hello%d' % i)

        self.assertRaises(exception.BufferUnderSize, self.c.get)

    def testGet6(self):
        for i in xrange(25):
            self.c.append('hello%d' % i)

        self.assertRaises(exception.BufferUnderSize, self.c.get, 13)

if __name__ == '__main__':
    unittest.main()
