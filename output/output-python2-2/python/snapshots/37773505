from common import CWidget
from lib.Project import CProject, CProjectNode
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
        'create-diagram':   (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_STRING, )),
        'repaint':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, ()),
        'close-drawing-area': (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_PYOBJECT,))
    }
    
    def __init__(self, app, wTree):
        CWidget.__init__(self, app, wTree)
        
        # vytvorime si model
        self.TreeStore = gtk.TreeStore(str, gtk.gdk.Pixbuf, str, object)
        self.EventButton = (0,0)
        
        for item in self.mnuTreeAddDiagram.get_children():
            self.mnuTreeAddDiagram.remove(item)
        
        for diagram in self.application.Project.GetDiagramFactory():
            mi = gtk.ImageMenuItem(diagram.GetId())
            
            img = gtk.Image()
            img.set_from_pixbuf(PixmapFromPath(self.application.Project.GetStorage(), diagram.GetIcon()))
            img.show()
            
            mi.set_image(img)
            mi.show()   
            mi.connect("activate", self.on_mnuTreeAddDiagram_activate, diagram.GetId())
            self.mnuTreeAddDiagram.append(mi)
        
        #projekt view, pametova reprezentacia
        #vytvorenie hlavneho uzla a nastavenie korena projektu
        #~ pckg = CElementObject( self.application.Project.GetElementFactory().GetElement('Package') )
        #~ pckg.SetAttribute('Name', 'Untitled') #defaultne meno projektu
        #~ project = CProjectNode(None, pckg, "Untitled:Package" )    
        #~ self.application.Project.SetRoot(project)
        
        #~ parent = self.TreeStore.append(None)
        #~ self.TreeStore.set(parent, 0, 'Untitled', 1, PixmapFromPath(None, VIEW_IMAGE), 2, 'Package', 3, project)
         
        
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
        #~ self.twProjectView.get_selection().select_iter(parent)
        
        self.TARGETS = [
        ('MY_TREE_MODEL_ROW', gtk.TARGET_SAME_WIDGET, 0),
        ('text/plain', 0, 1),
        ('TEXT', 0, 2),
        ('STRING', 0, 3),
        ]
        
        self.twProjectView.enable_model_drag_source(gtk.gdk.BUTTON1_MASK, self.TARGETS, gtk.gdk.ACTION_DEFAULT | gtk.gdk.ACTION_MOVE | gtk.gdk.ACTION_COPY)
        self.twProjectView.enable_model_drag_dest(self.TARGETS, gtk.gdk.ACTION_DEFAULT)
        

    
    def Redraw(self):
        
        project = self.application.Project
        root = project.GetRoot()
        self.TreeStore.clear()
        parent = self.TreeStore.append(None)
        self.TreeStore.set(parent, 0, root.GetName(), 1, PixmapFromPath(self.application.Project.GetStorage(), root.GetObject().GetType().GetIcon()), 2, root.GetType(), 3, root)
        self.__DrawTree(root, parent)
    
    
    def __DrawTree(self, root, parent):
        
        for area in root.GetDrawingAreas():
            novy = self.TreeStore.append(parent)
            self.TreeStore.set(novy, 0, area.GetName() , 1, PixmapFromPath(self.application.Project.GetStorage(), area.GetType().GetIcon()), 2, '=DrawingArea=',3,area)
        
        for node in root.GetChilds():
            novy = self.TreeStore.append(parent)
            self.TreeStore.set(novy, 0, node.GetName() , 1, PixmapFromPath(self.application.Project.GetStorage(), node.GetObject().GetType().GetIcon()), 2, node.GetType(),3,node)
            self.__DrawTree(node, novy)
            
         

    def get_iter_from_path(self, model, root, path):
        chld = root
        
        i = path.split('/')[0]
        j,k = i.split(':')
        name, type = model.get(root, 0, 2)
        
        if len(path.split('/')) == 1 and name == j and type == k:
            return root
            
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


    def get_iters_from_path(self, model, root, path):
        chld = root
        iter = []
        
        i = path.split('/')[0]
        j,k = i.split(':')
        name, type = model.get(root, 0, 2)
        endName, endType = path.split('/')[-1].split(':')
        
        if len(path.split('/')) == 1 and name == j and type == k:
            return [root]
        
        if name == j and type == k:
            def rekurzia(root,path):
                j, k = path.split('/')[0].split(':')
                for id in xrange(model.iter_n_children(root)):
                    chld = model.iter_nth_child(root, id)
                    name, type = model.get(chld, 0, 2)
                    
                    if k == "=DrawingArea=":
                        iter.append(root) 
                    
                    if name == j and type == k:
                        if len(path.split('/')) > 1:
                            rekurzia(chld,path.split('/',1)[1])
                        else:
                            iter.append(chld)
            
            rekurzia(root,path.split('/',1)[1])                        

        else:
            raise UMLException("BadPath4")
        return iter
        
        
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
            raise UMLException("BadPath5")
    
    
    def AddElement(self, element, drawingArea):
        path = drawingArea.GetPath()
        parent = self.application.Project.GetNode(path)
        node = CProjectNode(parent, element, parent.GetPath() + "/" + element.GetName() + ":" + element.GetType().GetId())
        node.AddAppears(drawingArea)
        self.application.Project.AddNode(node, parent)
        novy = self.TreeStore.append(self.get_iter_from_path(self.twProjectView.get_model(), self.twProjectView.get_model().get_iter_root() ,path))
        self.TreeStore.set(novy, 0, element.GetName() , 1, PixmapFromPath(self.application.Project.GetStorage(), element.GetType().GetIcon()), 2, element.GetType().GetId(),3,node)
        
    
    
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
        self.TreeStore.set(novy, 0, drawingArea.GetName() , 1, PixmapFromPath(self.application.Project.GetStorage(), drawingArea.GetType().GetIcon()), 2, '=DrawingArea=',3,drawingArea)
        path = self.TreeStore.get_path(novy)
        self.twProjectView.expand_to_path(path)
        self.twProjectView.get_selection().select_iter(novy)
        
    
    def UpdateElement(self, object):
        if isinstance(object, CElementObject):
            for iter in self.get_iters_from_path(self.twProjectView.get_model(),self.twProjectView.get_model().get_iter_root() ,object.GetPath()):
                node = self.twProjectView.get_model().get(iter,3)[0]
                if object is node.GetObject():
                    break
        node.Change()  
        
        model = self.twProjectView.get_model()
        print model.get(iter,3)[0].GetName()," Name"
        self.TreeStore.set_value(iter, 0, object.GetName())
        print iter
    
    
    @event("twProjectView","button-press-event")
    def button_clicked(self, widget, event):
        self.EventButton = (event.button, event.time) 
        
        
    
    @event("twProjectView", "row-activated")
    def on_twProjectView_set_selected(self, treeView, path, column):
        model = self.twProjectView.get_model()
        iter =  model.get_iter(path)
        if model.get(iter,2)[0] == "=DrawingArea=":
            print model.get(iter,3)[0].GetPath()
            area = model.get(iter,3)[0]
            if area is None:
                raise UMLException("None")
            else:
                self.emit('selected_drawing_area',area)
        else:
            print model.get(iter,3)[0].GetPath()
    
    @event("twProjectView", "cursor-changed")
    def on_twProjectView_change_selection(self, treeView):
        
        iter = treeView.get_selection().get_selected()[1]
        
        if self.EventButton[0] == 3:
            self.mnuTreeDelete.set_sensitive(len(treeView.get_model().get_path(iter)) > 1)
            self.menuTreeElement.popup(None,None,None,self.EventButton[0],self.EventButton[1])
            
        
        if treeView.get_model().get(iter,2)[0] == "=DrawingArea=":
            return
        
        self.emit('selected-item-tree',treeView.get_model().get(iter,3)[0])
            
    
    def on_mnuTreeAddDiagram_activate(self, widget, diagramId):
        self.emit('create-diagram', diagramId)
    
    def RemoveFromArea(self,node):
        for i in node.GetDrawingAreas():
            self.emit('close-drawing-area',i)
            
        for i in node.GetChilds():
            self.RemoveFromArea(i)
        
        for i in node.GetAppears():
            i.DeleteObject(node.GetObject())
    
    def DeleteElement(self, elementObject):
        iter = self.twProjectView.get_model().get_iter_root()
        if elementObject is self.twProjectView.get_model().get(iter,3)[0].GetObject():
            return
        
        for iter in self.get_iters_from_path(self.twProjectView.get_model(),self.twProjectView.get_model().get_iter_root() ,elementObject.GetPath()):
            node = self.twProjectView.get_model().get(iter,3)[0]
            if elementObject is node.GetObject():
                break
        self.TreeStore.remove(iter)
        self.RemoveFromArea(node)
        self.application.Project.RemoveNode(node)
    
    
    @event("mnuTreeDelete","activate")
    def on_mnuTreeDelete_activate(self, menuItem):
        iter = self.twProjectView.get_selection().get_selected()[1]
        model = self.twProjectView.get_model()
        if model.get(iter,2)[0] != "=DrawingArea=":
            node = model.get(iter,3)[0]
            self.TreeStore.remove(iter)
            self.RemoveFromArea(node)
            self.application.Project.RemoveNode(node)
            self.emit('repaint')
        else:
            area = model.get(iter,3)[0]
            itr = model.iter_parent(iter)
            node = model.get(itr,3)[0]
            node.RemoveDrawingArea(area)
            self.TreeStore.remove(iter)
            self.emit('close-drawing-area',area)



    def GetSelectedNode(self):
        iter = self.twProjectView.get_selection().get_selected()[1]
        node = self.twProjectView.get_model().get(iter,3)[0]
        if isinstance(node,CProjectNode):
            return node
        else:
            return None
        
        
        
    @event("twProjectView","drag-data-get")
    def on_drag_data_get(self, widget,drag_context, selection_data, info, time):
        treeselection = widget.get_selection()
        model, iter = treeselection.get_selected()
        data = model.get_value(iter, 0)
        selection_data.set(selection_data.target, 8, data)

        
    
    def CheckSanity(self, model, iter_to_copy, target_iter):
        path_of_iter_to_copy = model.get_path(iter_to_copy)
        path_of_target_iter = model.get_path(target_iter)
        if path_of_target_iter[0:len(path_of_iter_to_copy)] == path_of_iter_to_copy:
            return False
        elif len(path_of_target_iter) < 2:
            return False
        else:
            return True
    
    
    def IterCopy(self, treeview, model, iter_to_copy, target_iter, pos):
        
        if treeview.get_model().get(iter_to_copy,2)[0] == "=DrawingArea=":
            node_to_copy = treeview.get_model().get(treeview.get_model().iter_parent(iter_to_copy),3)[0]
        else:
            node_to_copy = treeview.get_model().get(iter_to_copy,3)[0]
        if treeview.get_model().get(target_iter,2)[0] == "=DrawingArea=":
            target_node = treeview.get_model().get(treeview.get_model().iter_parent(target_iter),3)[0]
        else:
            target_node = treeview.get_model().get(target_iter,3)[0]
        
        if (pos == gtk.TREE_VIEW_DROP_INTO_OR_BEFORE) or (pos == gtk.TREE_VIEW_DROP_INTO_OR_AFTER):
            if treeview.get_model().get(target_iter,2)[0] == "=DrawingArea=":
                raise UMLException("MoveElementToDrawingArea")
            elif treeview.get_model().get(iter_to_copy,2)[0] == "=DrawingArea=":
                node_to_copy.MoveDrawingAreaToNewNode(target_node,treeview.get_model().get(iter_to_copy,3)[0])
            else:
                node_to_copy.MoveNode(target_node)
            new_iter = model.prepend(target_iter, None)
        
        elif pos == gtk.TREE_VIEW_DROP_BEFORE:
            if treeview.get_model().get(iter_to_copy,2)[0] == "=DrawingArea=":
                if target_node.GetParent() is not None:
                    target_node = target_node.GetParent()
                node_to_copy.MoveDrawingAreaToNewNode(target_node,treeview.get_model().get(iter_to_copy,3)[0])
            elif treeview.get_model().get(target_iter,2)[0] == "=DrawingArea=":
                node_to_copy.MoveNode(target_node)
            else:
                node_to_copy.MoveNode(target_node.GetParent())
            new_iter = model.insert_before(None, target_iter)
        
        elif pos == gtk.TREE_VIEW_DROP_AFTER:
            if treeview.get_model().get(iter_to_copy,2)[0] == "=DrawingArea=":
                if target_node.GetParent() is not None:
                    target_node = target_node.GetParent()
                node_to_copy.MoveDrawingAreaToNewNode(target_node,treeview.get_model().get(iter_to_copy,3)[0])
            elif treeview.get_model().get(target_iter,2)[0] == "=DrawingArea=":
                node_to_copy.MoveNode(target_node)
            else:
                node_to_copy.MoveNode(target_node.GetParent())
            new_iter = model.insert_after(None, target_iter)
                    
        for i in range(4):
            model.set_value(new_iter, i, model.get_value(iter_to_copy, i))
              
        if model.iter_has_child(iter_to_copy):
            for i in range(0, model.iter_n_children(iter_to_copy)):
                next_iter_to_copy = model.iter_nth_child(iter_to_copy, i)
                self.IterCopy(treeview, model, next_iter_to_copy, new_iter, gtk.TREE_VIEW_DROP_INTO_OR_BEFORE)
    
    
    
    @event("twProjectView","drag_data_received")
    def on_drag_data_received(self, widget, context, x, y, selection, info, etime):
        if widget.get_dest_row_at_pos(x, y) is not None:
            path, pos = widget.get_dest_row_at_pos(x, y)
            model, iter_to_copy = widget.get_selection().get_selected()
            target_iter = model.get_iter(path)
                       
            if self.CheckSanity(model, iter_to_copy, target_iter):
                try:
                    self.IterCopy(widget, model, iter_to_copy, target_iter, pos)
                except UMLException, e:
                    if e.GetName() == "MoveElementToDrawingArea":
                        context.finish(False, False, etime)
                        return
                context.finish(True, True, etime)
            else:
                context.finish(False, False, etime)