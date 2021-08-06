# -*- coding: utf-8 -*-
import  pygtk
pygtk.require('2.0')
import gtk
import sys, gobject, socket
from gtk import gdk
import math
import pango


WIDTH = 1195
HEIGHT = 800

class Desenho:
    def __init__(self, d_):
        """Initialize Desenho object.

        Keyword arguments:
        self -- Desenho.Desenho instance
        d_ -- Area object (GtkDrawingArea)

        """
        self.d = d_
        
    def line(self, widget, coords):
        """Draw line.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """
        self.d.pixmap_temp.draw_drawable(self.d.gc,self.d.pixmap,  0 , 0 ,0,0, WIDTH, HEIGHT)
        self.d.pixmap_temp.draw_line(self.d.gc_line,self.d.oldx,self.d.oldy,coords[0],coords[1])
        self.d.newx = coords[0] 
        self.d.newy = coords[1]
        widget.queue_draw()
    
    def eraser(self, widget, coords, size = 30, shape = 'circle'):
        """Erase part of the drawing.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple
        size -- integer (default 30)
        shape -- string (default 'circle')

        """
        self.d.desenha = False
        if(shape == 'circle'):
            self.d.pixmap.draw_arc(self.d.gc_eraser, True, coords[0], coords[1], size, size, 0, 360*64)
            self.d.pixmap_temp.draw_arc(self.d.gc_eraser, True, coords[0], coords[1], size, size, 0, 360*64)
        if(shape == 'square'):
            self.d.pixmap.draw_rectangle(self.d.gc_borracha, True, coords[0], coords[1], size, size)
            self.d.pixmap_temp.draw_rectangle(self.d.gc_borracha, True, coords[0], coords[1], size, size)
        self.d.oldx = coords[0]
        self.d.oldy = coords[1]
        widget.queue_draw()
        
    def brush(self, widget, coords, size = 5, shape = 'circle'):
        """Paint with brush.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple
        size -- integer (default 30)
        shape -- string (default 'circle')

        """
        self.d.desenha = False
        if(shape == 'circle'):
            self.d.pixmap.draw_arc(self.d.gc_brush, True, coords[0], coords[1], size, size, 0, 360*64)
            self.d.pixmap_temp.draw_arc(self.d.gc_brush, True, coords[0], coords[1], size, size, 0, 360*64)
        if(shape == 'square'):
            self.d.pixmap.draw_rectangle(self.d.gc_brush, True, coords[0], coords[1], size, size)
            self.d.pixmap_temp.draw_rectangle(self.d.gc_brush, True, coords[0], coords[1], size, size)
        self.d.oldx = coords[0]
        self.d.oldy = coords[1]
        widget.queue_draw()
    
    def square(self, widget, coords):
        """Draw a square.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """
        widget.queue_draw()     

        if coords[0] > WIDTH:
            coords0 = WIDTH
        else:
            coords0 = coords[0]
            
        if coords [1] > HEIGHT:
            coords1 = HEIGHT
        else:
            coords1 = coords[1]
            
        self.d.newx_ = coords0 - self.d.oldx
        self.d.newy_ = coords1 - self.d.oldy

        if self.d.newx_ >= 0:
            self.d.newx = self.d.oldx   
        else:   
            if coords0 > 0:
                self.d.newx = coords0
                self.d.newx_ = - self.d.newx_
            else:
                self.d.newx = 0
                self.d.newx_ = self.d.oldx
                    
        if self.d.newy_ >= 0:
            self.d.newy = self.d.oldy   
        else:               
            if coords1 > 0:
                self.d.newy_ = - self.d.newy_
                self.d.newy = coords1
            else:
                self.d.newy = 0
                self.d.newy_ = self.d.oldy
                
        self.d.pixmap_temp.draw_drawable(self.d.gc,self.d.pixmap,  0 , 0 ,0,0, WIDTH, HEIGHT)
        self.d.pixmap_temp.draw_rectangle(self.d.gc, True ,self.d.newx,self.d.newy,self.d.newx_,self.d.newy_)
        self.d.pixmap_temp.draw_rectangle(self.d.gc_line, False ,self.d.newx,self.d.newy,self.d.newx_,self.d.newy_)

    
    def triangle(self, widget, coords):
        """Draw a triangle.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """
        widget.queue_draw()     

        if coords[0] > WIDTH:
            coords0 = WIDTH
        else:
            coords0 = coords[0]
            
        if coords [1] > HEIGHT:
            coords1 = HEIGHT
        else:
            coords1 = coords[1]

        if coords0 < 0:
            coords0 = 0

        if coords1 < 0:
            coords1 = 0

        self.points = [(self.d.oldx, self.d.oldy), (self.d.oldx+int((coords0-self.d.oldx)/2), coords1), (coords0,self.d.oldy)]                
        self.d.pixmap_temp.draw_drawable(self.d.gc,self.d.pixmap,  0 , 0 ,0,0, WIDTH, HEIGHT)
        self.d.pixmap_temp.draw_polygon(self.d.gc, True, self.points)
        self.d.pixmap_temp.draw_polygon(self.d.gc_line, False, self.points)

    def trapezoid(self, widget, coords):
        """Draw a trapezoid.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """
        widget.queue_draw()     

        if coords[0] > WIDTH:
            coords0 = WIDTH
        else:
            coords0 = coords[0]
            
        if coords[1] > HEIGHT:
            coords1 = HEIGHT
        else:
            coords1 = coords[1]

        if coords0 < 0:
            coords0 = 0

        if coords1 < 0:
            coords1 = 0

        dif = int((coords0 - self.d.oldx)/4)
        self.points = [(self.d.oldx, self.d.oldy), (self.d.oldx+dif, coords1), (coords0-dif, coords1) , (coords0,self.d.oldy)]                
        self.d.pixmap_temp.draw_drawable(self.d.gc,self.d.pixmap,  0 , 0 ,0,0, WIDTH, HEIGHT)
        self.d.pixmap_temp.draw_polygon(self.d.gc, True, self.points)
        self.d.pixmap_temp.draw_polygon(self.d.gc_line, False, self.points)


    def selection(self, widget, coords):
        """Make a selection.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """     
        widget.queue_draw()     

        if coords[0] > WIDTH:
            coords0 = WIDTH
        else:
            coords0 = coords[0]
            
        if coords [1] > HEIGHT:
            coords1 = HEIGHT
        else:
            coords1 = coords[1]
            
        self.d.newx_ = coords0 - self.d.oldx
        self.d.newy_ = coords1 - self.d.oldy

        if self.d.newx_ >= 0:
            self.d.newx = self.d.oldx   
        else:   
            if coords0 > 0:
                self.d.newx = coords0
                self.d.newx_ = - self.d.newx_
            else:
                self.d.newx = 0
                self.d.newx_ = self.d.oldx
                    
        if self.d.newy_ >= 0:
            self.d.newy = self.d.oldy   
        else:               
            if coords1 > 0:
                self.d.newy_ = - self.d.newy_
                self.d.newy = coords1
            else:
                self.d.newy = 0
                self.d.newy_ = self.d.oldy
                
        self.d.pixmap_temp.draw_drawable(self.d.gc,self.d.pixmap,  0 , 0 ,0,0, WIDTH, HEIGHT)
        self.d.pixmap_temp.draw_rectangle(self.d.gc_selection, False ,self.d.newx,self.d.newy,self.d.newx_,self.d.newy_)

    
    def circle(self, widget, coords):
        """Draw a circle.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """
        widget.queue_draw() 
        
        if coords[0] > WIDTH:
            coords0 = WIDTH
        else:
            coords0 = coords[0]
            
        if coords [1] > HEIGHT:
            coords1 = HEIGHT
        else:
            coords1 = coords[1]
            
        self.d.newx_ = coords0 - self.d.oldx
        self.d.newy_ = coords1 - self.d.oldy
        print "coords0", coords0

        if self.d.newx_ >= 0:
            self.d.newx = self.d.oldx   
        else:   
            if coords0 > 0:
                self.d.newx = coords0
                self.d.newx_ = - self.d.newx_
            else:
                self.d.newx = 0
                self.d.newx_ = self.d.oldx

        if self.d.newy_ >= 0:
            self.d.newy = self.d.oldy   
        else:   
            if coords1 > 0:             
                self.d.newy = coords1
                self.d.newy_ = - self.d.newy_
            else:
                self.d.newy = 0
                self.d.newy_ = self.d.oldy

        self.d.pixmap_temp.draw_drawable(self.d.gc,self.d.pixmap,  0 , 0 ,0,0, WIDTH, HEIGHT)   
        self.d.pixmap_temp.draw_arc(self.d.gc, True, self.d.newx, self.d.newy, self.d.newx_,self.d.newy_, 0, 360*64)
        self.d.pixmap_temp.draw_arc(self.d.gc_line, False, self.d.newx, self.d.newy, self.d.newx_, self.d.newy_, 0, 360*64)     

    
    def pencil(self, widget, coords):
        """Draw a pencil.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """
        self.d.pixmap_temp.draw_drawable(self.d.gc,self.d.pixmap,  0 , 0 ,0,0, WIDTH, HEIGHT)
        self.d.pixmap.draw_line(self.d.gc_line,self.d.oldx,self.d.oldy,coords[0],coords[1]) 
        self.d.oldx = coords[0]
        self.d.oldy = coords[1]
        widget.queue_draw()

    def clear(self):
        """Clear the drawing.

        Keyword arguments:
        self -- Desenho.Desenho instance

        """
        self.d.desenho = []
        self.d.textos = []      
        self.d.pixmap.draw_rectangle(self.d.get_style().white_gc, True,0, 0, WIDTH, HEIGHT)
        self.d.pixmap_temp.draw_rectangle(self.d.get_style().white_gc, True,0, 0, WIDTH, HEIGHT)
        self.d.queue_draw() 
    
    def text(self,widget,event):
        """Make a selection.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        event -- GdkEvent

        """
        if self.d.estadoTexto == 0:
            self.d.estadoTexto = 1
            print event.x
            self.d.janela._fixed.move(self.d.janela._textview, int(event.x)+200, int(event.y)+100)
            self.d.janela._textview.show()
        else:   
            self.d.estadoTexto = 0  
            texto = self.d.janela._textview.get_text()
            layout = self.d.create_pango_layout(texto)
            layout.set_font_description(self.d.font)
            self.d.pixmap.draw_layout(self.d.gc, self.d.oldx, self.d.oldy, layout)
            self.d.pixmap_temp.draw_layout(self.d.gc, self.d.oldx, self.d.oldy, layout)
            self.d.janela._textview.hide()
            self.d.janela._textview.set_text('')

            widget.queue_draw()

    def loadImage(self, name):
        """Load an image.

        Keyword arguments:
        self -- Desenho.Desenho instance
        name -- string (image file path)

        """
        pixbuf = gtk.gdk.pixbuf_new_from_file(name) 
        self.d.pixmap.draw_pixbuf(self.d.gc, pixbuf, 0, 0, 0, 0, width=-1, height=-1, dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
        self.d.pixmap_temp.draw_pixbuf(self.d.gc, pixbuf, 0, 0, 0, 0, width=-1, height=-1, dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
        self.d.queue_draw() 
        
    def moveSelection(self, widget, coords):
        """Move the selection.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """ 
        self.d.pixmap_temp.draw_rectangle(self.d.get_style().white_gc, True,0, 0, WIDTH, HEIGHT)
        self.d.pixmap_temp.draw_drawable(self.d.gc,self.d.pixmap,  0 , 0 ,0,0, WIDTH, HEIGHT)   
        
        if self.d.sx > self.d.oldx:
            x0 = self.d.oldx
        else:
            x0 = self.d.sx
            
        if self.d.sy > self.d.oldy:
            x1 = self.d.oldy
        else:
            x1 = self.d.sy
            
        w = self.d.sx - self.d.oldx
        if w < 0:
            w = - w
            
        h = self.d.sy - self.d.oldy         
        if h < 0:
            h = - h
        
        self.d.pixmap_temp.draw_rectangle(self.d.get_style().white_gc, True, x0, x1, w, h)
        self.d.pixmap_temp.draw_drawable(self.d.gc, self.d.pixmap, x0, x1, coords[0] - w/2, coords[1]- h/2, w, h)       
        widget.queue_draw()
    
    def polygon(self, widget, coords):
        """Draw polygon.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """ 
        self.d.pixmap_temp.draw_drawable(self.d.gc,self.d.pixmap,  0 , 0 ,0,0, WIDTH, HEIGHT)
        if self.d.polygon_start:
            self.d.pixmap_temp.draw_line(self.d.gc_line,self.d.oldx,self.d.oldy,coords[0],coords[1]) 
        else:
            self.d.pixmap_temp.draw_line(self.d.gc_line,int (self.d.lastx), int (self.d.lasty),coords[0],coords[1]) 
        self.d.newx = coords[0]     
        self.d.newy = coords[1]     
        widget.queue_draw()
        
        
    def fill(self, image, x, y, color):
        '''Fills a region with a given color.
        self    --
        image   -- a gtk.gdk.Image
        x,y     -- pixel coordinates
        color   -- a color to fill (decimal)
        
        '''
#         print 'entering (flood) fill function...'
#         print image
#         print x,y
#         print color, image.get_pixel(x,y)
        
        start_color = image.get_pixel(x,y)
        width, height = self.d.window.get_size()
        
        if x < 0 or x > width or y < 0 or y > height \
            or image.get_pixel(x,y) == color:
#             print 'leaving...'
            return
        
        edge = [(x, y)]
        image.put_pixel(x, y, color)
        while edge:
#             print edge
            newedge = []
            while gtk.events_pending ():gtk.main_iteration()
            for (x, y) in edge:
                for (s, t) in ((x+1, y), (x-1, y), (x, y+1), (x, y-1)):
                    if (s >= 0 and s < width) and (t >= 0 and t < height) \
                        and image.get_pixel(s, t) == start_color:
                        image.put_pixel(s, t, color)
                        newedge.append((s, t))
            edge = newedge
        
        return image

