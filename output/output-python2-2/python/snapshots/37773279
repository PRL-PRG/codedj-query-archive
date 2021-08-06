from common import CWidget
import gtk
import gtk.gdk
import gobject

from common import  event
from lib.Drawing import CDrawingArea
from lib.Drawing.Canvas.Gtk import PixmapFromPath
from twProjectView import CtwProjectView
from picDrawingArea import CpicDrawingArea

class CTabs(CWidget):
    name = 'nbTabs'
    widgets = ('nbTabs','twProjectView', 'picDrawingArea',
                #Context menu
                'menuTreeElement',
                'mnuTab', 'mnuTabExportSVG', 'mnuTabPages_menu', 'mnuTabCloseDiagram', 'mnuTabCloseAllDiagram',
                'mnuTabShowInProjectView',)
    
    __gsignals__ = {
        'change_current_page':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_PYOBJECT,) 
            ),
        'drawingArea-set-focus': (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, ()),
        'export-svg-from-TabMenu': (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, ()),
        'show-area-in-project': (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_PYOBJECT,)),
    }
    
    def __init__(self, app, wTree):
        CWidget.__init__(self, app, wTree)
        area = CDrawingArea(None,'StartPage')
        self.drawingAreas = [area]
        
        self.mnuTabExportSVG.set_sensitive(False)
        self.mnuTabCloseDiagram.set_sensitive(False)
        self.mnuTabCloseAllDiagram.set_sensitive(False)
        self.mnuTabShowInProjectView.set_sensitive(False)
        
        mi = gtk.RadioMenuItem(None,self.drawingAreas[0].GetName()) 
        mi.set_active(True)
        mi.show()   
        mi.connect("toggled", self.on_mnuTab_activate, self.drawingAreas[0])
        self.mnuTabPages_menu.remove(self.mnuTabPages_menu.get_children()[0])
        self.mnuTabPages_menu.append(mi)
        
    def AddTab(self, drawingArea):       
        for i in self.drawingAreas:
            if i is drawingArea:
                self.SetCurrentPage(self.drawingAreas.index(drawingArea))
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
        img.set_from_pixbuf(PixmapFromPath(self.application.Project.GetStorage(), drawingArea.GetType().GetIcon()))
        img.show()
        
        hboxbut.add(img)
        hboxbut.add(label1)
        hboxbut.add(button)
        self.nbTabs.append_page(hbox,hboxbut)
        button.connect("clicked", self.on_button_click, self.nbTabs.get_nth_page(self.nbTabs.get_n_pages()-1))
        self.drawingAreas.append(drawingArea)
       
        #~ mi = gtk.RadioMenuItem(None,i.GetName())  
        #~ mi.set_group(self.mnuTabPages_menu.get_children()[0])
        #~ mi.set_active(True)        
        #~ mi.show()   
        #~ mi.connect("activate", self.on_mnuTab_activate, i)
        #~ self.mnuTabPages_menu.append(mi)
        self.SetCurrentPage(self.nbTabs.get_n_pages()-1)
       
    def on_button_click(self, widget, page):
        self.CloseTab(self.drawingAreas[self.nbTabs.page_num(page)])

    @event("nbTabs", "switch-page")
    def on_change_current_page(self, notebook, page, page_num):   
        self.drawingAreas[page_num].DeselectAll()
        if page_num  == 0:
            self.emit("change_current_page", None)
            self.mnuTabExportSVG.set_sensitive(False)
            self.mnuTabCloseDiagram.set_sensitive(False)
            self.mnuTabShowInProjectView.set_sensitive(False)
            if len(self.drawingAreas) == 1:
                self.mnuTabCloseAllDiagram.set_sensitive(False)
        else:
            self.emit("change_current_page", self.drawingAreas[page_num])
            self.mnuTabExportSVG.set_sensitive(True)
            self.mnuTabCloseDiagram.set_sensitive(True)
            self.mnuTabCloseAllDiagram.set_sensitive(True)
            self.mnuTabShowInProjectView.set_sensitive(True)
           
    def CloseTab(self, drawingArea):
        if drawingArea in self.drawingAreas:
            num = self.drawingAreas.index(drawingArea)
            self.drawingAreas.remove(drawingArea)
            #self.mnuTabPages_menu.remove(self.mnuTabPages_menu.get_children()[num])
            self.nbTabs.remove_page(num)
            
    
    def CloseCurrentTab(self):
        if self.nbTabs.get_current_page() > 0:
            self.CloseTab(self.drawingAreas[self.nbTabs.get_current_page()])
    
    def NextTab(self):
        if len(self.drawingAreas) == self.nbTabs.get_current_page() + 1:
            self.SetCurrentPage(0)
        else:
            self.nbTabs.next_page()
            self.emit("drawingArea-set-focus")
    
    def PreviousTab(self):
        if self.nbTabs.get_current_page() == 0:
            self.SetCurrentPage(len(self.drawingAreas)-1)
        else:
            self.nbTabs.prev_page()
            if self.nbTabs.get_current_page() == 0:
                return
        self.emit("drawingArea-set-focus")
    
    def SetCurrentPage(self, page): 
        if page <= len(self.drawingAreas)-1:
            self.nbTabs.set_current_page(page)
    
    def CloseAll(self):
        for i in xrange(1, len(self.drawingAreas)):
            del self.drawingAreas[1]
            self.nbTabs.remove_page(1)


    
    
    def on_mnuTab_activate(self, widget, diagram):
        for id, a in enumerate(self.drawingAreas):
            if diagram is a:
                break
        else:
            return
        if self.nbTabs.get_current_page() != id:
            self.SetCurrentPage(id)
    
    @event("nbTabs","button-press-event")
    def button_clicked(self, widget, event):
        if event.button == 3:
            for i in self.mnuTabPages_menu.get_children():
                self.mnuTabPages_menu.remove(i)
                
            for id, i in enumerate(self.drawingAreas):
                mi = gtk.RadioMenuItem(None,i.GetName())  
                if id > 0:
                    mi.set_group(self.mnuTabPages_menu.get_children()[0])      
                mi.show()   
                mi.connect("toggled", self.on_mnuTab_activate, i)
                self.mnuTabPages_menu.append(mi)
            
            self.mnuTabPages_menu.get_children()[self.nbTabs.get_current_page()].set_property("active",True)
            self.mnuTab.popup(None,None,None,event.button,event.time)
    
    @event("mnuTabCloseDiagram", "activate")
    def on_mnuTabCloseDiagram_activate(self, menuItem):
        if self.nbTabs.get_current_page() == 0:
            return
        else:
            self.CloseTab(self.drawingAreas[self.nbTabs.get_current_page()])
    
    @event("mnuTabCloseAllDiagram", "activate")
    def on_mnuTabCloseAllDiagram_activate(self, menuItem):
        self.CloseAll()
    
    @event("mnuTabShowInProjectView","activate")
    def on_mnuTabShowInProjectView_activate(self, menuItem):
        self.emit('show-area-in-project',self.drawingAreas[self.nbTabs.get_current_page()])
    
    @event("mnuTabExportSVG", "activate")
    def on_mnuTabExportSVG_activate(self, menuItem):
        if self.nbTabs.get_current_page() == 0:
            return
        else:
            self.emit("export-svg-from-TabMenu")