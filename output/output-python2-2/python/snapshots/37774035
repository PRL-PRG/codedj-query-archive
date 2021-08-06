import gtk, gobject

from common import CWidget

class CmnuItems(CWidget):
    name = 'mnuItems'
    widgets = ('mnuDiagrams', )
    
    __gsignals__ = {
        'create_diagram':   (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, 
            (gobject.TYPE_STRING, ))
    }
    
    def Init(self):
        self.DiagramType = None
        
    def LoadDiagramsMenu(self):
        for item in self.mnuDiagrams.get_children():
            self.mnuDiagrams.remove(item)
        for diagram in self.application.DiagramFactory:
            newItem = gtk.ImageMenuItem(diagram.GetId())
            newItem.connect("activate", self.on_mnuDiagrams_activate, diagram.GetId())
            self.mnuDiagrams.append(newItem)
            newItem.show()
            
    def on_mnuDiagrams_activate(self, widget, diagramId):
        self.emit('create_diagram', diagramId)
        
