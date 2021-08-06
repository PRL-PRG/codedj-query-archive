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
sys.path.append('../servers')

from devclient.parse import *
from devclient.messages import Model
from generics import SmaugServer, AfkServer, Server


class TestParser(unittest.TestCase):

    def setUp(self):
        self.parser = Parser(Server)

    def testParseText(self):
        txt = 'hello'
        model = self.parser.buildModel(txt)
        self.assert_(txt == model.main_text)

    def testParseTextMultiline(self):
        txt = 'hello\nworld'
        model = self.parser.buildModel(txt)
        self.assert_('hello<br>world' == model.main_html)

    def testParseMultiText(self):
        txt1, txt2 = 'hello', 'world'
        m1 = self.parser.buildModel(txt1)
        m2 = self.parser.buildModel(txt2)
        self.assert_(txt1 == m1.main_html)
        self.assert_(txt2 == m2.main_html)

    def testParseMultiText2(self):
        txt1, txt2 = 'hello\x1b[0;', '33mworld'
        m1 = self.parser.buildModel(txt1)
        m2 = self.parser.buildModel(txt2)
        self.assert_('hello' == m1.main_html)
        self.assert_('<span style="color:#aaaa00">world</span>' == m2.main_html)

    def testParseMultiText3(self):
        txt1, txt2 = 'hello\x1b', '[0;42mworld'
        m1 = self.parser.buildModel(txt1)
        m2 = self.parser.buildModel(txt2)

        self.assert_('hello' == m1.main_html)
        self.assert_('<span style="background-color:#00aa00">world</span>' ==
                     m2.main_html)

    def testParseMultiText4(self):
        txt1, txt2 = '\x1b[0;','33mhello'
        m1 = self.parser.buildModel(txt1)
        m2 = self.parser.buildModel(txt2)
        self.assert_('<span style="color:#aaaa00">hello</span>' == m2.main_html)
        self.assert_(self.parser._normal_color[3] == m2.fg_color)

    def testParseMultiText5(self):
        txt1, txt2 = '\x1b[0;32mh\x1b[0;33mello', 'world,\x1b[0;34mhello'
        m1 = self.parser.buildModel(txt1)
        m2 = self.parser.buildModel(txt2)
        self.assert_('hello' == m1.main_text)
        self.assert_(self.parser._normal_color[2] == m1.fg_color)
        self.assert_('<span style="color:#aaaa00">world,</span>' + \
                     '<span style="color:#0000aa">hello</span>' ==
                     m2.main_html)

    def testParseMultiText6(self):
        txt1, txt2 = '\x1b[0;32mh\x1b[0;33mello', 'world'
        m1 = self.parser.buildModel(txt1)
        m2 = self.parser.buildModel(txt2)
        self.assert_('hello' == m1.main_text)
        self.assert_(self.parser._normal_color[2] == m1.fg_color)
        self.assert_('<span style="color:#aaaa00">world</span>' == m2.main_html)

    def testParseMultiText7(self):
        txt1, txt2 = 'h\x1b[0;33me\x1b[0;32m\x1b[0;33mllo', 'world'
        m1 = self.parser.buildModel(txt1)
        m2 = self.parser.buildModel(txt2)
        self.assert_('hello' == m1.main_text)
        self.assert_(self.parser._normal_color[3] == m1.fg_color)
        self.assert_('<span style="color:#aaaa00">world</span>' == m2.main_html)

    def testParseMultiText8(self):
        m = self.parser.buildModel('hello')
        txt = '\x1b[33m\x1b[42mworld'
        m = self.parser.buildModel(txt)
        self.assert_(m.main_html == '<span style="color:#aaaa00;' + \
                                    'background-color:#00aa00">world</span>')

    def testParseMultiText9(self):
        m = self.parser.buildModel('hello')
        txt = '\x1b[42m\x1b[33mworld'
        m = self.parser.buildModel(txt)
        self.assert_(m.main_html == '<span style="color:#aaaa00;' + \
                                    'background-color:#00aa00">world</span>')

    def testParseSpace(self):
        txt = 'hello world'
        m = self.parser.buildModel(txt)
        self.assert_(txt.replace(' ','&nbsp;') == m.main_html)

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
        self.assert_(text_res == 'hello')
        self.assert_(html_res == '<span style="color:#aaaa00">hello</span>')
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
        self.assert_(html_res == '<span style="color:#aaaa00">hello</span>')

    def testReplaceEmptyColor(self):
        m = Model()
        txt = '\x1b[mhello'
        html_res, text_res = self.parser._replaceAnsiColor(txt, m)
        self.assert_(html_res == 'hello' and text_res == 'hello')


