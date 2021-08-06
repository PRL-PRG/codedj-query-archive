#!/usr/bin/env python
import pygtk
pygtk.require( '2.0' )
import gtk

import Config
from Util.ThemeWidgets import *
    
class Welcome(gtk.EventBox):
    
    def __init__(self):
        gtk.EventBox.__init__(self)
        
        self.draw()
        self.show_all()
        
    def draw(self):
        
        actVBox = RoundVBox(fillcolor = Config.WS_BCK_COLOR, bordercolor = Config.WS_BCK_COLOR, radius = Config.PANEL_RADIUS)
        actHBox = gtk.HBox()
        
        for activity in ['mini','edit','type','synth']:
            actBtnBox = RoundVBox(fillcolor = Config.WS_PANEL_COLOR, bordercolor = Config.WS_BCK_COLOR, radius = Config.PANEL_RADIUS)
            actBtnBox.set_size_request(200,200)
            actBtnBox.set_border_width(Config.PANEL_SPACING)
            actBtn = ImageButton(Config.IMAGE_ROOT + activity +'Tam.png')
            actBtnBox.pack_start(actBtn,True,False,0)
            actHBox.pack_start(actBtnBox,True,False,0)
            
        title = gtk.Image()
        title.set_from_file(Config.IMAGE_ROOT + 'TamTam.png')
        
        actVBox.pack_start(actHBox,False,False, 100)
        actVBox.pack_start(title,False,False, 30)
        self.add(actVBox)
        
if __name__ == "__main__": 
    win = gtk.Window()
    wc = Welcome()
    win.add(wc)
    win.show()
    #start the gtk event loop
    gtk.main()
        
        