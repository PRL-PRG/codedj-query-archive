from common import CWidget
from lib.Projekt import CProjekt, CProjectNode
from lib.Elements import CElementFactory, CElementObject
from lib.Drawing import CElement
from gtk.gdk import pixbuf_new_from_file
from lib.consts import ELEMENT_IMAGE, VIEW_IMAGE, FOLDER_IMAGE

import gtk
import gtk.gdk

class CtwProjectView(CWidget):
    name = 'twProjectView'
    widgets = ('twProjectView', )
    
    def Init(self):
        # vytvorime si model
        self.TreeStore = gtk.TreeStore(str, gtk.gdk.Pixbuf)
        # ikonky
        self.icons = {  'View' : pixbuf_new_from_file(VIEW_IMAGE),
                        'Element' : pixbuf_new_from_file(ELEMENT_IMAGE),
                        'Folder' : pixbuf_new_from_file(FOLDER_IMAGE) }
        
        parent = self.TreeStore.append(None)
        self.TreeStore.set(parent, 0, 'Untitled', 1, self.icons['View'])
        x = self.TreeStore.append(parent)
        self.TreeStore.set(x, 0, 'Logical View', 1, self.icons['Folder'] )
        y = self.TreeStore.append(x)
        self.TreeStore.set(y, 0, 'Elements', 1, self.icons['Folder'] )
       
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
        #povolenie oznacit viac prvkov
        self.twProjectView.get_selection().set_mode(gtk.SELECTION_MULTIPLE)
        
        #projekt view, pametova reprezentacia
        self.Project = CProjekt()
        #hlavny uzol = package
        pckg = CElementObject( self.application.ElementFactory.GetElement('Package') )
        pckg.SetAttribute('Name', 'Untitled') #defaultne meno projektu
        project = CProjectNode(None, pckg)    
        self.Project.SetRoot(project)
        #syn projektu = logical view
        pckg1 = CElementObject( self.application.ElementFactory.GetElement('Package') )
        pckg1.SetAttribute('Name', 'Logical View')
        logicalView = CProjectNode(None, pckg1)
        self.Project.AddNode(logicalView, project)
        #syn logical view = elements
        pckg2 = CElementObject( self.application.ElementFactory.GetElement('Package') )
        pckg2.SetAttribute('Name', 'Elements')
        elements = CProjectNode(None, None)
        self.Project.AddNode(elements, logicalView)
    
    def AddElement(self, element):
        pass
        
    def GetSelected(self):
        pass