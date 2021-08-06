import gtk, gobject

from common import CWidget, event

class CmnuItems(CWidget):
    name = 'mnuItems'
    widgets = ('mnuDiagrams', )
    
    __gsignals__ = {
        'create-diagram':   (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, 
            (gobject.TYPE_STRING, ))
    }
    
    def __init__(self, app, wTree):
        CWidget.__init__(self, app, wTree)
        self.DiagramType = None
        
    def LoadDiagramsMenu(self):
        for item in self.mnuDiagrams.get_children():
            self.mnuDiagrams.remove(item)
        for diagram in self.application.DiagramFactory:
            newItem = gtk.ImageMenuItem(diagram.GetId())
            newItem.connect("activate", self.on_mnuDiagrams_activate, diagram.GetId())
            self.mnuDiagrams.append(newItem)
            newItem.show()
        
    #@event("mnuDiagrams", "activate")
    def on_mnuDiagrams_activate(self, widget, diagramId):
        self.emit('create-diagram', diagramId)
        
