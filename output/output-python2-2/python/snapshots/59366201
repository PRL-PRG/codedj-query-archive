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

import thread
import random
import os.path
import threading

import conf
from core import Core
from gui import Gui
from os.path import dirname


class Thread(threading.Thread):
    """
    Class to manage thread interaction between `Gui` and `Core`.

    The gui part run in main thread while core part run
    in secondary thread.
    """

    def __init__(self):
        """
        Create the `Thread` instance and run the `Gui` part.
        """

        threading.Thread.__init__(self)
        port = random.randint(2000, 10000)
        gui = Gui(port)
        thread.start_new_thread(self.run, (port,))
        gui.mainLoop()

    def run(self, port):
        """
        Run the `Core` part.
        """

        core = Core(port)
        core.mainLoop()


def main(argv, cfg_file):
    """
    The function is the client entry point.
    """

    os.chdir(os.path.join(os.getcwd(), dirname(argv[0]), dirname(cfg_file)))
    conf.loadConfiguration(os.path.basename(cfg_file))

    # Set current path on module path for external resources like images
    os.chdir(conf.config['devclient']['path'])

    Thread()