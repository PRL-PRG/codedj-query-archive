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

from devclient.model import Parser, SmaugParser, AfkParser, Model

class TestParser(unittest.TestCase):

    def setUp(self):
        self.parser = Parser()

    def testParseText(self):
        txt = 'hello'
        model = self.parser.buildModel(txt, None, None)
        self.assert_([txt] == model.main_text)

    def testParseTextMultiline(self):
        txt = 'hello\nworld'
        model = self.parser.buildModel(txt, None, None)
        self.assert_(['hello<br>', 'world'] == model.main_html)

    def testParseMultiText(self):
        txt1, txt2 = 'hello', 'world'
        m1 = self.parser.buildModel(txt1, None, None)
        m2 = self.parser.buildModel(txt2, None, None)
        self.assert_([txt1] == m1.main_html)
        self.assert_([txt2] == m2.main_html)

    def testParseMultiText2(self):
        txt1, txt2 = 'hello\x1b[0;', '33mworld'
        m1 = self.parser.buildModel(txt1, None, None)
        m2 = self.parser.buildModel(txt2, m1.bg_color, m1.fg_color)
        self.assert_(['hello'] == m1.main_html)
        self.assert_(['world'] == m2.main_html)

        self.assert_(self.parser._normal_color[3] == m2.fg_color)

    def testParseMultiText3(self):
        txt1, txt2 = 'hello\x1b', '[0;42mworld'
        m1 = self.parser.buildModel(txt1, None, None)
        m2 = self.parser.buildModel(txt2, m1.bg_color, m1.fg_color)

        self.assert_(['hello'] == m1.main_html)
        self.assert_(['world'] == m2.main_html)

        self.assert_(self.parser._normal_color[2] == m2.bg_color)

    def testParseSpace(self):
        txt = 'hello world'
        m = self.parser.buildModel(txt, None, None)
        self.assert_([txt.replace(' ','&nbsp;')] == m.main_html)

    def testEvalStyle1(self):
        m = Model()
        self.parser._evalStyle('31', m)
        self.assert_(self.parser._normal_color[1] == m.fg_color)

    def testEvalStyle2(self):
        m = Model()
        self.parser._evalStyle('42', m)
        self.assert_(self.parser._normal_color[2] == m.bg_color)

    def testEvalStyle3(self):
        m = Model()
        self.parser._evalStyle('35;40', m)
        self.assert_(self.parser._normal_color[5] == m.fg_color)
        self.assert_(self.parser._normal_color[0] == m.bg_color)

    def testEvalStyle4(self):
        m = Model()
        self.parser._evalStyle('1;36;41', m)
        self.assert_(self.parser._normal_color[1] == m.bg_color)
        self.assert_(self.parser._bright_color[6] == m.fg_color)

    def testEvalStyle5(self):
        m = Model()
        self.parser._evalStyle('0;42', m)
        self.assert_(self.parser._evalStyle('0;42', m) == '')

    def testEvalStyle6(self):
        m = Model()
        self.parser._evalStyle('0;42', m)
        style = self.parser._evalStyle('0;41', m)
        self.assert_(style == 'background-color:#%s' %
                               self.parser._normal_color[1])

    def testReplaceAnsiColor(self):
        txt = '\x1b[33mhello'
        m = Model()
        html_res, text_res = self.parser._replaceAnsiColor(txt, m)
        self.assert_(text_res == 'hello' and html_res == 'hello')
        self.assert_(m.fg_color == self.parser._normal_color[3])

    def testReplaceAnsiColor2(self):
        m = Model()
        self.parser._evalStyle('31', m)
        txt = '\x1b[33mhello'
        html_res, text_res = self.parser._replaceAnsiColor(txt, m)
        self.assert_(text_res == 'hello')

    def testReplaceAnsiColor3(self):
        m = Model()
        self.parser._evalStyle('31', m)
        txt = '\x1b[33mhello'
        html_res, text_res = self.parser._replaceAnsiColor(txt, m)
        self.assert_(html_res == '<span style="color:#%s">hello</span>' %
                     self.parser._normal_color[3])

    def testReplaceAnsiColor4(self):
        m = Model()
        self.parser._evalStyle('33', m)
        txt = '\x1b[33mhello'
        html_res, text_res = self.parser._replaceAnsiColor(txt, m)
        self.assert_(html_res == 'hello')

    def testReplaceEmptyColor(self):
        m = Model()
        txt = '\x1b[mhello'
        html_res, text_res = self.parser._replaceAnsiColor(txt, m)
        self.assert_(html_res == 'hello' and text_res == 'hello')


