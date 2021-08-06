# -*- coding: utf-8 -*-
"""
Area.py

Tools and events manipulation


Copyright 2007, NATE-LSI-EPUSP

Oficina is developed in Brazil at Escola Politécnica of 
Universidade de São Paulo. NATE is part of LSI (Integrable
Systems Laboratory) and stands for Learning, Work and Entertainment
Research Group. Visit our web page: 
www.lsi.usp.br/nate
Suggestions, bugs and doubts, please email oficina@lsi.usp.br

Oficina is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License 
as published by the Free Software Foundation version 2 of 
the License.

Oficina is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public
License along with Oficina; if not, write to the
Free Software Foundation, Inc., 51 Franklin St, Fifth Floor, 
Boston, MA  02110-1301  USA.
The copy of the GNU General Public License is found in the 
COPYING file included in the source distribution.


Authors:

Joyce Alessandra Saul               (joycealess@gmail.com)
Andre Mossinato                     (andremossinato@gmail.com)
Nathalia Sautchuk Patrício          (nathalia.sautchuk@gmail.com)
Pedro Kayatt                        (pekayatt@gmail.com)
Rafael Barbolo Lopes                (barbolo@gmail.com)
Alexandre A. Gonçalves Martinazzo   (alexandremartinazzo@gmail.com)

Colaborators:
Bruno Gola                          (brunogola@gmail.com)

Group Manager:
Irene Karaguilla Ficheman           (irene@lsi.usp.br)

Cientific Coordinator:
Roseli de Deus Lopes                (roseli@lsi.usp.br)

"""


# import  pygtk
# pygtk.require('2.0')
import gtk, gobject, logging
# import sys, socket
# from gtk import gdk
import math
import pango
from fill import *

from Desenho import Desenho

WIDTH = 800
HEIGHT = 600

