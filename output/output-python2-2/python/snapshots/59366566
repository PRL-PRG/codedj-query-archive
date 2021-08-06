#!/usr/bin/python
#-*- coding: utf-8 -*-

import Queue
import threading

import event_type

class Thread(threading.Thread):
    """
    Class to manage thread interaction between `Gui` and `Application`.

    The gui part run in main thread while application part run
    in secondary thread.
    """

    def __init__(self, classes, config):
        """
        Create the `Thread` instance and run the gui part.

        :Parameters:
          classes : dict
            a dictionary of the form {<className>: <classRef> } that
            contains all the specific classes used in client.
          config : dict
            a dictionary of the form [section][option] which contains the
            configuration that cannot setup at runtime.
        """

        threading.Thread.__init__(self)
        self.classes = classes  #: the dictionary of classes
        self.config = config  #: the dictionary of configuration
        self.q_app_gui = Queue.Queue()  #: events from app to gui
        self.q_gui_app = Queue.Queue()  #: events from gui to app

        gui = self.classes['Gui'](self.config, self.q_app_gui, self.q_gui_app)

        self.start()
        gui.mainLoop()

    def run(self):
        """
        Run the application part.
        """

        app = self.classes['Application'](self.classes, self.q_app_gui,
                                          self.q_gui_app)
        app.mainLoop()