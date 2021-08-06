#!/usr/bin/python
#-*- coding: utf-8 -*-

import sys
import time
import Queue

import event_type

class Thread(object):
    def __init__(self, classes):
        self.classes = classes
        self.q_app_gui = Queue.Queue() # eventi dall'app alla gui
        self.q_gui_app = Queue.Queue() # eventi dalla gui all'app

        self.gui = self.classes['Gui'](self.q_app_gui, self.q_gui_app)

        self.start()
        self.gui.mainLoop()

    def run(self):
        sock = self.classes['Socket']()
        while 1:
            time.sleep(1)
            self.q_app_gui.put((event_type.MSG, sock.read()))
            try:
                cmd, msg = self.q_gui_app.get(0)
                if cmd == event_type.END_APP:
                    return
            except Queue.Empty:
                pass