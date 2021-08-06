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

import exception

class CircularList(object):

    def __init__(self, max_size):
        self._max_size = max_size #: the maximum size of _data
        self._last_idx = 0 #: the index of last element append to _data
        self._counter = 0 #: the total number of element append to data
        self._data = [] #: the real data container

    def append(self, el):
        """
        Append an element to `CircularList` instance.

        :Parameters:
          el
            the element to append

        """

        if len(self._data) < self._max_size:
            self._data.append(el)
        else:
            self._data[self._last_idx] = el
        self._last_idx = (self._last_idx + 1) % self._max_size
        self._counter += 1

    def get(self, last_row=None):
        """
        Return a list of data since last_row.

        :Parameters:
          last_row : int
            the index of last row get (start by 0, None if nothing was read)

        :raise BufferUnderSize:
          rappresent an exception that happen on losing data because max_size
          is not sufficient

        :return: a list of data
        """

        if last_row is None:
            if len(self._data) < self._max_size:
                return self._data
            else:
                raise exception.BufferUnderSize()
        else:
            if self._counter - last_row - 1 > self._max_size:
                raise exception.BufferUnderSize()

            last_row = (last_row + 1) % self._max_size
            if last_row < self._last_idx:
                return self._data[last_row : self._last_idx]
            else:
                return self._data[last_row : ] + self._data[ : self._last_idx]

class Model(object):
    """
    Rappresent a model of data that can be viewed by a viewer.
    """

    def __init__(self):
        self.main_text = CircularList(100)
        self.main_html = CircularList(100)
        self.main_bgcolor = None
        self.main_fgcolor = None

