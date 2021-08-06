from common import event
import common
import gtk

class CfrmAttribute(common.CWindow):
    name = 'frmAttribute'
    widgets = ('edAtrName', 'cboxAtrType', 'cboxAtrScope', 'cboxAtrStereotype',
               'cboxAtrContainment', 'edAtrInitial', 'txtAtrDocumentation',
               'cbAtrDerived', 'cbAtrStatic', 'cbAtrProperty', 'cbAtrConst')
    
    def __init__(self, app, wTree):
        common.CWindow.__init__(self, app, wTree)
        self.operations = {}
        self.attribute = {}
            
    def ShowFrmAttribute(self, attribute = {}):
        self.operations['append'] = None
        self.operations['remove'] = None
        self.attribute = attribute
        if not self.attribute.has_key('getter'):
            self.attribute['getter'] = ""
        if not self.attribute.has_key('setter'):
            self.attribute['setter'] = ""
        
        self.FillType()
        if 'name' in attribute:
            self.edAtrName.set_text(attribute['name'])
        else:
            self.edAtrName.set_text("")
        if 'type' in attribute:
            self.cboxAtrType.child.set_text(attribute['type'])
        else:
            self.cboxAtrType.child.set_text("")
        if 'scope' in attribute:
            self.cboxAtrScope.set_active({'private':0, 'protected':1, 'public':2}[attribute['scope']])
        else:
            self.cboxAtrScope.set_active(0)
        if 'stereotype' in attribute:
            self.cboxAtrStereotype.child.set_text(attribute['stereotype'])
        else:
            self.cboxAtrStereotype.child.set_text("")
        if 'containment' in attribute:
            self.cboxAtrContainment.set_active(attribute['containment'])
        else:
            self.cboxAtrContainment.set_active(0)
        if 'initial' in attribute:
            self.edAtrInitial.set_text(attribute['initial'])
        else:
            self.edAtrInitial.set_text("")
        if 'doc' in attribute:
            self.txtAtrDocumentation.get_buffer().set_text(attribute['doc'])
        else:
            self.txtAtrDocumentation.get_buffer().set_text("")
        if 'derived' in attribute:
            self.cbAtrDerived.set_active(attribute['derived'])
        else:
            self.cbAtrDerived.set_active(False)
        if 'static' in attribute:
            self.cbAtrStatic.set_active(attribute['static'])
        else:
            self.cbAtrStatic.set_active(False)
        if 'property' in attribute:
            self.cbAtrProperty.set_active(attribute['property'])
        else:
            self.cbAtrProperty.set_active(False)
        if 'const' in attribute:
            self.cbAtrConst.set_active(attribute['const'])
        else:
            self.cbAtrConst.set_active(False)
        
        while True:
            response = self.form.run()
            if response != gtk.RESPONSE_OK:
                break
            if self.edAtrName.get_text().strip() != '' and \
                    self.cboxAtrType.child.get_text().strip() != '':
                break
            msg = gtk.MessageDialog(message_format = _("Fill the name and type fields"), parent = self.form, type = gtk.MESSAGE_ERROR,
                    buttons = gtk.BUTTONS_OK)
            msg.run()
            msg.destroy()
        ret = False
        if response == gtk.RESPONSE_OK:
            attribute['name'] = self.edAtrName.get_text()
            attribute['type'] = self.cboxAtrType.child.get_text()
            attribute['scope'] = ['private', 'protected', 'public'][self.cboxAtrScope.get_active()]
            attribute['stereotype'] = self.cboxAtrStereotype.child.get_text()
            attribute['containment'] = self.cboxAtrContainment.get_active()
            attribute['initial'] = self.edAtrInitial.get_text()
            buf = self.txtAtrDocumentation.get_buffer()
            attribute['doc'] = buf.get_text(buf.get_start_iter(), buf.get_end_iter())
            attribute['derived'] = self.cbAtrDerived.get_active()
            attribute['static'] = self.cbAtrStatic.get_active()
            attribute['property'] = self.cbAtrProperty.get_active()
            attribute['const'] = self.cbAtrStatic.get_active()
            attribute['setter'] = self.attribute['setter']
            attribute['getter'] = self.attribute['getter']
            ret = True
        self.Hide()
        if ret and (self.operations['append'] is not None or self.operations['remove'] is not None):
            return self.operations
        return ret

    def FillType(self):
        model = self.cboxAtrType.get_model()
        model.clear()
        self.cboxAtrType.set_model(model) 
        
        for i in self.application.GetProject().GetDataTypeFactory().GetDataType(self.application.GetProject().GetActualLanguage()).GetDataTypes(): 
            self.cboxAtrType.append_text(i)
        
        for i in self.application.GetProject().GetDataTypeFactory().GetDataType("own").GetDataTypes():
            self.cboxAtrType.append_text(i)
    
    
    @event("cbAtrProperty","clicked")
    def on_cbAtrProperty_activate(self, togglebutton):
        if self.edAtrName.get_text() == "":
            self.cbAtrProperty.set_property('active',False)
            
        if self.cbAtrProperty.get_property('active'):
            if self.attribute['getter'] != "" or self.attribute['setter'] != "":
                return
            self.attribute['getter'] = "Get" + self.edAtrName.get_text() + "():" + self.cboxAtrType.child.get_text()
            if self.cboxAtrType.child.get_text() != "":
                self.attribute['setter'] = "Set" + self.edAtrName.get_text() + "(" + self.edAtrName.get_text() + ":" + self.cboxAtrType.child.get_text() +")"
            else:
                self.attribute['setter'] = "Set" + self.edAtrName.get_text() + "(" + self.edAtrName.get_text() + ")"
            dlg = self.application.GetWindow('frmCreateProperty')
            dlg.SetParent(self)
            opers = dlg.ShowDialog(self.attribute['getter'], self.attribute['setter'])
            if opers[0] is not None:
                self.attribute['getter'] = opers[1][0]
                self.attribute['setter'] = opers[1][1]
                self.operations['append'] = opers[0]
        else:
            self.operations['remove'] = []
            if self.attribute['getter'] != "":
                self.operations['remove'].append(self.attribute['getter'])
            if self.attribute['setter'] != "":
                self.operations['remove'].append(self.attribute['setter'])
            self.attribute['setter'] = ""
            self.attribute['getter'] = ""