class TestSmaugParser(unittest.TestCase):

    def setUp(self):
        self.parser = SmaugParser()

    def testEmptyPrompt(self):
        self.assert_(self.parser.buildModel('', None, None).prompt is None)

    def testFakePrompt(self):
        stats = {'Hp' : '23/24', 'Mn': '102/102', 'Mv': '26/102'}
        p = 'PF:%(Hp)s Mn:%(Mn)s Mv:%(Mv)s' % stats
        m = self.parser.buildModel('', None, None)
        m.main_text.append(p)
        self.parser._parsePrompt(m)
        self.assert_(m.prompt is None)

    def testPrompt1(self):
        stats = {'Hp' : '23/24', 'Mn': '102/102', 'Mv': '26/102'}
        p = 'PF:%(Hp)s Mn:%(Mn)s Mv:%(Mv)s>' % stats
        m = self.parser.buildModel('', None, None)
        m.main_text.append(p)
        self.parser._parsePrompt(m)
        prompt = dict(zip(stats.keys(), [v.split('/') for v in stats.values()]))
        self.assert_(m.prompt == prompt)

    def testPrompt2(self):
        stats = {'Hp' : '23/24', 'Mn': '102/102', 'Mv': '26/102'}
        p = 'PF:%(Hp)s Mn:%(Mn)s Mv:%(Mv)s bla bla bla>' % stats
        m = self.parser.buildModel('', None, None)
        m.main_text.append(p)
        self.parser._parsePrompt(m)
        prompt = dict(zip(stats.keys(), [v.split('/') for v in stats.values()]))
        self.assert_(m.prompt == prompt)

    def testPrompt3(self):
        stats = {'Hp' : '23/24', 'Mn': '102/102', 'Mv': '26/102'}
        p = 'PF:  %(Hp)s Mn:  %(Mn)s Mv:  %(Mv)s>' % stats
        m = self.parser.buildModel('', None, None)
        m.main_text.append(p)
        self.parser._parsePrompt(m)
        prompt = dict(zip(stats.keys(), [v.split('/') for v in stats.values()]))
        self.assert_(m.prompt == prompt)

    def testPrompt4(self):
        stats = {'Hp' : '23/24', 'Mn': '102/102', 'Mv': '26/102'}
        p = 'pf:  %(Hp)s mn:  %(Mn)s Mv:  %(Mv)s>' % stats
        m = self.parser.buildModel('', None, None)
        m.main_text.append(p)
        self.parser._parsePrompt(m)
        prompt = dict(zip(stats.keys(), [v.split('/') for v in stats.values()]))
        self.assert_(m.prompt == prompt)


class TestAfkParser(unittest.TestCase):

    def setUp(self):
        self.parser = AfkParser()

    def testEmptyPrompt(self):
        self.assert_(self.parser.buildModel('', None, None).prompt is None)

    def testFakePrompt(self):
        stats = {'Hp' : '23-24', 'Mn': '102-102', 'Mv': '26-102'}
        p = '[pf: %(Hp)s] [mana:%(Mn)s] [mv:%(Mv)s] [mon:0]' % stats
        m = self.parser.buildModel('', None, None)
        m.main_text.append(p)
        self.parser._parsePrompt(m)
        self.assert_(m.prompt is None)

    def testPrompt1(self):
        stats = {'Hp' : '23-24', 'Mn': '102-102', 'Mv': '26-102'}
        p = '[Pf:%(Hp)s] [Mana:%(Mn)s] [Mv:%(Mv)s] [Mon:0] [S:Xp:0]' % stats
        m = self.parser.buildModel('', None, None)
        m.main_text.append(p)
        self.parser._parsePrompt(m)
        prompt = dict(zip(stats.keys(), [v.split('-') for v in stats.values()]))
        self.assert_(m.prompt == prompt)

    def testPrompt2(self):
        stats = {'Hp' : '23-24', 'Mn': '102-102', 'Mv': '26-102'}
        p = '[Pf: %(Hp)s] [Mana: %(Mn)s] [Mv: %(Mv)s] [Mon: 0] [S:Xp:0]' % stats
        m = self.parser.buildModel('', None, None)
        m.main_text.append(p)
        self.parser._parsePrompt(m)
        prompt = dict(zip(stats.keys(), [v.split('-') for v in stats.values()]))
        self.assert_(m.prompt == prompt)

    def testPrompt3(self):
        stats = {'Hp' : '23-24', 'Mn': '102-102', 'Mv': '26-102'}
        p = '[pf: %(Hp)s] [mana:%(Mn)s] [mv:%(Mv)s] [mon:0] [s:xp: 0]' % stats
        m = self.parser.buildModel('', None, None)
        m.main_text.append(p)
        self.parser._parsePrompt(m)
        prompt = dict(zip(stats.keys(), [v.split('-') for v in stats.values()]))
        self.assert_(m.prompt == prompt)


if __name__ == '__main__':
    unittest.main()
