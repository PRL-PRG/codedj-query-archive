#!/usr/bin/env python
# Proxy switcher applet for GNOME

__license__ = "GNU General Public License v.2"
__version__ = "0.1"
__author__ = "Pythy <the.pythy@gmail.com>"

import pygtk
pygtk.require('2.0')

import os
import sys
import gtk
import gconf
import gobject
import gnome.ui
import gnomeapplet

from applet_example import ProxyGnomeApplet, ProxyGconfClient

import gettext

gettext.install('proxyswitcher', localedir=os.path.dirname(os.path.abspath(__file__)), unicode=True)

class ProxySwitcherGnomeApplet(ProxyGnomeApplet):
    """ProxySwitcher GNOME applet"""
    
    def init_additional_widgets(self):
        """Create additional widgets"""
        self._init_pixbufs()
        self.image = gtk.Image()
        self.ev_box.add(self.image)

    def _init_pixbufs(self):
        """Init pixbufs from current theme, or from Tango, if 'proxy' icon not in current theme"""
        self.pixbufs = {}
        self.theme = self._get_theme()
        try:
            self._reload_pixbufs()
        except gobject.GError:
            self.theme = self._get_theme('Tango')
            self._reload_pixbufs()

    def _get_theme(self, name=None):
        """Return a theme by name, or current one if name is None"""
        if name is None:
            name = gconf.client_get_default().get_string('/desktop/gnome/interface/icon_theme')
        theme = gtk.IconTheme()
        theme.set_custom_theme(name)
        return theme

    def _reload_pixbufs(self, size=None):
        """Reload pixbufs from current theme for specified size, or for panel's size if size is None"""
        if size is None:
            size = self.applet.get_size()
        pixbuf = self.theme.load_icon('proxy', size, gtk.ICON_LOOKUP_FORCE_SVG)
        faded_pixbuf = gtk.gdk.Pixbuf(pixbuf.get_colorspace(),
                    pixbuf.get_has_alpha(),
                    pixbuf.get_bits_per_sample(),
                    pixbuf.get_width(),
                    pixbuf.get_height())
        pixbuf.saturate_and_pixelate(faded_pixbuf, 1, True)
        self.pixbufs[True] = pixbuf
        self.pixbufs[False] = faded_pixbuf

    def after_init(self):
        """Init additional attributes of applet"""
        self.proxy = ProxyGconfClient(callback=self._cb_proxy_change)
        self.proxy_state = self.proxy.get_state()
        self.proxy_is_on = self.proxy.is_on()
        self.set_visual_state(self.proxy_state, self.proxy_is_on)
        self.button_actions[1] = self.switch_proxy

    def set_visual_state(self, state, is_on):
        """Set overall visual state for corresponding proxy's state"""
        msg_on_state = _(u"Proxy is on")
        msg_off_state = _(u"Proxy is off")
        mode = _(u"mode: %s") % state
        variant = (is_on and msg_on_state) or msg_off_state
        self.info = u"%s (%s)" % (variant, mode)
        self._set_image(is_on)

    def _set_image(self, kind):
        """Set image for specified state"""
        self.image.set_from_pixbuf(self.pixbufs[kind])

    def _cb_proxy_change(self, client, cnxn_id, entry, params):
        """Callback for changing proxy, change visual state of applet"""
        self.proxy_state = self.proxy.get_state()
        self.proxy_is_on = self.proxy.is_on()
        self.set_visual_state(self.proxy_state, self.proxy_is_on)

    def on_enter(self, widget, event):
        """Callback for 'on-enter' event, show tooltip"""
        self.tooltips.set_tip(self.ev_box, self.info)

    def on_ppm_about(self, event, data=None):
        """Callback for pop-up menu item 'About', show About dialog"""
        pixbuf_logo = self.theme.load_icon('proxy', 80, gtk.ICON_LOOKUP_FORCE_SVG)
        msg_applet_name = _("Proxy switcher")
        msg_applet_description = _("Applet for turning proxy on/off")
        gnome.ui.About(msg_applet_name, __version__, __license__,
                       msg_applet_description,
                       [__author__,],   # programming
                       None,   # documentation
                       __author__,   # translation
                       pixbuf_logo,
                       ).show()

def proxy_applet_factory(applet, iid):
    """Applet's factory"""
    ProxySwitcherGnomeApplet(applet, iid)
    return True

def run_in_window():
    """Run applet in window"""
    import gtk
    import gnomeapplet
    main_window = gtk.Window(gtk.WINDOW_TOPLEVEL)
    main_window.set_title("ProxySwitcher. Python powered.")
    main_window.connect("destroy", gtk.main_quit) 
    app = gnomeapplet.Applet()
    proxy_applet_factory(app, None)
    app.reparent(main_window)
    main_window.show_all()
    gtk.main()
    sys.exit()

def run_in_panel():
    """Run applet in panel"""
    gnomeapplet.bonobo_factory("OAFIID:GNOME_ProxySwitcher_Factory",
                               ProxySwitcherGnomeApplet.__gtype__,
                               "Proxy switcher applet",
                               "0",
                               proxy_applet_factory)

def main(args):
    """Runner"""
    if len(args) == 2 and args[1] == "run-in-window":
        run_in_window()
    else:
        run_in_panel()

if __name__ == '__main__':
    main(sys.argv)
