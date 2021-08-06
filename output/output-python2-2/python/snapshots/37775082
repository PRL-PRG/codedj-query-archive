from lib.Depend.gtk2 import gtk
from lib.Depend.gtk2 import gobject

from lib.Drawing.Canvas.GtkPlus import PixmapFromPath

from common import CWidget, event

class CmnuItems(CWidget):
    name = 'mnuItems'
    widgets = ('mItemAddDiagram_menu','mItemAddElement_menu',)
    
    __gsignals__ = {
        'create-diagram':   (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_STRING, )),        'add-element':   (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_STRING,)),
    }
        
    def Redraw(self):
        """
        This function load Project Add menu from enabled diagrams and options of elements
        
        """
        for item in self.mItemAddDiagram_menu.get_children():
            self.mItemAddDiagram_menu.remove(item)
        
        for item in self.mItemAddElement_menu.get_children():
            self.mItemAddElement_menu.remove(item)

        for item in self.application.GetProject().GetElementFactory().GetElement().values():
            if ('DirectAdd', 'true') in item.GetOptions().items():
                newItem = gtk.ImageMenuItem(item.GetId())
                self.mItemAddElement_menu.append(newItem)
                newItem.connect("activate", self.on_mnuAddElement_activate, item.GetId())
                img = gtk.Image()
                img.set_from_pixbuf(PixmapFromPath(self.application.GetProject().GetStorage(), self.application.GetProject().GetElementFactory().GetElement(item.GetId()).GetIcon()))
                newItem.set_image(img)
                img.show()
                newItem.show()
        
        for diagram in self.application.GetProject().GetVersion().GetDiagrams():
            newItem = gtk.ImageMenuItem(diagram)
            newItem.connect("activate", self.on_mnuDiagrams_activate, diagram)
            img = gtk.Image()
            img.set_from_pixbuf(PixmapFromPath(self.application.GetProject().GetStorage(), self.application.GetProject().GetDiagramFactory().GetDiagram(diagram).GetIcon()))
            img.show()
            newItem.set_image(img)
            self.mItemAddDiagram_menu.append(newItem)
            newItem.show()
        
    def on_mnuDiagrams_activate(self, widget, diagramId):
        self.emit('create-diagram', diagramId)
        
    def on_mnuAddElement_activate(self, widget, element):
        self.emit('add-element', element)
        
