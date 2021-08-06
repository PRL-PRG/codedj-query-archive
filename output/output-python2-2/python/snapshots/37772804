from common import CWindow, event
from lib.Drawing import CElement, CConnection
import gtk
import gobject

class CfrmProperties(CWindow):
    widgets = ('nbProProperties', 'twAttributes', 'twOperations', 'twConnections', 'cmdDeleteAttribute', 'cmdDeleteOperation', 'cmdNewAttribute', 'cmdNewOperation',
               )
    name = 'frmProperties'
       
    def __init__(self, app, wTree):
        CWindow.__init__(self, app, wTree)
        self.attrModel = gtk.ListStore(gobject.TYPE_STRING, gobject.TYPE_STRING, gobject.TYPE_STRING)
        self.operModel = gtk.ListStore(gobject.TYPE_STRING, gobject.TYPE_STRING, gobject.TYPE_STRING, gobject.TYPE_STRING)
        self.connModel = gtk.ListStore(gobject.TYPE_STRING, gobject.TYPE_STRING, gobject.TYPE_BOOLEAN)
        
        
        
        #~ self.twAttributes.append_column(gtk.TreeViewColumn(_("Visible"), renderer, active = 0))
        self.twAttributes.append_column(gtk.TreeViewColumn(_("Scope"), gtk.CellRendererText(), text = 0))
        self.twAttributes.append_column(gtk.TreeViewColumn(_("Name"), gtk.CellRendererText(), text = 1))
        self.twAttributes.append_column(gtk.TreeViewColumn(_("Type"), gtk.CellRendererText(), text = 2))
        
        #~ self.twOperations.append_column(gtk.TreeViewColumn(_("Visible"), renderer, active = 0))
        self.twOperations.append_column(gtk.TreeViewColumn(_("Scope"), gtk.CellRendererText(), text = 0))
        self.twOperations.append_column(gtk.TreeViewColumn(_("Type"), gtk.CellRendererText(), text = 1))
        self.twOperations.append_column(gtk.TreeViewColumn(_("Name"), gtk.CellRendererText(), text = 2))
        self.twOperations.append_column(gtk.TreeViewColumn(_("Parameters"), gtk.CellRendererText(), text = 3))
        
        self.twConnections.append_column(gtk.TreeViewColumn(_("Object"), gtk.CellRendererText(), text = 0))
        self.twConnections.append_column(gtk.TreeViewColumn(_("Connection"), gtk.CellRendererText(), text = 1))
        renderer = gtk.CellRendererToggle()
        renderer.connect('toggled', self.__fixed_toggled)
        self.twConnections.append_column(gtk.TreeViewColumn(_("Visible"), renderer, active = 2))
        
        self.twAttributes.set_model(self.attrModel)
        self.twOperations.set_model(self.operModel)
        self.twConnections.set_model(self.connModel)
    
    
    def __fixed_toggled(self, cell, path):
        iter = self.connModel.get_iter((int(path),))
        self.connModel.set(iter, 2, not self.connModel.get_value(iter, 2))
        con = tuple(self.__elementObj.GetConnections())[int(path)]
        if con not in self.__connections:
            self.__connections.append(con)
        else:
            self.__connections.remove(con)
    
    def ShowProperties(self, what, elementObject):
        self.__saved = False
        if isinstance(elementObject, CElement):
            isElement = True
            self.__elementObj = elementObject.GetObject()
            self.__connections = []
            self.element = elementObject
            self.nbProProperties.set_current_page(0)
            if len(self.twConnections.get_columns()) < 3:
                renderer = gtk.CellRendererToggle()
                renderer.connect('toggled', self.__fixed_toggled)
                self.twConnections.append_column(gtk.TreeViewColumn(_("Visible"), renderer, active = 2))
        else:
            isElement = False
            self.__connections = None
            self.element = None
            self.__elementObj = elementObject
            if what == 'attrs':
                self.nbProProperties.set_current_page(0)
            elif what == 'opers':
                self.nbProProperties.set_current_page(1)
                
            if len(self.twConnections.get_columns()) == 3:
                self.twConnections.remove_column(self.twConnections.get_column(2))
        
        if self.__elementObj.HasAttribute('Attributes'):
            self.nbProProperties.get_nth_page(0).show()
            self.__attributes = self.__elementObj.GetAttribute("Attributes")[:]
            self.attrModel.clear()
            for attr in self.__attributes:
                iter = self.attrModel.append()
                self.__SetAttrLine(iter, attr)
        else:
            self.nbProProperties.get_nth_page(0).hide()
            self.__attributes = None
            
        if self.__elementObj.HasAttribute('Operations'):
            self.nbProProperties.get_nth_page(1).show()
            self.__operations = self.__elementObj.GetAttribute("Operations")[:]
            self.operModel.clear()
            for oper in self.__operations:
                iter = self.operModel.append()
                self.__SetOperLine(iter, oper)
        else:
            self.nbProProperties.get_nth_page(1).hide()
            self.__operations = None

        #Fill connections tree
        self.connModel.clear()
        for i in self.__elementObj.GetConnections():
            obj = i.GetConnectedObject(self.__elementObj)
            if isElement:
                self.connModel.set(self.connModel.append(), 0, obj.GetName(), 1, i.GetType().GetId(), 2, self.element.GetDrawingArea().HasConnection(i))
            else:
                self.connModel.set(self.connModel.append(), 0, obj.GetName(), 1, i.GetType().GetId())
        
        self.cmdDeleteAttribute.set_sensitive(False)
        self.cmdDeleteOperation.set_sensitive(False)
        response = self.form.run()
        while response == gtk.RESPONSE_APPLY:
            response = self.form.run()
            self.__Save()
        if response == gtk.RESPONSE_OK:
            self.__Save()
        self.Hide()


        return self.__saved
    
    def __Save(self):
        if self.__attributes is not None:
            self.__elementObj.SetAttribute("Attributes", self.__attributes)
        if self.__operations is not None:
            self.__elementObj.SetAttribute("Operations", self.__operations)
            self.__SetAbstractToElement()
        if self.__connections is not None:
            for i in self.__connections:
                con = self.element.GetDrawingArea().GetConnection(i)
                if con is not None:
                    self.element.GetDrawingArea().DeleteConnection(con)
                else:
                    area = self.element.GetDrawingArea()
                    if i.GetSource() is not self.__elementObj:
                        sour = area.HasElementObject(i.GetSource())
                        if sour is not None:
                            CConnection(area,i,sour,self.element)
                    elif i.GetDestination is not self.__elementObj:
                        dest = area.HasElementObject(i.GetDestination())
                        if dest is not None:
                            CConnection(area,i,self.element,dest)
            self.__connections = []
        self.__saved = True
    
    def __SetAttrLine(self, iter, attr):
        self.attrModel.set(iter, 0, attr['scope'], 1, attr['name'], 2, attr['type'])
    
    def __SetOperLine(self, iter, oper):
        self.operModel.set(iter, 0, oper['scope'], 1, oper['type'], 2, oper['name'], 3, oper['params'])
    
    def __RemoveOperations(self, text):
        if text == "":
            return
        o = {}
        o['name'] = text.split('(')[0]
        o['params'] = text.split('(')[1].split(')')[0]
        if text.split(')')[1] != "":
            o['type'] = text.split(')')[1].split(':')[1]
        else:
            o['type'] = ''
            
        for id, i in enumerate(self.__operations):
            if o['name'] == i['name'] and o['params'] == i['params'] and o['type'] == i['type']:
                model = self.twOperations.get_model()
                model.remove(model.get_iter(id))                
                self.__operations.remove(i)
                return
    
    def IsExistsOperation(self,oper):
        return False
    
    def __SetAbstractToElement(self):
        abstr = False
        for i in self.__operations:
            if i['pure']:
                abstr = True
                break
        self.__elementObj.SetAttribute('Abstract' ,abstr)
            
    
    '''Nastavuje overload override pre delphi a nastavuje aj abstraktnu triedu ak
       ma aspon jednu cisto virtualnu metodu '''
    def SetSpecifyProperty(self,oper):
        if oper['pure']:
            self.__elementObj.SetAttribute('Abstract' ,True)
        for i in self.__operations:
            if i['name'] == oper['name']:
                i['overload'] = True
                oper['overload'] = True
                break
        else:
            oper['overload'] = False
        
        if oper['abstract']:
            for con in self.__elementObj.GetConnections():
                if con.GetType().GetId() == "Generalization":
                    opers = con.GetDestination().GetAttribute("Operations")
                    for o in opers:
                        if o['name'] == oper['name'] and o['params'] == oper['params'] and o['abstract'] and o['type'] == oper['type']:
                            oper['override'] = True
                            break
                    else:
                        oper['override'] = False
        else:
            oper['override'] = False
    
    
    @event("cmdNewAttribute", "clicked")
    def on_cmdNewAttribute_clicked(self, widget):
        attr = {}
        tmp = self.application.GetWindow('frmAttribute')
        tmp.SetParent(self)
        retDlg = tmp.ShowFrmAttribute(attr)
        if retDlg or isinstance(retDlg, dict):
            self.__attributes.append(attr)
            iter = self.attrModel.append()
            self.__SetAttrLine(iter, attr)
            if isinstance(retDlg, dict) and isinstance(retDlg['append'], list):
                for i in retDlg['append']:
                    self.__operations.append(i)
                    iter = self.operModel.append()
                    self.__SetOperLine(iter, i)
        
    @event("cmdNewOperation", "clicked")
    def on_cmdNewOperation_clicked(self, widget):
        oper = {}
        tmp = self.application.GetWindow('frmOperation')
        tmp.SetParent(self)
        if tmp.ShowFrmOperation(oper):
            self.SetSpecifyProperty(oper)
            self.__operations.append(oper)
            iter = self.operModel.append()
            self.__SetOperLine(iter, oper)
    
    @event("cmdDeleteOperation", "clicked")
    def on_cmdDeleteOperation_clicked(self, widget):
        sel = self.twOperations.get_selection()
        model, iter = sel.get_selected()
        del self.__operations[model.get_path(iter)[0]]
        model.remove(iter)
        self.cmdDeleteOperation.set_sensitive(False)
    
    @event("cmdDeleteAttribute", "clicked")
    def on_cmdDeleteAttribute_clicked(self, widget):
        sel = self.twAttributes.get_selection()
        model, iter = sel.get_selected()
        atr = self.__attributes[model.get_path(iter)[0]]
        self.__RemoveOperations(atr['getter'])
        self.__RemoveOperations(atr['setter'])
        del self.__attributes[model.get_path(iter)[0]]
        model.remove(iter)
        self.cmdDeleteAttribute.set_sensitive(False)
    
    @event("twAttributes", "cursor-changed")
    def on_twAttributes_cursor_changed(self, widget):
        self.cmdDeleteAttribute.set_sensitive(True)
    
    @event("twOperations", "cursor-changed")
    def on_twOperations_cursor_changed(self, widget):
        self.cmdDeleteOperation.set_sensitive(True)
    
    @event("twAttributes", "row-activated")
    def on_twAttributes_row_activated(self, widget, path, column):
        attr = self.__attributes[path[0]]
        dlg = self.application.GetWindow('frmAttribute')
        dlg.SetParent(self)
        retDlg = dlg.ShowFrmAttribute(attr)
        if retDlg or isinstance(retDlg, dict):
            iter = self.attrModel.get_iter(path)
            self.__SetAttrLine(iter, attr)
            if isinstance(retDlg, dict):
                if isinstance(retDlg['append'],list):
                    for i in retDlg['append']:
                        self.__operations.append(i)
                        iter = self.operModel.append()
                        self.__SetOperLine(iter, i)
                if isinstance(retDlg['remove'], list):
                    for i in retDlg['remove']:
                        self.__RemoveOperations(i)
    
    @event("twOperations", "row-activated")
    def on_twOperations_row_activated(self, widget, path, column):
        oper = self.__operations[path[0]]
        dlg = self.application.GetWindow('frmOperation')
        dlg.SetParent(self)
        if dlg.ShowFrmOperation(oper):
            iter = self.operModel.get_iter(path)
            self.__SetOperLine(iter, oper)
    
    