class Area(gtk.DrawingArea):

    __gsignals__ = {
        'undo' : (gobject.SIGNAL_ACTION, gobject.TYPE_NONE, ([])),
        'redo' : (gobject.SIGNAL_ACTION, gobject.TYPE_NONE, ([])),
        #TODO: these signals still not used.
#         'copy' : (gobject.SIGNAL_ACTION, gobject.TYPE_NONE, ([])),
#         'selected' : (gobject.SIGNAL_ACTION, gobject.TYPE_NONE, ([])),
    }

    def __init__(self, janela):
        """ Initialize the object from class Area which is derived from gtk.DrawingArea.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)
        janela -- the parent window

        """
        logging.debug('Area.__init__(self, janela)')
        
        gtk.DrawingArea.__init__(self)
        self.set_size_request(WIDTH, HEIGHT)
        self.set_events(gtk.gdk.POINTER_MOTION_MASK |
                gtk.gdk.POINTER_MOTION_HINT_MASK |
                gtk.gdk.BUTTON_PRESS_MASK |
                gtk.gdk.BUTTON_RELEASE_MASK|
                gtk.gdk.EXPOSURE_MASK)  
                
        self.connect("expose_event",self.expose)
        self.connect("motion_notify_event", self.mousemove)
        self.connect("button_press_event", self.mousedown)
        self.connect("button_release_event", self.mouseup)      

        self.set_extension_events(gtk.gdk.EXTENSION_EVENTS_CURSOR)
        
        self.tool = None
        self.desenha = False
        self.selmove = False
        self.sel_get_out = False
        self.connect("configure_event", self.configure_event)
        self.oldx = 0
        self.oldy = 0
        self.newx = 0
        self.newy = 0
        self.newx_ = 0
        self.newy_ = 0
        self.color_dec = 0
        self.polygon_start = True
        self.gc = None
        self.gc_line = None
        self.gc_eraser = None
        self.gc_brush = None
        self.gc_selection = None
        self.pixmap = None  
        self.pixmap_temp = None
        self.pixmap_sel = None
        self.pixmap_copy = None
        self.desenho = []   
        self.textos = []
        self.estadoTexto = 0
        self.janela = janela    
        self.d = Desenho(self)
        self.line_size = 2
        self.brush_shape = 'circle'
        self.eraser_shape = 'circle'
                
        self.font = pango.FontDescription('Sans 9')
        
        #start of UNDO and REDO
        self.first_undo = True
        self.undo_surf = False
        self.undo_times = 0
        self.redo_times = 0
        self.undo_list=[]#pixmaps list to Undo func
        

    # Create a new backing pixmap of the appropriate size
    def configure_event(self, widget, event):
        """Configure the Area object.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)
        widget -- the Area object (GtkDrawingArea)
        event -- GdkEvent

        """
        logging.debug('Area.configure_event(self, widget, event)')
        
        win = widget.window
        width = win.get_geometry()[2]
        height = win.get_geometry()[3]  
        
        self.pixmap = gtk.gdk.Pixmap(win, width, height, -1)
        self.pixmap.draw_rectangle(widget.get_style().white_gc, True, 0, 0, width, height)
        
        self.pixmap_temp = gtk.gdk.Pixmap(win, width, height, -1)
        self.pixmap_temp.draw_rectangle(widget.get_style().white_gc, True, 0, 0, width, height)
        
        self.pixmap_sel = gtk.gdk.Pixmap(win, width, height, -1)
        self.pixmap_sel.draw_rectangle(widget.get_style().white_gc, True, 0, 0, width, height)
        
        self.gc = widget.window.new_gc()    
        self.gc_eraser = widget.window.new_gc()
        colormap = self.get_colormap()
        white = colormap.alloc_color('#ffffff', True, True) # white      
        self.gc_eraser.set_foreground(white)
        
        self.gc_brush = widget.window.new_gc()      
        self.gc_brush.set_foreground(white)
                
        self.gc_line = widget.window.new_gc()   

        self.gc_selection = widget.window.new_gc()  
        self.gc_selection.set_line_attributes(1, gtk.gdk.LINE_ON_OFF_DASH, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)
        black = colormap.alloc_color('#000000', True, True)  # black
        self.gc_selection.set_foreground(black)
        
        self.gc_selection1 = widget.window.new_gc()  #this make another white line out of the black line
        self.gc_selection1.set_line_attributes(1, gtk.gdk.LINE_ON_OFF_DASH, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)
        self.gc_selection1.set_foreground(white)
        
        
        self.enableUndo(widget)
        # forcing self.undo_times to zero; self.enableUndo() increases it
        # wrongly at this point... bad hacking, I know.
        #self.undo_times = 0
        #self.emit('undo')
        
        return True
        
    # set the new line size
    def configure_line(self, size):
        """Configure the line's size.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)
        size -- 

        """     
        #logging.debug('Area.configure_line(self, size)')
        
        self.line_size = size
        self.gc_line.set_line_attributes(size, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)

    def expose(self, widget, event):
        """Show up the Area object (GtkDrawingArea).

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)
        widget -- the Area object (GtkDrawingArea)
        event -- GdkEvent

        """
        #logging.debug('Area.expose(self, widget, event)')
        
        area = event.area      
        if self.desenha:
            if self.selmove :
                widget.window.draw_drawable(self.gc, self.pixmap_sel, area[0], area[1], area[0], area[1], area[2], area[3])
            else:
                widget.window.draw_drawable(self.gc, self.pixmap_temp, area[0], area[1], area[0], area[1], area[2], area[3])  
        else:
            widget.window.draw_drawable(self.gc, self.pixmap, area[0], area[1], area[0], area[1], area[2], area[3])     
        return False

    def mousedown(self,widget,event):
        """Make the Area object (GtkDrawingArea) recognize that the mouse button was pressed.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)
        widget -- the Area object (GtkDrawingArea)
        event -- GdkEvent

        """
        # text
        if self.tool == 'text':
            self.d.text(widget,event)
        if not self.selmove or self.tool != 'marquee-rectangular':
            self.oldx = int(event.x)
            self.oldy = int(event.y)
        if self.selmove and self.tool != 'marquee-rectangular': #get out of the func selection
            self.pixmap.draw_drawable(self.gc, self.pixmap_temp, 0,0,0,0, WIDTH, HEIGHT)
            self.selmove = False
            self.enableUndo(widget)
        x , y, state = event.window.get_pointer()
        if state & gtk.gdk.BUTTON3_MASK:
            self.sel_get_out = True
            self.pixmap_sel.draw_drawable(self.gc, self.pixmap_temp, 0,0,0,0, WIDTH, HEIGHT)
        if state & gtk.gdk.BUTTON1_MASK:
            self.pixmap_temp.draw_drawable(self.gc, self.pixmap, 0,0,0,0, WIDTH, HEIGHT)
        widget.queue_draw()
        self.desenha = True  
           
        
    def mousemove(self,widget,event):
        """Make the Area object (GtkDrawingArea) recognize that the mouse is moving.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)
        widget -- the Area object (GtkDrawingArea)
        event -- GdkEvent

        """
        x , y, state = event.window.get_pointer()   
        coords = int(x), int(y)
                        
        if state & gtk.gdk.BUTTON1_MASK and self.pixmap != None:
            #eraser
            if self.tool == 'eraser':
                self.d.eraser(widget, coords, self.line_size, self.eraser_shape)
            #brush
            elif self.tool == 'brush':
                self.d.brush(widget, coords, self.line_size, self.brush_shape)
            if self.desenha:
                # line
                if self.tool == 'line':
                    self.configure_line(self.line_size)
                    self.d.line(widget, coords) 
                # pencil
                elif self.tool == 'pencil':
                    self.configure_line(self.line_size)
                    self.d.pencil(widget, coords)       
                # ellipse
                elif self.tool == 'ellipse':
                    self.configure_line(self.line_size)
                    self.d.circle(widget,coords)    
                # rectangle
                elif self.tool == 'rectangle':
                    self.configure_line(self.line_size)
                    self.d.square(widget,coords)    
                # selection
                elif self.tool == 'marquee-rectangular' and not self.selmove:
                    self.d.selection(widget,coords)                     
                # selection
                elif self.tool == 'marquee-rectangular' and self.selmove:
                    self.d.moveSelection(widget, coords)
                #polygon    
                elif self.tool == 'polygon':
                    self.configure_line(self.line_size)
                    self.d.polygon(widget, coords)  
                #triangle
                elif self.tool == 'triangle':
                    self.configure_line(self.line_size)
                    self.d.triangle(widget,coords)
                #trapezoid
                elif self.tool == 'trapezoid':
                    self.configure_line(self.line_size)
                    self.d.trapezoid(widget,coords)
                #arrow
                elif self.tool == 'arrow':
                    self.configure_line(self.line_size)
                    self.d.arrow(widget,coords)
                #parallelogram
                elif self.tool == 'parallelogram':
                    self.configure_line(self.line_size)
                    self.d.parallelogram(widget,coords)
                #star
                elif self.tool == 'star':
                    self.configure_line(self.line_size)
                    self.d.star(widget,coords)

    def mouseup(self,widget,event): 
        """Make the Area object (GtkDrawingArea) recognize that the mouse was released.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)
        widget -- the Area object (GtkDrawingArea)
        event -- GdkEvent

        """
        if self.desenha == True:
            # line
            if self.tool == 'line':
                self.pixmap.draw_line(self.gc_line,self.oldx,self.oldy, int (event.x), int(event.y))                
                widget.queue_draw()
                self.enableUndo(widget)
            # ellipse
            elif self.tool == 'ellipse':
                self.pixmap.draw_arc(self.gc, True, self.newx, self.newy, self.newx_, self.newy_, 0, 360*64)
                self.pixmap.draw_arc(self.gc_line, False, self.newx, self.newy, self.newx_, self.newy_, 0, 360*64)
                widget.queue_draw()
                self.enableUndo(widget)
            # rectangle
            elif self.tool == 'rectangle':    
                self.pixmap.draw_rectangle(self.gc, True, self.newx,self.newy, self.newx_,self.newy_)
                self.pixmap.draw_rectangle(self.gc_line, False, self.newx,self.newy, self.newx_,self.newy_)
                widget.queue_draw()
                self.enableUndo(widget)
            # selection
            elif self.tool == 'marquee-rectangular':
            # FIXME: Adicionar cursor formato selecao
                if self.selmove == False:
                    self.pixmap_temp.draw_drawable(self.gc,self.pixmap,  0 , 0 ,0,0, WIDTH, HEIGHT)
                    self.pixmap_sel.draw_drawable(self.gc,self.pixmap,  0 , 0 ,0,0, WIDTH, HEIGHT)#avoid blink
                    self.sx = int (event.x)
                    self.sy = int(event.y)
                    self.window.set_cursor(gtk.gdk.Cursor(gtk.gdk.FLEUR))
                    self.selmove = True
                    self.sel_get_out = False
                elif self.selmove and self.sel_get_out: #get out of the func selection
                    self.pixmap.draw_drawable(self.gc, self.pixmap_temp, 0,0,0,0, WIDTH, HEIGHT)
                    self.selmove = False
                    self.window.set_cursor(gtk.gdk.Cursor(gtk.gdk.TCROSS))
                    self.oldx = event.x
                    self.oldy = event.y
                    self.enableUndo(widget)
                # polygon
            elif self.tool == 'polygon':
                if self.polygon_start:
                    self.enableUndo(widget)
                    self.pixmap.draw_line(self.gc_line,self.oldx,self.oldy, int (event.x), int( event.y ))
                    self.lastx = event.x
                    self.lasty = event.y
                    self.firstx = self.oldx
                    self.firsty = self.oldy
                    self.polygon_start = False
                else:
                    self.dx = math.fabs(event.x - self.firstx)
                    self.dy = math.fabs(event.y - self.firsty)
                    if (self.dx < 20) & (self.dy < 20):
                        self.pixmap.draw_line(self.gc_line,int (self.firstx), int (self.firsty), int (self.lastx), int (self.lasty))
                        self.polygon_start = True
                        self.undo_times -= 1#destroy the undo screen of polygon start 
                        self.enableUndo(widget)
                    else:   
                        self.pixmap.draw_line(self.gc_line,int (self.lastx),int (self.lasty), int (event.x), int( event.y ))
                    self.lastx = event.x
                    self.lasty = event.y
                widget.queue_draw() 

            elif self.tool == 'pencil': #to undo pencil
                widget.queue_draw() 
                self.enableUndo(widget)

            #bucket
            elif self.tool == 'bucket':
            # New algorithm. See Desenho.py
                width, height = self.window.get_size()
                fill(self.pixmap, self.gc, int(event.x), int(event.y), width, height, self.color_dec)
                widget.queue_draw()
                self.enableUndo(widget)
                
            elif self.tool == 'triangle':
                self.pixmap.draw_polygon(self.gc, True, self.d.points)
                self.pixmap.draw_polygon(self.gc_line, False, self.d.points)
                widget.queue_draw()
                self.enableUndo(widget)

            elif self.tool == 'trapezoid':
                self.pixmap.draw_polygon(self.gc, True, self.d.points)
                self.pixmap.draw_polygon(self.gc_line, False, self.d.points)
                widget.queue_draw()
                self.enableUndo(widget)

            elif self.tool == 'arrow':
                self.pixmap.draw_polygon(self.gc, True, self.d.points)
                self.pixmap.draw_polygon(self.gc_line, False, self.d.points)
                widget.queue_draw()
                self.enableUndo(widget)

            elif self.tool == 'parallelogram':
                self.pixmap.draw_polygon(self.gc, True, self.d.points)
                self.pixmap.draw_polygon(self.gc_line, False, self.d.points)
                widget.queue_draw()
                self.enableUndo(widget)

            elif self.tool == 'star':
                self.pixmap.draw_polygon(self.gc, True, self.d.points)
                self.pixmap.draw_polygon(self.gc_line, False, self.d.points)
                widget.queue_draw()
                self.enableUndo(widget)

        if self.tool == 'brush' or self.tool == 'eraser':
            widget.queue_draw() 
            self.enableUndo(widget)
        self.desenha = False
        
        
    #this func make a basic Undo
    def undo(self):
        """Undo the last drawing change.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)

        """
        logging.debug('Area.undo(self)')
        
        if self.first_undo:#if is the first time you click on UNDO
            self.undo_times -= 1
            
        #print "Undo no.%d" %(self.undo_times)
        if self.undo_times >0 : 
            self.undo_times -= 1
            self.redo_times += 1
            try: #to not try paint someting wrong
                #print "Drawing undo[%d]" %(self.undo_times)
                self.pixmap.draw_drawable(self.gc, self.undo_list[self.undo_times], 0,0,0,0, WIDTH, HEIGHT)
            except:
                print "Can't draw"
                pass
            self.queue_draw()
        else:   
            self.undo_times = 0
            
        self.first_undo=False
        self.undo_surf = True


	    #special case for func polygon
        if self.tool == 'polygon':		
                self.polygon_start = True #start the polygon again
        
        # emits 'undo' and 'redo' signals only in case of first action,
        # (first undo or first redo) or no actions available
        # FIXME: this way, things work strangely; emiting signals everytime
