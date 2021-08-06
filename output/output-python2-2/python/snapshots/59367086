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
from re import escape, compile

import storage


class Trigger(object):
    _SPECIAL_CHARS = {'\\*': '(.*?)', '\\%d': '(\d+)', '\\%w': '(\w+)'}

    def __init__(self, conn_name):
        self._list = storage.triggers(conn_name)

    def checkActions(self, main_text):

        actions = []
        text = main_text.split('\n')
        for pattern, ignore_case, command in self._list:
            p = escape(pattern)
            for old, new in self._SPECIAL_CHARS.iteritems():
                p = p.replace(old, new)

            reg = compile(p, re.I if ignore_case else 0)

            for row in text:
                m = reg.match(row)
                if m:
                    for i, var in enumerate(m.groups()):
                        command = command.replace('%%%d' % (i + 1), var)
                    actions.append(command)

        return actions
