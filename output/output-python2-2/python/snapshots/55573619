#!/usr/bin/env python
import pygtk
pygtk.require( '2.0' )
import gtk
import random
from ThemeWidgets import keyButton

class KeyboardWindow(gtk.Window):
    def __init__(self):
        gtk.Window.__init__(self , gtk.WINDOW_TOPLEVEL)
        self.set_type_hint(gtk.gdk.WINDOW_TYPE_HINT_DIALOG)
        color = gtk.gdk.color_parse("#000000")
        self.modify_bg(gtk.STATE_NORMAL, color)
        
        self.connect("destroy",gtk.main_quit)
        self.connect("key-press-event",self.handle_keypress)
        self.connect("key-release-event",self.handle_keyrelease)
        
        self.PIXEL_SPACE = 15
        self.HEIGHT = 45    
        
        self.rows = {}
        self.rows[1] = [(49,1), (10,3), (11,3), (12,3), (13,3), (14,3), (15,3), (16,3), (17,3), (18,3), (19,3), (20,3), (21,5)]
        self.rows[2] = [(23,3), (24,3), (25,3), (26,3), (27,3), (28,3), (29,3), (30,3), (31,3), (32,3), (33,3), (34,3), (35,4)]
        self.rows[3] = [(37,4), (38,3), (39,3), (40,3), (41,3), (42,3), (43,3), (44,3), (45,3), (46,3), (47,3), (48,3), (51,3)]
        self.rows[4] = [(50,6), (52,3), (53,3), (54,3), (55,3), (56,3), (57,3), (58,3), (59,3), (60,3), (61,3), (62,5)]
        self.rows[5] = [(216,1),(133,4),(64,4), (65,25), (108,4), (134,4,), (113,3)]
        
        self.right_section = [(22,7),(36,(7,7)),(111,3),(219,3),(116,3),(114,3)]
        
        self.btn_dic = {}
        
        mainhbox = gtk.HBox()
        
        vbox = gtk.VBox()
        for row in [1,2,3,4,5]:
            hbox = gtk.HBox()
            for key in self.rows[row]:
                self.btn_dic[key[0]] = keyButton(self.PIXEL_SPACE * key[1], self.HEIGHT, [0,0,0], [0.5,0.5,0.5])
                self.btn_dic[key[0]].connect("enter",self.handle_mouseEnter)
                self.btn_dic[key[0]].connect("leave",self.handle_mouseLeave)
                hbox.pack_start(self.btn_dic[key[0]], padding = self.PIXEL_SPACE//2)
            vbox.pack_start(hbox, padding = self.PIXEL_SPACE//2)
        mainhbox.pack_start(vbox)
        
        right_vbox = gtk.VBox()
        right_tophbox = gtk.HBox()
        right_lowhbox = gtk.HBox()
        
        self.btn_dic[self.right_section[0][0]] = keyButton(self.PIXEL_SPACE * self.right_section[0][1], self.HEIGHT, [0,0,0], [0.5,0.5,0.5])
        self.btn_dic[self.right_section[1][0]] = keyButton(self.PIXEL_SPACE * self.right_section[1][1][0], self.PIXEL_SPACE * self.right_section[1][1][1], [0,0,0], [0.5,0.5,0.5])
        self.btn_dic[self.right_section[2][0]] = keyButton(self.PIXEL_SPACE * self.right_section[2][1], self.HEIGHT, [0,0,0], [0.5,0.5,0.5])
        self.btn_dic[self.right_section[3][0]] = keyButton(self.PIXEL_SPACE * self.right_section[3][1], self.HEIGHT, [0,0,0], [0.5,0.5,0.5])
        self.btn_dic[self.right_section[4][0]] = keyButton(self.PIXEL_SPACE * self.right_section[4][1], self.HEIGHT, [0,0,0], [0.5,0.5,0.5])
        self.btn_dic[self.right_section[5][0]] = keyButton(self.PIXEL_SPACE * self.right_section[5][1], self.HEIGHT, [0,0,0], [0.5,0.5,0.5])
        
        for key in self.right_section:
            self.btn_dic[key[0]].connect("enter",self.handle_mouseEnter)
            self.btn_dic[key[0]].connect("leave",self.handle_mouseLeave)
            
        right_vbox.pack_start(self.btn_dic[self.right_section[0][0]], padding = self.PIXEL_SPACE//2)
        right_vbox.pack_start(self.btn_dic[self.right_section[1][0]], padding = self.PIXEL_SPACE//2)
        right_tophbox.pack_start(self.btn_dic[self.right_section[2][0]], padding = self.PIXEL_SPACE//2)
        right_tophbox.pack_start(self.btn_dic[self.right_section[3][0]], padding = self.PIXEL_SPACE//2)
        right_lowhbox.pack_start(self.btn_dic[self.right_section[4][0]], padding = self.PIXEL_SPACE//2)
        right_lowhbox.pack_start(self.btn_dic[self.right_section[5][0]], padding = self.PIXEL_SPACE//2)
        right_vbox.pack_start(right_tophbox, padding = self.PIXEL_SPACE//2)
        right_vbox.pack_start(right_lowhbox, padding = self.PIXEL_SPACE//2)
    
        mainhbox.pack_start(right_vbox)        
        
        self.add(mainhbox)
    
    def handle_keypress(self,widget,event):
        print "allo"
        self.btn_dic[event.hardware_keycode].set_fillcolor(random.random(),random.random(),random.random())
    
    def handle_keyrelease(self,widget,event):
        self.btn_dic[event.hardware_keycode].set_fillcolor(0,0,0)
        
    def handle_mouseEnter(self,widget,event = None):
        widget.set_strokecolor(1,1,1)
            
    def handle_mouseLeave(self,widget,event = None):
        widget.set_strokecolor(0.5,0.5,0.5)
    
        
        
if __name__ == "__main__":
    win = KeyboardWindow()
    win.show_all()
    gtk.main()