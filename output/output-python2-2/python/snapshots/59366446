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

import Queue
import threading

from modules.application import Application
from modules.gui import Gui

import event_type

class Thread(threading.Thread):
    """
    Class to manage thread interaction between `Gui` and `Application`.

    The gui part run in main thread while application part run
    in secondary thread.
    """

    def __init__(self):
        """
        Create the `Thread` instance and run the gui part.

        """

        threading.Thread.__init__(self)
        self.q_app_gui = Queue.Queue()  #: events from app to gui
        self.q_gui_app = Queue.Queue()  #: events from gui to app

        gui = Gui(self.q_app_gui, self.q_gui_app)

        self.start()
        gui.mainLoop()

    def run(self):
        """
        Run the application part.
        """

        app = Application(self.q_app_gui, self.q_gui_app)
        app.mainLoop()