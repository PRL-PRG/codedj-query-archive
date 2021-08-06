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
import os.path
import unittest

sys.path.append('..')

from devclient.conf import config
from devclient.viewer import *
from devclient.model import Model

class TextMock(object):

    def __init__(self):
        self._html = ''

    def clear(self):
        self._html = ''

    def moveCursor(self, cursor):
        pass

    def insertHtml(self, html):
        self._html += html


class BarMock(object):

    def __init__(self):
        self._value = None

    def setValue(self, value):
        self._value = value


class WidgetMock(object):

    def __init__(self):
        self.text_output = TextMock()
        self.bar_health = BarMock()
        self.bar_mana = BarMock()
        self.bar_movement = BarMock()

        self._bg = None
        self._fg = None

    def _setOutputColors(self, bg, fg):
        self._bg = bg
        self._fg = fg

    def update(self):
        pass

class TestTextViewer(unittest.TestCase):

    def setUp(self):
        self.model = Model()
        self.widget = WidgetMock()
        self.viewer = TextViewer(self.widget)

    def testTextProcess1(self):
        text = 'hello world'
        self.model.main_html.append(text)
        self.viewer.process(self.model)

        self.assert_('<br>' + text == self.widget.text_output._html)

    def testTextProcess2(self):
        elem = ['hello', 'world']
        for text in elem:
            self.model.main_html.append(text)
        self.viewer.process(self.model)

        self.assert_('<br>' + ''.join(elem) == self.widget.text_output._html)

    def testTextProcess3(self):
        elem = ['hello', 'world']
        for text in elem:
            self.model.main_html.append(text)
        self.viewer.process(self.model)

        elem2 = ['another', 'hello', 'world']
        for text in elem2:
            self.model.main_html.append(text)
        self.viewer.process(self.model)

        text = '<br>' + ''.join(elem) + '<br>' + ''.join(elem2)
        self.assert_(text == self.widget.text_output._html)

    def testTextProcess4(self):
        self.model.main_bgcolor = '000000'
        self.model.main_fgcolor = 'FFFFFF'
        self.viewer.process(self.model)
        self.assert_(self.model.main_bgcolor == self.widget._bg and
                     self.model.main_fgcolor == self.widget._fg)


class TestStatusViewer(unittest.TestCase):

    def setUp(self):
        self.model = Model()
        self.widget = WidgetMock()
        self.viewer = StatusViewer(TextViewer(self.widget))

    def testPromptProcess1(self):
        self.model.prompt = {'Hp': '22/22', 'Mn': '117/117', 'Mv': '108/108'}
        self.viewer.process(self.model)

        self.assert_(self.widget.bar_health._value == 100 and
                     self.widget.bar_mana._value == 100 and
                     self.widget.bar_movement._value == 100)

    def testPromptProcess2(self):
        self.model.prompt = {'Hp': '0/22', 'Mn': '0/117', 'Mv': '0/108'}
        self.viewer.process(self.model)

        self.assert_(self.widget.bar_health._value == 0 and
                     self.widget.bar_mana._value == 0 and
                     self.widget.bar_movement._value == 0)

    def testPromptProcess3(self):
        self.model.prompt = {'Hp': '10/100', 'Mn': '10/50', 'Mv': '20/40'}
        self.viewer.process(self.model)

        self.assert_(self.widget.bar_health._value == 10 and
                     self.widget.bar_mana._value == 20 and
                     self.widget.bar_movement._value == 50)

    def testPromptProcess4(self):
        self.model.prompt = {'Hp': '-10/100', 'Mn': '60/50', 'Mv': '10/40'}
        self.viewer.process(self.model)

        self.assert_(self.widget.bar_health._value == 0 and
                     self.widget.bar_mana._value == 100 and
                     self.widget.bar_movement._value == 25)


if __name__ == '__main__':
    unittest.main()


