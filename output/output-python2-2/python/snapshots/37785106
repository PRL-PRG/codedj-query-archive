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

class CfrmGenerateDocumentation(CWindow):
    name = 'frmGenerateDocumentation'
    
    widgets = ("edtOutput", "edtPackage", "edtTitle", "btnChoose")
    
    __gsignals__ = {
        'create_svg_diagrams': (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_PYOBJECT, gobject.TYPE_PYOBJECT)), 
    }
    
    def __init__(self, app, wTree):
        common.CWindow.__init__(self, app, wTree)
        
    
    def ShowDialog(self, packageNode):
        response = self.form.run() 
        while True:
            if response != gtk.RESPONSE_OK:
                self.form.hide()
                return
            
            path = self.edtOutput.get_text()
            #~ if self.cbTargetLanguage.get_active() >= 0:
            gen = CGenerator(self.application.GetProject().GetCodeEngineering().GetType("HTML"), path)
                #~ model = self.twGenerateObjects.get_model()
                #~ for id in xrange(model.iter_n_children(None)):
                    #~ if model.get_value(model.iter_nth_child(None, id),0):
            #~ gen.GenerateElement("documentation")
            
            os.mkdir(TMP_IMAGES)
            path = TMP_IMAGES
            self.emit('create_svg_diagrams', self.application.GetProject().GetRoot(), path)
            gen.GenerateDocumentation(self.edtTitle.get_text(), self.application.GetProject())
            for i in os.listdir(path):
                path_file = os.path.join(path,i)
                if os.path.isfile(path_file):
                    os.remove(path_file)
            os.rmdir(path)
                
            self.form.hide()
            return
            #~ else:
                #~ CWarningDialog(self.form, _('Fill the target language field')).run()
                #~ response = self.form.run()


    @event("btnChoose","clicked")
    def on_btnChooseFolder_click(self, widget):
        path = self.application.GetWindow('frmChooseFolder').ShowDialog()
        if path is not None:
            self.edtOutput.set_text(path)
    