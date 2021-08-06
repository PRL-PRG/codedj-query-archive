from lib.Depend.gtk2 import gobject
from lib.Depend.gtk2 import gtk

from common import CWidget, CellRendererButton, event
from lib.Drawing import CDiagram
from lib.Elements.Object import CElementObject
from lib.Connections.Object import CConnectionObject

ID_ID, ID_NAME, ID_VALUE, ID_TEXT_VISIBLE, ID_COMBO_VISIBLE, ID_EDITABLE, ID_BUTTON_VISIBLE, ID_MODEL, ID_BUTTON_TEXT, ID_ACTION = range(10)

class ClwProperties(CWidget):
    name = 'lwProperties'
    widgets = ('lwProperties',)
    
    __gsignals__ = {
        'content_update':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, 
            (gobject.TYPE_PYOBJECT, gobject.TYPE_STRING)),      
    }
    
    def __init__(self, app, wTree):
        
        self.treeStore = gtk.TreeStore(gobject.TYPE_STRING, gobject.TYPE_STRING, gobject.TYPE_STRING, gobject.TYPE_BOOLEAN, gobject.TYPE_BOOLEAN, gobject.TYPE_BOOLEAN, gobject.TYPE_BOOLEAN, gtk.TreeModel, gobject.TYPE_STRING, gobject.TYPE_STRING)
        
        renderer = gtk.CellRendererText()
        self.Column1 = gtk.TreeViewColumn(_('Name'))
        self.Column1.pack_start(renderer, True)
        self.Column1.add_attribute(renderer, 'text', ID_NAME)
                 
        self.StrRenderer = gtk.CellRendererText()
        self.StrRenderer.set_property('editable', True)
        self.ComboRenderer = gtk.CellRendererCombo()
        self.ComboRenderer.set_property('text-column', 0)
        self.ComboRenderer.set_property('editable', True)
        
        self.Column2 = gtk.TreeViewColumn(_('Value'))
        self.Column2.pack_start(self.StrRenderer, True)
        self.Column2.pack_start(self.ComboRenderer, True)
        
        self.Column2.add_attribute(self.StrRenderer, 'text', ID_VALUE)
        self.Column2.add_attribute(self.StrRenderer, 'editable', ID_EDITABLE)
        self.Column2.add_attribute(self.StrRenderer, 'visible', ID_TEXT_VISIBLE)
        
        self.Column2.add_attribute(self.ComboRenderer, 'model', ID_MODEL)
        self.Column2.add_attribute(self.ComboRenderer, 'has-entry', ID_EDITABLE)
        self.Column2.add_attribute(self.ComboRenderer, 'visible', ID_COMBO_VISIBLE)
        self.Column2.add_attribute(self.ComboRenderer, 'text', ID_VALUE)
        
        self.ButtonRenderer = CellRendererButton()
        CWidget.__init__(self, app, wTree)
        
        self.Column2.pack_start(self.ButtonRenderer, False)
        self.Column2.add_attribute(self.ButtonRenderer, 'visible', ID_BUTTON_VISIBLE)
        self.Column2.add_attribute(self.ButtonRenderer, 'text', ID_BUTTON_TEXT)
        
        
        self.lwProperties.append_column(self.Column1)
        self.lwProperties.append_column(self.Column2)
        self.lwProperties.set_model(self.treeStore)
        
    
    def _FillListItem(self, object, parent, prefix, idx):
        itemrow = self.treeStore.append(parent)
        self.treeStore.set(itemrow,
            ID_ID, prefix + '[%i]' % idx,
            ID_NAME, str(idx), 
            ID_VALUE, '', #text representation of item in list
            ID_TEXT_VISIBLE, False, 
            ID_COMBO_VISIBLE, False, 
            ID_BUTTON_VISIBLE, True, 
            ID_EDITABLE, False, #Change to True if has parser
            ID_BUTTON_TEXT, 'Delete',
            ID_ACTION, 'listdel')
        self._FillBody(object, itemrow, prefix + '[%i]' % idx)
    
    def _FillBody(self, object, parent, prefix):
        
        DType = object.GetDomainType(prefix)
        
        for attrID in DType.IterAttributeIDs():
            row = self.treeStore.append(parent)
            identifier = (prefix + '.' if prefix else '') + attrID
            type = DType.GetAttribute(attrID)['type']
            name = DType.GetAttribute(attrID)['name']
            
            if not DType.IsAtomic(domain = type):
                self.treeStore.set(row, 
                    ID_ID, identifier,
                    ID_NAME, name, 
                    ID_VALUE, '', #text representation of nested item
                    ID_TEXT_VISIBLE, False, 
                    ID_COMBO_VISIBLE, False, 
                    ID_BUTTON_VISIBLE, False, 
                    ID_EDITABLE, False)#Change to True if has parser
                self._FillBody(object, row, identifier)
            
            elif type in ('str', 'int', 'float', 'text'):
                self.treeStore.set(row, 
                    ID_ID, identifier,
                    ID_NAME, name, 
                    ID_VALUE, str(object.GetValue(identifier)), 
                    ID_TEXT_VISIBLE, True, 
                    ID_COMBO_VISIBLE, False, 
                    ID_BUTTON_VISIBLE, False, 
                    ID_EDITABLE, True)
            
            elif type in ('enum', 'bool'):
                model = gtk.ListStore(gobject.TYPE_STRING)
                for item in (DType.GetAttribute(attrID)['enum'] if type == 'enum' else ('True', 'False')):
                    model.set(model.append(), 0 , item)
                self.treeStore.set(row, 
                    ID_ID, identifier,
                    ID_NAME, name, 
                    ID_VALUE, str(object.GetValue(identifier)), 
                    ID_TEXT_VISIBLE, False, 
                    ID_COMBO_VISIBLE, True, 
                    ID_BUTTON_VISIBLE, False, 
                    ID_EDITABLE, False, 
                    ID_MODEL, model)
            
            elif type == 'list':
                self.treeStore.set(row, 
                    ID_ID, identifier,
                    ID_NAME, name, 
                    ID_VALUE, '', #text representation of list
                    ID_TEXT_VISIBLE, False, 
                    ID_COMBO_VISIBLE, False, 
                    ID_BUTTON_VISIBLE, True, 
                    ID_EDITABLE, False, #Change to True if has parser
                    ID_BUTTON_TEXT, 'Add item',
                    ID_ACTION, 'listadd')
                for idx, item in enumerate(object.GetValue(identifier)):
                    self._FillListItem(object, row, identifier, idx)
    
    def Fill(self, Element):
        self.element = Element
        self.treeStore.clear()
        
        if Element is  None:
            return
        
        if isinstance(self.element, CDiagram):
            v = self.element.GetName()
            row = self.treeStore.append(None)
            self.treeStore.set(row, ID_NAME, 'Name', ID_VALUE, v, ID_TEXT_VISIBLE, True, ID_COMBO_VISIBLE, False, ID_BUTTON_VISIBLE, False, ID_EDITABLE, True)
            return
        else:
            self._FillBody(self.element.GetObject(), None, '')
    
    def Clear(self):
        self.element = None
        self.treeStore.clear()
    
    @event("StrRenderer", "edited")
    def on_change_text(self, cellrenderer, path, new_value):
        model = self.lwProperties.get_model()
        iter = model.get_iter_from_string(path)
        model.set(iter, ID_VALUE, new_value) 
        if isinstance(self.element, CDiagram):
            name, = model.get(iter, ID_NAME)
            self.element.SetName(new_value)
            self.emit('content_update', self.element, name)
        else:
            key, = model.get(iter, ID_ID)
            self.element.GetObject().SetValue(key, new_value)
            self.emit('content_update', self.element, key)
        
    @event("ComboRenderer", "edited")
    def on_change_combo(self, cellrenderer, path, new_value):
        model = self.lwProperties.get_model()
        iter = model.get_iter_from_string(path)
        model.set(iter, ID_VALUE, new_value)
        key, = model.get(iter, ID_ID)
        self.element.GetObject().SetValue(key, new_value)
        self.emit('content_update', self.element, key)
    
    @event("ButtonRenderer", "click")
    def on_change_button(self, cellrenderer, path):
        model = self.lwProperties.get_model()
        iter = model.get_iter_from_string(path)
        key, action = model.get(iter, ID_ID, ID_ACTION)
        if action == 'listadd':
            self.element.GetObject().AppendItem(key)
            self._FillListItem(self.element.GetObject(), iter, key, len(self.element.GetObject().GetValue(key)) - 1)
        elif action == 'listdel':
            self.element.GetObject().RemoveItem(key)
            self.treeStore.remove(iter)
            
            for idx in xrange(int(path.rsplit(':', 1)[-1]), len(self.element.GetObject().GetValue(key.rsplit('[', 1)[0]))):
                npath = path.rsplit(':', 1)[0] + ':' + str(idx)
                niter = model.get_iter_from_string(npath)
                self.treeStore.set(niter,
                    ID_ID, key.rsplit('[', 1)[0] + '[%i]' % idx,
                    ID_NAME, str(idx))
        
        self.emit('content_update', self.element, key)
        
