#!/usr/bin/env python
import pygtk
pygtk.require( '2.0' )
import gtk
import gobject

import Config

KEY_MAP_PIANO = Config.KEY_MAP_PIANO

class Trackpad:
    def __init__(self, win, client):
        self.win = win
        self.csnd = client
        win.add_events(gtk.gdk.POINTER_MOTION_MASK)
        win.add_events(gtk.gdk.BUTTON_PRESS_MASK)
        win.add_events(gtk.gdk.BUTTON_RELEASE_MASK)
        win.connect('motion-notify-event',self.handle_motion)
        win.connect('key-press-event',self.handle_keyPress)
        win.connect('key-release-event',self.handle_keyRelease)

        self.first_x = None
        self.current_x = None
        self.final_x = None
        self.first_y = None
        self.current_y = None
        self.final_y = None
        
        self.buttonPressed = False
        
        self.create_invisible_cursor()
        
        self.display = self.win.get_display()
        self.screen = gtk.gdk.Display.get_default_screen(self.display)
        
        
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
        if event.x < 0:
            X = 0
        elif event.x > self.screen.get_width():
            X = self.screen.get_width()
        else:
            X = event.x

        if event.y < 0:
            Y = 0
        elif event.y > self.screen.get_height():
            Y = self.screen.get_height()
        else:
            Y = event.y

        self.current_x = X
        self.current_y = Y
        if self.buttonPressed:
            self.final_x = X - self.first_x 
            self.final_y = Y - self.first_y
            self.csnd.setTrackpadX(self.final_x)
            self.csnd.setTrackpadY(self.final_y)
        
    def handle_keyPress(self,widget,event):
        if KEY_MAP_PIANO.has_key(event.hardware_keycode) and self.buttonPressed == False:
            gtk.gdk.Display.warp_pointer(self.display, self.screen, self.screen.get_width() / 2, self.screen.get_height() / 2)
            gtk.gdk.pointer_grab(self.win.window, event_mask = gtk.gdk.POINTER_MOTION_MASK, cursor = self.invisible_cursor)
            self.buttonPressed = True
            self.first_x = self.current_x
            self.first_y = self.current_y
    
    def handle_keyRelease(self,widget,event):
        if KEY_MAP_PIANO.has_key(event.hardware_keycode):            
            gtk.gdk.pointer_ungrab(time = 0L)
            self.buttonPressed = False
            self.restoreDelay = gobject.timeout_add(120, self.restore)

    def restore( self ):
        self.csnd.setTrackpadX(0)
        self.csnd.setTrackpadY(0)
        gobject.source_remove( self.restoreDelay )

