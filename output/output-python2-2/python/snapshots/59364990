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

from glob import glob
from os.path import join, basename

from conf import config

server_list = {}
"""
The dictionary that contains the list of servers with a specific configuration.
"""

def _loadServerList():
    """Load the server list from the files in server's directory."""

    server_list.clear()
    files = [basename(f) for f in glob(join(config['servers']['path'], '*.py'))]
    for f in files:
        execfile(join(config['servers']['path'], f), server_list)

    del server_list['__builtins__']

def getServer(host, port):
    """
    Return the Server class that match best with host and port.

    :Parameters:
      host : str
        the host of Server to find
      port : int
        the port of Server to find
    """

    if not server_list:
        _loadServerList()
    for s in server_list.itervalues():
        for n in [''] + map(str, range(2,5)):
            if hasattr(s, 'host%s' % n) and getattr(s, 'host%s' % n) == host and \
               hasattr(s, 'port%s' % n) and getattr(s, 'port%s' % n) == port:
                return s

    return server_list['Server']
