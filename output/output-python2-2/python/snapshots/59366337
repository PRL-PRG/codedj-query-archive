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

from parser import *
from viewer import *

GENERIC = 0
DDE = 1
CLESSIDRA = 2
ANCESTRAL = 3

def getMudType(host, port):
    # FIX
    mud_list = {('dde.homelinux.com', 5000): DDE,
                ('mud.clessidra.it', 4000): CLESSIDRA,
                ('ancestralmud.it', 4000): ANCESTRAL}

    if mud_list.has_key((host, port)):
        return mud_list[(host, port)]

    return GENERIC

class ComponentFactory(object):

    def __init__(self, name):
        self.name = name

    def parser(self):
        if self.name in (DDE, CLESSIDRA):
            return SmaugParser()
        elif self.name == ANCESTRAL:
            return AfkParser()
        return Parser()

    def _rightPanelIdx(self):
        # FIX
        if self.name in (DDE, CLESSIDRA, ANCESTRAL):
            return 1
        return 0

    def viewer(self, widget):
        widget.rightpanel.setCurrentIndex(self._rightPanelIdx())

        if self.name in (DDE, CLESSIDRA, ANCESTRAL):
            return StatusViewer(TextViewer(widget))
        return TextViewer(widget)
