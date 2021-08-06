from common import CWindow, event
import gtk
import gobject
import common


class CfrmChooseFolder(CWindow):
    name = 'frmChooseFolder'
    
    widgets = ("fchSelectFolder", "btnCancel", "btnSave")
    
    def __init__(self, app, wTree):
        common.CWindow.__init__(self, app, wTree)
    
    
    def ShowDialog(self):
        response = self.form.run() 
        self.form.hide()
        if response != gtk.RESPONSE_OK:
            return None
        else:
            return self.fchSelectFolder.get_current_folder()