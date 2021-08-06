#!/usr/bin/env python
import pygtk
pygtk.require( '2.0' )
import gtk

import Config

KEY_MAP_PIANO = Config.KEY_MAP_PIANO

class Trackpad:
    def __init__(self, win):
        self.win = win
        win.add_events(gtk.gdk.POINTER_MOTION_MASK)
        win.add_events(gtk.gdk.BUTTON_PRESS_MASK)
        win.add_events(gtk.gdk.BUTTON_RELEASE_MASK)
        win.connect('motion-notify-event',self.handle_motion)
        win.connect('key-press-event',self.handle_keyPress)
        win.connect('key-release-event',self.handle_keyRelease)

        self.first_x = None
        self.current_x = None
        self.final_x = None
        
        self.buttonPressed = False
        
        self.create_invisible_cursor()
        
        
    def create_invisible_cursor(self):
        pix_data = """/* XPM */
        static char * invisible_xpm[] = {
        "1 1 1 1",
        "       c None",
        " "};"""
        color = gtk.gdk.Color()
        pix = gtk.gdk.pixmap_create_from_data(None, pix_data, 1, 1, 1, color, color)
        self.invisible_cursor = gtk.gdk.Cursor(pix,pix,color,color,0,0)
        
    def handle_motion(self,widget,event):
        self.current_x = event.x
        if self.buttonPressed:
            self.final_x = event.x - self.first_x 
            print int(self.final_x)
        
    def handle_keyPress(self,widget,event):
        if KEY_MAP_PIANO.has_key(event.hardware_keycode) and self.buttonPressed == False:
            self.win.window.set_cursor(self.invisible_cursor)
            self.buttonPressed = True
            self.first_x = self.current_x
    
    def handle_keyRelease(self,widget,event):
        if KEY_MAP_PIANO.has_key(event.hardware_keycode):
            self.win.window.set_cursor(None)
            self.buttonPressed = False
