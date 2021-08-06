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

import sys
import random
import os.path
import subprocess
from os.path import dirname, join

import conf
from core import Core
from gui import Gui


def main(argv, cfg_file):
    """
    The function is the client entry point.
    """

    os.chdir(join(os.getcwd(), dirname(argv[0]), dirname(cfg_file)))
    cfg_file = join(os.getcwd(), os.path.basename(cfg_file))
    conf.loadConfiguration(cfg_file)

    port = random.randint(2000, 10000)

    if sys.platform == 'win32':  # Hide console on win32 platform
        startupinfo = subprocess.STARTUPINFO()
        startupinfo.dwFlags |= subprocess.STARTF_USESHOWWINDOW
    else:
        startupinfo = None

    subprocess.Popen(['python',
                      join(conf.config['devclient']['path'], 'core.py'),
                      '--config=%s' % cfg_file,
                      '--port=%d' % port], startupinfo=startupinfo)

    # FIX! To prevent connectionRefused from SocketToGui
    import time
    time.sleep(.5)

    # Set current path on module path for external resources like images
    os.chdir(conf.config['devclient']['path'])
    gui = Gui(port)
    gui.mainLoop()
