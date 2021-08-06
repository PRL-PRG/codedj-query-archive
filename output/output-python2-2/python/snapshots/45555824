#!/usr/bin/env python
# Sample applet for gnome

import pygtk
pygtk.require('2.0')

import sys

import gtk
import gnome.ui
import gnomeapplet

class GnomeAppletSkeleton(gnomeapplet.Applet):
    """Simple applet skeleton"""

    def __init__(self, applet, iid):
        """Create applet for proxy"""
        self.applet = applet
        self.__init_core_widgets()
        self.init_additional_widgets()
        self.init_ppmenu()
        self.__connect_events()
        self.applet.connect("destroy", self._cleanup)
        self.after_init()
        self.applet.show_all()

    def __init_core_widgets(self):
        """Create internal widgets"""
        # making internal widgets
        self.tooltips = gtk.Tooltips()
        self.hbox = gtk.HBox()
        self.ev_box = gtk.EventBox()
        # making widgets' ierarchy
        self.applet.add(self.hbox)
        self.hbox.add(self.ev_box)

    def init_additional_widgets(self):
        """Create additional widgets"""
        self.label = gtk.Label("Dummy")
        self.ev_box.add(self.label)

    def init_ppmenu(self):
        """Create popup menu for properties"""
        self.ppmenu_xml = """
        <popup name="button3">
        <menuitem name="About Item" verb="About" stockid="gtk-about"/>
        </popup>        
        """
        self.ppmenu_verbs = [("About", self.on_ppm_about),
            ]

    def __connect_events(self):
        """Connect applet's events to callbacks"""
        self.ev_box.connect("button-press-event", self.on_button)
        self.ev_box.connect("enter-notify-event", self.on_enter)
        self.button_actions = {
            1: lambda: None,
            2: lambda: None,
            3: self._show_ppmenu,
            }

    def after_init(self):
        """After-init hook"""
        pass

    def _cleanup(self, event):
        """Cleanup callback (on destroy)"""
        del self.applet

    def on_button(self, widget, event):
        """Action on pressing button in applet"""
        if event.type == gtk.gdk.BUTTON_PRESS:
            self.button_actions[event.button]()

    def _show_ppmenu(self):
        """Show popup menu"""
        print self.applet.setup_menu.__doc__
        self.applet.setup_menu(self.ppmenu_xml,
                               self.ppmenu_verbs,
                               None)

    def on_enter(self, widget, event):
        """Action on entering mouse to widget"""
        info = "Hey, it just skeleton  \nAnd on_enter event time is %d" % \
               event.time
        self.tooltips.set_tip(self.ev_box, info)

    def on_ppm_about(self, event, data=None):
        """Action on chose 'about' in pop-up menu"""
        gnome.ui.About("GnomeApplet skeleton", "0.1", "GNU General Public License v.2",
                       "Simple skeleton for Python powered GNOME applet",
                       ["Pythy <the.pythy@gmail.com>",],
                       ).show()

def applet_factory(applet, iid):
    GnomeAppletSkeleton(applet, iid)
    return True

def run_in_panel():
    gnomeapplet.bonobo_factory("OAFIID:GNOME_AppletSkeleton_Factory",
                               GnomeAppletSkeleton.__gtype__,
                               "Applet skeleton",
                               "0",
                               applet_factory)


def run_in_window():
    main_window = gtk.Window(gtk.WINDOW_TOPLEVEL)
    main_window.set_title("GNOME Applet Skeleton")
    main_window.connect("destroy", gtk.main_quit)
    app = gnomeapplet.Applet()
    applet_factory(app, None)
    app.reparent(main_window)
    main_window.show_all()
    gtk.main()
    sys.exit()

def main(args):
    if len(args) == 2 and args[1] == "run-in-window":
        run_in_window()
    else:
        run_in_panel()

if __name__ == '__main__':
    main(sys.argv)