#         if self.undo_times <= 1:
#             self.emit('undo')
#         if self.redo_times <= 1:
#             self.emit('redo')
        self.emit('undo')
        self.emit('redo')
        
    def redo(self):
        """Redo the last undo operation.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)

        """
        logging.debug('Area.redo(self)')
        
        #print "REDO no.%d" %(self.redo_times)
        if  (self.redo_times>0):
            self.redo_times -= 1
            self.undo_times += 1
            
            try: #to not try paint someting wrong 
                #print "Drawing undo[%d]" %(self.undo_times)
                self.pixmap.draw_drawable(self.gc, self.undo_list[self.undo_times], 0,0,0,0, WIDTH, HEIGHT)
            except:
                print "Can't draw"
                self.undo_times-=1
        self.queue_draw()
        
        # emits 'undo' and 'redo' signals only in case of first action,
        # (first undo or first redo) or no actions available
        # FIXME: this way, things work strangely; emiting signals everytime
#         if self.undo_times <= 1:
#             self.emit('undo')
#         if self.redo_times <= 1:
#             self.emit('redo')
        self.emit('undo')
        self.emit('redo')
            
    def enableUndo(self,widget):
        """Keep the last change in a list for Undo/Redo commands.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)
        widget -- the Area object (GtkDrawingArea)

        """
        logging.debug('Area.enableUndo(self,widget)')
        
        if self.undo_surf:
            self.undo_times += 1
        
        self.undo_list.append(None)#alloc memory
        self.undo_list[self.undo_times] = gtk.gdk.Pixmap(widget.window, WIDTH, HEIGHT, -1) #define type
        self.undo_list[self.undo_times].draw_drawable(self.gc,self.pixmap,0,0,0,0, WIDTH, HEIGHT) #copy workarea
        self.undo_times += 1
        self.redo_times = 0 
        self.first_undo = True
        self.undo_surf = False
        
        #this is the part where we can limit the steps of undo/redo     
        if self.undo_times==12:
           self.undo_list.pop(0)
           self.undo_times-=1
           #print "estourou"
        
        # emits 'undo' and 'redo' signals only in case of first action,
        # (first undo or first redo) or no actions available
        # FIXME: this way, things work strangely; emiting signals everytime
