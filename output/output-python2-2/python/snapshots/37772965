from common import CWidget
import gtk
import gtk.gdk
import gobject

from lib.consts import STARTPAGE_IMAGE

from common import  event

class CtabStartPage(CWidget):
    name = 'tabStartPage'
    widgets = ('ebStartPage', )
    
    def __init__(self, app, wTree):
        CWidget.__init__(self, app, wTree)
        
        style = self.ebStartPage.get_style().copy()
        pixbuf = gtk.gdk.pixbuf_new_from_file(self.GetRelativeFile(STARTPAGE_IMAGE))
        pixmap = gtk.gdk.Pixmap(self.ebStartPage.window, 2000, 2000)
        cmap = self.ebStartPage.get_colormap()
        gc = self.ebStartPage.window.new_gc(foreground = cmap.alloc_color("#FFFFFF"))
        pixmap.draw_rectangle(gc, True, 0, 0, 2000, 2000)
        pixbuf.render_to_drawable(pixmap, gc, 0, 0, 0, 0, -1, -1, 0, 0, 0)
        style.bg_pixmap[gtk.STATE_NORMAL] = pixmap
        self.ebStartPage.set_style(style)
        

