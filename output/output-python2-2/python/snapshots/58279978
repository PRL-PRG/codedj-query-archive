#!/usr/bin/python

import logging
import os

import gobject
import gtk
import wnck


def get_proc_name(pid):
    if pid == 0:
        return "unknown"
    else:
        return os.path.basename(os.readlink("/proc/%d/exe" % pid))


def log_window(window):
    logging.info("%s %s", get_proc_name(window.get_application().get_pid()),
                 window.get_name())
    return True


class WindowLogger(object):

    def __init__(self):
        self.active_window = None
        self.name_changed_handler_id = None

    def window_changed(self, screen, previously_active_window):
        if self.active_window is not None and \
                self.name_changed_handler_id is not None:
            self.active_window.disconnect(self.name_changed_handler_id)
        self.active_window = screen.get_active_window()
        if self.active_window is not None:
            self.name_changed_handler_id = \
                self.active_window.connect("name-changed", log_window)
            log_window(self.active_window)

    def register(self):
        screen = wnck.screen_get_default()
        assert screen is not None
        self.window_changed(screen, None)
        screen.connect("active-window-changed", self.window_changed)    
        return False


logging.basicConfig(filename=os.path.expanduser("~/.wnck.log"),
                    format="%(asctime)s %(message)s",
                    level=logging.INFO)
window_logger = WindowLogger()
gobject.idle_add(window_logger.register)
try:
    gtk.main()
except KeyboardInterrupt:
    pass



