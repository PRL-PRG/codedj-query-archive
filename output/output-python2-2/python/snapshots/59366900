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
import storage
import logging

from sip import delete
from PyQt4.QtCore import SIGNAL, QObject, QEvent, QRect
from PyQt4.QtGui import QWidget, QTextCursor, QPainter, QPixmap, QColor


logger = logging.getLogger('viewer')

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

    if hasattr(server, 'icons_set'):
        __import__(server.icons_set + '_rc', globals(), locals())

    viewer = TextViewer(widget)
    if _setRightPanel(widget, server.right_widget):
        if hasattr(server, 'prompt_reg') or custom_prompt:
            viewer = StatusViewer(viewer)

        if hasattr(server, 'ext_info') and server.ext_info:
            viewer = ExtInfoViewer(viewer, server)
        elif hasattr(server, 'char2icon'):
            viewer = GraphMapViewer(viewer, server)
        elif hasattr(server, 'wild_chars'):
            viewer = TextMapViewer(viewer)

    if hasattr(server, 'gui_width'):
        widget.resize(server.gui_width, widget.height())

    viewer._resetWidgets()
    return viewer


class TextViewer(QObject):
    """
    Build the html visualization from model.
    """

    MAX_ROWS = 500
    """The max number of rows displayed in textEdit field"""

    _ROW_BLOCK = 10
    """The max number of rows per block"""

    def __init__(self, widget):
        QObject.__init__(self)
        self.w = widget
        doc = self.w.text_output.document()
        doc.setMaximumBlockCount(self.MAX_ROWS / self._ROW_BLOCK)
        self.w.text_output_noscroll.document().setMaximumBlockCount(40)
        self._cur_rows = 0

    def _resetWidgets(self):
        self._textEditColors(self.w.text_output_noscroll)
        self._textEditColors(self.w.text_output)
        self.w.text_output_noscroll.clear()
        self.w.text_output.clear()

    def copySelectedText(self):
        no_scroll = self.w.text_output_noscroll
        if no_scroll.textCursor().hasSelection() and no_scroll.isVisible():
           no_scroll.copy()
           cursor = no_scroll.textCursor()
           cursor.clearSelection()
           no_scroll.setTextCursor(cursor)
        else:
            self.w.text_output.copy()
            cursor = self.w.text_output.textCursor()
            cursor.clearSelection()
            self.w.text_output.setTextCursor(cursor)

    def appendHtml(self, html):
        new_html = html.split('<br>')
        cursor = self.w.text_output.textCursor()
        cursor.beginEditBlock()
        cursor.movePosition(QTextCursor.End)

        new_block = True
        while(new_html):
            if not self._cur_rows and not new_block:
                cursor.insertBlock()

            num_rows = len(new_html[:self._ROW_BLOCK - self._cur_rows])
            cursor.insertHtml('<br>'.join(new_html[:num_rows]))
            new_html = new_html[num_rows:]
            self._cur_rows = (num_rows + self._cur_rows) % self._ROW_BLOCK
            new_block = False

        cursor.endEditBlock()
        if not self.w.text_output_noscroll.isVisible():
            self.w.text_output.setTextCursor(cursor)

        new_html = html.split('<br>')
        cursor = self.w.text_output_noscroll.textCursor()
        cursor.beginEditBlock()
        cursor.movePosition(QTextCursor.End)
        for i, row in enumerate(new_html):
            if i:
                cursor.insertBlock()
            cursor.insertHtml(row)
        cursor.endEditBlock()
        self.w.text_output_noscroll.setTextCursor(cursor)

    def _textEditColors(self, text_edit):

        style = unicode(text_edit.styleSheet())
        m = re.search('QTextEdit\s*{(.*)}', style)
        if m:
            oldstyle = m.group(1)
            tmp = [el.split(':') for el in oldstyle.split(';') if el]
            d = dict([(k.strip(), v.strip()) for k, v in tmp])
        else:
            oldstyle = None
            d = {}

        d['color'] = storage.option('fg_color')
        d['background-color'] = storage.option('bg_color')

        newstyle = ';'.join([k + ':' + v for k, v in d.iteritems()])
        if oldstyle:
            text_edit.setStyleSheet(style.replace(oldstyle, newstyle))
        else:
            text_edit.setStyleSheet('QTextEdit {%s}' % newstyle)

    def process(self, model):
        self.appendHtml(model.main_html)


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

    def _resetWidgets(self):
        self.v._resetWidgets()