class TestSmaugParser(unittest.TestCase):

    def setUp(self):
        self.parser = PromptParser(Parser(SmaugServer))
        self.m = self.parser.buildModel('')

    def testEmptyPrompt(self):
        self.assert_(self.m.prompt is None)

    def testFakePrompt(self):
        stats = {'Hp' : '23/24', 'Mn': '102/102', 'Mv': '26/102'}
        self.m.main_text = 'PF:%(Hp)s Mn:%(Mn)s Mv:%(Mv)s' % stats
        self.parser._parsePrompt(self.m)
        self.assert_(self.m.prompt is None)

    def testPrompt1(self):
        stats = {'Hp' : '23/24', 'Mn': '102/102', 'Mv': '26/102'}
        self.m.main_text = 'PF:%(Hp)s Mn:%(Mn)s Mv:%(Mv)s>' % stats
        self.parser._parsePrompt(self.m)
        prompt = dict(zip(stats.keys(), [v.split('/') for v in stats.values()]))
        self.assert_(self.m.prompt == prompt)

    def testPrompt2(self):
        stats = {'Hp' : '23/24', 'Mn': '102/102', 'Mv': '26/102'}
        self.m.main_text = 'PF:%(Hp)s Mn:%(Mn)s Mv:%(Mv)s bla bla bla>' % stats
        self.parser._parsePrompt(self.m)
        prompt = dict(zip(stats.keys(), [v.split('/') for v in stats.values()]))
        self.assert_(self.m.prompt == prompt)

    def testPrompt3(self):
        stats = {'Hp' : '23/24', 'Mn': '102/102', 'Mv': '26/102'}
        self.m.main_text = 'PF:  %(Hp)s Mn:  %(Mn)s Mv:  %(Mv)s>' % stats
        self.parser._parsePrompt(self.m)
        prompt = dict(zip(stats.keys(), [v.split('/') for v in stats.values()]))
        self.assert_(self.m.prompt == prompt)

    def testPrompt4(self):
        stats = {'Hp' : '23/24', 'Mn': '102/102', 'Mv': '26/102'}
        self.m.main_text = 'pf:  %(Hp)s mn:  %(Mn)s Mv:  %(Mv)s>' % stats
        self.parser._parsePrompt(self.m)
        prompt = dict(zip(stats.keys(), [v.split('/') for v in stats.values()]))
        self.assert_(self.m.prompt == prompt)


class TestAfkParser(unittest.TestCase):

    def setUp(self):
        self.parser = PromptParser(Parser(AfkServer))
        self.m = self.parser.buildModel('')

    def testEmptyPrompt(self):
        self.assert_(self.m.prompt is None)

    def testFakePrompt(self):
        stats = {'Hp' : '23-24', 'Mn': '102-102', 'Mv': '26-102'}
        p = '[pf: %(Hp)s] [mana:%(Mn)s] [mv:%(Mv)s] [mon:0]' % stats
        self.m.main_text = p
        self.parser._parsePrompt(self.m)
        self.assert_(self.m.prompt is None)

    def testPrompt1(self):
        stats = {'Hp' : '23-24', 'Mn': '102-102', 'Mv': '26-102'}
        p = '[Pf:%(Hp)s] [Mana:%(Mn)s] [Mv:%(Mv)s] [Mon:0] [S:Xp:0]' % stats
        self.m.main_text = p
        self.parser._parsePrompt(self.m)
        prompt = dict(zip(stats.keys(), [v.split('-') for v in stats.values()]))
        self.assert_(self.m.prompt == prompt)

    def testPrompt2(self):
        stats = {'Hp' : '23-24', 'Mn': '102-102', 'Mv': '26-102'}
        p = '[Pf: %(Hp)s] [Mana: %(Mn)s] [Mv: %(Mv)s] [Mon: 0] [S:Xp:0]' % stats
        self.m.main_text = p
        self.parser._parsePrompt(self.m)
        prompt = dict(zip(stats.keys(), [v.split('-') for v in stats.values()]))
        self.assert_(self.m.prompt == prompt)

    def testPrompt3(self):
        stats = {'Hp' : '23-24', 'Mn': '102-102', 'Mv': '26-102'}
        p = '[pf: %(Hp)s] [mana:%(Mn)s] [mv:%(Mv)s] [mon:0] [s:xp: 0]' % stats
        self.m.main_text = p
        self.parser._parsePrompt(self.m)
        prompt = dict(zip(stats.keys(), [v.split('-') for v in stats.values()]))
        self.assert_(self.m.prompt == prompt)