#         if self.undo_times <= 1:
#             self.emit('undo')
#         if self.redo_times <= 1:
#             self.emit('redo')
        self.emit('undo')
        self.emit('redo')
        
        
    def copy(self):
        """ Copy Image.
        When the tool selection is working make the change the copy of selectioned area"""
        logging.debug('Area.copy(self)')
        
        if self.selmove:
    
            if self.sx > self.oldx:
                x = self.oldx
            else:
                x = self.sx
            
            if self.sy > self.oldy:
                y = self.oldy
            else:
                y = self.sy

            w = self.sx - self.oldx
            if w < 0:
                w = - w
                
            h = self.sy - self.oldy         
            if h < 0:
                h = - h

            self.pixmap_copy = gtk.gdk.Pixmap(self.window, w, h, -1)
            self.pixmap_copy.draw_drawable(self.gc, self.pixmap, x, y, 0, 0, w, h)  
        else :
            print "Please select some area first"
            self.pixmap_copy == None
            
    def past(self):
        """ Past image.
        Past image that is in pixmap_copy"""
        logging.debug('Area.past(self)')
        
        if self.pixmap_copy != None :
            

            w, h = self.pixmap_copy.get_size()
            
            #to draw everthing done until this moment
            #self.pixmap.draw_drawable(self.gc, self.pixmap_temp, 0,0,0,0, WIDTH, HEIGHT)
            
            #to get out of sel func
            if self.tool == 'marquee-rectangular': 
                self.pixmap_sel.draw_drawable(self.gc, self.pixmap_copy, 0,0,0,0, w, h)   
                #self.enableUndo(self)
                self.sel_get_out = True
                self.selmove = False
                
            #to draw the new area on screen
            self.pixmap.draw_drawable(self.gc, self.pixmap_copy, 0,0,0,0, w, h)  
            
                      
            self.enableUndo(self)     
            self.queue_draw()
        else :
            print "Nothing is copied yet"
            
    def _set_fill_color(self, color):
        """Set fill color.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)
        color -- a gdk.Color object

        """
        logging.debug('Area._set_fill_color(self, color)')
        
        self.gc.set_foreground(color)
        
 
    def _set_stroke_color(self, color):
        """Set stroke color.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)
        color -- a gdk.Color object

        """
        logging.debug('Area._set_stroke_color(self, color)')
        
        self.gc_line.set_foreground(color)
        self.gc_line.set_line_attributes(1, gtk.gdk.LINE_ON_OFF_DASH, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)  
        self.gc_brush.set_foreground(color)
        self.color_dec = color.pixel

    def _set_grayscale(self,widget):
        """Apply grayscale effect.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)

        """
        logging.debug('Area._set_grayscale(self,widget)')
        
        pix = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB, False, 8, WIDTH, HEIGHT)
        pix_ = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB, False, 8, WIDTH, HEIGHT)
        pix.get_from_drawable(self.pixmap, gtk.gdk.colormap_get_system(), 0, 0, 0, 0, WIDTH, HEIGHT)
        pix.saturate_and_pixelate(pix_, 0 ,0)

        self.pixmap.draw_pixbuf(self.gc, pix_, 0, 0, 0, 0, WIDTH, HEIGHT, dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)

        self.pixmap_temp.draw_pixbuf(self.gc, pix_, 0, 0, 0, 0, WIDTH, HEIGHT, dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
        self.queue_draw()
        self.enableUndo(widget)

    def _rotate_left(self):
        """Rotate the image.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)

        """
        logging.debug('Area._rotate_left(self)')
        
        pix = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB, False, 8, WIDTH, HEIGHT)
        pix_ = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB, False, 8, WIDTH, HEIGHT)
        pix.get_from_drawable(self.pixmap, gtk.gdk.colormap_get_system(), 0, 0, 0, 0, -1, -1)
        pix_ = pix.rotate_simple(gtk.gdk.PIXBUF_ROTATE_COUNTERCLOCKWISE)
        self.pixmap.draw_pixbuf(self.gc, pix_, 0, 0, 0, 0, width=-1, height=-1, dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
        self.queue_draw()
        
    def can_undo(self):
        '''
        Indicate if is there some action to undo
        '''
        logging.debug('Area.can_undo(self)')
        
        if self.undo_times < 1:
            return False
        else:
            return True
            
    def can_redo(self):
        '''
        Indicate if is there some action to redo
        '''
        logging.debug('Area.can_redo(self)')
        
        if self.redo_times < 1:
            return False
        else:
            return True
        
