from lib.Depend.gtk2 import gtk

import common
from lib.config import config
from common import event

class CfrmOptions(common.CWindow):
    #widgets = ('labelOptions',)
    widgets = ('cbElementLine', 'cbElementFill', 'cbElementFill2', 'cbElementFill3', 'cbElementShadow', 'cbElementNameText', 'cbElementText', 'fbElementNameText','fbElementText' ,'cbConnectionLine', 'cbConnectionArrow', 'cbConnectionArrowFill', 'cbConnectionNameText', 'cbConnectionText', 'fbConnectionNameText', 'fbConnectionText', 'sbSelectionPointsSize', 'cbSelectionPoints', 'cbSelectionRectangle' ,'sbSelectionRectangleWidth', 'cbDragRectangle', 'sbDragRectangleWidth', 'txtRootPath', 'txtTemplatesPath', 'txtImagesPath', 'txtGuiPath', 'txtLocalesPath', 'txtUserDirPath', 'txtUserConfigDirPath', 'txtRecentFilesPath', 'expElement', 'expSelection', 'expConnection', 'expDrag')
    name = 'frmOptions'
    
    def GtkColorToStr(self, color):
        return '#%02x%02x%02x'%(color.red >> 8, color.green >> 8, 
            color.blue >> 8)


    def Show(self):
        #self.labelAbout.get_use_markup(True)
        #nText = self.__GetAboutText()
        #self.labelAbout.get_label(nText)
        #self.form.get_transient_for(parent.form)
        
        if self.form.run() == gtk.RESPONSE_OK:
            #config['/Styles/Element/LineColor'] = self.cbElementLine.get_color()
            #(gtk.gdk.color_parse(config['/Styles/Element/NameTextColor'])
            config['/Styles/Element/LineColor'] = self.GtkColorToStr(self.cbElementLine.get_color())
            config['/Styles/Element/FillColor'] = self.GtkColorToStr(self.cbElementFill.get_color())
            config['/Styles/Element/Fill2Color'] = self.GtkColorToStr(self.cbElementFill2.get_color())
            config['/Styles/Element/Fill3Color'] = self.GtkColorToStr(self.cbElementFill3.get_color())
            config['/Styles/Element/ShadowColor'] = self.GtkColorToStr(self.cbElementShadow.get_color())
            config['/Styles/Element/NameTextColor'] = self.GtkColorToStr(self.cbElementNameText.get_color())
            config['/Styles/Element/TextColor'] = self.GtkColorToStr(self.cbElementText.get_color())
            config['/Styles/Connection/LineColor'] = self.GtkColorToStr(self.cbConnectionLine.get_color())
            config['/Styles/Connection/ArrowColor'] = self.GtkColorToStr(self.cbConnectionArrow.get_color())
            config['/Styles/Connection/ArrowFillColor'] = self.GtkColorToStr(self.cbConnectionArrowFill.get_color())
            config['/Styles/Connection/NameTextColor'] = self.GtkColorToStr(self.cbConnectionNameText.get_color())
            config['/Styles/Connection/TextColor'] = self.GtkColorToStr(self.cbConnectionText.get_color())
            config['/Styles/Selection/PointsColor'] = self.GtkColorToStr(self.cbSelectionPoints.get_color())
            config['/Styles/Selection/RectangleColor'] = self.GtkColorToStr(self.cbSelectionRectangle.get_color())
            config['/Styles/Drag/RectangleColor'] = self.GtkColorToStr(self.cbDragRectangle.get_color())
            config['/Styles/Element/NameTextFont'] = self.fbElementNameText.get_font_name()
            config['/Styles/Element/TextFont'] = self.fbElementText.get_font_name()
            config['/Styles/Connection/NameTextFont'] = self.fbConnectionNameText.get_font_name()
            config['/Styles/Connection/TextFont'] = self.fbConnectionText.get_font_name()
            config['/Styles/Selection/PointsSize'] = self.sbSelectionPointsSize.get_value_as_int()
            config['/Styles/Selection/RectangleWidth'] = self.sbSelectionRectangleWidth.get_value_as_int()
            config['/Styles/Drag/RectangleWidth'] = self.sbDragRectangleWidth.get_value_as_int()
            config['/Paths/Root'] = self.txtRootPath.get_text()
            config['/Paths/Templates'] = self.txtTemplatesPath.get_text()
            config['/Paths/Images'] = self.txtImagesPath.get_text()
            config['/Paths/Gui'] = self.txtGuiPath.get_text()
            config['/Paths/Locales'] = self.txtLocalesPath.get_text()
            config['/Paths/UserDir'] = self.txtUserDirPath.get_text()
            config['/Paths/UserConfig'] = self.txtUserConfigDirPath.get_text()
            config['/Paths/RecentFiles'] = self.txtRecentFilesPath.get_text()

        self.Hide()
        
    @event("expElement", "activate")
    @event("expConnection", "activate")
    @event("expSelection", "activate")
    @event("expDrag", "activate")
    def on_exapander_activate(self, widget):
        if widget is self.expElement:
            #self.expElement.set_expanded(False)
            self.expConnection.set_expanded(False)
            self.expSelection.set_expanded(False)
            self.expDrag.set_expanded(False)
        if widget is self.expConnection:
            self.expElement.set_expanded(False)
            #self.expConnection.set_expanded(False)
            self.expSelection.set_expanded(False)
            self.expDrag.set_expanded(False)
        if widget is self.expSelection:
            self.expElement.set_expanded(False)
            self.expConnection.set_expanded(False)
            #self.expSelection.set_expanded(False)
            self.expDrag.set_expanded(False)
        if widget is self.expDrag:
            self.expElement.set_expanded(False)
            self.expConnection.set_expanded(False)
            self.expSelection.set_expanded(False)
            #self.expDrag.set_expanded(False)
