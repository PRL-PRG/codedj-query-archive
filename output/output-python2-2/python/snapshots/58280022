#!/usr/bin/python

import os
import signal

import gobject
import gtk


CONFIG = """\
10143:localhost:143
10119:news.eclipse.co.uk:119
#12000:localhost:2000
"""


class TunnelStarter(object):

    def start(self, label):
        self.label = label
        self._check_agent_is_loaded()
        self.pid = None
        
    def stop(self):
        if self.pid:
            os.kill(self.pid, signal.SIGTERM)
            print "pid", self.pid, "killed"
            self.pid.close()
            self.pid = None
        
    def _check_agent_is_loaded(self):
        self.label.set_text("checking agent")
        pid, stdin, stdout, stderr = gobject.spawn_async(
                ["ssh-add", "-L"],
                flags=gobject.SPAWN_DO_NOT_REAP_CHILD|
                      gobject.SPAWN_STDOUT_TO_DEV_NULL|
                      gobject.SPAWN_SEARCH_PATH)
        pid.close()
        gobject.child_watch_add(pid, self._check_agent_is_loaded_cb)

    def _check_agent_is_loaded_cb(self, pid, condition):
        if condition == 0:
            self.label.set_text("agent is loaded")
            self._start_tunnels()
        else:
            print "not loaded"
            gobject.timeout_add(2000, self._check_agent_is_loaded)

    def _start_tunnels(self):
#        forwarded = []
#        for line in CONFIG.split("\n"):
#            if rx.match(line):
#                forwarded.append("-L")
#                forwarded.append(line)
#        
        self.pid, stdin, stdout, stderr = gobject.spawn_async(
             ["ssh", "-N",
              "-L", "10143:localhost:143",
              "-L", "10119:news.eclipse.co.uk:119",
              "tiber.cehill.co.uk"],
              flags=gobject.SPAWN_SEARCH_PATH)
        print "ssh", self.pid, "started"
        self.label.set_text("tunnel started")

# http://svn.gnome.org/viewcvs/pygobject/trunk/tests/test_subprocess.py?view=markup


def main():
    window = gtk.Window()
    window.set_border_width(10)
    vbox = gtk.VBox()
    window.add(vbox)
    label = gtk.Label()
    vbox.pack_start(label, padding=6)
    button = gtk.Button("_Finish")
    vbox.pack_start(button, padding=6)
    ts = TunnelStarter()
    ts.start(label)
    def stop(w):
        ts.stop()
        gtk.main_quit()
    window.connect("destroy", stop)
    button.connect("clicked", stop)
    window.show_all()
    gtk.main()


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        pass
