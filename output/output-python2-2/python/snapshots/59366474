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

import os
import os.path
import ConfigParser

config = {}

def loadConfiguration(filename):
    """
    Load configuration

    :Parameters:
      filename : str
         the path of config file

    """

    global config

    def changePath(k, v):
        if k == 'path':
            v = os.path.abspath(os.path.join(os.getcwd(), v))
        return v

    cp = ConfigParser.SafeConfigParser()
    cp.read(filename)
    for s in cp.sections():
        config[s] = dict([(k, changePath(k, v)) for k, v in cp.items(s)])
