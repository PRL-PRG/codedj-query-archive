from common import CWidget, event
from lib.lib import UMLException
import gobject

class CtxtNotes(CWidget):
    name = 'txtNotes'
    widgets = ('txtNotes', )
    
    __gsignals__ = {
        'content-update':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, 
            (gobject.TYPE_PYOBJECT, gobject.TYPE_STRING)),
    }
    
    def __init__(self, app, wTree):
        CWidget.__init__(self, app, wTree)
        self.txtNotes.set_sensitive(False)
    
    def Fill(self, Element):
        self.element = Element
        if Element is None:
            self.txtNotes.get_buffer().set_text("")
            self.txtNotes.set_sensitive(False)
            return
        object = Element.GetObject()
        type = Element.GetObject().GetType()
        cnt = 0
        for k in type.GetAttributes():
            v = object.GetAttribute(k)
            atrtype = type.GetAttribute(k)
            if atrtype[0] == 'note':
                if cnt > 0:
                    self.element = None
                    raise UMLException("TooMuchNotes")
                if v is None:
                    v = ''
                self.txtNotes.get_buffer().set_text(v)
                self.txtNotes.set_sensitive(True)
                self.attr = k
                cnt += 1
    
    @event("txtNotes.buffer", "changed")
    def on_txtNotes_changed(self, buffer):
        if self.element is not None:
            self.element.GetObject().SetAttribute(self.attr, buffer.get_text(buffer.get_start_iter(), buffer.get_end_iter()))
            
            self.emit('content_update', self.element, self.attr)
