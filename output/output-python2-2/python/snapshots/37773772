import pygtk
import gtk
from common import CWindow, event
import common

from lib.Elements import CElementObject
from lib.Drawing import CDrawingArea
from tbToolBox import CtbToolBox
from twProjectView import CtwProjectView
from mnuItems import CmnuItems
from picDrawingArea import CpicDrawingArea
from nbProperties import CnbProperties
from tabs import CTabs

class CfrmMain(CWindow):
    name = 'frmMain'
    widgets = ('hboxWorkSpace', 'mnuUseCaseDiagram', 
        'twProjectView', 'lwProperties', 
        #mItemFile
        'mnuOpen', 'mnuSave', 'mnuSaveAs', 'mnuQuit',
        #mItemEdit
        'mnuCut', 'mnuCopy', 'mnuPaste', 'mnuDelete', 
        #mItemDiagrams
        #mItemView
        'mnuViewTools', 
        #mItemHelp
        'mnuAbout',
        #tabs
        'nbTabs',
        #toolbar
        'cmdOpen', 'cmdSave',
        )
    
    complexWidgets = (CtbToolBox, CtwProjectView, CmnuItems, CpicDrawingArea, CnbProperties, CTabs)
    
    def __init__(self, app, wTree):
        CWindow.__init__(self, app, wTree)
        
        self.form.maximize()
    
    # Diagrams
    @event("mnuViewTools", "activate")
    def on_mnuViewTools_activate(self, mnu):
        self.tbToolBox.SetVisible(mnu.get_active())
            
    
    # Help
    @event("mnuAbout", "activate")
    def on_mnuAbout_activate(self, mnu):
        tmp = self.application.GetWindow('frmAbout')
        tmp.SetParent(self)
        tmp.Show()
       
       
    # Actions
    @event("form", "destroy")
    @event("mnuQuit", "activate")
    def ActionQuit(self, widget):
        self.application.Quit()
        
    @event("cmdOpen", "clicked")
    @event("mnuOpen", "activate")
    def ActionOpen(self, widget):
        filename, copy = self.application.GetWindow("frmOpen").ShowDialog()
        if filename is not None:
            print filename
        
    @event("cmdSave", "clicked")
    @event("mnuSave", "activate")
    def ActionSave(self, widget):
        filename = self.application.GetWindow("frmSave").ShowDialog()
        if filename is not None:
            self.twProjectView.Project.SaveProject(filename)
    
    def ActionLoadToolBar(self, widget):
        pass
    
    
    
    # Moje vlastne signale
    @event("picDrawingArea", "add-element")
    def on_add_element(self, widget, Element, drawingArea):
        self.twProjectView.AddElement(Element, drawingArea.GetPath())
    
    
    
    @event("mnuItems", "create-diagram")
    @event("twProjectView","create-diagram")
    def on_mnuItems_create_diagram(self, widget, diagramId):      
        area = CDrawingArea(self.application.DiagramFactory.GetDiagram(diagramId), "New " + diagramId)
        self.twProjectView.AddDrawingArea(area)
        self.nbTabs.AddTab(area)
        self.picDrawingArea.SetDrawingArea(area)
        self.tbToolBox.SetButtons(diagramId)
   
    @event("picDrawingArea", "get-selected")
    def on_picDrawingArea_get_selected(self, widget):
        return self.tbToolBox.GetSelected()


    @event("twProjectView", "selected_drawing_area")
    def on_select_drawing_area(self, widget, drawingArea):
        self.nbTabs.AddTab(drawingArea)
        self.picDrawingArea.SetDrawingArea(drawingArea)
      
      
    @event("nbTabs", "change_current_page")
    def on_change_drawing_area(self, widget, drawingArea):
        if drawingArea is None:
            self.picDrawingArea.Hide()
            self.tbToolBox.SetButtons(None)
        else:
            self.picDrawingArea.Show()
            self.picDrawingArea.SetDrawingArea(drawingArea)
            self.tbToolBox.SetButtons(drawingArea.GetType().GetId())
        
    @event("picDrawingArea", "set-selected")
    def on_picDrawingArea_set_selected(self, widget, selected):
        self.tbToolBox.SetSelected(selected)
                
    @event("picDrawingArea", "selected-item")
    def on_picDrawingArea_selected_item(self, widget, selected):
        self.nbProperties.Fill(selected)
        
    @event("twProjectView", "selected-item-tree")
    def on_twTreeView_selected_item(self, widget, selected):
        self.picDrawingArea.DrawingArea.DeselectAll()
        self.picDrawingArea.Paint()
        self.nbProperties.Fill(selected) 
    
    @event("twProjectView", "repaint")
    def on_repaint_picDravingArea(self, widget):
        self.picDrawingArea.Paint()

    
    @event("nbProperties", "content-update")
    def on_nbProperties_content_update(self, widget, element, property):
        if element.GetObject().GetType().HasVisualAttribute(property):
            self.picDrawingArea.Paint()
            self.twProjectView.UpdateElement(element.GetObject())
