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

import re
import logging

from PyQt4 import QtGui

logger = logging.getLogger('viewer')


def _setRightPanel(widget, widget_name):

    # FIX: find a way to delete the RightWidget instance (child of rightpanel)
    for w in widget.rightpanel.children():
        w.setVisible(False)

    if widget_name:
        try:
            module = __import__(widget_name, globals(), locals())

            class RightWidget(QtGui.QWidget, module.Ui_RightWidget):
                def __init__(self, parent):
                    QtGui.QWidget.__init__(self, parent)
                    self.module_name = widget_name
                    self.setupUi(self)

        except ImportError:
            logger.warning('_setRightPanel: Unknown widget %s' % widget_name)
            return False
        else:
            for w in widget.rightpanel.children():
                if w.module_name == widget_name:
                    widget.rightwidget = w
                    break
            else:
                widget.rightwidget = RightWidget(widget.rightpanel)

            widget.rightwidget.setVisible(True)

    return True


def getViewer(widget, server):

    viewer = TextViewer(widget)
    if not _setRightPanel(widget, server.right_widget):
        return viewer

    if hasattr(server, 'prompt_reg') and hasattr(server, 'prompt_sep'):
        viewer = StatusViewer(viewer)

    if hasattr(server, 'wild_map'):
        viewer = WildMapViewer(viewer)

    return viewer


class TextViewer(object):
    """
    Build the html visualization from model.
    """

    MAX_ROWS = 200
    """The max number of rows displayed in textEdit field"""

    def __init__(self, widget):
        self.w = widget
        self.w.text_output.clear()
        self.data = ''
        self._bg = None
        self._fg = None

    def _process(self, model):

        new_html = model.main_html
        if not new_html.count('<br>'):
            new_html = '<br>' + new_html

        rows = self.data.count('<br>') + new_html.count('<br>')
        if rows > self.MAX_ROWS:
            data = self.data.split('<br>')[rows - self.MAX_ROWS:]
            self.data = '<br>'.join(data) + new_html
        else:
            self.data += new_html

        self.w.text_output.clear()
        set_color = False

        if model.bg_color is not None and self._bg != model.bg_color:
            self._bg = model.bg_color
            set_color = True

        if model.fg_color is not None and self._fg != model.fg_color:
            self._fg = model.fg_color
            set_color = True

        if set_color:
            self._setOutputColors(self._bg, self._fg)

        self.w.text_output.setHtml(self.data)
        self.w.text_output.moveCursor(QtGui.QTextCursor.End)

    def _setOutputColors(self, bg, fg):
        """
        Set output default colors.
        """

        style = unicode(self.w.text_output.styleSheet())
        m = re.search('QTextEdit\s*{(.*)}', style)
        if m:
            oldstyle = m.group(1)
            tmp = [el.split(':') for el in oldstyle.split(';') if el]
            d = dict([(k.strip(), v.strip()) for k, v in tmp])
        else:
            oldstyle = None
            d = {}

        if bg: d['background-color'] = '#' + bg
        if fg: d['color'] = '#' + fg

        newstyle = ';'.join([k + ':' + v for k, v in d.iteritems()])

        if oldstyle:
            self.w.text_output.setStyleSheet(style.replace(oldstyle, newstyle))
        else:
            self.w.text_output.setStyleSheet('QTextEdit {%s}' % newstyle)

    def resetOutputColors(self, style):
        self.w.text_output.setStyleSheet(style)

    def process(self, model):
        self._process(model)
        self.w.update()


class StatusViewer(TextViewer):
    """
    Build the status visualization from model.

    This class is a subclass of `TextViewer` that take an instance of it
    as argument on __init__ (see `decorator pattern`_)

.. _decorator pattern: http://en.wikipedia.org/wiki/Decorator_pattern
    """

    def __init__(self, v):
        super(StatusViewer, self).__init__(v.w)
        self.v = v
        self._last_values = {'Hp': None, 'Mn': None, 'Mv': None}

    def _process(self, model):
        self.v._process(model)

        stats = {'Hp': self.w.rightwidget.bar_health,
                 'Mn': self.w.rightwidget.bar_mana,
                 'Mv': self.w.rightwidget.bar_movement}

        if model.prompt:
            for k, bar in stats.iteritems():
                cur_value, max_value = model.prompt[k]
                cur_value = max(int(cur_value), 0)
                cur_value = min(int(max_value), cur_value)
                v = int(100 * cur_value / int(max_value))
                if v != self._last_values[k]:
                    self._last_values[k] = v
                    bar.setValue(v)


class WildMapViewer(TextViewer):

    def __init__(self, v):
        super(WildMapViewer, self).__init__(v.w)
        self.v = v

    def _centerMap(self, model, width, height):
        #html_list = model.wild_html.split('<br>')
        text_list = model.wild_text.split('\n')

        text, html = [], []
        for i, p in enumerate(text_list):
            if p.strip():
                #html.append(html_list[i])
                text.append(p)

        delta_y = (height - len(text)) / 2
        delta_x = (width - len(max(text_list, key=len))) / 2
        model.wild_text = '\n' * delta_y + \
            '\n'.join([' ' * delta_x + r for r in text])
        #model.wild_html = '<br>' * delta_y + \
        #    '<br>'.join(['&nbsp;' * delta_x + r for r in text])

    def _process(self, model):
        self.v._process(model)
        if model.wild_text:
            w = self.w.rightwidget.wild_map.property('char_width').toInt()[0]
            h = self.w.rightwidget.wild_map.property('char_height').toInt()[0]
            self._centerMap(model, w, h)
            #self.w.rightwidget.wild_map.setHtml(model.wild_html)
            self.w.rightwidget.wild_map.setText(model.wild_text)
