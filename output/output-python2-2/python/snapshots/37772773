from common import CWidget, event
from lib.lib import UMLException
from lib.Drawing import CDrawingArea
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
        self.attr = ""
        if Element is None:
            self.txtNotes.get_buffer().set_text("")
            self.txtNotes.set_sensitive(False)
            return
        if isinstance(Element, CDrawingArea):
            object = Element
            type = Element.GetType()
        else:
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
                self.txtNotes.get_buffer().set_text(v)
                self.txtNotes.set_sensitive(True)
                self.attr = k
                cnt += 1
    
    @event("txtNotes.buffer", "changed")
    def on_txtNotes_changed(self, buffer):
        if self.element is not None:
            if isinstance(self.element, CDrawingArea):
                self.element.SetAttribute(self.attr, buffer.get_text(buffer.get_start_iter(), buffer.get_end_iter()))          
            else:
                if self.element.GetObject().HasAttribute(self.attr):
                    self.element.GetObject().SetAttribute(self.attr, buffer.get_text(buffer.get_start_iter(), buffer.get_end_iter()))
                    self.emit('content_update', self.element, self.attr)
