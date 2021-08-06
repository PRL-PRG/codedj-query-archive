import common
import gtk
import lib.consts

class CfrmOpen(common.CWindow):
    name = 'frmOpen'
    
    widgets = ("ivOpenNew", "fwOpenExisting", "chkOpenAsCopyExisting", "twOpenRecent", "chkOpenAsCopyRecent", "nbOpen")
    
    def __init__(self, app, wTree):
        common.CWindow.__init__(self, app, wTree)
        
        filter = gtk.FileFilter()
        filter.set_name("UML .FRI Projects")
        filter.add_pattern('*'+lib.consts.PROJECT_EXTENSION)
        self.fwOpenExisting.add_filter(filter)
        filter = gtk.FileFilter()
        filter.set_name("UML .FRI Project templates")
        filter.add_pattern('*'+lib.consts.PROJECT_TPL_EXTENSION)
        self.fwOpenExisting.add_filter(filter)
        filter = gtk.FileFilter()
        filter.set_name("All files")
        filter.add_pattern("*")
        self.fwOpenExisting.add_filter(filter)
    
    def ShowDialog(self):
        if self.form.run() == gtk.RESPONSE_CANCEL:
            self.form.hide()
            return None, False
        self.form.hide()
        if self.nbOpen.get_current_page() == 0:
            return None, True # template
        elif self.nbOpen.get_current_page() == 1:
            return self.fwOpenExisting.get_filename(), self.chkOpenAsCopyExisting.get_active() # existing
        else:
            return None, self.chkOpenAsCopyRecent.get_active() # recent
