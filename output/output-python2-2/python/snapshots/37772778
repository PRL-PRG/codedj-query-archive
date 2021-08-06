import gtk, gobject

from lib.Drawing.Canvas.Gtk import PixmapFromPath

from common import CWidget, event

class CmnuItems(CWidget):
    name = 'mnuItems'
    widgets = ('mItemAddDiagram_menu', 'mnuSelectLanguage_menu',)
    
    __gsignals__ = {
        'create-diagram':   (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, 
            (gobject.TYPE_STRING, ))
    }
        
    def Redraw(self):
        for item in self.mItemAddDiagram_menu.get_children():
            self.mItemAddDiagram_menu.remove(item)
        
        for item in self.mnuSelectLanguage_menu.get_children():
            self.mnuSelectLanguage_menu.remove(item)
        
        for diagram in self.application.GetProject().GetVersion().GetDiagrams():
            newItem = gtk.ImageMenuItem(diagram)
            newItem.connect("activate", self.on_mnuDiagrams_activate, diagram)
            img = gtk.Image()
            img.set_from_pixbuf(PixmapFromPath(self.application.GetProject().GetStorage(), self.application.GetProject().GetDiagramFactory().GetDiagram(diagram).GetIcon()))
            img.show()
            newItem.set_image(img)
            self.mItemAddDiagram_menu.append(newItem)
            newItem.show()
        
        for id, language in enumerate(self.application.GetProject().GetDataTypeFactory().GetLanguages()):
            if language == "own":
                continue
            newItem = gtk.RadioMenuItem(None,language)
            if len(self.mnuSelectLanguage_menu.get_children()) > 0:
                newItem.set_group(self.mnuSelectLanguage_menu.get_children()[0])
            newItem.connect("activate", self.on_mnuLanguages_activate, language)
            if language == self.application.GetProject().GetActualLanguage():
                newItem.set_property("active",True)
            self.mnuSelectLanguage_menu.append(newItem)
            newItem.show()
        
    def on_mnuLanguages_activate(self, widget, language):
        self.application.GetProject().SetActualLanguage(language)
        
    def on_mnuDiagrams_activate(self, widget, diagramId):
        self.emit('create-diagram', diagramId)
        
