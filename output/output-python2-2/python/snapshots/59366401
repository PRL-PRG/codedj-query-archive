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

class History(object):
    def __init__(self):
        self._list = []
        self._max_size = 30
        self._curr_idx = -1

    def clear(self):
        self._curr_idx = -1
        self._list = []

    def add(self, text):
        if not unicode(text).strip():
            return

        if self._list.count(text):
            self._list.remove(text)

        self._list.append(text)

        # Remove the older element
        if len(self._list) > self._max_size:
            self._list.pop(0)

        self._curr_idx = -1

    def getPrev(self):
        """
        Return the previous element of history list or the last element if the
        last operation is the adding of a new element.
        """

        if not len(self._list):
            return ''

        if self._curr_idx == -1:
            self._curr_idx = len(self._list) - 1
        else:
            self._curr_idx -= 1
            if self._curr_idx < 0:
                self._curr_idx = len(self._list) - 1

        return self._list[self._curr_idx]

    def getNext(self):
        """
        Return the next element of history list or the first element if the
        last operation is the adding of a new element.
        """

        if not len(self._list):
            return ''

        if self._curr_idx == -1:
            self._curr_idx = 0
        else:
            self._curr_idx += 1
            if self._curr_idx > len(self._list) - 1:
                self._curr_idx = 0

        return self._list[self._curr_idx]
