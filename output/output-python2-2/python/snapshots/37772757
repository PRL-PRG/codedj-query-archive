from common import CWindow, event
from dialogs import CWarningDialog
from lib.Drawing.Canvas.Gtk import PixmapFromPath
from lib.Project.ProjectNode import CProjectNode
import gtk
import gobject
import common
from lib.CodeEngineering.Generator import CGenerator
from lib.consts import TMP_IMAGES
import os
import os.path

class CfrmGenerateDocumentation(CWindow):
    name = 'frmGenerateDocumentation'
    
    widgets = ("edtOutput", "edtPackage", "edtTitle", "btnChoose")
    
    __gsignals__ = {
        'create_svg_diagrams': (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_PYOBJECT, gobject.TYPE_PYOBJECT)), 
    }
    
    def __init__(self, app, wTree):
        common.CWindow.__init__(self, app, wTree)
     
    def ShowDialog(self, packageNode):
        self.edtPackage.set_text(packageNode.GetShortPath())
        while True:
            response = self.form.run() 
            if response != gtk.RESPONSE_OK:
                self.form.hide()
                return
                
            if self.edtOutput.get_text().strip(' ') == "":
                CWarningDialog(self.form, _('Fill the output field')).run()
                continue
            else:
                if not os.path.isdir(self.edtOutput.get_text()):
                    CWarningDialog(self.form, _('Path in output field not is directory')).run()
                    continue 
        
            path = self.edtOutput.get_text()
            gen = CGenerator(self.application.GetProject().GetCodeEngineering().GetType("HTML"), path)
            path = TMP_IMAGES
            if os.path.exists(path):
                for i in os.listdir(path):
                    path_file = os.path.join(path,i)
                    if os.path.isfile(path_file):
                        os.remove(path_file)
                os.rmdir(path)
            os.mkdir(TMP_IMAGES)
            self.emit('create_svg_diagrams', self.application.GetProject().GetRoot(), path)
            gen.GenerateDocumentation(self.edtTitle.get_text(), self.application.GetProject(), packageNode)
            for i in os.listdir(path):
                path_file = os.path.join(path,i)
                if os.path.isfile(path_file):
                    os.remove(path_file)
            os.rmdir(path)
                
            self.form.hide()
            return
            

    @event("btnChoose","clicked")
    def on_btnChooseFolder_click(self, widget):
        path = self.application.GetWindow('frmChooseFolder').ShowDialog()
        if path is not None:
            self.edtOutput.set_text(path)
    