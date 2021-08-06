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
import shutil
import os.path
import unittest

from PyQt4.QtGui import QApplication, QTextEdit, QProgressBar, QFrame, QSplitter

# FIX
sys.path.append('..')
sys.path.append('../configobj')

import devclient.storage
from devclient.viewer import *
from devclient.conf import config
from devclient.messages import Model


class FrameMock(object):
    def setVisible(self, display):
        pass


class RightWidget(object):
    def __init__(self):
        self.bar_health = QProgressBar()
        self.bar_mana = QProgressBar()
        self.bar_movement = QProgressBar()
        self.box_status = FrameMock()


class WidgetMock(object):

    def __init__(self):
        self.text_output = QTextEdit()
        self.rightwidget = RightWidget()
        self.text_output_noscroll = QTextEdit()
        self.text_output_noscroll.setVisible(False)
        self.output_splitter = QSplitter()

    def update(self):
        pass

    def connect(self, widget, signal, callback):
        pass


class ViewerTest(unittest.TestCase):

    def __init__(self, methodName='runTest'):
        super(ViewerTest, self).__init__(methodName)
        if not QApplication.instance():
            self.app = QApplication([])

        self.test_dir = '../../data/storage/test_dir'

    def setUp(self):
        if os.path.exists(self.test_dir):
            shutil.rmtree(self.test_dir)
        os.mkdir(self.test_dir)
        config['storage'] = {'path': os.path.abspath(self.test_dir)}
        storage.loadStorage()
        self.widget = WidgetMock()
        self.viewer = TextViewer(self.widget)
        self.m = Model()

    def tearDown(self):
        if os.path.exists(self.test_dir):
            shutil.rmtree(self.test_dir)


class TestTextViewer(ViewerTest):

    def testTextProcess1(self):
        """Test processing of a model of a single string"""

        text = 'hello world'
        self.m.main_html = text
        self.viewer.process(self.m)
        self.assert_(text == self.widget.text_output.toPlainText())
        self.assert_(text == self.widget.text_output_noscroll.toPlainText())

    def testTextProcess2(self):
        """Test processing of a model of two string"""

        elem = ['hello', 'world']
        self.m.main_html = ''.join(elem)
        self.viewer.process(self.m)
        w = self.widget
        self.assert_(''.join(elem) == w.text_output.toPlainText())
        self.assert_(''.join(elem) == w.text_output_noscroll.toPlainText())

    def testTextProcess3(self):
        """Test the sequence of two call at process"""

        elem = ['hello', 'world']
        self.m.main_html = ''.join(elem)
        self.viewer.process(self.m)

        elem2 = ['another', 'hello', 'world']
        self.m.main_html = ''.join(elem2)
        self.viewer.process(self.m)

        text = ''.join(elem) + ''.join(elem2)
        self.assert_(text == self.widget.text_output.toPlainText())
        self.assert_(text == self.widget.text_output.toPlainText())

    def testResetWidgets1(self):
        """Verify background and text color without a previus style"""

        self.viewer._resetWidgets()
        s = 'QTextEdit {color:#AAAAAA;background-color:#000000}'
        self.assert_(s == self.widget.text_output.styleSheet())
        self.assert_(s == self.widget.text_output_noscroll.styleSheet())

    def testResetWidgets2(self):
        """Verify background and text color with a previus style"""

        viewer = TextViewer(self.widget)
        self.widget.text_output.setStyleSheet('QTextEdit {color:#FFFF00}')
        viewer._resetWidgets()
        s = 'QTextEdit {color:#AAAAAA;background-color:#000000}'
        self.assert_(s == self.widget.text_output.styleSheet())
        self.assert_(s == self.widget.text_output_noscroll.styleSheet())

    def testResetWidgets3(self):
        self.widget.text_output.setHtml('Hello!')
        self.widget.text_output_noscroll.setHtml('Hello!')
        self.viewer._resetWidgets()
        self.assert_(self.widget.text_output.toPlainText() == '')
        self.assert_(self.widget.text_output_noscroll.toPlainText() == '')


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

        self.assert_(self.widget.rightwidget.bar_health.value() == 100 and
                     self.widget.rightwidget.bar_mana.value() == 100 and
                     self.widget.rightwidget.bar_movement.value() == 100)

    def testPromptProcess2(self):
        self.model.prompt = {'Hp': ('0', '22'),
                             'Mn': ('0', '117'),
                             'Mv': ('0', '108')}
        self.viewer.process(self.model)

        self.assert_(self.widget.rightwidget.bar_health.value() == 0 and
                     self.widget.rightwidget.bar_mana.value() == 0 and
                     self.widget.rightwidget.bar_movement.value() == 0)

    def testPromptProcess3(self):
        self.model.prompt = {'Hp': ('10', '100'),
                             'Mn': ('10', '50'),
                             'Mv': ('20', '40')}
        self.viewer.process(self.model)

        self.assert_(self.widget.rightwidget.bar_health.value() == 10 and
                     self.widget.rightwidget.bar_mana.value() == 20 and
                     self.widget.rightwidget.bar_movement.value() == 50)

    def testPromptProcess4(self):
        self.model.prompt = {'Hp': ('-10', '100'),
                             'Mn': ('60', '50'),
                             'Mv': ('10', '40')}
        self.viewer.process(self.model)

        self.assert_(self.widget.rightwidget.bar_health.value() == 0 and
                     self.widget.rightwidget.bar_mana.value() == 100 and
                     self.widget.rightwidget.bar_movement.value() == 25)


if __name__ == '__main__':
    unittest.main()


