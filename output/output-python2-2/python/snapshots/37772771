from common import event
from dialogs import CWarningDialog
import common
import gtk
import gobject

class CfrmCreatePropertyImpl(common.CWindow):
    name = 'frmCreateProperty'
    
    widgets = ('btnDetailGetter', 'btnDetailSetter', 'edtGetter', 'edtSetter', 'edtName')
    
    #~ __gsignals__ = {
        #~ 'on_create_operations':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_PYOBJECT,)),
    #~ }
    
    def __init__(self, app, wTree):
        common.CWindow.__init__(self, app, wTree)
        
    def ShowDialog(self, getter, setter, attrName):
        self.edtGetter.set_text(getter)
        self.edtSetter.set_text(setter)
        self.edtName.set_text("")
        while True:
            response = self.form.run()
            if response != gtk.RESPONSE_OK:
                self.Hide()
                return (None,(self.edtGetter.get_text(),self.edtSetter.get_text()))
            else:
                if self.edtName.get_text().strip(' ') == "":
                    CWarningDialog(self.form, _('Fill the name field')).run()
                elif self.edtName.get_text().strip(' ') == attrName:
                    CWarningDialog(self.form, _('Invalid name')).run()
                else:
                    self.Hide()
                    ret = []
                    if self.edtGetter.get_text() != "":
                        ret.append(self.__CreateOperationFromText(self.edtGetter.get_text()))
                    if self.edtSetter.get_text() != "":
                        ret.append(self.__CreateOperationFromText(self.edtSetter.get_text()))
                    
                    if len(ret) > 0:
                        return (ret,(self.edtGetter.get_text(),self.edtSetter.get_text(), self.edtName.get_text()))
                    return (None,(self.edtGetter.get_text(),self.edtSetter.get_text(), self.edtName.get_text()))
    
    def __CreateOperationFromText(self, text):
        o = {}
        o['abstract'] = False
        o['static'] = False
        o['const'] = False
        o['returnarray'] = False
        o['pure'] = False
        o['synchronize'] = False
        o['isquery'] = False
        o['stereotype'] = ""
        o['doc'] = ""
        o['initial'] = "" 
        o['scope'] = 'private'
        o['name'] = text.split('(')[0]
        o['params'] = text.split('(')[1].split(')')[0]
        if text.split(')')[1] != "":
            o['type'] = text.split(')')[1].split(':')[1]
        else:
            o['type'] = ''
        return o
    
    @event("btnDetailGetter","clicked")
    def on_btnDetailGetter_click(self, button):
        dlg = self.application.GetWindow('frmOperation')
        dlg.SetParent(self)
        o = self.__CreateOperationFromText(self.edtGetter.get_text())
        ret = dlg.ShowFrmOperation(o)
        if ret:
            text = o['name'] + "(" + o['params'] + "):" + o['type']
            self.edtGetter.set_text(text)
        
    
    @event("btnDetailSetter","clicked")
    def on_btnDetailSetter_click(self, button):
        dlg = self.application.GetWindow('frmOperation')
        dlg.SetParent(self)
        o = self.__CreateOperationFromText(self.edtSetter.get_text())
        if dlg.ShowFrmOperation(o):
            if o['type'] != "":
                text = o['name'] + "(" + o['params'] + "):" + o['type']
            else:
                text = o['name'] + "(" + o['params'] + ")"
            self.edtSetter.set_text(text)