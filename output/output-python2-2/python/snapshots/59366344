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

from PyQt4 import QtGui

class TextViewer(object):
    """
    Build the html visualization from model.
    """

    def __init__(self, widget):
        self.widget = widget
        self.widget.text_output.clear()
        self.last_row = None

    def _process(self, model):
        new_html = model.main_html.get(self.last_row)
        bgcolor = None
        fgcolor = None

        if self.last_row is None:
            self.last_row = len(new_html) - 1
            bgcolor = model.main_bgcolor
            fgcolor = model.main_fgcolor
        else:
            self.last_row += len(new_html)

        new_html = ''.join(new_html)
        if not new_html.count('<br>'):
            new_html = '<br>' + new_html

        self.widget.text_output.moveCursor(QtGui.QTextCursor.End)
        self.widget.text_output.insertHtml(new_html)
        self.widget.text_output.moveCursor(QtGui.QTextCursor.End)
        if bgcolor or fgcolor:
            self.widget._setOutputColors(bgcolor, fgcolor)

    def process(self, model):
        self._process(model)
        self.widget.update()


class StatusViewer(TextViewer):
    """
    Build the status visualization from model.
    """

    def __init__(self, v):
        super(StatusViewer, self).__init__(v.widget)
        self.v = v

    def _process(self, model):
        self.v._process(model)

        stats = {'Hp': self.widget.bar_health,
                 'Mn': self.widget.bar_mana,
                 'Mv': self.widget.bar_movement}

        if model.prompt:
            for k, bar in stats.iteritems():
                cur_value, max_value = model.prompt[k].split('/')
                cur_value = max(int(cur_value), 0)
                cur_value = min(int(max_value), cur_value)
                bar.setValue(int(100 * cur_value / int(max_value)))