class TestWildMapParser(unittest.TestCase):

    def setUp(self):
         self.parser = WildMapParser(Parser(Server))

    def testWild1(self):
        """Check parsing of simple wild map"""

        start = 'La montagna05\n'
        wild = '             \n             \n             \n' + \
               '             \n     ^X^     \n     ^^^     \n' + \
               '             \n             \n             \n'
        end = '\n[Uscite: Est Sud Ovest]'

        m = self.parser.buildModel(start + wild + end)
        self.assert_(m.wild_text == wild)

    def testWild2(self):
        """Check extraction of wild map from main_text"""

        start = 'La montagna05\n'
        wild = '             \n             \n             \n' + \
               '             \n     ^X^     \n     ^^^     \n' + \
               '             \n             \n             \n'
        end = '\n[Uscite: Est Sud Ovest]'

        m = self.parser.buildModel(start + wild + end)
        self.assert_(m.main_text == start + end)

    def testWild3(self):
        """Check parsing of wild map in two step"""

        start = 'La montagna05\n'
        wild = '             \n             \n             \n' + \
               '             \n     ^X^     \n     ^^^     \n' + \
               '             \n             \n             \n'
        end = '\n[Uscite: Est Sud Ovest]'

        m = self.parser.buildModel(start + wild[:len(wild) / 2])
        self.assert_(m.wild_text == '')
        m = self.parser.buildModel(wild[len(wild) / 2:] + end)
        self.assert_(m.wild_text == wild)

    def testWild4(self):
        """
        Check parsing of wild map in two step (with end sequence truncated)
        """

        start = 'La montagna05\n'
        wild = '             \n             \n             \n' + \
                '             \n     ^X^     \n     ^^^     \n' + \
                '             \n             \n             \n'
        end = '\n[Uscite: Est Sud Ovest]'

        m = self.parser.buildModel(start + wild + end[:3])
        self.assert_(m.wild_text == '')
        m = self.parser.buildModel(end[3:])
        self.assert_(m.wild_text == wild)

    def testWild5(self):
        """
        Check extraction of wild map from main_text (parsed in two step)
        """

        start = 'La montagna05\n'
        wild = '             \n             \n             \n' + \
               '             \n     ^X^     \n     ^^^     \n' + \
               '             \n             \n             \n'
        end = '\n[Uscite: Est Sud Ovest]'

        m = self.parser.buildModel(start + wild[:len(wild) / 2])
        self.assert_(m.main_text == '')
        m = self.parser.buildModel(wild[len(wild) / 2:] + end)
        self.assert_(m.main_text == start + end)

    def testWild6(self):
        """
        Check extraction of wild map from main_text (parsed in two step with
        end sequence truncated)
        """

        start = 'La montagna05\n'
        wild = '             \n             \n             \n' + \
               '             \n     ^X^     \n     ^^^     \n' + \
               '             \n             \n             \n'
        end = '\n[Uscite: Est Sud Ovest]'

        m = self.parser.buildModel(start + wild + end[:3])
        self.assert_(m.main_text == '')
        m = self.parser.buildModel(end[3:])
        self.assert_(m.main_text == start + end)

    def testWild7(self):
        """Test sequence of parsing a fake map after a wild map"""

        start = 'La montagna05\n'
        wild = '             \n             \n             \n' + \
               '             \n     ^X^     \n     ^^^     \n' + \
               '             \n             \n             \n'
        end = '\n[Uscite: Est Sud Ovest]'

        m = self.parser.buildModel(start + wild + end)
        self.assert_(m.wild_text == wild)
        m = self.parser.buildModel('fake wild')
        self.assert_(m.wild_text == '')

    def testWild8(self):
        """
        Test sequence of parsing a fake map after a wild map (parsed in two
        step)
        """

        start = 'La montagna05\n'
        wild = '             \n             \n             \n' + \
               '             \n     ^X^     \n     ^^^     \n' + \
               '             \n             \n             \n'
        end = '\n[Uscite: Est Sud Ovest]'

        m = self.parser.buildModel(start + wild + end[:3])
        self.assert_(m.wild_text == '')
        m = self.parser.buildModel(end[3:])
        self.assert_(m.wild_text == wild)
        m = self.parser.buildModel('fake wild')
        self.assert_(m.wild_text == '')

    def testWild9(self):
        """Test parsing a map with a right part1 and a wrong part2"""

        start = 'La montagna05\n'
        wild = '             \n             \n             \n' + \
               '             \n     ^X^     \n     ^^^     \n' + \
               '             \n             \n             \n'
        end = 'fake end'

        m = self.parser.buildModel(start + wild[:len(wild) / 2])
        self.assert_(m.wild_text == '')
        m = self.parser.buildModel(wild[len(wild) / 2:] + end)
        self.assert_(m.wild_text == '')

    def testWild10(self):
        """
        Test integrity of main_text after parsing a map with a right part1
        and a wrong part2
        """

        start = 'La montagna05\n'
        wild = '             \n             \n             \n' + \
               '             \n     ^X^     \n     ^^^     \n' + \
               '             \n             \n             \n'
        end = 'fake end'

        m = self.parser.buildModel(start + wild[:len(wild) / 2])
        self.assert_(m.wild_text == '')
        m = self.parser.buildModel(wild[len(wild) / 2:] + end)
        self.assert_(m.main_text == start + wild + end)

    def testWild11(self):
        """Check parsing of wild map in three step: two part1 and a part2"""

        start = 'La montagna05\n'
        wild = '             \n             \n             \n' + \
               '             \n     ^X^     \n     ^^^     \n' + \
               '             \n             \n             \n'
        end = '\n[Uscite: Est Sud Ovest]'

        m = self.parser.buildModel(start + wild[:len(wild) / 2])
        self.assert_(m.wild_text == '')
        m = self.parser.buildModel(start + wild[:len(wild) / 2])
        m = self.parser.buildModel(wild[len(wild) / 2:] + end)
        self.assert_(m.wild_text == wild)

    def testWild12(self):
        """Test parsing a map with a right part1 and a wrong part2"""

        start = 'La montagna05\n'
        wild = '             \n             \n             \n' + \
               '             \n     ^X^     \n     ^^^     \n' + \
               '             \n             \n             \n'
        end = 'fake end'

        p1 = start + wild[:len(wild) / 2]
        m = self.parser.buildModel(p1)
        self.assert_(self.parser._incomplete_map[0] == p1)
        m = self.parser.buildModel(wild[len(wild) / 2:] + end)
        self.assert_(not self.parser._incomplete_map)

    def testWild13(self):
        """
        Test the parsing of two map, with first model built with a text
        composed by the first map and a slice of second map.
        """

        start = 'La montagna05\n'
        wild = '             \n             \n             \n' + \
               '             \n     ^X^     \n     ^^^     \n' + \
               '             \n             \n             \n'
        end = '\n[Uscite: Est Sud Ovest]'

        txt = start + wild + end
        m = self.parser.buildModel(txt + start + wild[:len(wild) / 2])
        self.assert_(m.wild_text == wild)
        m = self.parser.buildModel(wild[len(wild) / 2:] + end)
        self.assert_(m.wild_text == wild)

    def testWild14(self):
        """
        Test the parsing of two map, with first model built with a text
        composed by the first map and a slice of second map.
        """

        s1 = 'La campagna02\n'
        p1a = '     *.* .   \n.. * *.*.**  \n...*.*....**.\n' + \
             '....x...@....\n......X......\n..*..*....'
        m = self.parser.buildModel(s1 + p1a)

        p1b = '..@\n  .. ......*.\n... ........ \n.. ..........\n'
        e1 = '\n[Uscite: Nord Est Sud Ovest]\n\n> '
        s2 = 'La campagna02\n'
        p2a = '  * *.*.**  \n   *.*....**.\n    x...@....\n     ........\n' + \
              '     *X.....@\n     ......*.\n    ....'
        m = self.parser.buildModel(p1b + e1 + s2 + p2a)
        self.assert_(m.wild_text == p1a + p1b)

        p2b = '.... \n   ..........\n  ...........\n'
        e2 = '\n[Uscite: Nord Est Sud Ovest]\n\n> '

        m = self.parser.buildModel(p2b + e2)
        self.assert_(m.wild_text == p2a + p2b)

    def testExtractHtml1(self):
        html = '<span color="#cc00cc">hello</span>&nbsp;world'
        parts = self.parser._getHtmlFromText(html, ('hello',' world'))
        self.assert_(parts[0] == '<span color="#cc00cc">hello</span>')
        self.assert_(parts[1] == '&nbsp;world')

    def testExtractHtml2(self):
        html = 'hello&nbsp;<span color="#cc00cc">world</span>'
        parts = self.parser._getHtmlFromText(html, ('hello ','world'))
        self.assert_(parts[0] == 'hello&nbsp;')
        self.assert_(parts[1] == '<span color="#cc00cc">world</span>')


if __name__ == '__main__':
    unittest.main()
