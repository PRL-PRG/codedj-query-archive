from common import CWidget
from lib.lib import UMLException
import gobject

class CtxtNotes(CWidget):
    name = 'txtNotes'
    widgets = ('txtNotes', )
    
    __gsignals__ = {
        'content_update':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, 
            (gobject.TYPE_PYOBJECT, gobject.TYPE_STRING)),
    }
    
    def Init(self):
        self.txtNotes.set_sensitive(False)
        
        self.txtNotes.get_buffer().connect('changed', self.on_txtNotes_changed)
    
    def Fill(self, Element):
        self.element = Element
        if Element is None:
            self.txtNotes.get_buffer().set_text("")
            self.txtNotes.set_sensitive(False)
            return
        attrs = Element.GetObject().GetAttributes()
        type = Element.GetObject().GetType()
        cnt = 0
        for k in type.GetAttributes(): # attrs.items():
            v = attrs[k]
            atrtype = type.GetAttribute(k)
            if atrtype[0] == 'note':
                if cnt > 0:
                    self.element = None
                    raise UMLException("TooMuchNotes")
                self.txtNotes.get_buffer().set_text(v)
                self.txtNotes.set_sensitive(True)
                self.attr = k
                cnt += 1
    
    def on_txtNotes_changed(self, buffer):
        if self.element is not None:
            self.element.GetObject().SetAttribute(self.attr, buffer.get_text(buffer.get_start_iter(), buffer.get_end_iter()))
            
            self.emit('content_update', self.element, self.attr)
