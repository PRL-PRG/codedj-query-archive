import common
import gtk
from lib.config import config
from common import event

class CfrmOptions(common.CWindow):
    #widgets = ('labelOptions',)
    widgets = ('cbElementLine', 'cbElementFill', 'cbElementFill2', 'cbElementFill3', 'cbElementShadow', 'cbElementNameText', 'cbElementText', 'fbElementNameText', 'cbConnectionLine', 'cbConnectionArrow', 'cbConnectionArrowFill', 'cbConnectionNameText', 'cbConnectionText', 'fbConnectionNameText', 'fbConnectionText', 'sbSelectionPointsSize', 'cbSelectionPoints', 'cbSelectionRectangle' ,'sbSelectionRectangleWidth', 'cbDragRectangle', 'sbDragRectangleWidth', 'txtRootPath', 'txtTemplatesPath', 'txtImagesPath', 'txtGuiPath', 'txtLocalesPath', 'txtUserDirPath', 'txtUserConfigDirPath', 'txtRecentFilesPath', 'expElement', 'expSelection', 'expConnection', 'expDrag')
    name = 'frmOptions'
    
    def Show(self):
        #self.labelAbout.get_use_markup(True)
        #nText = self.__GetAboutText()
        #self.labelAbout.get_label(nText)
        #self.form.get_transient_for(parent.form)
        
        if self.form.run() == gtk.RESPONSE_OK:
            #config['/Styles/Element/LineColor'] = self.cbElementLine.get_color()
            #(gtk.gdk.color_parse(config['/Styles/Element/NameTextColor'])
            tmpStr = self.cbElementLine.get_color().to_string()
            config['/Styles/Element/LineColor'] = tmpStr[0:3] + tmpStr[5:7] + tmpStr[9:11]
            tmpStr = self.cbElementFill.get_color().to_string()
            config['/Styles/Element/FillColor'] = tmpStr[0:3] + tmpStr[5:7] + tmpStr[9:11]
            tmpStr = self.cbElementFill2.get_color().to_string()
            config['/Styles/Element/Fill2Color'] = tmpStr[0:3] + tmpStr[5:7] + tmpStr[9:11]
            tmpStr = self.cbElementFill3.get_color().to_string()
            config['/Styles/Element/Fill3Color'] = tmpStr[0:3] + tmpStr[5:7] + tmpStr[9:11]
            tmpStr = self.cbElementShadow.get_color().to_string()
            config['/Styles/Element/ShadowColor'] = tmpStr[0:3] + tmpStr[5:7] + tmpStr[9:11]
            tmpStr = self.cbElementNameText.get_color().to_string()
            config['/Styles/Element/NameTextColor'] = tmpStr[0:3] + tmpStr[5:7] + tmpStr[9:11]
            tmpStr = self.cbElementText.get_color().to_string()
            config['/Styles/Element/TextColor'] = tmpStr[0:3] + tmpStr[5:7] + tmpStr[9:11]
            tmpStr = self.cbConnectionLine.get_color().to_string()
            config['/Styles/Connection/LineColor'] = tmpStr[0:3] + tmpStr[5:7] + tmpStr[9:11]
            tmpStr = self.cbConnectionArrow.get_color().to_string()
            config['/Styles/Connection/ArrowColor'] = tmpStr[0:3] + tmpStr[5:7] + tmpStr[9:11]
            tmpStr = self.cbConnectionArrowFill.get_color().to_string()
            config['/Styles/Connection/ArrowFillColor'] = tmpStr[0:3] + tmpStr[5:7] + tmpStr[9:11]
            tmpStr = self.cbConnectionNameText.get_color().to_string()
            config['/Styles/Connection/NameTextColor'] = tmpStr[0:3] + tmpStr[5:7] + tmpStr[9:11]
            tmpStr = self.cbConnectionText.get_color().to_string()
            config['/Styles/Connection/TextColor'] = tmpStr[0:3] + tmpStr[5:7] + tmpStr[9:11]
            tmpStr = self.cbSelectionPoints.get_color().to_string()
            config['/Styles/Selection/PointsColor'] = tmpStr[0:3] + tmpStr[5:7] + tmpStr[9:11]
            tmpStr = self.cbSelectionRectangle.get_color().to_string()
            config['/Styles/Selection/RectangleColor'] = tmpStr[0:3] + tmpStr[5:7] + tmpStr[9:11]
            tmpStr = self.cbDragRectangle.get_color().to_string()
            config['/Styles/Drag/RectangleColor'] = tmpStr[0:3] + tmpStr[5:7] + tmpStr[9:11]
            config['/Styles/Element/NameTextFont'] = self.fbElementNameText.get_font_name()
            config['/Styles/Element/NameTextFont'] = self.fbElementNameText.get_font_name()
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
        if widget is not self.expElement:
            #self.expElement.set_expanded(False)
            self.expConnection.set_expanded(False)
            self.expSelection.set_expanded(False)
            self.expDrag.set_expanded(False)
        if widget is not self.expConnection:
            self.expElement.set_expanded(False)
            #self.expConnection.set_expanded(False)
            self.expSelection.set_expanded(False)
            self.expDrag.set_expanded(False)
        if widget is not self.expSelection:
            self.expElement.set_expanded(False)
            self.expConnection.set_expanded(False)
            #self.expSelection.set_expanded(False)
            self.expDrag.set_expanded(False)
        if widget is not self.expDrag:
            self.expElement.set_expanded(False)
            self.expConnection.set_expanded(False)
            self.expSelection.set_expanded(False)
            #self.expDrag.set_expanded(False)
