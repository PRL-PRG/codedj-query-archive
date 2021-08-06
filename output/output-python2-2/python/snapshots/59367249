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

from sip import delete
from PyQt4.QtGui import QWidget, QTextCursor

logger = logging.getLogger('viewer')

_default_styles = {}

def _setRightPanel(widget, widget_name):

    # delete the old widget
    map(delete, widget.rightpanel.children())

    if widget_name:
        try:
            parent = __import__('gui_src.' + widget_name, globals(), locals())
            module = getattr(parent, widget_name)

            class RightWidget(QWidget, module.Ui_RightWidget):
                def __init__(self, parent):
                    QWidget.__init__(self, parent)
                    self.module_name = widget_name
                    self.setupUi(self)

        except ImportError:
            logger.warning('_setRightPanel: Unknown widget %s' % widget_name)
            return False
        else:
            widget.rightwidget = RightWidget(widget.rightpanel)
            widget.rightwidget.box_status.setVisible(False)
            widget.rightwidget.setVisible(True)

            # resize the window to display properly the new widget
            cur = widget.rightpanel.minimumWidth()
            new = widget.rightwidget.minimumWidth()
            widget.setMinimumWidth(widget.minimumWidth() + new - cur)
            widget.rightpanel.setMinimumWidth(new)

    return True


def getViewer(widget, server, custom_prompt=False):

    viewer = TextViewer(widget)
    if _setRightPanel(widget, server.right_widget):
        if hasattr(server, 'prompt_reg') or custom_prompt:
            viewer = StatusViewer(viewer)

        if hasattr(server, 'wild_chars'):
            viewer = MapViewer(viewer)

    if hasattr(server, 'gui_width'):
        widget.resize(server.gui_width, widget.height())

    viewer._resetWidgets()
    return viewer


class TextViewer(object):
    """
    Build the html visualization from model.
    """

    MAX_ROWS = 300
    """The max number of rows displayed in textEdit field"""

    ROW_BLOCK = 10
    """The max number of rows per block"""

    def __init__(self, widget):
        self.w = widget
        self._bg = None
        self._fg = None
        doc = self.w.text_output.document()
        doc.setMaximumBlockCount(self.MAX_ROWS / self.ROW_BLOCK)
        self._cur_rows = 0

    def _resetWidgets(self):
        t_out = self.w.text_output
        if not _default_styles.has_key('text_output'):
            _default_styles['text_output'] = unicode(t_out.styleSheet())
        else:
            t_out.setStyleSheet(_default_styles['text_output'])
            t_out.clear()

    def appendHtml(self, html):
        new_html = html.split('<br>')
        cursor = self.w.text_output.textCursor()
        cursor.movePosition(QTextCursor.End)
        cursor.beginEditBlock()

        new_block = True
        while(new_html):
            if not self._cur_rows and not new_block:
                cursor.insertBlock()

            num_rows = len(new_html[:self.ROW_BLOCK - self._cur_rows])
            cursor.insertHtml('<br>'.join(new_html[:num_rows]))
            new_html = new_html[num_rows:]
            self._cur_rows = (num_rows + self._cur_rows) % self.ROW_BLOCK
            new_block = False

        cursor.endEditBlock()
        self.w.text_output.setTextCursor(cursor)

    def _textEditColors(self, model, text_edit):
        set_colors = False

        if model.bg_color is not None and self._bg != model.bg_color:
            self._bg = model.bg_color
            set_colors = True

        if model.fg_color is not None and self._fg != model.fg_color:
            self._fg = model.fg_color
            set_colors = True

        if not set_colors:
            return

        style = unicode(text_edit.styleSheet())
        m = re.search('QTextEdit\s*{(.*)}', style)
        if m:
            oldstyle = m.group(1)
            tmp = [el.split(':') for el in oldstyle.split(';') if el]
            d = dict([(k.strip(), v.strip()) for k, v in tmp])
        else:
            oldstyle = None
            d = {}

        if self._bg: d['background-color'] = '#' + self._bg
        if self._fg: d['color'] = '#' + self._fg

        newstyle = ';'.join([k + ':' + v for k, v in d.iteritems()])

        if oldstyle:
            text_edit.setStyleSheet(style.replace(oldstyle, newstyle))
        else:
            text_edit.setStyleSheet('QTextEdit {%s}' % newstyle)

    def process(self, model):
        self._textEditColors(model, self.w.text_output)
        self.appendHtml(model.main_html)

    def selectedText(self):
        return self.w.text_output.textCursor().selection().toPlainText()

    def clearSelection(self):
        cursor = self.w.text_output.textCursor()
        cursor.clearSelection()
        self.w.text_output.setTextCursor(cursor)


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
        v.w.rightwidget.box_status.setVisible(True)
        self._last_values = {'Hp': None, 'Mn': None, 'Mv': None}

    def process(self, model):
        self.v.process(model)

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


class MapViewer(TextViewer):
    """
    Build the visualization of a map from model.

    This class is a subclass of `TextViewer` that take an instance of it
    as argument on __init__ (see `decorator pattern`_)

.. _decorator pattern: http://en.wikipedia.org/wiki/Decorator_pattern
    """

    def __init__(self, v):
        super(MapViewer, self).__init__(v.w)
        self.v = v

    def _centerMap(self, model, width, height):
        html_list = model.map_html.split('<br>')
        text_list = model.map_text.split('\n')

        text, html = [], []
        for i, p in enumerate(text_list):
            if p.strip():
                html.append(html_list[i])
                text.append(p)

        delta_y = (height - len(text)) / 2
        delta_x = (width - len(max(text_list, key=len))) / 2
        model.map_text = '\n' * delta_y + \
            '\n'.join([' ' * delta_x + r for r in text])
        model.map_html = '<br>' * delta_y + \
            '<br>'.join(['&nbsp;' * delta_x + r for r in html])

    def process(self, model):
        self.v.process(model)
        w_map = self.w.rightwidget.text_map
        self._textEditColors(model, w_map)

        if model.map_text:
            w = w_map.property('char_width').toInt()[0]
            h = w_map.property('char_height').toInt()[0]
            self._centerMap(model, w, h)
            w_map.document().setHtml(model.map_html)
        elif model.map_text is None:
            w_map.document().clear()
