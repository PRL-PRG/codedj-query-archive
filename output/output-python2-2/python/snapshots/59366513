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
        """
        Create the `Application` instance.

        :Parameters:
          classes : dict
            a dictionary of the form {<className>: <classRef> } that
            contains all the specific classes use in client.
          q_app_gui : Queue
            a Queue used to send message from `Application` to `Gui`
          q_gui_app : Queue
            a Queue used to send message from `Gui` to `Application`
        """

        self.classes = classes
        self.q_app_gui = q_app_gui
        self.q_gui_app = q_gui_app

        self.sock = self.classes['Socket']()

    def mainLoop(self):
        """
        Realize the main loop of application.

        Manage `Socket` input/output and take care of exchange messages with
        the `Gui` part.
        """

        parser = None

        while 1:

            if self.sock.connected:
                data = self.sock.read()
                if data:
                    parser.parse(data)
                    self.q_app_gui.put((event_type.MODEL,
                                        copy.deepcopy(parser.model)))

            try:
                cmd, msg = self.q_gui_app.get(0)
                if cmd == event_type.MSG and self.sock.connected:
                    self.sock.write(msg)
                elif cmd == event_type.END_APP:
                    self.sock.disconnect()
                    return
                elif cmd == event_type.CONNECT and not self.sock.connected:
                    parser = self.classes['Parser']()
                    self.sock.connect(*msg)
            except Queue.Empty:
                pass
