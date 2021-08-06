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

from devclient.history import History

class TestHistory(unittest.TestCase):

    def testHistoryPrevEmpty(self):
        history = History()
        self.assert_('' == history.getPrev())

    def testHistoryPrev1(self):
        txt = 'hello'
        history = History()
        history.add(txt)
        self.assert_(txt == history.getPrev())

    def testHistoryPrev2(self):
        history = History()
        for txt in ('hello', 'world'):
            history.add(txt)
        self.assert_('world' == history.getPrev())

    def testHistoryPrev3(self):
        history = History()
        for txt in ('hello', 'world'):
            history.add(txt)

        history.getPrev()
        self.assert_('hello' == history.getPrev())

    def testHistoryPrev4(self):
        history = History()
        for txt in ('aaa', 'bbb', 'ccc'):
            history.add(txt)

        history.getPrev()
        history.getPrev()
        self.assert_('aaa' == history.getPrev())

    def testHistoryPrev5(self):
        history = History()
        for txt in ('aaa', 'bbb', 'ccc'):
            history.add(txt)

        history.getPrev()
        history.getPrev()
        history.getPrev()
        self.assert_('ccc' == history.getPrev())

    def testHistoryPrev6(self):
        history = History()
        for txt in ('aaa', 'bbb'):
            history.add(txt)

        history.getPrev()
        history.add('ccc')
        self.assert_('ccc' == history.getPrev())

    def testHistoryNextEmpty(self):
        history = History()
        self.assert_('' == history.getNext())

    def testHistoryNext1(self):
        txt = 'hello'
        history = History()
        history.add(txt)
        self.assert_(txt == history.getNext())

    def testHistoryNext2(self):
        history = History()
        for txt in ('hello', 'world'):
            history.add(txt)

        self.assert_('hello' == history.getNext())

    def testHistoryNext3(self):
        history = History()
        for txt in ('hello', 'world'):
            history.add(txt)

        history.getNext()
        self.assert_('world' == history.getNext())

    def testHistoryNext4(self):
        history = History()
        for txt in ('aaa', 'bbb', 'ccc'):
            history.add(txt)

        history.getNext()
        history.getNext()
        self.assert_('ccc' == history.getNext())

    def testHistoryNext5(self):
        history = History()
        for txt in ('aaa', 'bbb', 'ccc'):
            history.add(txt)

        history.getNext()
        history.getNext()
        history.getNext()
        self.assert_('aaa' == history.getNext())

    def testHistoryNext6(self):
        history = History()
        for txt in ('aaa', 'bbb'):
            history.add(txt)

        history.getNext()
        history.add('ccc')
        self.assert_('aaa' == history.getNext())

    def testHistoryPrevNext(self):
        history = History()
        for txt in ('hello', 'world'):
            history.add(txt)

        history.getPrev()
        self.assert_('hello' == history.getNext())

    def testHistoryNextPrev(self):
        history = History()
        for txt in ('hello', 'world'):
            history.add(txt)

        history.getNext()
        self.assert_('world' == history.getPrev())

    def testHistoryEmpyAdd(self):
        history = History()
        history.add('  \t \r\n')

        self.assert_(len(history._list) == 0)

    def testHistoryClear(self):
        history = History()
        for txt in ('aaa', 'bbb', 'ccc'):
            history.add(txt)

        history.clear()
        self.assert_(len(history._list) == 0)

if __name__ == '__main__':
    unittest.main()

