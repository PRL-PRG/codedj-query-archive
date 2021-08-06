import gtk

class CWarningDialog:
    def __init__(self, form, message):
        self.dialog = gtk.MessageDialog(form, 0, gtk.MESSAGE_INFO, gtk.BUTTONS_OK)
        self.dialog.set_markup(message)
        self.dialog.set_title("Warning")
        
    def run(self):
        result = self.dialog.run()
        self.dialog.destroy()
        return result
        
        