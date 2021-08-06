#!/usr/bin/env python
# Sample applet for GNOME

__license__ = "GNU General Public License v.2"
__version__ = "0.1"
__author__ = "Pythy <the.pythy@gmail.com>"

import pygtk
pygtk.require('2.0')

import sys
import gconf
import gnomeapplet

from applet_skeleton import GnomeAppletSkeleton

class ProxyGconfClient(object):
    """Get/set proxy states"""
    proxy_dir = "/system/proxy"
    proxy_key = "/system/proxy/mode"
    on_state = 'manual'
    off_state = 'none'

    def __init__(self, callback=None):
        """
        GConf client for getting/setting proxy states

        @param callback: callback function. Executing
            when proxy state changed. It calls with params:
              * client - GConf client
              * cnxn_id - connection ID
              * entry - changed entry
              * params - additional params
        @type callback: callable
        """
        if callback is None:
            callback = lambda client, cnxn_id, entry, params: None
        # make connection to GConfD
        self.client = gconf.client_get_default()
        # add proxy_dir for inspection, without preload
        self.client.add_dir(self.proxy_dir,
                            gconf.CLIENT_PRELOAD_NONE)
        # add callback for notifying about changes
        self.client.notify_add(self.proxy_key,
                               callback)

    def get_state(self):
        """Returns state of proxy"""
        return self.client.get_string(self.proxy_key)

    def set_state(self, value):
        """
        Set state of proxy

        @param value: state of proxy, may be
          * 'none' - direct connection, proxy off
          * 'manual' - manual settings, proxy on
          * 'auto' - auto settings, proxy on 
          if value neither 'manual', no 'auto', it means
          direct connection, i.e. proxy off.
        @raise RuntimeError: cannot set value to GConf's key
        """
        if not self.client.set_string(self.proxy_key,
                                      value):
            raise RuntimeError("Unable to change key %s" % \
                               self.proxy_key)

    def on(self):
        """Turn proxy on (i.e. set proxy mode 'manual')"""
        self.set_state(self.on_state)

    def is_on(self):
        """Is proxy on? (i.e. proxy in 'manual' mode)"""
        return self.get_state() == self.on_state

    def off(self):
        """Turn proxy off (i.e. set direct connection)"""
        self.set_state(self.off_state)


class ProxyGnomeApplet(GnomeAppletSkeleton):

    def after_init(self):
        self.proxy = ProxyGconfClient(callback=self._cb_proxy_change)
        self.proxy_state = self.proxy.get_state()
        self.button_actions[1] = self.switch_proxy
        self.label.set_text(self.proxy_state)

    def _cb_proxy_change(self, client, cnxn_id, entry, params):
        """Callback for changing proxy"""
        self.proxy_state = self.proxy.get_state()
        ## alternative variant for getting state
        # self.proxy_state = entry.get_value().get_string()
        self.label.set_text(self.proxy_state)

    def on_enter(self, widget, event):
        info = "Proxy mode: %s" % self.proxy_state
        self.tooltips.set_tip(self.ev_box, info)

    def switch_proxy(self):
        if self.proxy.is_on():
            self.proxy.off()
        else:
            self.proxy.on()


def proxy_applet_factory(applet, iid):
    ProxyGnomeApplet(applet, iid)
    return True

def run_in_window():
    import gtk
    import gnomeapplet
    main_window = gtk.Window(gtk.WINDOW_TOPLEVEL)
    main_window.set_title("Gnome Applet Example. Python powered.")
    main_window.connect("destroy", gtk.main_quit) 
    app = gnomeapplet.Applet()
    proxy_applet_factory(app, None)
    app.reparent(main_window)
    main_window.show_all()
    gtk.main()
    sys.exit()

def run_in_panel():
    gnomeapplet.bonobo_factory("OAFIID:GNOME_AppletExample_Factory",
                               ProxyGnomeApplet.__gtype__,
                               "Applet example",
                               "0",
                               proxy_applet_factory)

def main(args):
    if len(args) == 2 and args[1] == "run-in-window":
        run_in_window()
    else:
        run_in_panel()

if __name__ == '__main__':
    main(sys.argv)
