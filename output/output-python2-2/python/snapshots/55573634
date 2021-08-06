#!/usr/bin/env python
import pygtk
pygtk.require( '2.0' )
import gtk, cairo

class keyBtn(gtk.Button):
    def __init__(self, width, height, fillcolor, strokecolor):
        gtk.Button.__init__(self)
        self.alloc = None
        win = gtk.gdk.get_default_root_window()
        self.gc = gtk.gdk.GC(win)
        
        self.connect('expose-event', self.expose)
        self.connect('size-allocate', self.size_allocate)
        
        self.width = width
        self.height = height
        self.fillcolor = fillcolor
        self.strokecolor = strokecolor
        
        self.set_size_request(self.width,self.height)
        
    def size_allocate(self, widget, allocation):
        self.alloc = allocation
        self.drawX = allocation.x + allocation.width//2
        self.drawY = allocation.y + allocation.height//2
        
    def expose(self, widget, event):
        self.draw()
        return True
    
    def draw(self):
        self.cr = self.window.cairo_create()
        self.cr.set_source_rgb(self.fillcolor[0],self.fillcolor[1],self.fillcolor[2])
        self.draw_round_rect(self.cr,self.drawX - self.width//2, self.drawY - self.height //2, self.width,self.height,10)
        self.cr.fill()
        self.cr.set_line_width(3)
        self.cr.set_source_rgb(self.strokecolor[0],self.strokecolor[1],self.strokecolor[2])
        self.draw_round_rect(self.cr,self.drawX - self.width//2, self.drawY - self.height //2, self.width,self.height,10)
        self.cr.stroke()
        
    def draw_round_rect(self,context,x,y,w,h,r):    
        context.move_to(x+r,y)                      # Move to A
        context.line_to(x+w-r,y)                    # Straight line to B
        context.curve_to(x+w,y,x+w,y,x+w,y+r)       # Curve to C, Control points are both at Q
        context.line_to(x+w,y+h-r)                  # Move to D
        context.curve_to(x+w,y+h,x+w,y+h,x+w-r,y+h) # Curve to E
        context.line_to(x+r,y+h)                    # Line to F
        context.curve_to(x,y+h,x,y+h,x,y+h-r)       # Curve to G
        context.line_to(x,y+r)                      # Line to H
        context.curve_to(x,y,x,y,x+r,y)             # Curve to A
        return
 
    
    def set_fillcolor(self,r,g,b):
        self.fillcolor = [r,g,b]
        self.queue_draw()
    
    def set_strokecolor(self,r,g,b):
        self.strokecolor = [r,g,b]
        self.queue_draw()

import random
class keyboardImg(gtk.Window):
    def __init__(self):
        gtk.Window.__init__(self)
        color = gtk.gdk.color_parse("#000000")
        self.modify_bg(gtk.STATE_NORMAL, color)
        
        self.connect("destroy",gtk.main_quit)
        self.connect("key-press-event",self.handle_keypress)
        self.connect("key-release-event",self.handle_keyrelease)
        
        self.PIXEL_SPACE = 15
        self.HEIGHT = 45
        
        self.rows = {}
        self.rows[1] = [(49,1), (10,3), (11,3), (12,3), (13,3), (14,3), (15,3), (16,3), (17,3), (18,3), (19,3), (20,3), (21,6)]
        self.rows[2] = [(23,3), (24,3), (25,3), (26,3), (27,3), (28,3), (29,3), (30,3), (31,3), (32,3), (33,3), (34,3), (35,4)]
        self.rows[3] = [(37,4), (38,3), (39,3), (40,3), (41,3), (42,3), (43,3), (44,3), (45,3), (46,3), (47,3), (48,3), (51,3)]
        self.rows[4] = [(50,6), (52,3), (53,3), (54,3), (55,3), (56,3), (57,3), (58,3), (59,3), (60,3), (61,3), (62,5)]
        
        self.btn_dic = {}
        
        vbox = gtk.VBox()
        for row in [1,2,3,4]:
            hbox = gtk.HBox()
            for key in self.rows[row]:
                self.btn_dic[key[0]] = keyBtn(self.PIXEL_SPACE * key[1], self.HEIGHT, [0,0,0], [0.5,0.5,0.5])
                self.btn_dic[key[0]].connect("enter",self.handle_mouseEnter)
                self.btn_dic[key[0]].connect("leave",self.handle_mouseLeave)
                hbox.pack_start(self.btn_dic[key[0]], padding = self.PIXEL_SPACE//2)
            vbox.pack_start(hbox, padding = self.PIXEL_SPACE//2)
        self.add(vbox)
    
    def handle_keypress(self,widget,event):
        self.btn_dic[event.hardware_keycode].set_fillcolor(random.random(),random.random(),random.random())
    
    def handle_keyrelease(self,widget,event):
        self.btn_dic[event.hardware_keycode].set_fillcolor(0,0,0)
        
    def handle_mouseEnter(self,widget,event = None):
        widget.set_strokecolor(1,1,1)
            
    def handle_mouseLeave(self,widget,event = None):
        widget.set_strokecolor(0.5,0.5,0.5)
    
        
        
if __name__ == "__main__":
    win = keyboardImg()
    win.show_all()
    gtk.main()