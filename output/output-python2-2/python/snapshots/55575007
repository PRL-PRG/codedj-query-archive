import pygtk
pygtk.require('2.0')
import gtk

import Config
from Util.ThemeWidgets import *

Tooltips = Config.Tooltips

class Parameter( gtk.Window ):
    def __init__( self, string ):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        self.set_type_hint(gtk.gdk.WINDOW_TYPE_HINT_DIALOG)
        color = gtk.gdk.color_parse(Config.INST_BCK_COLOR)
        self.modify_bg(gtk.STATE_NORMAL, color)
        self.move(15, 650)
        self.set_size_request(450, 40)
        self.set_decorated(False)
        mainBox = RoundHBox(fillcolor=Config.INST_BCK_COLOR, bordercolor=Config.INST_BCK_COLOR)
        mainBox.set_border_width(4)
        mainBox.set_radius(10)
        self.text = gtk.Label(string)
        mainBox.pack_start(self.text, False, False, 5)
        self.add(mainBox)
        self.show_all()

    def update( self, string ):
        self.text.set_text(string)

