import pygtk
pygtk.require( '2.0' )
import gtk

class Credits( gtk.Window ):
    def __init__(self , handleCreditsCloseCallback , pos):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        self.connect('destroy' , self.destroy)
        #self.set_decorated(False)
        self.handleCreditsCloseCallback = handleCreditsCloseCallback
        self.move(pos[0] , pos[1])
        
        self.vbox = gtk.VBox()
        self.tb = gtk.TextBuffer()
        self.tb.set_text('TamTam\n\nby\nOlivier Bélanger\nJames Bergstra\nNathanaël Lécaudé\nAdrian Martin\nJean Piché\nSean Wood')
        self.tw = gtk.TextView(buffer = self.tb)
        self.tw.connect('button-press-event' , self.destroy, self.tw)
        self.vbox.add(self.tw)
        
        self.closeButton = gtk.Button(label='Close')
        self.closeButton.connect('clicked' , self.destroy)
        self.vbox.add(self.closeButton)
        self.add(self.vbox)
        self.show_all()
    
    def destroy(self , widget):
        self.handleCreditsCloseCallback(False)
        self.hide_all()
        

if __name__ == '__main__':
    credits = Credits()
    gtk.main()