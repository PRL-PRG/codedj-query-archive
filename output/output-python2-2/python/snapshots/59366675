#!/usr/bin/python
#-*- coding: utf-8 -*-

import sys
import time
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
        sock = self.classes['Socket']()

        while 1:
            time.sleep(0.2)
            if sock.connected:
                data = sock.read()
                if data:
                    self.q_app_gui.put((event_type.MSG, data))

            try:
                cmd, msg = self.q_gui_app.get(0)
                if cmd == event_type.MSG and sock.connected:
                    sock.write(msg)
                elif cmd == event_type.END_APP:
                    return
                elif cmd == event_type.CONNECT:
                    sock.connect("dde.homelinux.com", 5000) #FIX
            except Queue.Empty:
                pass