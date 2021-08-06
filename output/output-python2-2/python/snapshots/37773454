import common
import gtk
import lib.consts
import os.path

class CfrmSave(common.CWindow):
    name = 'frmSave'
    
    
    def __init__(self, app, wTree):
        common.CWindow.__init__(self, app, wTree)
        
        filter = gtk.FileFilter()
        filter.set_name("UML .FRI Projects")
        filter.add_pattern('*'+lib.consts.PROJECT_EXTENSION)
        self.form.add_filter(filter)
        filter = gtk.FileFilter()
        filter.set_name("UML .FRI Project templates")
        filter.add_pattern('*'+lib.consts.PROJECT_TPL_EXTENSION)
        self.form.add_filter(filter)
        filter = gtk.FileFilter()
        filter.set_name("All files")
        filter.add_pattern("*")
        self.form.add_filter(filter)
    
    def ShowDialog(self, parent):
        self.form.set_transient_for(parent.form)
        try:
            while True:
                if self.form.run() == gtk.RESPONSE_CANCEL:
                    self.form.hide()
                    return None
                filter = self.form.get_filter().get_name()
                filename = self.form.get_filename()
                if filename is None:
                    continue
                if '.' not in os.path.basename(filename):
                    if filter == "UML .FRI Projects":
                        filename += lib.consts.PROJECT_EXTENSION
                    elif filter == "UML .FRI Project templates":
                        filename += lib.consts.PROJECT_TPL_EXTENSION
                if not os.path.isdir(filename):
                    self.application.GetRecentFiles().AddFile(filename)
                    return filename
        finally:
            self.form.hide()
