from common import CWidget
from lwProperties import ClwProperties
from txtNotes import CtxtNotes
import gobject

class CnbProperties(CWidget):
    name = 'nbProperties'
    complexWidgets = (ClwProperties, CtxtNotes)
    
    __gsignals__ = {
        'content_update':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, 
            (gobject.TYPE_PYOBJECT, gobject.TYPE_STRING)),
    }
    
    def Init(self):
        self.lwProperties.connect('content_update', self.on_lwProperties_content_update)
        self.txtNotes.connect('content_update', self.on_txtNotes_content_update)
    
    def Fill(self, element):
        self.lwProperties.Fill(element)
        self.txtNotes.Fill(element)
    
    def on_lwProperties_content_update(self, widget, element, property):
        self.emit("content_update", element, property)
    
    def on_txtNotes_content_update(self, widget, element, property):
        self.emit("content_update", element, property)
