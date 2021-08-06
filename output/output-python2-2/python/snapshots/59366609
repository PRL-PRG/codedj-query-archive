#!/usr/bin/python
#-*- coding: utf-8 -*-

import copy
import Queue

import event_type

class Application(object):
    """
    Main class for the application part of client.
    """

    def __init__(self, classes, q_app_gui, q_gui_app):
        self.classes = classes
        self.q_app_gui = q_app_gui
        self.q_gui_app = q_gui_app

        self.sock = self.classes['Socket']()

    def mainLoop(self):

        parser = self.classes['Parser']()

        while 1:

            if self.sock.connected:
                data = self.sock.read()
                if data:
                    parser.parse(data)
                    self.q_app_gui.put((event_type.MODEL, \
                                        copy.deepcopy(parser.model)))

            try:
                cmd, msg = self.q_gui_app.get(0)
                if cmd == event_type.MSG and self.sock.connected:
                    self.sock.write(msg)
                elif cmd == event_type.END_APP:
                    self.sock.disconnect()
                    return
                elif cmd == event_type.CONNECT and not self.sock.connected:
                    self.sock.connect("localhost", 6666)
            except Queue.Empty:
                pass
