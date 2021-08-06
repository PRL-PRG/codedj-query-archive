#!/usr/bin/python

import errno
import fcntl
import logging
import os
import sys

import gobject
import gtk
import gtop
import wnck


def get_proc_name(pid):
    if pid == 0:
        return "unknown"
    else:
	return os.path.basename(gtop.proc_args(pid)[0])


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


log_filepath = os.path.expanduser("~/.wnck.log")
log_file = open(log_filepath, "a")
try:
    fcntl.lockf(log_file, fcntl.LOCK_EX|fcntl.LOCK_NB)
except IOError, e:
    if e.errno in (errno.EACCES, errno.EAGAIN):
        sys.exit(1)
    else:
        raise
logging.basicConfig(format="%(asctime)s %(message)s",
                    level=logging.INFO,
                    stream=log_file)
window_logger = WindowLogger()
gobject.idle_add(window_logger.register)
try:
    gtk.main()
except KeyboardInterrupt:
    pass
logging.shutdown()
log_file.close()

