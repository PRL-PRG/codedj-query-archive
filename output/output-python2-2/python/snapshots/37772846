import pygtk
import gtk
from common import CWindow, event
import common
import lib.consts

import os.path

from lib.Drawing import CElement
from lib.Elements import CElementObject
from dialogs import CWarningDialog, CQuestionDialog, ECancelPressed
from lib.Drawing import CDrawingArea
from tbToolBox import CtbToolBox
from twProjectView import CtwProjectView
from mnuItems import CmnuItems
from picDrawingArea import CpicDrawingArea
from nbProperties import CnbProperties
from tabs import CTabs
from frmFindInDiagram import CFindInDiagram
from tabStartPage import CtabStartPage
from lib.lib import UMLException

class CfrmMain(CWindow):
    name = 'frmMain'
    widgets = ('hboxWorkSpace', 'mnuUseCaseDiagram',
        #menu
        #############
        'mItemFile',
        'mnuOpen', 'mnuSave', 'mnuSaveAs', 'mnuQuit',
        #############
        'mItemEdit',
        'mnuCut', 'mnuCopy', 'mnuPaste', 'mnuDelete',
        #############
        'mItemProject',
        #############
        'mItemDiagram',
        'mnuExportSvg',
        #############
        'mItemView',
        'mnuViewTools',
        #############
        'mItemHelp',
        'mnuAbout',
        #############
        'mItemElement',
        'mmShift_SendBack', 'mmShift_BringForward', 'mmShift_ToBottom', 'mmShift_ToTop',
        #############
        #toolbar
        'cmdOpen', 'cmdSave', 'cmdCopy', 'cmdCut', 'cmdPaste',
        )

    complexWidgets = (CtbToolBox, CtwProjectView, CmnuItems, CpicDrawingArea, CnbProperties, CTabs,
                      CtabStartPage, CFindInDiagram)

    def __init__(self, app, wTree):
        CWindow.__init__(self, app, wTree)

        self.form.maximize()
        
        self.__sensitivity_project = None
        self.UpdateMenuSensitivity(project = False)
        
        self.ReloadTitle()
        
        
    def SetSensitiveMenuChilds(self, MenuItem, value):
        for i in MenuItem.get_submenu().get_children():
            i.set_sensitive(value)
    
    def UpdateMenuSensitivity(self, project = None, diagram = None, element = None):
        if self.__sensitivity_project is None:
            self.__sensitivity_project = [True, True, True]
        changes = 0
        if project is not None:
            if not project:
                diagram = False
            if project != self.__sensitivity_project[0]:
                changes += 1
            self.__sensitivity_project[0] = project
        else:
            project = self.__sensitivity_project[0]
        if diagram is not None:
            if not diagram:
                element = False
            if diagram != self.__sensitivity_project[1]:
                changes += 1
            self.__sensitivity_project[1] = diagram
        else:
            diagram = self.__sensitivity_project[1]
        if element is not None:
            if element != self.__sensitivity_project[2]:
                changes += 1
            self.__sensitivity_project[2] = element
        else:
            element = self.__sensitivity_project[2]
        
        if changes == 0:
            return
        
        self.SetSensitiveMenuChilds(self.mItemProject, project)
        self.SetSensitiveMenuChilds(self.mItemDiagram, diagram)
        self.SetSensitiveMenuChilds(self.mItemElement, element)
        self.mnuSave.set_sensitive(project)
        self.mnuSaveAs.set_sensitive(project)
        self.cmdSave.set_sensitive(project)
        self.cmdCopy.set_sensitive(element)
        self.cmdCut.set_sensitive(element)
        self.cmdPaste.set_sensitive(diagram)
        self.mnuSave.set_sensitive(project)
        self.mnuCopy.set_sensitive(element)
        self.mnuCut.set_sensitive(element)
        self.mnuPaste.set_sensitive(diagram)
        self.mnuDelete.set_sensitive(element)
    
    def LoadProject(self, filename, copy):
        self.application.ProjectInit()
        try:
            self.application.GetProject().LoadProject(filename, copy)
        except Exception:
            if lib.consts.DEBUG:
                raise
            self.application.GetRecentFiles().RemoveFile(filename)
            self.application.ProjectDelete()
            self.nbTabs.CloseAll()
            self.twProjectView.ClearProjectView()
            self.ReloadTitle()
            self.nbProperties.Fill(None)
            return CWarningDialog(self.form, _('Error opening file')).run()
            
        self.ReloadTitle()
        self.nbTabs.CloseAll()
        self.twProjectView.Redraw()
        self.mnuItems.Redraw()
        self.nbProperties.Fill(None)
        self.picDrawingArea.Redraw()
        self.UpdateMenuSensitivity(project = True)
        
    # Diagrams
    @event("mnuViewTools", "activate")
    def on_mnuViewTools_activate(self, mnu):
        self.tbToolBox.SetVisible(mnu.get_active())

    # Help
    @event("tabStartPage","open-about-dialog")
    @event("mnuAbout", "activate")
    def on_mnuAbout_activate(self, mnu):
        tmp = self.application.GetWindow('frmAbout')
        tmp.SetParent(self)
        tmp.Show()
    
    @event('nbTabs','export-svg-from-TabMenu')
    @event('mnuExportSvg', 'activate')
    def on_mnuExportSvg_activate(self, widget):
        filedlg = gtk.FileChooserDialog(_('Choose SVG file'), self.form, gtk.FILE_CHOOSER_ACTION_SAVE, (gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_SAVE,gtk.RESPONSE_OK))
        filter = gtk.FileFilter()
        filter.set_name("SVG vector images")
        filter.add_pattern('*.svg')
        filedlg.add_filter(filter)
        try:
            while True:
                if filedlg.run() == gtk.RESPONSE_OK:
                    filename = filedlg.get_filename()
                    if '.' not in os.path.basename(filename):
                        filename += '.svg'
                    if not os.path.isdir(filename):
                        self.picDrawingArea.ExportSvg(filename)
                        return
                else:
                    return
        finally:
            filedlg.destroy()
    
    def ReloadTitle(self):
        if self.application.GetProject() is None or self.application.GetProject().GetFileName() is None:
            self.form.set_title(_('UML .FRI designer'))
        else:
            self.form.set_title(_('UML .FRI designer [%s]')%self.application.GetProject().GetFileName())

    # Actions
    @event("form", "delete-event")
    @event("mnuQuit", "activate")
    def ActionQuit(self, widget, event = None):
        try:
            if self.application.GetProject() is not None and CQuestionDialog(self.form, _('Do you want to save project?'), True).run():
                self.ActionSave(widget)
        except ECancelPressed:
            return True
        self.application.Quit()
    
    @event("tabStartPage","open-file")
    def on_open_file(self, widget, filename):
        if filename is not None:
            try:
                if self.application.GetProject() is not None and CQuestionDialog(self.form, _('Do you want to save project?'), True).run():
                    self.ActionSave(widget)
            except ECancelPressed:
                return
            self.LoadProject(filename, False)
            self.tabStartPage.Fill()

    @event("tabStartPage","open-project")
    @event("cmdOpen", "clicked")
    @event("mnuOpen", "activate")
    def ActionOpen(self, widget,tab = 0):
        filename, copy = self.application.GetWindow("frmOpen").ShowDialog(self,tab)
        if filename is not None:
            try:
                if self.application.GetProject() is not None and CQuestionDialog(self.form, _('Do you want to save project?'), True).run():
                    self.ActionSave(widget)
            except ECancelPressed:
                return
            self.LoadProject(filename, copy)
            self.tabStartPage.Fill()
    
    @event("form", "key-press-event")
    def on_key_press_event(self, widget, event):
        if event.keyval in (gtk.keysyms.Tab, gtk.keysyms.ISO_Left_Tab):
            if event.state == (gtk.gdk.CONTROL_MASK | gtk.gdk.SHIFT_MASK):
                self.nbTabs.PreviousTab()
                self.form.emit_stop_by_name('key-press-event')
            elif event.state == gtk.gdk.CONTROL_MASK:
                self.nbTabs.NextTab()
                self.form.emit_stop_by_name('key-press-event')
        if event.state == gtk.gdk.CONTROL_MASK | gtk.gdk.SHIFT_MASK:
            if event.keyval == gtk.keysyms.F4:
                self.nbTabs.CloseAll()
        if event.state == gtk.gdk.CONTROL_MASK:
            if event.keyval  in (gtk.keysyms.F4, gtk.keysyms.w):
                self.nbTabs.CloseCurrentTab()
        if event.state == gtk.gdk.MOD1_MASK:
            Keys = [gtk.keysyms._1, gtk.keysyms._2, gtk.keysyms._3, gtk.keysyms._4, gtk.keysyms._5, 
                    gtk.keysyms._6, gtk.keysyms._7, gtk.keysyms._8, gtk.keysyms._9, gtk.keysyms._0]
            if event.keyval in Keys:
                self.nbTabs.SetCurrentPage(Keys.index(event.keyval))

    @event("nbTabs","drawingArea-set-focus")
    def on_DrawingArea_set_focus(self,widget):
        self.picDrawingArea.SetFocus()
    
    @event("cmdSave", "clicked")
    @event("mnuSave", "activate")
    def ActionSave(self, widget):
        if self.application.GetProject().GetFileName() is None:
            self.ActionSaveAs(widget)
            self.tabStartPage.Fill()
        else:
            self.application.GetProject().SaveProject()

    @event("mnuSaveAs", "activate")
    def ActionSaveAs(self, widget):
        filename = self.application.GetWindow("frmSave").ShowDialog(self)
        if filename is not None:
            self.application.GetProject().SaveProject(filename)
            self.ReloadTitle()

    @event("mnuDelete","activate")
    def on_mnuDelete_click(self, widget):
        self.picDrawingArea.DeleteElements()
    
    @event("cmdCut", "clicked")
    @event("mnuCut","activate")
    def on_mnuCut_click(self, widget):
        self.picDrawingArea.ActionCut()
    
    @event("cmdCopy", "clicked")
    @event("mnuCopy","activate")
    def on_mnuCopy_click(self, widget):
        self.picDrawingArea.ActionCopy()
    
    @event("cmdPaste", "clicked")
    @event("mnuPaste","activate")
    def on_mnuPaste_click(self, widget):
        try:
            self.picDrawingArea.ActionPaste()
        except UMLException, e:
            if e.GetName() == "ElementAlreadyExists":
                return CWarningDialog(self.form, _('Unable to insert element')).run()
            elif e.GetName() == "DiagramHaveNotThisElement":
                return CWarningDialog(self.form, _('Wrong element: ') + e.GetParam(0).GetObject().GetType().GetId()).run()
    
    def ActionLoadToolBar(self, widget):
        pass

    # Moje vlastne signale
    @event("picDrawingArea", "add-element")
    def on_add_element(self, widget, Element, drawingArea):
        self.twProjectView.AddElement(Element, drawingArea)



    @event("mnuItems", "create-diagram")
    @event("twProjectView","create-diagram")
    def on_mnuItems_create_diagram(self, widget, diagramId):
        area = CDrawingArea(self.application.GetProject().GetDiagramFactory().GetDiagram(diagramId))
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

    @event("twProjectView", "close-drawing-area")
    def on_remove_drawing_area(self, widget, drawingArea):
        self.nbTabs.CloseTab(drawingArea)

    @event("nbTabs", "change_current_page")
    def on_change_drawing_area(self, widget, drawingArea):
        if drawingArea is None:
            self.picDrawingArea.Hide()
            self.tbToolBox.SetButtons(None)
            self.UpdateMenuSensitivity(diagram = False)
        else:
            self.picDrawingArea.Show()
            self.picDrawingArea.SetDrawingArea(drawingArea)
            self.tbToolBox.SetButtons(drawingArea.GetType().GetId())
            self.UpdateMenuSensitivity(diagram = True)
    
    @event("nbTabs","show-area-in-project")
    def on_show_area_in_project(self, widget, drawingArea):
        self.twProjectView.ShowDrawingArea(drawingArea)
    
    @event("picDrawingArea", "set-selected")
    def on_picDrawingArea_set_selected(self, widget, selected):
        self.tbToolBox.SetSelected(selected)

    @event("picDrawingArea", "selected-item")
    def on_picDrawingArea_selected_item(self, widget, selected):
        self.UpdateMenuSensitivity(element = len(selected) > 0)
        if len(selected) == 1:
            self.nbProperties.Fill(selected[0])
        else:
            self.nbProperties.Fill(None)

    @event("picDrawingArea","delete-element-from-all")
    def on_picDrawingArea_delete_selected_item(self, widget, selected):
        self.twProjectView.DeleteElement(selected)

    @event("twProjectView", "selected-item-tree")
    def on_twTreeView_selected_item(self, widget, selected):
        self.picDrawingArea.DrawingArea.DeselectAll()
        self.picDrawingArea.Paint()
        self.nbProperties.Fill(selected)

    @event("twProjectView", "repaint")
    def on_repaint_picDravingArea(self, widget):
        self.picDrawingArea.Paint()
    
    @event("frmFindInDiagram","selected_drawingArea_and_Element")
    @event("twProjectView","selected_drawing_area_and_select_element")
    def on_select_area_and_element(self, widget, drawingArea, object):
        self.picDrawingArea.SetDrawingArea(drawingArea)
        self.nbTabs.AddTab(drawingArea)
        drawingArea.AddToSelection(drawingArea.HasElementObject(object))
        self.picDrawingArea.Paint()
    
    @event("twProjectView","show_frmFindInDiagram")
    def on_show_frmFindInDiagram(self, widget, drawingAreas, object):
        self.frmFindInDiagram.ShowDialog(drawingAreas, object)

    @event("nbProperties", "content-update")
    def on_nbProperties_content_update(self, widget, element, property):
        if element.GetObject().GetType().HasVisualAttribute(property):
            self.picDrawingArea.Paint()
            self.twProjectView.UpdateElement(element.GetObject())

    @event("tbToolBox", "toggled")
    def on_tbToolBox_toggled(self, widget, ItemId, ItemType):
        self.picDrawingArea.ResetAction()
        
    @event("picDrawingArea","drop-from-treeview")
    def on_drop_from_treeview(self, widget, position):
        node = self.twProjectView.GetSelectedNode()
        if node is not None:
            drawingArea = self.picDrawingArea.GetDrawingArea()
            try:
                Element = CElement(drawingArea, node.GetObject()).SetPosition(position)
            except UMLException, e:
                if e.GetName() == "ElementAlreadyExists":
                    return CWarningDialog(self.form, _('Unable to insert element')).run()
                elif e.GetName() == "DiagramHaveNotThisElement":
                    return CWarningDialog(self.form, _('Wrong element: ') + node.GetObject().GetType().GetId()).run()
            
    
    @event("picDrawingArea", "run-dialog")
    def on_run_dialog(self, widget, type, message):
        if type == 'warning':
            return CWarningDialog(self.form, message).run()
        else:
            pass
    
    
    @event("picDrawingArea","show-element-in-treeView")
    def on_show_element_in_treeView(self, widget, Element):
        self.twProjectView.ShowElement(Element)
    
    @event("picDrawingArea","open-specification")
    def on_show_open_specification(self, widget, Element):
        tmp = self.application.GetWindow('frmProperties')
        tmp.SetParent(self.application.GetWindow('frmMain'))
        tmp.ShowProperties('',Element)
        self.picDrawingArea.Paint()
    
    #Z-Order 
# 'mmShift_SendBack', 'mmShift_BringForward', 'mmShift_ToBottom', 'mmShift_ToTop'    
    @event("mmShift_SendBack", "activate")
    def on_mnuItems_mmShift_SendBack(self, menuItem):
        self.picDrawingArea.Shift_activate('SendBack')
        
    @event("mmShift_BringForward", "activate")
    def on_mnuItems_mmShift_BringForward(self, menuItem):
        self.picDrawingArea.Shift_activate('BringForward')
        
    @event("mmShift_ToBottom", "activate")
    def on_mnuItems_mmShift_ToBottom(self, menuItem):
        self.picDrawingArea.Shift_activate('ToBottom')
        
    @event("mmShift_ToTop", "activate")
    def on_mnuItems_mmShift_ToTop(self, menuItem):
        self.picDrawingArea.Shift_activate('ToTop')        
