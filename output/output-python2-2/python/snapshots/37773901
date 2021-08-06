from common import CWidget
import gtk
import gtk.gdk
import gobject
from common import  event
from lib.Drawing import CDrawingArea
from gtk.gdk import pixbuf_new_from_file
from lib.consts import DIAGRAMS

class CTabs(CWidget):
    name = 'nbTabs'
    widgets = ('nbTabs',)
    
    __gsignals__ = {
        'change_current_page':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_PYOBJECT,) 
            ),
    }
    
    def __init__(self, app, wTree):
        CWidget.__init__(self, app, wTree)
        area = CDrawingArea(None,'StartPage')
        self.drawingAreas = [area]
        
    def AddTab(self, drawingArea):
        for i in self.drawingAreas:
            if i is drawingArea:
                self.nbTabs.set_current_page(self.drawingAreas.index(drawingArea))
                return

        hbox = gtk.HBox()
        hbox.show()  
        
        hboxbut = gtk.HBox(spacing = 3)
        hboxbut.show()     

        button = gtk.Button()
        image = gtk.image_new_from_stock(gtk.STOCK_CLOSE, gtk.ICON_SIZE_SMALL_TOOLBAR)       
        image.show()
        button.add(image)
        button.set_relief(gtk.RELIEF_NONE)
        button.show()                
        label1 = gtk.Label(drawingArea.GetName())
        label1.show() 
        
        img = gtk.Image()
        img.set_from_pixbuf(pixbuf_new_from_file(DIAGRAMS[drawingArea.GetDiagram()]))
        img.show()
        
        hboxbut.add(img)
        hboxbut.add(label1)
        hboxbut.add(button)
        self.nbTabs.append_page(hbox,hboxbut)
        button.connect("clicked", self.on_button_click, self.nbTabs.get_nth_page(self.nbTabs.get_n_pages()-1))
        self.drawingAreas.append(drawingArea)
        self.nbTabs.set_current_page(self.nbTabs.get_n_pages()-1)
    
    def on_button_click(self, widget, page):
        number = self.nbTabs.page_num(page)
        self.drawingAreas.remove(self.drawingAreas[number])
        self.nbTabs.remove_page(number)

    @event("nbTabs", "switch-page")
    def on_change_current_page(self, notebook, page, page_num):
        if page_num  == 0:
            self.emit("change_current_page", None)
        else:
            self.emit("change_current_page", self.drawingAreas[page_num])
        
        
