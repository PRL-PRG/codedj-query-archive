from common import CWidget, CellRendererButton

import gobject
import gtk
import gtk.gdk

ID_NAME, ID_VALUE, ID_TEXT_VISIBLE, ID_COMBO_VISIBLE, ID_EDITABLE ,ID_BUTTON_VISIBLE, ID_MODEL, ID_TYPE = range(8)

class ClwProperties(CWidget):
    name = 'lwProperties'
    widgets = ('lwProperties',)
    
    __gsignals__ = {
        'content_update':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, 
            (gobject.TYPE_PYOBJECT, gobject.TYPE_STRING)),      
    }
    
    def Init(self):
        self.listStore = gtk.ListStore(gobject.TYPE_STRING, gobject.TYPE_STRING, gobject.TYPE_BOOLEAN, gobject.TYPE_BOOLEAN, gobject.TYPE_BOOLEAN, gobject.TYPE_BOOLEAN, gtk.TreeModel, gobject.TYPE_STRING)
        self.StrRenderer = gtk.CellRendererText()
        
        self.Column1 = gtk.TreeViewColumn('Name')
        self.Column1.pack_start(self.StrRenderer, True)
        self.Column1.add_attribute(self.StrRenderer, 'text', ID_NAME)
                 
        self.StrRenderer = gtk.CellRendererText()
        self.StrRenderer.set_property('editable', True)
        self.ComboRenderer = gtk.CellRendererCombo()
        self.ComboRenderer.set_property('text-column', 0)
        self.ComboRenderer.set_property('editable', True)
        self.ButtonRenderer = CellRendererButton()
        
        
        self.StrRenderer.connect('edited',self.on_change_text)
        self.ComboRenderer.connect('edited',self.on_change_combo)
        self.ButtonRenderer.connect('click',self.on_change_button)
        
 
        self.Column2 = gtk.TreeViewColumn('Value')
        self.Column2.pack_start(self.StrRenderer, True)
        self.Column2.pack_start(self.ComboRenderer, True)
        self.Column2.pack_start(self.ButtonRenderer, True)
        
        self.Column2.add_attribute(self.StrRenderer, 'text', ID_VALUE)
        self.Column2.add_attribute(self.StrRenderer, 'editable', ID_TEXT_VISIBLE)
        self.Column2.add_attribute(self.StrRenderer, 'visible', ID_TEXT_VISIBLE)
        
        self.Column2.add_attribute(self.ComboRenderer, 'model', ID_MODEL)
        self.Column2.add_attribute(self.ComboRenderer, 'has-entry', ID_EDITABLE)
        self.Column2.add_attribute(self.ComboRenderer, 'visible', ID_COMBO_VISIBLE)
        self.Column2.add_attribute(self.ComboRenderer, 'text', ID_VALUE)
        
        self.Column2.add_attribute(self.ButtonRenderer, 'text', ID_VALUE)
        self.Column2.add_attribute(self.ButtonRenderer, 'visible', ID_BUTTON_VISIBLE)
        
        
        self.lwProperties.append_column(self.Column1)
        self.lwProperties.append_column(self.Column2)
        self.lwProperties.set_model(self.listStore)
        
    
    def Fill(self, Element):
        self.element = Element
        self.listStore.clear()
        if Element is  None:
            return
        attrs = Element.GetObject().GetAttributes()
        type = Element.GetObject().GetType()
        for k in type.GetAttributes(): # attrs.items():
            v = attrs[k]
            row = self.listStore.append()
            atrtype = type.GetAttribute(k)
            if atrtype[0] == 'bool':
                model = gtk.ListStore(gobject.TYPE_STRING)
                model.set(model.append(), 0, 'True')
                model.set(model.append(), 0, 'False')
                self.listStore.set(row, ID_TYPE, atrtype[0], ID_NAME, str(k) , ID_VALUE, str(v),  ID_TEXT_VISIBLE, False, ID_COMBO_VISIBLE, True, ID_BUTTON_VISIBLE, False, ID_EDITABLE, False, ID_MODEL, model)
            elif len(atrtype[1]) > 0:
                model = gtk.ListStore(gobject.TYPE_STRING)
                for i in atrtype[1]:
                    model.set(model.append(), 0 , str(i))
                editable = not (atrtype[0] == 'enum')
                self.listStore.set(row, ID_NAME, str(k), ID_VALUE, str(v), ID_TEXT_VISIBLE, False, ID_COMBO_VISIBLE, True, ID_BUTTON_VISIBLE, False, ID_EDITABLE, editable, ID_MODEL, model)
            elif atrtype[0] == 'str':
                self.listStore.set(row, ID_TYPE, atrtype[0], ID_NAME, str(k), ID_VALUE, str(v), ID_TEXT_VISIBLE, True, ID_COMBO_VISIBLE, False, ID_BUTTON_VISIBLE, False, ID_EDITABLE, True)
            elif atrtype[0] == 'note':
                pass
            else:
                self.listStore.set(row, ID_TYPE, atrtype[0], ID_NAME, str(k), ID_VALUE, "<<list>>", ID_TEXT_VISIBLE, False, ID_COMBO_VISIBLE, False, ID_BUTTON_VISIBLE, True)
    
    def Clear(self):
        self.element = None
        self.listStore.clear()
    
    def on_change_text(self, cellrenderer, path, new_value):
        model = self.lwProperties.get_model()
        iter = model.get_iter_from_string(path)
        model.set(iter, ID_VALUE, new_value) 
        name, = model.get(iter, ID_NAME)
        self.element.GetObject().SetAttribute(name ,new_value)
        self.emit('content_update', self.element, name)
        
    def on_change_combo(self, cellrenderer, path, new_value):
        model = self.lwProperties.get_model()
        iter = model.get_iter_from_string(path)
        model.set(iter, ID_VALUE, new_value)
        name, = model.get(iter, ID_NAME)
        self.element.GetObject().SetAttribute(name ,new_value)
        self.emit('content_update', self.element, name)
    
    def on_change_button(self, cellrenderer, path):
        model = self.lwProperties.get_model()
        iter = model.get_iter_from_string(path)
        type, name = model.get(iter, ID_TYPE, ID_NAME)
        if type == 'attrs':
            tmp = self.application.GetWindow('frmProperties')
            tmp.SetParent(self.application.GetWindow('frmMain'))
            if tmp.ShowProperties('attrs',self.element):
                self.emit('content_update', self.element, name)
        elif type == 'opers':
            tmp = self.application.GetWindow('frmProperties')
            tmp.SetParent(self.application.GetWindow('frmMain'))
            if tmp.ShowProperties('opers',self.element):
                self.emit('content_update', self.element, name)
