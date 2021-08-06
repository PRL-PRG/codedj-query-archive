import common
import gtk

class CfrmAttribute(common.CWindow):
    name = 'frmAttribute'
    widgets = ('edAtrName', 'cboxAtrType', 'cboxAtrScope', 'cboxAtrStereotype',
               'cboxAtrContainment', 'edAtrInitial', 'txtAtrDocumentation',
               'cbAtrDerived', 'cbAtrStatic', 'cbAtrProperty', 'cbAtrConst')
    
    def ShowFrmAttribute(self, attribute = {}):
        if 'name' in attribute:
            self.edAtrName.set_text(attribute['name'])
        else:
            self.edAtrName.set_text("")
        if 'type' in attribute:
            self.cboxAtrType.child.set_text(attribute['type'])
        else:
            self.cboxAtrType.child.set_text("")
        if 'scope' in attribute:
            self.cboxAtrScope.set_active({'public':0, 'private':1, 'protected':2}[attribute['scope']])
        else:
            self.cboxAtrScope.set_active(0)
        if 'stereotype' in attribute:
            self.cboxAtrStereotype.child.set_text(attribute['stereotype'])
        else:
            self.cboxAtrStereotype.child.set_text("")
        if 'containment' in attribute:
            self.cboxAtrContainment.child.set_text(attribute['containment'])
        else:
            self.cboxAtrContainment.child.set_text("")
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
            attribute['scope'] = ['public', 'private', 'protected'][self.cboxAtrScope.get_active()]
            attribute['stereotype'] = self.cboxAtrStereotype.child.get_text()
            attribute['containment'] = self.cboxAtrContainment.child.get_text()
            attribute['initial'] = self.edAtrInitial.get_text()
            buf = self.txtAtrDocumentation.get_buffer()
            attribute['doc'] = buf.get_text(buf.get_start_iter(), buf.get_end_iter())
            attribute['derived'] = self.cbAtrDerived.get_active()
            attribute['static'] = self.cbAtrStatic.get_active()
            attribute['property'] = self.cbAtrProperty.get_active()
            attribute['const'] = self.cbAtrStatic.get_active()
            ret = True
        self.Hide()
        return ret
