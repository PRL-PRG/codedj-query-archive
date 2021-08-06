# -*- coding: UTF-8 -*-
# Copyright 2007-2008 One Laptop Per Child
# Copyright 2007 Gerard J. Cerchio <www.circlesoft.com>
# Copyright 2008 Andrés Ambrois <andresambrois@gmail.com>
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program; if not, write to the Free Software
# Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

import gtk
from os.path import join, dirname

from gettext import gettext as _
from sugar.graphics.toolbutton import ToolButton
from sugar.graphics.toolcombobox import ToolComboBox
from sugar.graphics.objectchooser import ObjectChooser
import logging
from gobject import SIGNAL_RUN_FIRST, TYPE_PYOBJECT, TYPE_NONE, TYPE_INT

from gtp import search_for_gnugo

logger = logging.getLogger('PlayGo')

class GameToolbar(gtk.Toolbar):
    __gtype_name__ = 'GameToolbar'

    __gsignals__ = {
        'game-restart': (SIGNAL_RUN_FIRST, TYPE_NONE, []), 
        'ai-activated': (SIGNAL_RUN_FIRST, TYPE_NONE, []), 
        'ai-deactivated': (SIGNAL_RUN_FIRST, TYPE_NONE, []), 
        'game-board-size': (SIGNAL_RUN_FIRST, TYPE_NONE, [TYPE_INT]), 
    }
    
    def __init__(self, activity):
        gtk.Toolbar.__init__(self)
        self.activity = activity
        
        # Reset Button
        restart_icon = join(dirname(__file__), 'images', 'gtk-refresh.svg')
        restart_image = gtk.Image()
        restart_image.set_from_file(restart_icon)
        self._restart_button = ToolButton()
        self._restart_button.set_icon_widget(restart_image)
        self._restart_button.connect('clicked', self._game_restart_cb)
        self._restart_button.set_tooltip(_('Restart Game'))
        self.insert(self._restart_button, -1)
        self._restart_button.show()
        
        # Separator
        separator = gtk.SeparatorToolItem()
        separator.set_draw(True)
        self.insert(separator, -1)
        
        self._add_widget(gtk.Label(_('Board size') + ': '))
        # Change size combobox
        self._size_combo = ToolComboBox()
        self._sizes = ['19 X 19', '13 X 13', '9 X 9']
        for i, f in enumerate(self._sizes):
            self._size_combo.combo.append_item(i, f)
        self._size_combo.combo.connect('changed', self._game_size_cb)
        self._add_widget(self._size_combo)
        self._size_combo.combo.set_active(0)
        
        # Separator
        separator = gtk.SeparatorToolItem()
        separator.set_draw(True)
        self.insert(separator, -1)
        
        # Artificial Intelligence Button
        self._ai_button = gtk.ToggleToolButton()
        if search_for_gnugo():
            self._ai_button.connect('toggled', self._ai_toggled_cb)
            self._ai_button.set_label(_('Play against PlayGo!'))
        else:
            self._ai_button.set_label(_('You need to install gnugo to play against PlayGo'))
            self._ai_button.set_sensitive(False)
        self.insert(self._ai_button, -1)
        self._ai_button.show()
        
    def _add_widget(self, widget, expand=False):
        tool_item = gtk.ToolItem()
        tool_item.set_expand(expand)
        tool_item.add(widget)
        widget.show()
        self.insert(tool_item, -1)
        tool_item.show()
        
    def _game_restart_cb(self, widget):
        self._size_combo.set_sensitive(True)
        self.emit('game-restart')
    
    def grey_out_restart(self):
        self._restart_button.set_sensitive(False)
    
    def _game_size_cb(self, widget):
        game_size = int(self._sizes[self._size_combo.combo.get_active()][:2])
        self.emit('game-board-size', game_size)
        
    def grey_out_size_change(self):
        self._size_combo.set_sensitive(False)
        
    def update_toolbar(self, widget, data, grid):
        size = data.get('size')
        self._size_combo.combo.handler_block(self.size_handle_id)
        size_index = self._sizes.index(size+' X '+size)
        self._size_combo.combo.set_active(int(size_index))
        self._size_combo.combo.handler_unblock(self.size_handle_id)

    def _ai_toggled_cb(self, widget):
        if widget.get_active():
            self.emit('ai-activated')
        else:
            self.emit('ai-deactivated')
        
    def grey_out_ai(self):
        self._ai_button.set_sensitive(False)
        
    def set_ai_button_state(self, value):
        self._ai_button.set_active(value)
