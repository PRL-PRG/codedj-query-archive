#!/usr/bin/env python
#-*- coding: utf-8 -*-
#

import sys
import unittest

sys.path.append('../modules')

import exception
from parser_module import Parser

class TestParser(unittest.TestCase):

    def setUp(self):
        self.parser = Parser()

    def testParseText(self):
        txt = 'hello'
        self.parser.parse(txt)
        self.assert_(txt == self.parser.model.main_text[0])

    def testParseTextMultiline(self):
        txt = 'hello\nworld'
        self.parser.parse(txt)
        self.assert_(txt.replace('\n','<br>') == self.parser.model.main_text[0])

    def testParseMultiText(self):
        txt1, txt2 = 'hello', 'world'
        self.parser.parse(txt1)
        self.parser.parse(txt2)
        self.assert_('%s<br>%s' % (txt1, txt2) == \
                     '<br>'.join(self.parser.model.main_text))

    def testParseSpace(self):
        txt = 'hello world'
        self.parser.parse(txt)
        self.assert_(txt.replace(' ','&nbsp;') == self.parser.model.main_text[0])

    def testGetStyle1(self):
        self.assert_("color:#%s" % self.parser._normal_color[1] == \
                     self.parser._getStyle('31'))

    def testGetStyle2(self):
        self.assert_("background-color:#%s" % self.parser._normal_color[2] == \
                     self.parser._getStyle('0;42'))

    def testGetStyle3(self):
        styles = self.parser._getStyle('35;40').split(';')
        bg = "background-color:#%s" % self.parser._normal_color[0]
        fg = "color:#%s" % self.parser._normal_color[5]
        self.assert_(set([bg, fg]) == set(styles))

    def testGetStyle4(self):
        styles = self.parser._getStyle('1;36;41').split(';')
        bg = "background-color:#%s" % self.parser._normal_color[1]
        fg = "color:#%s" % self.parser._bright_color[6]
        self.assert_(set([bg, fg]) == set(styles))

    def testReplaceAnsiColor(self):
        txt = '\x1b[33mhello'
        res = self.parser._replaceAnsiColor(txt)

        self.assert_(res == '<span style="color:#%s">hello</span>' % \
                     self.parser._normal_color[3])


if __name__ == '__main__':
    unittest.main()
