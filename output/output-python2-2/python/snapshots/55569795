# -*- coding: utf-8 -*-
import pygtk
pygtk.require( '2.0' )
import gtk

class Credits( gtk.Window ):
    def __init__(self , handleCreditsCloseCallback , pos):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        self.set_type_hint(gtk.gdk.WINDOW_TYPE_HINT_DIALOG)
        self.connect('destroy' , self.destroy)
        #self.set_decorated(False)
        self.handleCreditsCloseCallback = handleCreditsCloseCallback
        self.move(pos[0] , pos[1])
        
        self.vbox = gtk.VBox()
        self.tb = gtk.TextBuffer()
        self.tb.set_text('miniTamTam\n\nby\n\nOlivier Bélanger\nJames Bergstra\nEric Lamothe\nNathanaël Lécaudé\nAdrian Martin\nJean Piché\nSean Wood\n\nThanks to\n\nBarry Vercoe\nVictor Lazzarini\nMarco Pesenti Gritti\nSimon Schampijer\nPeter Kirn\n\nUniversité de Montréal 2006')
        self.tw = gtk.TextView(buffer = self.tb)
        self.tw.set_editable(False)
        self.tw.set_cursor_visible(False)
        #self.tw.set_right_margin(10)
        #self.tw.set_left_margin(10)
        self.tw.set_justification(gtk.JUSTIFY_CENTER)
        self.vbox.add(self.tw)
        
        self.closeButton = gtk.Button(label='X')
        self.closeButton.connect('clicked' , self.destroy)
        self.vbox.add(self.closeButton)
        self.add(self.vbox)
        self.show_all()
    
    def destroy(self, widget):
        self.handleCreditsCloseCallback(False)
        self.hide_all()
        

if __name__ == '__main__':
    credits = Credits()
    gtk.main()