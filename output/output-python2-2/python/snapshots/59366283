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

sys.path.append('..')

from devclient.history import History

class TestHistory(unittest.TestCase):

    def setUp(self):
        self.history = History()

    def testHistoryPrevEmpty(self):
        self.assert_('' == self.history.getPrev())

    def testHistoryPrev1(self):
        txt = 'hello'
        self.history.add(txt)
        self.assert_(txt == self.history.getPrev())

    def testHistoryPrev2(self):
        for txt in ('hello', 'world'):
            self.history.add(txt)
        self.assert_('world' == self.history.getPrev())

    def testHistoryPrev3(self):
        for txt in ('hello', 'world'):
            self.history.add(txt)

        self.history.getPrev()
        self.assert_('hello' == self.history.getPrev())

    def testHistoryPrev4(self):
        for txt in ('aaa', 'bbb', 'ccc'):
            self.history.add(txt)

        self.history.getPrev()
        self.history.getPrev()
        self.assert_('aaa' == self.history.getPrev())

    def testHistoryPrev5(self):
        for txt in ('aaa', 'bbb', 'ccc'):
            self.history.add(txt)

        self.history.getPrev()
        self.history.getPrev()
        self.history.getPrev()
        self.assert_('ccc' == self.history.getPrev())

    def testHistoryPrev6(self):
        for txt in ('aaa', 'bbb'):
            self.history.add(txt)

        self.history.getPrev()
        self.history.add('ccc')
        self.assert_('ccc' == self.history.getPrev())

    def testHistoryNextEmpty(self):
        self.assert_('' == self.history.getNext())

    def testHistoryNext1(self):
        txt = 'hello'
        self.history.add(txt)
        self.assert_(txt == self.history.getNext())

    def testHistoryNext2(self):
        for txt in ('hello', 'world'):
            self.history.add(txt)

        self.assert_('hello' == self.history.getNext())

    def testHistoryNext3(self):
        for txt in ('hello', 'world'):
            self.history.add(txt)

        self.history.getNext()
        self.assert_('world' == self.history.getNext())

    def testHistoryNext4(self):
        for txt in ('aaa', 'bbb', 'ccc'):
            self.history.add(txt)

        self.history.getNext()
        self.history.getNext()
        self.assert_('ccc' == self.history.getNext())

    def testHistoryNext5(self):
        for txt in ('aaa', 'bbb', 'ccc'):
            self.history.add(txt)

        self.history.getNext()
        self.history.getNext()
        self.history.getNext()
        self.assert_('aaa' == self.history.getNext())

    def testHistoryNext6(self):
        for txt in ('aaa', 'bbb'):
            self.history.add(txt)

        self.history.getNext()
        self.history.add('ccc')
        self.assert_('aaa' == self.history.getNext())

    def testHistoryPrevNext(self):
        for txt in ('hello', 'world'):
            self.history.add(txt)

        self.history.getPrev()
        self.assert_('hello' == self.history.getNext())

    def testHistoryNextPrev(self):
        for txt in ('hello', 'world'):
            self.history.add(txt)

        self.history.getNext()
        self.assert_('world' == self.history.getPrev())

    def testHistoryEmpyAdd(self):
        self.history.add('  \t \r\n')

        self.assert_(len(self.history._list) == 0)

    def testHistoryClear(self):
        for txt in ('aaa', 'bbb', 'ccc'):
            self.history.add(txt)

        self.history.clear()
        self.assert_(len(self.history._list) == 0)

    def testHistoryAll(self):
        elements = ['aaa', 'bbb', 'ccc']
        for txt in elements:
            self.history.add(txt)
        self.assert_(elements == self.history.get())

    def testHistoryAll2(self):
        for txt in ('aaa', 'bbb', 'ccc', 'bbb'):
            self.history.add(txt)
        self.assert_(['aaa', 'ccc', 'bbb'] == self.history.get())


if __name__ == '__main__':
    unittest.main()

