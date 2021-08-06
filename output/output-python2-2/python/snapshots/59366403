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

from storage import Storage

class Alias(object):
    _PLACEHOLDER = '%s'

    def __init__(self, conn_name):
        self._list = Storage().aliases(conn_name)

    def check(self, msg):
        """
        Check if a message contains an alias and replace its label with body.

        Loop on aliases to find if the message starts with a label, then replace
        the label with body and placeholders with the positional arguments.

        :Parameters:
          msg : str
            the original message

        :return: the modified message
        """

        for label, body in self._list:
            if msg.startswith(label):
                txt = msg.split(label, 1)[1]
                ph = body.count(self._PLACEHOLDER)
                if ph:
                    parts = txt.split(None, ph)
                    tokens = parts[:ph]
                    txt = ''
                    if parts[ph:]:
                        txt = parts[ph:][0]

                    for el in tokens:
                        body = body.replace(self._PLACEHOLDER, el, 1)

                    # delete the placeholders that exceed
                    body = body.replace(self._PLACEHOLDER, '')

                msg = body + txt
                break

        return msg

