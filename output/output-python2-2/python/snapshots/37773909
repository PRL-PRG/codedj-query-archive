from common import CWidget
from lib.Projekt import CProjekt, CProjectNode
from lib.Elements import CElementFactory, CElementObject
from lib.Drawing import CElement
from gtk.gdk import pixbuf_new_from_file
from lib.consts import ELEMENT_IMAGE, VIEW_IMAGE, FOLDER_IMAGE
from lib.lib import UMLException

from common import  event
import common
import gobject

import gtk
import gtk.gdk

class CtwProjectView(CWidget):
    name = 'twProjectView'
    widgets = ('twProjectView', )
    
    __gsignals__ = {
        'selected_drawing_area':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_PYOBJECT,) 
            ), 
        'selected-item-tree':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, 
            (gobject.TYPE_PYOBJECT, )), 
    }
    
    def __init__(self, app, wTree):
        # vytvorime si model
        self.TreeStore = gtk.TreeStore(str, gtk.gdk.Pixbuf, str, object)
        CWidget.__init__(self, app, wTree)
        # ikonky
        self.icons = {  'View' : pixbuf_new_from_file(VIEW_IMAGE),
                        'Element' : pixbuf_new_from_file(ELEMENT_IMAGE),
                        'Folder' : pixbuf_new_from_file(FOLDER_IMAGE) }
        
        #projekt view, pametova reprezentacia
        self.Project = CProjekt()
       
        #vytvorenie hlavneho uzla a nastavenie korena projektu
        pckg = CElementObject( self.application.ElementFactory.GetElement('Package') )
        pckg.SetAttribute('Name', 'Untitled') #defaultne meno projektu
        project = CProjectNode(None, pckg)    
        project.SetPath("Untitled:Package")
        self.Project.SetRoot(project)
        
        
        parent = self.TreeStore.append(None)
        self.TreeStore.set(parent, 0, 'Untitled', 1, self.icons['View'], 2, 'Package', 3, project)
         
        
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

    
    
    def AddElement(self, element, path):
        parent = self.Project.GetNode(path)
        node = CProjectNode(parent, element)
        node.SetPath(parent.GetPath() + "/" + node.GetName() + ":" + node.GetType())
        self.Project.AddNode(node, parent)
        
        novy = self.TreeStore.append(self.get_iter_from_path(self.twProjectView.get_model(), self.twProjectView.get_model().get_iter_root() ,path))
        self.TreeStore.set(novy, 0, element.GetName() , 1, self.icons['Folder'], 2, element.GetType().GetId(),3,node)
        
    
    
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
        self.TreeStore.set(novy, 0, drawingArea.GetName() , 1, self.icons['Element'], 2, '=DrawingArea=',3,drawingArea)

    
    def UpdateElement(self, object):
        iter = self.get_iter_from_path(self.twProjectView.get_model(),self.twProjectView.get_model().get_iter_root() ,object.GetPath())
        node = self.twProjectView.get_model().get(iter,3)[0]
        node.Change()        
        self.TreeStore.set_value(iter, 0, object.GetName())
    
    
        
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
        iter = treeView.get_selection().get_selected()[1]
        if treeView.get_model().get(iter,2)[0] == "=DrawingArea=":
            return
        self.emit('selected-item-tree',treeView.get_model().get(iter,3)[0])
        
