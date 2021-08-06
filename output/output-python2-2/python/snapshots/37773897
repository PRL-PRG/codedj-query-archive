import gtk, gobject
from lib.consts import DIAGRAMS
from common import CWidget, event
from gtk.gdk import pixbuf_new_from_file

class CmnuItems(CWidget):
    name = 'mnuItems'
    widgets = ('mnuAddDiagram', )
    
    __gsignals__ = {
        'create-diagram':   (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, 
            (gobject.TYPE_STRING, ))
    }
    
    def __init__(self, app, wTree):
        CWidget.__init__(self, app, wTree)
        self.DiagramType = None
        
    def LoadDiagramsMenu(self):
        for item in self.mnuAddDiagram.get_children():
            self.mnuAddDiagram.remove(item)
        for diagram in self.application.version.GetDiagrams():
            newItem = gtk.ImageMenuItem(diagram)
            newItem.connect("activate", self.on_mnuDiagrams_activate, diagram)
            img = gtk.Image()
            img.set_from_pixbuf(pixbuf_new_from_file(DIAGRAMS[diagram]))
            img.show()
            newItem.set_image(img)
            self.mnuAddDiagram.append(newItem)
            newItem.show()
        
    #@event("mnuDiagrams", "activate")
    def on_mnuDiagrams_activate(self, widget, diagramId):
        self.emit('create-diagram', diagramId)
        
