from common import CWidget, event
from lib.Exceptions.UserException import *
from lib.Drawing import CDiagram
import gobject
from lib.Elements.Object import CElementObject
from lib.Connections.Object import CConnectionObject

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
        
        if isinstance(self.element, CDiagram):
            return
        
        object = Element.GetObject()
        if isinstance(object, (CElementObject, CConnectionObject)):
            if object.GetDomainType().HasAttribute('note'):
                self.txtNotes.get_buffer().set_text(object.GetValue('note'))
                self.txtNotes.set_sensitive(True)
        elif isinstance(object, CConnectionObject):
            type = Element.GetObject().GetType()
            cnt = 0
            for k in type.GetAttributes():
                v = object.GetAttribute(k)
                atrtype = type.GetAttribute(k)
                if atrtype[0] == 'note':
                    if cnt > 0:
                        self.element = None
                        raise ProjectError("TooMuchNotes")
                    self.txtNotes.get_buffer().set_text(v)
                    self.txtNotes.set_sensitive(True)
                    self.attr = k
                    cnt += 1
    
    @event("txtNotes.buffer", "changed")
    def on_txtNotes_changed(self, buffer):
        if self.element is not None:
            if isinstance(self.element, CDiagram):
                pass    #maybe, In the future, We can add notes to diagram
            elif isinstance(self.element.GetObject(), (CElementObject, CConnectionObject)):
                self.element.GetObject().SetValue('note', buffer.get_text(buffer.get_start_iter(), buffer.get_end_iter()))
                self.emit('content_update', self.element, 'note')
            elif isinstance(self.element.GetObject(), CConnectionObject):
                self.element.GetObject().SetAttribute(self.attr, buffer.get_text(buffer.get_start_iter(), buffer.get_end_iter()))
                self.emit('content_update', self.element, self.attr)
