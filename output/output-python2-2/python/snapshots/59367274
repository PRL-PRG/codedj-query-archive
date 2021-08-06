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
import os.path
import unittest

sys.path.append('..')

from devclient.viewer import *
from devclient.conf import config
from devclient.messages import Model


class TextDocMock(object):
    def setMaximumBlockCount(self, num):
        pass


class TextCursorMock(object):
    def __init__(self, text_mock):
        self.text_mock = text_mock

    def movePosition(self, where):
        pass

    def insertBlock(self):
        pass

    def insertHtml(self, html):
        self.text_mock._html += html


class TextMock(object):

    def __init__(self):
        self._html = ''
        self._style = ''

    def setTextCursor(self, cursor):
        pass

    def textCursor(self):
        return TextCursorMock(self)

    def document(self):
        return TextDocMock()

    def styleSheet(self):
        return self._style

    def setStyleSheet(self, style):
        self._style = style


class BarMock(object):

    def __init__(self):
        self._value = None

    def setValue(self, value):
        self._value = value


class FrameMock(object):
    def setVisible(self, display):
        pass


class RightWidget(object):
    def __init__(self):
        self.bar_health = BarMock()
        self.bar_mana = BarMock()
        self.bar_movement = BarMock()
        self.box_status = FrameMock()

class WidgetMock(object):

    def __init__(self):
        self.text_output = TextMock()
        self.rightwidget = RightWidget()

    def update(self):
        pass


class TestTextViewer(unittest.TestCase):

    def setUp(self):
        self.widget = WidgetMock()
        self.viewer = TextViewer(self.widget)
        self.m = Model()

    def testTextProcess1(self):
        """Test processing of a model of a single string"""

        text = 'hello world'
        self.m.main_html = text
        self.viewer.process(self.m)
        self.assert_(text == self.widget.text_output._html)

    def testTextProcess2(self):
        """Test processing of a model of two string"""

        elem = ['hello', 'world']
        self.m.main_html = ''.join(elem)
        self.viewer.process(self.m)
        self.assert_(''.join(elem) == self.widget.text_output._html)

    def testTextProcess3(self):
        """Test the sequence of two call at process"""

        elem = ['hello', 'world']
        self.m.main_html = ''.join(elem)
        self.viewer.process(self.m)

        elem2 = ['another', 'hello', 'world']
        self.m.main_html = ''.join(elem2)
        self.viewer.process(self.m)

        text = ''.join(elem) + ''.join(elem2)
        self.assert_(text == self.widget.text_output._html)

    def testTextProcess4(self):
        """Verify background and text color without a previus style"""

        self.m.bg_color = '000000'
        self.m.fg_color = 'FFFFFF'
        self.viewer.process(self.m)
        self.assert_('QTextEdit {color:#FFFFFF;background-color:#000000}' ==
                     self.widget.text_output.styleSheet())

    def testTextProcess5(self):
        """Verify background and text color with a previus style"""

        viewer = TextViewer(self.widget)
        self.m.bg_color = 'FF00FF'
        self.m.fg_color = 'CCCCCC'
        self.widget.text_output.setStyleSheet('QTextEdit {color:#FFFF00}')
        viewer.process(self.m)
        self.assert_('QTextEdit {color:#CCCCCC;background-color:#FF00FF}' ==
                     self.widget.text_output.styleSheet())


class TestStatusViewer(unittest.TestCase):

    def setUp(self):
        self.model = Model()
        self.widget = WidgetMock()
        self.viewer = StatusViewer(TextViewer(self.widget))

    def testPromptProcess1(self):
        self.model.prompt = {'Hp': ('22', '22'),
                             'Mn': ('117', '117'),
                             'Mv': ('108', '108')}
        self.viewer.process(self.model)

        self.assert_(self.widget.rightwidget.bar_health._value == 100 and
                     self.widget.rightwidget.bar_mana._value == 100 and
                     self.widget.rightwidget.bar_movement._value == 100)

    def testPromptProcess2(self):
        self.model.prompt = {'Hp': ('0', '22'),
                             'Mn': ('0', '117'),
                             'Mv': ('0', '108')}
        self.viewer.process(self.model)

        self.assert_(self.widget.rightwidget.bar_health._value == 0 and
                     self.widget.rightwidget.bar_mana._value == 0 and
                     self.widget.rightwidget.bar_movement._value == 0)

    def testPromptProcess3(self):
        self.model.prompt = {'Hp': ('10', '100'),
                             'Mn': ('10', '50'),
                             'Mv': ('20', '40')}
        self.viewer.process(self.model)

        self.assert_(self.widget.rightwidget.bar_health._value == 10 and
                     self.widget.rightwidget.bar_mana._value == 20 and
                     self.widget.rightwidget.bar_movement._value == 50)

    def testPromptProcess4(self):
        self.model.prompt = {'Hp': ('-10', '100'),
                             'Mn': ('60', '50'),
                             'Mv': ('10', '40')}
        self.viewer.process(self.model)

        self.assert_(self.widget.rightwidget.bar_health._value == 0 and
                     self.widget.rightwidget.bar_mana._value == 100 and
                     self.widget.rightwidget.bar_movement._value == 25)


if __name__ == '__main__':
    unittest.main()


