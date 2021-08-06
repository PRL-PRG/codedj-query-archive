from common import CWidget, event

import gtk
from gtk.gdk import pixbuf_new_from_file

from lib.consts import ARROW_IMAGE

class CtbToolBox(CWidget):
    name = 'tbToolBox'
    widgets = ('tbToolBox', 'hboxWorkSpace')
    
    __visible = True
    
    def __init__(self, app, wTree):
        CWidget.__init__(self, app, wTree)
        self.Selected = None
        
        self.tooltips = gtk.Tooltips()
        
        pixbuf = pixbuf_new_from_file(ARROW_IMAGE)
        newIconWidget = gtk.Image()
        newIconWidget.set_from_pixbuf(pixbuf)
        newIconWidget.show()
        self.ArrowButton = self.tbToolBox.get_nth_item(0)
        self.ArrowButton.connect("toggled", self.on_tbArrowBtn_toggled)
        self.ArrowButton.set_icon_widget(newIconWidget)
        self.ArrowButton.set_tooltip(self.tooltips, "Selection tool")
        
    def __InsertButton(self, Type, TypeDesc, Group):
        newIconWidget = gtk.Image()
        newIconWidget.set_from_pixbuf(Type.GetIcon())
        newIconWidget.show()
        newButton = gtk.RadioToolButton(Group, None)
        newButton.set_icon_widget(newIconWidget)
        newButton.set_tooltip(self.tooltips, Type.GetId())
        newButton.connect("toggled", self.on_tbButton_toggled, Type.GetId(), TypeDesc)
        newButton.show()
        self.tbToolBox.insert(newButton, -1)
        
    def __InsertSeparator(self):
        newSeparator = gtk.SeparatorToolItem()
        newSeparator.show()
        self.tbToolBox.insert(newSeparator, -1)
                
    def SetButtons(self, DiagramId):
        if DiagramId is None:
            ArrowButton = self.tbToolBox.get_nth_item(0)
            for button in self.tbToolBox.get_children():
                self.tbToolBox.remove(button)
                
            self.tbToolBox.insert(ArrowButton, -1)
            return
            
        self.DiagramType = self.application.DiagramFactory.GetDiagram(DiagramId)
        if self.DiagramType is None:
            raise Exception('tbToolBox.DiagramType is None')
        ArrowButton = self.tbToolBox.get_nth_item(0)
        for button in self.tbToolBox.get_children():
            self.tbToolBox.remove(button)
            
        self.tbToolBox.insert(ArrowButton, -1)
        
        ElementNameList = self.DiagramType.GetElements()
        if len(ElementNameList) > 0:
            self.__InsertSeparator()
            for ElementName in ElementNameList:
                ElementType = self.application.ElementFactory.GetElement(ElementName)
                self.__InsertButton(ElementType, 'Element', ArrowButton)
            
        ConnectionNameList = self.DiagramType.GetConnections()
        if len(ConnectionNameList) > 0:
            self.__InsertSeparator()
            for ConnectionName in ConnectionNameList:
                ConnectionType = self.application.ConnectionFactory.GetConnection(ConnectionName)
                self.__InsertButton(ConnectionType, 'Connection', ArrowButton)
                
    def __ResetSelected(self):
        self.ArrowButton.set_active(True)
            
    #@event("tbArrowBtn", "toggled")
    def on_tbArrowBtn_toggled(self, widget):
        self.Selected = None
        
    #@event("tbButton", "toggled")
    def on_tbButton_toggled(self, widget, ItemId, ItemType):
        self.Selected = (ItemType, ItemId)
        
    def GetSelected(self):
        return self.Selected
        
    def SetSelected(self, message):
        self.Selected = message
        if self.Selected  == None:
            self.__ResetSelected()
        else:
            pass
            
    def Show(self):
        if not self.__visible:
            self.hboxWorkSpace.pack_start(self.tbToolBox, expand=False, fill=False)
            self.hboxWorkSpace.reorder_child(self.tbToolBox, 0)
            self.__visible = True
        
    def Hide(self):
        if self.__visible:
            self.hboxWorkSpace.remove(self.tbToolBox)
            self.__visible = False
        
    def SetVisible(self, value):
        if value:
            self.Show()
        else:
            self.Hide()
