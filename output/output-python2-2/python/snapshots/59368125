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

import os
import re
import sys
import shutil
import os.path
import unittest

# FIX
sys.path.append('..')
sys.path.append('../configobj')

import devclient.storage as storage
from devclient.conf import config
from devclient.trigger import Trigger
from devclient.messages import Model


class TestTrigger(unittest.TestCase):
    def __init__(self, methodName='runTest'):
        super(TestTrigger, self).__init__(methodName)
        self.test_dir = '../../data/storage/test_dir'

    def setUp(self):
        if os.path.exists(self.test_dir):
            shutil.rmtree(self.test_dir)
        os.mkdir(self.test_dir)
        config['storage'] = {'path': os.path.abspath(self.test_dir)}
        storage.loadStorage()

        self.conn_name = 'conn'
        conn = [0, self.conn_name, 'host', 111]
        storage.addConnection(conn)

    def tearDown(self):
        if os.path.exists(self.test_dir):
            shutil.rmtree(self.test_dir)

    def testGetActions1(self):
        triggers = [('dwarf', 0, 'bow dwarf', '', '')]
        storage.saveTriggers(self.conn_name, triggers)
        text = "A dwarf with a long red beard mindlessly bumps into people."
        self.assert_(Trigger(self.conn_name).getActions(text) == ['bow dwarf'])

    def testGetActions2(self):
        triggers = [('* (White Aura) %w', 0, 'look %1', '', '')]
        storage.saveTriggers(self.conn_name, triggers)
        text = "(Translucent) (White Aura) Ravi, granter of deeds, sits " + \
               "calmly waiting for the next adventurer."
        self.assert_(Trigger(self.conn_name).getActions(text) == ['look Ravi'])

    def checkHighlights(self, html, highlights):
        storage.saveTriggers(self.conn_name, highlights)
        model = Model()
        model.main_html = html
        text = re.compile('(<span.*?>|</span>)').sub('', html)
        model.main_text = text.replace('<br>', '\n')
        Trigger(self.conn_name).highlights(model)
        return model

    def testHighlights1(self):
        highlights = [('* (White Aura) %w', 0, '', '#FF0000', '#000000')]
        html = "(Translucent) (White Aura) Ravi, granter of deeds"
        model = self.checkHighlights(html, highlights)

        self.assert_('<span style="background-color:#FF0000;color:#000000">'+ \
                     '(Translucent) (White Aura) Ravi</span>, granter of ' + \
                     'deeds' == model.main_html)

    def testHighlights2(self):
        highlights = [('* (White Aura) %w', 0, '', '#FF0000', '#000000')]
        html = '<span style="color:#C0C0C0">(Translucent) (White</span> ' + \
               'Aura) Ravi, granter of deeds'
        model = self.checkHighlights(html, highlights)

        self.assert_('<span style="background-color:#FF0000;color:#000000">'+ \
                     '(Translucent) (White Aura) Ravi</span>, granter of ' + \
                     'deeds' == model.main_html)

    def testHighlights3(self):
        highlights = [('* (White Aura) %w', 0, '', '#FF0000', '#000000')]
        html = '<span style="color:#C0C0C0">(Translucent) (White Aura) ' + \
               'Ravi, granter</span> of deeds'
        model = self.checkHighlights(html, highlights)

        self.assert_('<span style="background-color:#FF0000;color:#000000">'+ \
                     '(Translucent) (White Aura) Ravi</span><span style="' + \
                     'color:#C0C0C0">, granter</span> of deeds' == model.main_html)

    def testHighlights4(self):
        highlights = [('* (White Aura) %w', 0, '', '#FF0000', '#000000')]
        html = "(Translucent) <br> (White Aura) Ravi, granter of deeds"
        model = self.checkHighlights(html, highlights)

        self.assert_('(Translucent) <br><span style="background-color:' + \
                     '#FF0000;color:#000000"> (White Aura) Ravi</span>, ' + \
                     'granter of deeds' == model.main_html)

    def testHighlights5(self):
        highlights = [('* (White Aura) %w', 0, '', '#FF0000', '#000000')]
        html = '<span style="color:#C0C0C0">(Translucent) <br> (White' + \
               '</span> Aura) Ravi, granter of deeds'
        model = self.checkHighlights(html, highlights)

        self.assert_('<span style="color:#C0C0C0">(Translucent) <br>' + \
                     '<span style="background-color:#FF0000;color:#000000">' + \
                     ' (White Aura) Ravi</span></span>, granter of deeds' ==
                     model.main_html)

    def testHighlights6(self):
        highlights = [('* (White Aura) %w', 0, '', '#FF0000', '#000000')]
        html = '<span style="color:#C0C0C0">(Translucent) <br> (White' + \
               ' Aura) Ravi, granter</span> of deeds'
        model = self.checkHighlights(html, highlights)

        self.assert_('<span style="color:#C0C0C0">(Translucent) <br>' + \
                     '<span style="background-color:#FF0000;color:#000000">' + \
                     ' (White Aura) Ravi</span>, granter</span> of deeds' ==
                     model.main_html)

    def testHighlights7(self):
        highlights = [('* (White Aura) %w', 0, '', '#FF0000', '#000000')]
        html = '<span style="color:#C0C0C0">(Translucent) <br></span> (White' + \
               ' Aura) <span style="color:#FFFFFF">Ravi, granter</span> of deeds'
        model = self.checkHighlights(html, highlights)

        self.assert_('<span style="color:#C0C0C0">(Translucent) <br><span' + \
                     ' style="background-color:#FF0000;color:#000000">'+ \
                     ' (White Aura) Ravi</span></span><span style="color' + \
                     ':#FFFFFF">, granter</span> of deeds' == model.main_html)


if __name__ == '__main__':
    unittest.main()