class TextMapViewer(TextViewer):
    """
    Build the visualization of a textual map from model.

    This class is a subclass of `TextViewer` that take an instance of it
    as argument on __init__ (see `decorator pattern`_)

.. _decorator pattern: http://en.wikipedia.org/wiki/Decorator_pattern
    """

    def __init__(self, v):
        super(TextMapViewer, self).__init__(v.w)
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

        if model.map_text:
            w = w_map.property('char_width').toInt()[0]
            h = w_map.property('char_height').toInt()[0]
            self._centerMap(model, w, h)
            w_map.document().setHtml(model.map_html)
        elif model.map_text is None:
            w_map.document().clear()

    def _resetWidgets(self):
        self.v._resetWidgets()
        self._textEditColors(self.w.rightwidget.text_map)


class MapViewer(TextViewer):
    """
    The abstract class to draw a graphical map regardless of the model structure.
    """

    def __init__(self, viewer):
        super(MapViewer, self).__init__(viewer.w)
        self.v = viewer
        self.w.rightwidget.graph_map.installEventFilter(self)
        self._icon_map = []

    def _getRect(self, x, y):
        offset_x, offset_y = 5, 5
        dim_x, dim_y = 16, 16
        return QRect(x * dim_x + offset_x, y * dim_y + offset_y, dim_x, dim_y)

    def eventFilter(self, target, event):
        graph_map = self.w.rightwidget.graph_map
        if target == graph_map and event.type() == QEvent.Paint and \
           self._icon_map:
            painter = QPainter(graph_map)
            for y, row in enumerate(self._icon_map):
                for x, icon in enumerate(row):
                    if icon:
                        painter.drawPixmap(self._getRect(x, y),
                                           QPixmap(":/icons_map/wild%d" % icon))
                    else:
                        r = self._getRect(x, y)
                        painter.drawRect(r)
                        painter.fillRect(r, QColor(self._empty_icon_color))
            self._onPaintEvent(painter)
            return True
        return False

    def process(self, model):
        self.v.process(model)
        if model.map_text:
            self._parseModel(model)
        elif model.map_text is None:
            self._icon_map = []

    def _resetWidgets(self):
        self.v._resetWidgets()

    def _onPaintEvent(self, painter):
        pass


class GraphMapViewer(MapViewer):
    def __init__(self, viewer, server):
        super(GraphMapViewer, self).__init__(viewer)
        self._empty_icon_color = server.empty_icon_color
        self._char2icon = server.char2icon
        self._icons_set = server.icons_set

    def _parseModel(self, model):
        html = model.map_html
        _html_entities = ('&nbsp;', '&lt;', '&gt;', '&amp;', '&quot;', '<br>')

        self._icon_map = []
        rows = [r for r in html.split('<br>')
                if re.compile('(<span.*?>|</span>)').sub('', r)]
        color = ''
        for r in rows:
            icon_rows = []
            while r:
                if r.startswith('</span>'):
                    r = r[7:]
                elif r.startswith('<span'):
                    pos = r.find('>') + 1
                    color = re.compile('color:(\#\w{6})').search(r[:pos]).group(1)
                    r = r[pos:]
                else:
                    for h in _html_entities:
                        if r[:len(h)] == h:
                            icon_rows.append(self._char2icon.get((color, h)))
                            r = r[len(h):]
                            break
                    else:
                        icon_rows.append(self._char2icon.get((color, r[0])))
                        r = r[1:]

            self._icon_map.append(icon_rows)


class ExtInfoViewer(MapViewer):
    def __init__(self, viewer, server):
        super(ExtInfoViewer, self).__init__(viewer)
        self._empty_icon_color = server.empty_icon_color
        self._icons_set = server.icons_set
        self._chars = []

    def _parseModel(self, model):
        self._icon_map = []
        data = model.map_text.split('~')
        rows = data[:data.index('')]
        for row in rows:
            icon_rows = []
            for c in row:
                if QPixmap(":/icons_map/wild%d" % ord(c)).isNull():
                    icon_rows.append(None)
                else:
                    icon_rows.append(ord(c))
            self._icon_map.append(icon_rows)

        if model.map_text:
            data = model.map_text.split('~')
            self._chars = [c for c in data[data.index('') + 1:] if c]

    def _onPaintEvent(self, painter):
        for c in self._chars:
            x, y, icon = int(c[1]), int(c[2]), int(c[0]) #int(c[0]), int(c[1]), int(c[2]) ## Waiting for ADdE fix..
            painter.drawPixmap(self._getRect(x, y),
                               QPixmap(":/icons_map/wild%d" % icon))
