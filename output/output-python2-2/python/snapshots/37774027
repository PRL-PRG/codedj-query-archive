import pygtk
import gtk
import common

from tbToolBox import CtbToolBox
from twProjectView import CtwProjectView
from mnuItems import CmnuItems
from picDrawingArea import CpicDrawingArea
from nbProperties import CnbProperties

class CfrmMain(common.CWindow):
    name = 'frmMain'
    widgets = ('hboxWorkSpace', 'mnuUseCaseDiagram', 
        'twProjectView', 'lwProperties')
    
    complexWidgets = (CtbToolBox, CtwProjectView, CmnuItems, CpicDrawingArea, CnbProperties)
    
    def Init(self):
        #self.mnuItems.connect('create_diagram', self.on_mnuItems_create_diagram)
        #self.picDrawingArea.connect('get_selected', self.on_picDrawingArea_get_selected)
        #self.picDrawingArea.connect('set_selected', self.on_picDrawingArea_set_selected)
        #self.picDrawingArea.connect('selected_item', self.on_picDrawingArea_selected_item)
        #self.nbProperties.connect('content_update', self.on_nbProperties_content_update)
        self.mnuItems.LoadDiagramsMenu()
        
        self.form.maximize()
    
    # ** Main menu **
    # File
    @common.event("form", "destroy")
    def on_frmMain_destroy(self, frm):
        self.ActionQuit(frm)
    
    def on_mnuNew_activate(self, mnu):
        pass
        
    def on_mnuOpen_activate(self, mnu):
        pass
        
    def on_mnuSave_activate(self, mnu):
        pass
        
    def on_mnuSaveAs_activate(self, mnu):
        pass
        
    def on_mnuQuit_activate(self, mnu):
        self.ActionQuit(mnu)
    
    # Edit
    def on_mnuCut_activate(self, mnu):
        pass
        
    def on_mnuCopy_activate(self, mnu):
        pass
        
    def on_mnuPaste_activate(self, mnu):
        pass
        
    def on_mnuDelete_activate(self, mnu):
        pass
        
    # Diagrams
    def on_mnuViewTools_activate(self, mnu):
        self.tbToolBox.SetVisible(mnu.get_active())
            
    # View
    def on_mnuClassDiahram_activate(self, mnu):
        pass
        
    def on_mnuUseCaseDiahram_activate(self, mnu):
        pass
        
    # Help
    def on_mnuAbout_activate(self, mnu):
        tmp = self.application.GetWindow('frmAbout')
        tmp.SetParent(self)
        tmp.Show()
        
    # Actions
    def ActionQuit(self, widget):
        self.application.Quit()
    
    def ActionNew(self, widget):
        pass
        
    def ActionOpen(self, widget):
        pass
    
    def ActionLoadToolBar(self, widget):
        pass
    
    # Moje vlastne signale
    def on_mnuItems_create_diagram(self, widget, diagramId):
        self.tbToolBox.SetButtons(diagramId)
        
    def on_picDrawingArea_get_selected(self, widget):
        return self.tbToolBox.GetSelected()
        
    def on_picDrawingArea_set_selected(self, widget, selected):
        self.tbToolBox.SetSelected(selected)
        
    def on_picDrawingArea_selected_item(self, widget, selected):
        self.nbProperties.Fill(selected)
    
    def on_nbProperties_content_update(self, widget, element, property):
        if element.GetObject().GetType().HasVisualAttribute(property):
            self.picDrawingArea.Paint()
