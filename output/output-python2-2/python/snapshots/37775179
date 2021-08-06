from lib.Depend.gtk2 import gtk
from lib.Depend.gtk2 import gobject

from lib.Drawing.Canvas.GtkPlus import PixmapFromPath

from common import CWidget, event

class CmnuItems(CWidget):
    name = 'mnuItems'
    widgets = ('mItemAddDiagram_menu',)
    
    __gsignals__ = {
        'create-diagram':   (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, 
            (gobject.TYPE_STRING, ))
    }
        
    def Redraw(self):
        for item in self.mItemAddDiagram_menu.get_children():
            self.mItemAddDiagram_menu.remove(item)
        for diagram in self.application.GetProject().GetVersion().GetDiagrams():
            newItem = gtk.ImageMenuItem(diagram)
            newItem.connect("activate", self.on_mnuDiagrams_activate, diagram)
            img = gtk.Image()
            img.set_from_pixbuf(PixmapFromPath(self.application.GetProject().GetStorage(), self.application.GetProject().GetDiagramFactory().GetDiagram(diagram).GetIcon()))
            img.show()
            newItem.set_image(img)
            self.mItemAddDiagram_menu.append(newItem)
            newItem.show()
        
    def on_mnuDiagrams_activate(self, widget, diagramId):
        self.emit('create-diagram', diagramId)
        
