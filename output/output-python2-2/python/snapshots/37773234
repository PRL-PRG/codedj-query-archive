from common import CWindow, event
from lib.Drawing.Canvas.Gtk import PixmapFromPath
import gtk
import gobject
import common

class CFindInDiagram(CWindow):
    name = 'frmFindInDiagram'
    
    widgets = ("twFindInDiagram", )
    
    __gsignals__ = {
        'selected_drawingArea_and_Element': (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_PYOBJECT, gobject.TYPE_PYOBJECT)), 
    }
    
    
    def __init__(self, app, wTree):
        common.CWindow.__init__(self, app, wTree)
        
        self.listStore = gtk.ListStore(gobject.TYPE_STRING, gobject.TYPE_STRING, gtk.gdk.Pixbuf )
        self.twFindInDiagram.set_model(self.listStore)
        self.twFindInDiagram.append_column(gtk.TreeViewColumn('', gtk.CellRendererPixbuf(), pixbuf = 2))
        self.twFindInDiagram.append_column(gtk.TreeViewColumn(_('Diagram name'), gtk.CellRendererText(), text = 0))
        self.twFindInDiagram.append_column(gtk.TreeViewColumn(_('Diagram type'), gtk.CellRendererText(), text = 1))
        
        
    def Fill(self):
        self.listStore.clear()
        for i in self.drawingAreas:
            iter = self.listStore.append()
            self.listStore.set(iter,0,i.GetName(), 1, i.GetType().GetId(), 2, PixmapFromPath(self.application.Project.GetStorage(), i.GetType().GetIcon()))
    
    def ShowDialog(self, drawingAreas, object):
        self.drawingAreas = drawingAreas
        self.object = object
        self.Fill()
        response = self.form.run()
        while True:
            if response != gtk.RESPONSE_OK:
                    self.form.hide()
                    return
            if response == gtk.RESPONSE_OK:
                iter = self.twFindInDiagram.get_selection().get_selected()[1]
                if iter is not None:
                    self.form.hide()
                    return self.emit('selected_drawingArea_and_Element',self.drawingAreas[self.twFindInDiagram.get_model().get_path(iter)[0]],self.object)
    
    def hide(self):
        del self.drawingAreas
        del self.object
        self.Hide()
    
    @event("twFindInDiagram", "row-activated")
    def on_twFindInDiagram_doubleclick(self, treeView, path, column):
        self.emit('selected_drawingArea_and_Element',self.drawingAreas[path[0]],self.object)
        
    