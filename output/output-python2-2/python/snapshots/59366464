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

from modules.parser import Parser

class TestParser(unittest.TestCase):

    def setUp(self):
        self.parser = Parser()

    def testParseText(self):
        txt = 'hello'
        self.parser.parse(txt)
        self.assert_([txt] == self.parser.model.main_text.get())

    def testParseTextMultiline(self):
        txt = 'hello\nworld'
        self.parser.parse(txt)
        self.assert_([txt.replace('\n','<br>')] ==
                     self.parser.model.main_text.get())

    def testParseMultiText(self):
        txt1, txt2 = 'hello', 'world'
        self.parser.parse(txt1)
        self.parser.parse(txt2)
        self.assert_('%s<br>%s' % (txt1, txt2) ==
                     '<br>'.join(self.parser.model.main_text.get()))

    def testParseMultiText2(self):
        txt1, txt2 = 'hello\x1b[0;', '33mworld'
        self.parser.parse(txt1)
        self.parser.parse(txt2)

        self.assert_('hello<br>world' ==
                     '<br>'.join(self.parser.model.main_text.get()))

        self.assert_(self.parser._normal_color[3] ==
                     self.parser.model.main_fgcolor)

    def testParseMultiText3(self):
        txt1, txt2 = 'hello\x1b', '[0;33mworld'
        self.parser.parse(txt1)
        self.parser.parse(txt2)

        self.assert_('hello<br>world' ==
                     '<br>'.join(self.parser.model.main_text.get()))

        self.assert_(self.parser._normal_color[3] ==
                     self.parser.model.main_fgcolor)

    def testParseSpace(self):
        txt = 'hello world'
        self.parser.parse(txt)
        self.assert_([txt.replace(' ','&nbsp;')] ==
                     self.parser.model.main_text.get())

    def testEvalStyle1(self):
        self.parser._evalStyle('31')
        self.assert_(self.parser._normal_color[1] ==
                     self.parser.model.main_fgcolor)

    def testEvalStyle2(self):
        self.parser._evalStyle('0;42')
        self.assert_(self.parser._normal_color[2] ==
                     self.parser.model.main_bgcolor)

    def testEvalStyle3(self):
        self.parser._evalStyle('35;40')
        self.assert_(self.parser._normal_color[0] ==
                     self.parser.model.main_bgcolor)
        self.assert_(self.parser._normal_color[5] ==
                     self.parser.model.main_fgcolor)

    def testEvalStyle4(self):
        self.parser._evalStyle('1;36;41')
        self.assert_(self.parser._normal_color[1] ==
                     self.parser.model.main_bgcolor)
        self.assert_(self.parser._bright_color[6] ==
                     self.parser.model.main_fgcolor)

    def testEvalStyle5(self):
        self.parser._evalStyle('0;42')
        style = self.parser._evalStyle('0;42')
        self.assert_(style == '')

    def testEvalStyle6(self):
        self.parser._evalStyle('0;42')
        style = self.parser._evalStyle('0;41')
        self.assert_(style == 'background-color:#%s' %
                               self.parser._normal_color[1])

    def testReplaceAnsiColor(self):
        txt = '\x1b[33mhello'
        res = self.parser._replaceAnsiColor(txt)
        self.assert_(res == 'hello')
        self.assert_(self.parser.model.main_fgcolor ==
                     self.parser._normal_color[3])

    def testReplaceAnsiColor2(self):
        self.parser._evalStyle('31')
        txt = '\x1b[33mhello'
        res = self.parser._replaceAnsiColor(txt)

        self.assert_(res == '<span style="color:#%s">hello</span>' %
                     self.parser._normal_color[3])

    def testReplaceEmptyColor(self):
        txt = '\x1b[mhello'
        res = self.parser._replaceAnsiColor(txt)
        self.assert_(res == 'hello')

if __name__ == '__main__':
    unittest.main()
