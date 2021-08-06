from lib.Depend.gtk2 import gtk

import common

class CfrmOperation(common.CWindow):
    name = 'frmOperation'
    widgets = ('edOprName', 'edOprParameters', 'cboxOprReturnType', 'cboxOprScope', 
              'cboxOprStereotype', 'txtOprDocumentation', 'cmdOprOK', 'cmdOprCancel',
              'cbOprAbstract', 'cbOprStatic', 'cbOprConst', 'cbOprReturnArray', 'cbOprPure',
              'cbOprSynchronize', 'cbOprIsQuery')
    
    def ShowFrmOperation(self, operation = {}):
        if 'name' in operation:
            self.edOprName.set_text(operation['name'])
        else:
            self.edOprName.set_text("")

        if 'params' in operation:
            self.edOprParameters.set_text(operation['params'])
        else:
            self.edOprParameters.set_text("")
        
        if 'abstract' in operation:
            self.cbOprAbstract.set_active(operation['abstract'])
        else:
            self.cbOprAbstract.set_active(0)
        
        if 'static' in operation:
            self.cbOprStatic.set_active(operation['static'])
        else:
            self.cbOprStatic.set_active(0)
        
        if 'const' in operation:
            self.cbOprConst.set_active(operation['const'])
        else:
            self.cbOprConst.set_active(0)
            
        if 'returnarray' in operation:
            self.cbOprReturnArray.set_active(operation['returnarray'])
        else:
            self.cbOprReturnArray.set_active(0)
        
        if 'pure' in operation:
            self.cbOprPure.set_active(operation['pure'])
        else:
            self.cbOprPure.set_active(0)
        
        if 'synchronize' in operation:
            self.cbOprSynchronize.set_active(operation['synchronize'])
        else:
            self.cbOprSynchronize.set_active(0)
        
        if 'isquery' in operation:
            self.cbOprIsQuery.set_active(operation['isquery'])
        else:
            self.cbOprIsQuery.set_active(0)
            
        if 'type' in operation:
            self.cboxOprReturnType.child.set_text(operation['type'])
        else:
            self.cboxOprReturnType.child.set_text("")
        
        if 'scope' in operation:
            self.cboxOprScope.set_active({'public':0, 'private':1, 'protected':2}[operation['scope']])
        else:
            self.cboxOprScope.set_active(0)
        
        if 'stereotype' in operation:
            self.cboxOprStereotype.child.set_text(operation['stereotype'])
        else:
            self.cboxOprStereotype.child.set_text("")
    
        if 'documentation' in operation:
            self.txtOprDocumentation.get_buffer().set_text(operation['doc'])
        else:
            self.txtOprDocumentation.get_buffer().set_text("")
        
        while True:
            response = self.form.run()
            if response != gtk.RESPONSE_OK:
                break
            if self.edOprName.get_text().strip() != '' and \
                    self.cboxOprReturnType.child.get_text().strip() != '':
                break
            msg = gtk.MessageDialog(message_format = _("Fill the name and type fields"), parent = self.form, type = gtk.MESSAGE_ERROR,
                    buttons = gtk.BUTTONS_OK)
            msg.run()
            msg.destroy()
        ret = False
        if response == gtk.RESPONSE_OK:
            operation['name'] = self.edOprName.get_text()
            operation['params'] = self.edOprParameters.get_text()
            operation['abstract'] = self.cbOprAbstract.get_active()
            operation['static'] = self.cbOprStatic.get_active()
            operation['const'] = self.cbOprConst.get_active()
            operation['returnarray'] = self.cbOprReturnArray.get_active()
            operation['pure'] = self.cbOprPure.get_active()
            operation['synchronize'] = self.cbOprSynchronize.get_active()
            operation['isquery'] = self.cbOprIsQuery.get_active()
            operation['scope'] = ['public', 'private', 'protected'][self.cboxOprScope.get_active()]
            operation['type'] = self.cboxOprReturnType.child.get_text()
            operation['stereotype'] = self.cboxOprStereotype.child.get_text()
            buf = self.txtOprDocumentation.get_buffer()
            operation['doc'] = buf.get_text(buf.get_start_iter(), buf.get_end_iter())
            
            ret = True
        self.Hide()      
        return ret
