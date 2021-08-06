from common import CWidget
from lib.Projekt import CProjekt, CProjectNode
from lib.Elements import CElementFactory, CElementObject
from lib.Drawing import CElement
from lib.consts import VIEW_IMAGE
from lib.lib import UMLException
from lib.Drawing.Canvas.Gtk import PixmapFromPath

from common import  event
import common
import gobject

import gtk
import gtk.gdk

class CtwProjectView(CWidget):
    name = 'twProjectView'
    widgets = ('twProjectView','menuTreeElement', 'mnuTreeAddDiagram', 'mnuTreeDelete',)
    
    __gsignals__ = {
        'selected_drawing_area':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_PYOBJECT,)), 
        'selected-item-tree':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_PYOBJECT, )), 
        'create-diagram':   (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, 
            (gobject.TYPE_STRING, )),
        'repaint':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, () )
    }
    
    def __init__(self, app, wTree):
        CWidget.__init__(self, app, wTree)
        
        # vytvorime si model
        self.TreeStore = gtk.TreeStore(str, gtk.gdk.Pixbuf, str, object)
        
        self.EventButton = (0,0)
        
        for item in self.mnuTreeAddDiagram.get_children():
            self.mnuTreeAddDiagram.remove(item)
        
        for diagram in self.application.DiagramFactory:
            mi = gtk.ImageMenuItem(diagram.GetId())
            
            img = gtk.Image()
            img.set_from_pixbuf(PixmapFromPath(diagram.GetIcon()))
            img.show()
            
            mi.set_image(img)
            mi.show()   
            mi.connect("activate", self.on_mnuTreeAddDiagram_activate, diagram.GetId())
            self.mnuTreeAddDiagram.append(mi)
        
        #projekt view, pametova reprezentacia
        self.Project = CProjekt()
       
        #vytvorenie hlavneho uzla a nastavenie korena projektu
        pckg = CElementObject( self.application.ElementFactory.GetElement('Package') )
        pckg.SetAttribute('Name', 'Untitled') #defaultne meno projektu
        project = CProjectNode(None, pckg)    
        project.SetPath("Untitled:Package")
        self.Project.SetRoot(project)
        
        
        parent = self.TreeStore.append(None)
        self.TreeStore.set(parent, 0, 'Untitled', 1, PixmapFromPath(VIEW_IMAGE), 2, 'Package', 3, project)
         
        
        #spravime jeden column
        self.Column = gtk.TreeViewColumn('Elements')
        self.twProjectView.append_column(self.Column)
        self.twProjectView.set_reorderable(True)
        
        #nastavime renderer
        self.StrRenderer = gtk.CellRendererText()
        self.PbRenderer = gtk.CellRendererPixbuf()
        
        self.Column.pack_start(self.PbRenderer, False)
        self.Column.add_attribute(self.PbRenderer, 'pixbuf', 1)
        self.Column.pack_start(self.StrRenderer, True)
        self.Column.add_attribute(self.StrRenderer, 'text', 0)
            
        
        self.twProjectView.set_model(self.TreeStore)
        #povolenie oznacit jeden prvkov
        self.twProjectView.get_selection().set_mode(gtk.SELECTION_SINGLE)
        
        #oznacenie korena
        self.twProjectView.get_selection().select_iter(parent)

        
        
    
    def get_iter_from_path(self, model, root, path):
        chld = root
        
        i = path.split('/')[0]
        j,k = i.split(':')
        name, type = model.get(root, 0, 2)
        if name == j and type == k:
            for i in path.split('/')[1:]:
                j, k = i.split(':')
                for id in xrange(model.iter_n_children(root)):
                    chld = model.iter_nth_child(root, id)
                    name, type = model.get(chld, 0, 2)
                    
                    if k == "=DrawingArea=":
                        return root
                        
                    if name == j and type == k:
                        break 
                        
                root = chld
            return root
        else:
            raise UMLException("BadPath4")

    
    def get_area_from_path(self, model, root, path):
        chld = root
        
        i = path.split('/')[0]
        j,k = i.split(':')
        name, type = model.get(root, 0, 2)
        if name == j and type == k:
            for i in path.split('/')[1:]:
                j, k = i.split(':')
                for id in xrange(model.iter_n_children(root)):
                    chld = model.iter_nth_child(root, id)
                    name, type = model.get(chld, 0, 2)
                    
                    if k == "=DrawingArea=":
                        return model.get(chld, 3)[0]
                root = chld
        else:
            raise UMLException("BadPath4")
    
    
    def AddElement(self, element, path):
        
        parent = self.Project.GetNode(path)
        node = CProjectNode(parent, element)
        node.SetPath(parent.GetPath() + "/" + node.GetName() + ":" + node.GetType())
        if (path.split('/')[-1]).split(':')[-1] == "=DrawingArea=":
            node.AddDrawingArea(self.get_area_from_path(self.twProjectView.get_model(), self.twProjectView.get_model().get_iter_root() ,path))
        self.Project.AddNode(node, parent)
            
        novy = self.TreeStore.append(self.get_iter_from_path(self.twProjectView.get_model(), self.twProjectView.get_model().get_iter_root() ,path))
        self.TreeStore.set(novy, 0, element.GetName() , 1, PixmapFromPath(element.GetType().GetIcon()), 2, element.GetType().GetId(),3,node)
        
    
    
    def AddDrawingArea(self, drawingArea):
                
        iter = self.twProjectView.get_selection().get_selected()[1]
        
        if iter is None:
            iter = self.twProjectView.get_model().get_iter_root()
            self.twProjectView.get_selection().select_iter(iter)
            
        model = self.twProjectView.get_model()
        
        if model.get(iter,2)[0] == "=DrawingArea=":
            iter = model.iter_parent(iter)
        node = model.get(iter,3)[0]
        drawingArea.SetPath(node.GetPath() + "/" + drawingArea.GetName() + ":=DrawingArea=")
        node.AddDrawingArea(drawingArea)
        novy = self.TreeStore.append(iter)
        self.TreeStore.set(novy, 0, drawingArea.GetName() , 1, PixmapFromPath(drawingArea.GetType().GetIcon()), 2, '=DrawingArea=',3,drawingArea)

    
    def UpdateElement(self, object):
        iter = self.get_iter_from_path(self.twProjectView.get_model(),self.twProjectView.get_model().get_iter_root() ,object.GetPath())
        node = self.twProjectView.get_model().get(iter,3)[0]
        node.Change()        
        self.TreeStore.set_value(iter, 0, object.GetName())
    
    
    @event("twProjectView","button-press-event")
    def button_clicked(self, widget, event):
        self.EventButton = (event.button, event.time)       
     
    
    @event("twProjectView", "row-activated")
    def on_twProjectView_set_selected(self, treeView, path, column):
        model = self.twProjectView.get_model()
        iter =  model.get_iter(path)
        if model.get(iter,2)[0] == "=DrawingArea=":
            area = model.get(iter,3)[0]
            if area is None:
                raise UMLException("None")
            else:
                self.emit('selected_drawing_area',area)

    
    @event("twProjectView", "cursor-changed")
    def on_twProjectView_change_selection(self, treeView):
        if self.EventButton[0] == 3:
            self.menuTreeElement.popup(None,None,None,self.EventButton[0],self.EventButton[1])
            
        iter = treeView.get_selection().get_selected()[1]
        if treeView.get_model().get(iter,2)[0] == "=DrawingArea=":
            return
        self.emit('selected-item-tree',treeView.get_model().get(iter,3)[0])
    
    
    def on_mnuTreeAddDiagram_activate(self, widget, diagramId):
        self.emit('create-diagram', diagramId)
        
    
    
    @event("mnuTreeDelete","activate")
    def on_mnuTreeDelete_activate(self, menuItem):
        iter = self.twProjectView.get_selection().get_selected()[1]
        model = self.twProjectView.get_model()
        if model.get(iter,2)[0] != "=DrawingArea=":
            node = model.get(iter,3)[0]
            for i in node.GetDrawingAreas():
                i.DeleteObject(node.GetObject())
                self.emit('repaint')
                self.TreeStore.remove(iter)
                
            
            
        