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

__version__ = "$Revision:$"[11:-2]
__docformat__ = 'restructuredtext'

class Viewer(object):
    """
    Build a graphical rappresentation of model.
    """

    def __init__(self):
        self.last_row = None

    def process(self, model):
        new_text = model.main_text.get(self.last_row)
        bgcolor = None
        fgcolor = None

        if self.last_row is None:
            self.last_row = len(new_text) - 1
            bgcolor = model.main_bgcolor
            fgcolor = model.main_fgcolor
        else:
            self.last_row += len(new_text)

        return (''.join(new_text), bgcolor, fgcolor)

