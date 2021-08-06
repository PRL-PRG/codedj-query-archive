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

import glob
import os.path
from conf import config

server_list = None

def _getServerList():
    files = glob.glob(os.path.join(config['servers']['path'], '*.py'))
    files = [os.path.basename(f) for f in files]
    d = {}
    for f in files:
        execfile(os.path.join(config['servers']['path'], f), d)

    del d ['__builtins__']
    return d

def getServer(host, port):
    global server_list

    if not server_list:
        server_list = _getServerList()
    for s in server_list.itervalues():
        if hasattr(s, 'host') and s.host == host and \
           hasattr(s, 'port') and s.port == port:
            return s

    return server_list['Server']
