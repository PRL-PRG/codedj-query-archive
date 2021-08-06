#!/usr/bin/python
#-*- coding: utf-8 -*-

import Queue
import threading

import event_type

class Thread(threading.Thread):
    """
    Class to manage thread interaction between gui and application.
    """

    def __init__(self, classes):
        threading.Thread.__init__(self)
        self.classes = classes
        self.q_app_gui = Queue.Queue() # events from app to gui
        self.q_gui_app = Queue.Queue() # events from gui to app

        self.gui = self.classes['Gui'](self.q_app_gui, self.q_gui_app)

        self.start()
        self.gui.mainLoop()

    def run(self):
        app = self.classes['Application'](self.classes, self.q_app_gui,
                                          self.q_gui_app)
        app.mainLoop()