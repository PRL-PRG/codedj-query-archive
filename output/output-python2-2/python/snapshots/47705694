import gtk
import pango
from sugar.graphics import style

class InfoPanel(gtk.VBox):
    def __init__(self):
        gtk.VBox.__init__(self)
        self.status_label = gtk.Label('Status')
        self.pack_start(self.status_label, False, True, 10)
        self.score_label = gtk.Label('Score')
        self.pack_start(self.score_label,  False, True, 10)
        self.show_all()

    def show(self, text):
        self.status_label.set_text(text)

    def show_score(self,  text):
        self.score_label.set_text(text)


