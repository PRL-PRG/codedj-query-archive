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

import os
import ctypes
import random
import signal
import os.path
import subprocess
from os.path import dirname, join, abspath, normpath
from sys import path, argv, platform

import exception
from constants import PROJECT_NAME
from conf import loadConfiguration, config

_DEF_CONFIG_FILE = "../../etc/devclient.cfg"
cfg_file = normpath(join(dirname(abspath(__file__)), _DEF_CONFIG_FILE))

def terminateProcess(pid):
    """
    Kill a process.

    :Parameters:
      pid : int
        the id of the process to kill
    """

    if platform == 'win32':
        handle = ctypes.windll.kernel32.OpenProcess(1, False, pid)
        ctypes.windll.kernel32.TerminateProcess(handle, -1)
        ctypes.windll.kernel32.CloseHandle(handle)
    else:
        os.kill(pid, signal.SIGKILL)


def startProcess(cmd):
    """
    Launch a subprocess, hiding the console on win32.

    :Parameters:
      cmd : tuple
        the name and parameters of the process to launch.
    """

    if platform == 'win32':  # Hide console on win32 platform
        startupinfo = subprocess.STARTUPINFO()
        startupinfo.dwFlags |= subprocess.STARTF_USESHOWWINDOW
    else:
        startupinfo = None

    return subprocess.Popen(cmd, startupinfo=startupinfo)

def main(argv=argv, cfg_file=cfg_file, update=1):
    """
    The function is the client entry point.
    """

    os.chdir(join(os.getcwd(), dirname(argv[0]), dirname(cfg_file)))
    cfg_file = join(os.getcwd(), os.path.basename(cfg_file))
    loadConfiguration(cfg_file)
    path.append(config['servers']['path'])
    path.append(config['resources']['path'])

    # this import must stay here, after the appending of resources path to path
    from gui import Gui

    port = random.randint(2000, 10000)

    p = startProcess(['python',
                      join(config['devclient']['path'], 'core.py'),
                      '--config=%s' % cfg_file,
                      '--port=%d' % port])

    # FIX! To prevent connectionRefused from SocketToGui
    import time
    time.sleep(.5)

    try:
        gui = Gui(port)
        if not update:
            gui.displayWarning(PROJECT_NAME, gui._text['UpdateFail'])
        gui.mainLoop()
    except exception.IPCError:
        terminateProcess(p.pid)

