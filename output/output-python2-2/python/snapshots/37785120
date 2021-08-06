from common import CWindow, event
from dialogs import CWarningDialog
from lib.Drawing.Canvas.Gtk import PixmapFromPath
from lib.CodeEngineering.Generator import CGenerator
from lib.Project.ProjectNode import CProjectNode
import gtk
import gobject
import common


class CfrmGenerateCode(CWindow):
    name = 'frmGenerateSourceCode'
    
    widgets = ("twGenerateObjects", "chckSelectAll", "chckChildPackages", "btnGenerate", "edtRootPackage", "cbTargetLanguage", "btnChooseFolder", "edtTargetPath" )
    
    def __init__(self, app, wTree):
        common.CWindow.__init__(self, app, wTree)
    
        self.Model = gtk.ListStore(gobject.TYPE_BOOLEAN, gobject.TYPE_STRING, gobject.TYPE_STRING, object)
        renderer = gtk.CellRendererToggle()
        renderer.connect('toggled', self.__fixed_toggled)
        self.twGenerateObjects.append_column(gtk.TreeViewColumn(_("Generate"), renderer, active = 0))
        self.twGenerateObjects.append_column(gtk.TreeViewColumn(_("Object"), gtk.CellRendererText(), text = 1))
        self.twGenerateObjects.append_column(gtk.TreeViewColumn(_("Type"), gtk.CellRendererText(), text = 2))
        
        self.twGenerateObjects.set_model(self.Model)
    
    def __fixed_toggled(self, cell, path):
        iter = self.Model.get_iter((int(path),))
        self.Model.set(iter, 0, not self.Model.get_value(iter, 0))

        
    def __FillGenerateObject(self):
        self.Model.clear()
        if isinstance(self.packageNode, CProjectNode):
            for i in self.packageNode.GetNodeSpecifyElements(self.packageNode, ("Class", "Package"), self.chckChildPackages.get_active()):
                self.Model.set(self.Model.append(), 0, self.chckSelectAll.get_active(), 1, i.GetName(), 2, i.GetType(), 3, i.GetObject())
        else:
            for i in self.packageNode:
                self.Model.set(self.Model.append(), 0, self.chckSelectAll.get_active(), 1, i.GetObject().GetName(), 2, i.GetObject().GetType().GetId(), 3, i.GetObject())
            
    def ShowDialog(self, packageNode):
        self.packageNode = packageNode
        if isinstance(packageNode, CProjectNode):
            self.edtRootPackage.set_text(packageNode.GetShortPath())
        self.chckSelectAll.set_active(True)
        
        model = self.cbTargetLanguage.get_model()
        model.clear()
        self.cbTargetLanguage.set_model(model)
        for id, language in enumerate(self.application.GetProject().GetCodeEngineering()):
            self.cbTargetLanguage.append_text(language.GetLanguage())
            if self.cbTargetLanguage.child.get_text() != "":
                if language.GetLanguage() == self.cbTargetLanguage.child.get_text():
                    self.cbTargetLanguage.set_active(id)
            elif language.GetLanguage() == self.application.GetProject().GetActualLanguage():
                self.cbTargetLanguage.set_active(id)
            
        
        self.__FillGenerateObject()
        response = self.form.run() 
        while True:
            if response != gtk.RESPONSE_OK:
                self.form.hide()
                return

            path = self.edtTargetPath.get_text()
            if self.cbTargetLanguage.get_active() >= 0:
                gen = CGenerator(self.application.GetProject().GetCodeEngineering().GetType(self.cbTargetLanguage.get_active_text()), path)
                model = self.twGenerateObjects.get_model()
                for id in xrange(model.iter_n_children(None)):
                    if model.get_value(model.iter_nth_child(None, id),0):
                        gen.GenerateElement(model.get_value(model.iter_nth_child(None, id),3))
                self.form.hide()
                return
            else:
                CWarningDialog(self.form, _('Fill the target language field')).run()
                response = self.form.run()
        
    @event("chckSelectAll","toggled")
    def on_chckSelectAll_toggled(self, cell):
        self.__FillGenerateObject()

    @event("chckChildPackages","toggled")
    def on_chckChildPackage_toggled(self, cell):
        self.__FillGenerateObject()
    
    @event("btnChooseFolder","clicked")
    def on_btnChooseFolder_click(self, widget):
        path = self.application.GetWindow('frmChooseFolder').ShowDialog()
        if path is not None:
            self.edtTargetPath.set_text(path)