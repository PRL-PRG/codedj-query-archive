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

from generics import *

class DdE(DikuServer):
    right_widget = 'gui_map_graph'
    wild_chars = '\^\.xoX@\*\s&-\?'
    wild_end_text = '\n[Uscite:'
    wild_prec_char = '123456789'
    room_map = '|\-#?@'
    host = 'dde.homelinux.com'
    port = 5000
    host2 = 'dde.homelinux.com'
    port2 = 4000
    empty_icon_color = '#868686'
    char2icon = {('#00aa00', '*') : "7",
                 ('#ffffff', '@') : "1",
                 ('#4444ff', '@') : "2",
                 ('#aa0000', '@') : "3",
                 ('#aa00aa', '@') : "4",
                 ('#aaaaaa', '^') : "11",
                 ('#aaaa00', '^') : "9",
                 ('#ffffff', '^') : "12",
                 ('#ffffff', '.') : "0",
                 ('#00aa00', '.') : "5",
                 ('#aaaa00', '.') : "6",
                 ('#4444ff', '.') : "13",
                 ('#ffff44', '.') : "17",
                 ('#00aaaa', '.') : "14",
                 ('#aaaaaa', 'o') : "21",
                 ('#aaaa00', '-') : "10",
                 ('#00aa00', '&amp;') : "8",
                 ('#ffffff', 'X') : "26",
                 ('#ff4444', 'x') : "27",
                }


class ADdE(DdE):
    host = 'algor.homelinux.com'
    port = 5556
    host2 = None
    port2 = None
