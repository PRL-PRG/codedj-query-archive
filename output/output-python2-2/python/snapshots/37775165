from lib.Depend.gtk2 import gtk

class ECancelPressed(Exception):
    pass

class CWarningDialog:
    def __init__(self, form, message):
        self.dialog = gtk.MessageDialog(form, 0, gtk.MESSAGE_INFO, gtk.BUTTONS_OK)
        self.dialog.set_markup(message)
        self.dialog.set_title(_("Warning"))

    def run(self):
        self.dialog.run()

    def __del__(self):
        self.dialog.destroy()

class CQuestionDialog:
    def __init__(self, form, message, allow_cancel = False):
        self.dialog = gtk.MessageDialog(form, 0, gtk.MESSAGE_INFO, gtk.BUTTONS_YES_NO)
        if allow_cancel:
             self.dialog.add_buttons(gtk.STOCK_CANCEL, gtk.RESPONSE_CANCEL)
        self.dialog.set_markup(message)
        self.dialog.set_title(_("Question"))

    def run(self):
        tmp = self.dialog.run()
        if tmp == gtk.RESPONSE_CANCEL:
            raise ECancelPressed
        return tmp == gtk.RESPONSE_YES

    def __del__(self):
        self.dialog.destroy()

class CExceptionDialog:
    def __init__(self, form, message):
        self.dialog = gtk.MessageDialog(form, 0, gtk.MESSAGE_WARNING, gtk.BUTTONS_OK)
        self.dialog.set_markup(message)
        self.dialog.set_keep_above(True)
        self.dialog.set_title(_("Exception"))

    def run(self):
        self.dialog.run()

    def __del__(self):
        self.dialog.destroy()
