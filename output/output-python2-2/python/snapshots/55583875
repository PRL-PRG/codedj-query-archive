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
import gtk, gobject, logging, os
# import sys, socket
# from gtk import gdk
import math
import pango
from fill import *
# import Image
# import StringIO
from Desenho import Desenho

WIDTH = 800
HEIGHT = 600

class Area(gtk.DrawingArea):

    __gsignals__ = {
        'undo' : (gobject.SIGNAL_ACTION, gobject.TYPE_NONE, ([])),
        'redo' : (gobject.SIGNAL_ACTION, gobject.TYPE_NONE, ([])),
        'action-saved' : (gobject.SIGNAL_ACTION, gobject.TYPE_NONE, ([])),
        #TODO: this signal still not used.
#         'copy' : (gobject.SIGNAL_ACTION, gobject.TYPE_NONE, ([])),
        'selected' : (gobject.SIGNAL_ACTION, gobject.TYPE_NONE, ([])),
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
        self.polygon_start = True
        self.points = []
        self.gc = None
        self.gc_rainbow = None
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
        self.last = -1, -1
        self.rainbow_counter = 0
                
        self.font = pango.FontDescription('Sans 9')
        self._set_selection_bounds(0,0,0,0)
        
        #start of UNDO and REDO
        self.first_undo = True
        self.undo_surf = False
        self.undo_times = 0
        self.redo_times = 0
        self.undo_list=[]#pixmaps list to Undo func
        
        # Number of sides for regular polygon
        self.polygon_sides = 5
        
        # Shapes will be filled or not?
        self.fill = True

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
        self.gc_rainbow = widget.window.new_gc()
        
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
        width, height = self.window.get_size()
        # text
        coords = int(event.x), int(event.y)
        if self.tool is 'text':
            self.d.text(widget,event)
            
        # This fixes a bug that made the text viewer get stuck in the canvas
        elif self.estadoTexto is 1:
            try:
            # This works for a gtk.Entry
                text = self.janela._textview.get_text()
            except:
            # This works for a gtk.TextView
                buf = self.janela._textview.get_buffer()
                start, end = buf.get_bounds()
                text = buf.get_text(start, end)
            if text is not None:
                self.d.text(widget,event)
            self.estadoTexto = 0
            self.janela._textview.hide()
            
        if not self.selmove or self.tool != 'marquee-rectangular':
            self.oldx = int(event.x)
            self.oldy = int(event.y)
        if self.selmove and self.tool != 'marquee-rectangular': #get out of the func selection
            self.pixmap.draw_drawable(self.gc, self.pixmap_temp, 0,0,0,0, width, height)
            self.selmove = False
            self.enableUndo(widget)
        if self.tool == 'eraser':
            self.last = -1, -1
            self.d.eraser(widget, coords, self.last, self.line_size, self.eraser_shape)
            self.last = coords
        if self.tool == 'brush':
            self.last = -1, -1
            self.d.brush(widget, coords, self.last, self.line_size, self.brush_shape)
            self.last = coords
        if self.tool == 'rainbow':
            self.last = -1, -1
            self.d.rainbow(widget, coords, self.last, self.rainbow_counter,self.line_size, self.brush_shape)
            self.last = coords
            
        x , y, state = event.window.get_pointer()
        x0, y0, x1, y1 = self.get_selection_bounds()
        
        if (state & gtk.gdk.BUTTON3_MASK) or not (x0<x<x1 and y0<y<y1):
            self.sel_get_out = True
            self.pixmap.draw_drawable(self.gc, self.pixmap_temp, 0,0,0,0, width, height)
            self.pixmap_sel.draw_drawable(self.gc, self.pixmap_temp, 0,0,0,0, width, height)
        if state & gtk.gdk.BUTTON1_MASK:
            self.pixmap_temp.draw_drawable(self.gc, self.pixmap, 0,0,0,0, width, height)
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
                self.d.eraser(widget, coords, self.last, self.line_size, self.eraser_shape)
                self.last = coords
            #brush
            elif self.tool == 'brush':
                self.d.brush(widget, coords, self.last, self.line_size, self.brush_shape)
                self.last = coords
            elif self.tool == 'rainbow':
                self.d.rainbow(widget, coords, self.last, self.rainbow_counter,self.line_size, self.brush_shape)
                self.rainbow_counter += 1
                if self.rainbow_counter > 11:
                    self.rainbow_counter = 0
                self.last = coords
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
                    #self.d.circle(widget,coords,True,True)
                    self.d.circle(widget,coords,True,self.fill)
                # rectangle
                elif self.tool == 'rectangle':
                    self.configure_line(self.line_size)
                    #self.d.square(widget,coords,True,True)
                    self.d.square(widget,coords,True,self.fill)
                # selection
                elif self.tool == 'marquee-rectangular' and not self.selmove:
                    x1, y1, x2, y2 = self.d.selection(widget,coords,True,False)
                    self._set_selection_bounds(x1, y1, x2, y2)                   
                # selection
                elif self.tool == 'marquee-rectangular' and self.selmove:
                    self.d.moveSelection(widget,coords)
                #polygon    
                elif self.tool == 'polygon':
                    self.configure_line(self.line_size)
                    #self.d.polygon(widget,coords,True,False)
                    self.d.polygon(widget,coords,True,self.fill)
                #triangle
                elif self.tool == 'triangle':
                    self.configure_line(self.line_size)
                    #self.d.triangle(widget,coords,True,True)
                    self.d.triangle(widget,coords,True,self.fill)
                #trapezoid
                elif self.tool == 'trapezoid':
                    self.configure_line(self.line_size)
                    #self.d.trapezoid(widget,coords,True,True)
                    self.d.trapezoid(widget,coords,True,self.fill)
                #arrow
                elif self.tool == 'arrow':
                    self.configure_line(self.line_size)
                    #self.d.arrow(widget,coords,True,True)
                    self.d.arrow(widget,coords,True,self.fill)
                #parallelogram
                elif self.tool == 'parallelogram':
                    self.configure_line(self.line_size)
                    #self.d.parallelogram(widget,coords,True,True)
                    self.d.parallelogram(widget,coords,True,self.fill)
                #star
                elif self.tool == 'star':
                    self.configure_line(self.line_size)
                    #self.d.star(widget,coords,True,True)
                    self.d.star(widget,coords,True,self.fill)
                #polygon regular
                elif self.tool == 'polygon_regular':
                    self.configure_line(self.line_size)
                    #n = 7
                    #self.d.polygon_regular(widget,coords,self.polygon_sides,True,True)
                    self.d.polygon_regular(widget,coords,self.polygon_sides,True,self.fill)
                #Heart
                elif self.tool == 'heart':
                    self.configure_line(self.line_size)
                    #self.d.heart(widget,coords,True,True)
                    self.d.heart(widget,coords,True,self.fill)


    def mouseup(self,widget,event): 
        """Make the Area object (GtkDrawingArea) recognize that the mouse was released.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)
        widget -- the Area object (GtkDrawingArea)
        event -- GdkEvent

        """
        coords = int(event.x), int(event.y)
        width, height = self.window.get_size()
        if self.desenha == True:
            # line
            if self.tool == 'line':
                self.pixmap.draw_line(self.gc_line,self.oldx,self.oldy, int (event.x), int(event.y))                
                widget.queue_draw()
                self.enableUndo(widget)
            # ellipse
            elif self.tool == 'ellipse':
                #self.d.circle(widget,coords,False,True)
                self.d.circle(widget,coords,False,self.fill)
                self.enableUndo(widget)
            # rectangle
            elif self.tool == 'rectangle':
                #self.d.square(widget,coords,False,True)
                self.d.square(widget,coords,False,self.fill)
                self.enableUndo(widget)
            # selection
            elif self.tool == 'marquee-rectangular':
            # FIXME: Adicionar cursor formato selecao
                if self.selmove == False:
                    self.pixmap_temp.draw_drawable(self.gc,self.pixmap, 0,0,0,0, width, height)
                    self.pixmap_sel.draw_drawable(self.gc,self.pixmap, 0,0,0,0, width, height)#avoid blink
                    self.sx = int (event.x)
                    self.sy = int(event.y)
                    self.window.set_cursor(gtk.gdk.Cursor(gtk.gdk.FLEUR))
                    self.selmove = True
                    self.sel_get_out = False
                elif self.sel_get_out: #get out of the func selection
                    self.pixmap.draw_drawable(self.gc, self.pixmap_temp, 0,0,0,0, width, height)
                    self.selmove = False
                    self.window.set_cursor(gtk.gdk.Cursor(gtk.gdk.TCROSS))
                    self.oldx = event.x
                    self.oldy = event.y
                    self.enableUndo(widget)
                self.emit('selected')
            # polygon
            elif self.tool == 'polygon':
                #self.d.polygon(widget, coords, False, False)
                self.d.polygon(widget, coords, False, self.fill)
            #to undo pencil
            elif self.tool == 'pencil':
                widget.queue_draw() 
                self.enableUndo(widget)
            #bucket
            elif self.tool == 'bucket':
                width, height = self.window.get_size()
                fill(self.pixmap, self.gc, coords[0], coords[1], width, height, self.gc_line.foreground.pixel)
                widget.queue_draw()
                self.enableUndo(widget)
            #triangle
            elif self.tool == 'triangle':
                #self.d.triangle(widget,coords,False,True)
                self.d.triangle(widget,coords,False,self.fill)
                self.enableUndo(widget)
            #trapezoid
            elif self.tool == 'trapezoid':
                #self.d.trapezoid(widget,coords,False,True)
                self.d.trapezoid(widget,coords,False,self.fill)
                self.enableUndo(widget)
            #arrow
            elif self.tool == 'arrow':
                #self.d.arrow(widget,coords,False,True)
                self.d.arrow(widget,coords,False,self.fill)
                self.enableUndo(widget)
            #parallelogram
            elif self.tool == 'parallelogram':
                #self.d.parallelogram(widget,coords,False,True)
                self.d.parallelogram(widget,coords,False,self.fill)
                self.enableUndo(widget)
            #star
            elif self.tool == 'star':
                #self.d.star(widget,coords,False,True)
                self.d.star(widget,coords,False,self.fill)
                self.enableUndo(widget)
            #polygon regular
            elif self.tool == 'polygon_regular':
                #n = 7
                #self.d.polygon_regular(widget,coords,self.polygon_sides,False,True)
                self.d.polygon_regular(widget,coords,self.polygon_sides,False,self.fill)
                self.enableUndo(widget)
            #heart
            elif self.tool == 'heart':
                #self.d.heart(widget,coords,False,True)
                self.d.heart(widget,coords,False,self.fill)
                self.enableUndo(widget)

        if self.tool == 'brush' or self.tool == 'eraser' or self.tool == 'rainbow':
            self.last = -1, -1
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
        width, height = self.window.get_size()
        
        if self.first_undo:#if is the first time you click on UNDO
            self.undo_times -= 1
            
        #print "Undo no.%d" %(self.undo_times)
        if self.undo_times >0 : 
            self.undo_times -= 1
            self.redo_times += 1
            try: #to not try paint someting wrong
                #print "Drawing undo[%d]" %(self.undo_times)
                self.pixmap.draw_drawable(self.gc, self.undo_list[self.undo_times], 0,0,0,0, width, height)
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
        #self.emit('redo')
        
    def redo(self):
        """Redo the last undo operation.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)

        """
        logging.debug('Area.redo(self)')
        width, height = self.window.get_size()
        
        #print "REDO no.%d" %(self.redo_times)
        if  (self.redo_times>0):
            self.redo_times -= 1
            self.undo_times += 1
            
            try: #to not try paint someting wrong 
                #print "Drawing undo[%d]" %(self.undo_times)
                self.pixmap.draw_drawable(self.gc, self.undo_list[self.undo_times], 0,0,0,0, width, height)
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
        #self.emit('undo')
        self.emit('redo')
            
    def enableUndo(self,widget):
        """Keep the last change in a list for Undo/Redo commands.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)
        widget -- the Area object (GtkDrawingArea)

        """
        logging.debug('Area.enableUndo(self,widget)')
        width, height = self.window.get_size()
        
        if self.undo_surf:
            self.undo_times += 1
        
        self.undo_list.append(None)#alloc memory
        self.undo_list[self.undo_times] = gtk.gdk.Pixmap(widget.window, width, height, -1) #define type
        self.undo_list[self.undo_times].draw_drawable(self.gc,self.pixmap,0,0,0,0, width, height) #copy workarea
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
        #self.emit('undo')
        #self.emit('redo')
        self.emit('action-saved')
        
        
    def copy(self):
        """ Copy Image.
        When the tool selection is working make the change the copy of selectioned area
        
        Keyword arguments:
        self -- the Area object (GtkDrawingArea)
        """
        clipBoard = gtk.Clipboard()
        tempPath = os.path.join("/tmp", "tempFile")
        tempPath = os.path.abspath(tempPath)  
        
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

            pixbuf_copy = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB, True, 8, w, h)
            pixbuf_copy.get_from_drawable(self.pixmap, gtk.gdk.colormap_get_system(), x, y, 0, 0, w, h)

            pixbuf_copy.save(tempPath,'png')
            #gtk.Clipboard().set_with_data( [('text/uri-list', 0, 0)], self._copyGetFunc, self._copyClearFunc, tempPath )
            clipBoard.set_image(pixbuf_copy)
            
        else :
            print "Please select some area first"
            
    def _copyGetFunc(  self, clipboard, selection_data, info, data ):
        selection_data.set( "text/uri-list", 8, data)
    
    def _copyClearFunc( self, clipboard, data ):
        if (data != None):
            if (os.path.exists(data)):
                os.remove( data )
        data = None
	       
    def past(self):
        """ Past image.
        Past image that is in pixmap_copy
        
        Keyword arguments:
        self -- the Area object (GtkDrawingArea)
        """
        width, height = self.window.get_size()
        
        tempPath = os.path.join("/tmp", "tempFile")
        tempPath = os.path.abspath(tempPath)
        
        clipBoard = gtk.Clipboard()
        
        if clipBoard.wait_is_image_available():
            pixbuf_copy = clipBoard.wait_for_image()
            self.pixmap.draw_pixbuf(self.gc, pixbuf_copy, 0, 0, 0, 0, width=-1, height=-1, dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
            self.queue_draw()
        else:
            self.d.loadImage(tempPath, self)
            
        #to get out of sel func
        if self.tool == 'marquee-rectangular': 
            self.pixmap_sel.draw_drawable(self.gc, self.pixmap_copy, 0,0,0,0, width, height)   
            #self.enableUndo(self)
            self.sel_get_out = True
            self.selmove = False
            
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

    def _set_grayscale(self,widget):
        """Apply grayscale effect.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)

        """
        logging.debug('Area._set_grayscale(self,widget)')
        width, height = self.window.get_size()
        
        pix = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB, False, 8, width, height)
        pix_ = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB, False, 8, width, height)
        pix.get_from_drawable(self.pixmap, gtk.gdk.colormap_get_system(), 0, 0, 0, 0, width, height)
        pix.saturate_and_pixelate(pix_, 0 ,0)

        self.pixmap.draw_pixbuf(self.gc, pix_, 0, 0, 0, 0, width, height, dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)

        self.pixmap_temp.draw_pixbuf(self.gc, pix_, 0, 0, 0, 0, width, height, dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
        self.queue_draw()
        self.enableUndo(widget)

    def _pixbuf2Image(self, pb):
        width,height = pb.get_width(),pb.get_height()
        return Image.fromstring("RGB",(width,height),pb.get_pixels() )
        
    def _image2pixbuf(self, im):  
        file1 = StringIO.StringIO()  
        im.save(file1, "ppm")  
        contents = file1.getvalue()  
        file1.close()  
        loader = gtk.gdk.PixbufLoader("pnm")  
        loader.write(contents, len(contents))  
        pixbuf = loader.get_pixbuf()  
        loader.close()  
        return pixbuf  
    
    def _rotate_left(self, widget):
        """Rotate the image.

        Keyword arguments:
        self -- the Area object (GtkDrawingArea)

        """
        logging.debug('Area._rotate_left(self)')

        x1, y1, x2, y2 = self.get_selection_bounds()
        #x1, y1, x2, y2 = 0, 0, 100, 200
        if self.selmove:
            pix = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB, False, 8, x2 - x1, y2 - y1)
            pix_ = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB, False, 8, x2 - x1, y2 - y1)
            pix.get_from_drawable(self.pixmap, gtk.gdk.colormap_get_system(), x1, y1, 0, 0, x2 - x1, y2 - y1)

            im = self._pixbuf2Image(pix)
            #pix_ = pix.rotate_simple(gtk.gdk.PIXBUF_ROTATE_CLOCKWISE)

            im_ = im.rotate(90)

            pix_ = self._image2pixbuf(im_)

            self.pixmap.draw_pixbuf(self.gc, pix_, 0, 0, x1, y1, width=-1, height=-1, dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
            self.queue_draw()
            self.enableUndo(widget)
                
        else :
            print "Please select some area first"
        
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
            
    def _set_selection_bounds(self, x1, y1, x2, y2):
        self._selection_corners = (x1, y1, x2, y2)
     
    def get_selection_bounds(self):
        return self._selection_corners[0], self._selection_corners[1], self._selection_corners[2], self._selection_corners[3]

    def loadImage(self, name, widget, load_selected):
        """Load an image.

        Keyword arguments:
        self -- Area.area instance
        name -- string (image file path)

        """
        pixbuf = gtk.gdk.pixbuf_new_from_file(name) 
        size = (int)(pixbuf.get_width()), (int)(pixbuf.get_height())
        
        self.pixmap.draw_pixbuf(self.gc, pixbuf, 0, 0, 0, 0, size[0], size[1], dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
        self.pixmap_temp.draw_pixbuf(self.gc, pixbuf, 0, 0, 0, 0, size[0], size[1], dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
        self.pixmap_sel.draw_pixbuf(self.gc, pixbuf, 0, 0, 0, 0, size[0], size[1], dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
        
        
        
        if not load_selected :
            self.enableUndo(widget)
        else :
            self.selmove = True
            self.desenha = True
            self.oldx, self.oldy = 0,0
            self.d.selection(self, size, True, False)
            self.pixmap_sel.draw_rectangle(self.gc_selection, True ,0,0,size[0],size[1])
            self.sx, self.sy = size
            self.tool = 'marquee-rectangular'
            self.window.set_cursor(gtk.gdk.Cursor(gtk.gdk.FLEUR)) 
            
        self.queue_draw()
        
    def clear(self):
        self.d.clear()
        self.enableUndo(self)
        
