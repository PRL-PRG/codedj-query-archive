from common import CWindow, event
from dialogs import CWarningDialog
from lib.CodeEngineering.Generator import CGenerator
from lib.Project.ProjectNode import CProjectNode
import gtk
import gobject
import common

from lib.ReverseEngineering import build_parser

class CfrmGenerateDiagrams(CWindow):
    name = 'frmGenerateDiagrams'
    
    widgets = ("btnOK", "edtTargetPackage", "cbSourceLanguage", "btnChooseFolder2", "edtSourcePath" )
    #~ widgets = ("twGenerateObjects", "chckSelectAll", "chckChildPackages", "btnOK", "edtTargetPackage", "cbSourceLanguage", "btnChooseFolder2", "edtSourcePath" )
    
    def __init__(self, app, wTree):
        common.CWindow.__init__(self, app, wTree)
    
            
    def ShowDialog(self, packageNode):
        self.packageNode = packageNode
        if isinstance(packageNode, CProjectNode):
            self.edtTargetPackage.set_text(packageNode.GetShortPath())
        
        model = self.cbSourceLanguage.get_model()
        model.clear()
        self.cbSourceLanguage.set_model(model)
        for id, language in enumerate(self.application.GetProject().GetCodeEngineering()):
            self.cbSourceLanguage.append_text(language.GetLanguage())
            if self.cbSourceLanguage.child.get_text() != "":
                if language.GetLanguage() == self.cbSourceLanguage.child.get_text():
                    self.cbSourceLanguage.set_active(id)
            elif language.GetLanguage() == self.application.GetProject().GetActualLanguage():
                self.cbSourceLanguage.set_active(id)
                
        while True:
            response = self.form.run()
            if response != gtk.RESPONSE_OK:
                break
            path = self.edtSourcePath.get_text()
            if self.cbSourceLanguage.get_active() >= 0:
                elem_fact = self.application.GetProject().GetElementFactory()
                conn_fact = self.application.GetProject().GetConnectionFactory()
                project = self.application.GetProject()
                language = project.GetCodeEngineering().GetType(self.cbSourceLanguage.get_active_text())
                node = build_parser( language, path, packageNode )
                node.Create(elem_fact)
                node.Search(self.application.GetProject())
                node.Connect(conn_fact)
                self.application.GetWindow('frmMain').twProjectView.Redraw()
                break
        self.Hide()
        
            
    @event("btnChooseFolder2","clicked")
    def on_btnChooseFolder2_click(self, widget):
        dlg = self.application.GetWindow('frmChooseFolder')
        path = dlg.ShowDialog()
        if path is not None:
            self.edtSourcePath.set_text(path)
    