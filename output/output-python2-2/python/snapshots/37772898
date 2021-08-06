from common import CWindow, event
from lib.consts import SPLASH_IMAGE

import gtk, gtk.gdk, pango

gtk.rc_parse_string("""
    style "test"
    {
        bg_pixmap[NORMAL] = "<none>"
    } widget "frmSplash" style "test"
""")

class CfrmSplash(CWindow):
    name = 'frmSplash'
    widgets = ('fixMain', 'lblVersion', )
    
    def __init__(self, app, wTree):
        CWindow.__init__(self, app, wTree)
        
        self.lblVersion.set_label(('<span foreground="white" font_desc="Arial bold 9">'+_("Version: %s")+'</span>')%self.application.GetVersion())
        
        style = self.form.get_style().copy()
        pixbuf = gtk.gdk.pixbuf_new_from_file(self.GetRelativeFile(SPLASH_IMAGE))
        pixmap = gtk.gdk.Pixmap(self.form.window, pixbuf.get_width(), pixbuf.get_height())
        gc = self.fixMain.window.new_gc()
        pixbuf.render_to_drawable(pixmap, gc, 0, 0, 0, 0, -1, -1, 0, 0, 0)
        style.bg_pixmap[gtk.STATE_NORMAL] = pixmap
        self.form.set_style(style)
