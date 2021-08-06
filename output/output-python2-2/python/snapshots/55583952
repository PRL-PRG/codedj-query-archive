# -*- coding: utf-8 -*-
"""
Desenho.py

Pixmap manipulation


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
            self.d.pixmap.draw_rectangle(self.d.gc_eraser, True, coords[0], coords[1], size, size)
            self.d.pixmap_temp.draw_rectangle(self.d.gc_eraser, True, coords[0], coords[1], size, size)
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

    def arrow(self, widget, coords):
        """Draw a arrow.

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
        self.width = coords0 - self.d.oldx
        self.height = coords1 - self.d.oldy
        self.points = [(self.d.oldx,self.d.oldy), (self.d.oldx+int(self.width/6),coords1), (self.d.oldx+int(self.width/6),self.d.oldy+int(self.height/3)), (coords0,self.d.oldy+int(self.height/3)), (coords0,self.d.oldy-int(self.height/3)), (self.d.oldx+int(self.width/6),self.d.oldy-int(self.height/3)), (self.d.oldx+int(self.width/6),self.d.oldy-self.height)]
        self.d.pixmap_temp.draw_drawable(self.d.gc,self.d.pixmap,  0 , 0 ,0,0, WIDTH, HEIGHT)
        self.d.pixmap_temp.draw_polygon(self.d.gc, True, self.points)
        self.d.pixmap_temp.draw_polygon(self.d.gc_line, False, self.points)

    def parallelogram(self, widget, coords):
        """Draw a parallelogram.

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

        self.width = int((coords0 - self.d.oldx)/4)
        self.points = [(self.d.oldx,self.d.oldy), (coords0-self.width, self.d.oldy), (coords0,coords1), (self.d.oldx+self.width,coords1)]
        self.d.pixmap_temp.draw_drawable(self.d.gc,self.d.pixmap,  0 , 0 ,0,0, WIDTH, HEIGHT)
        self.d.pixmap_temp.draw_polygon(self.d.gc, True, self.points)
        self.d.pixmap_temp.draw_polygon(self.d.gc_line, False, self.points)

    def star(self, widget, coords):
        """Draw a arrow.

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
        self.width = coords0 - self.d.oldx
        self.height = coords1 - self.d.oldy
        self.points = [(self.d.oldx,self.d.oldy),\
(self.d.oldx+int(self.width*0.25), self.d.oldy+int(self.height*0.4)),\
(self.d.oldx+int(self.width),self.d.oldy+int(self.height*0.4)),\
(self.d.oldx+int(self.width*0.35), self.d.oldy+int(self.height*0.6)),\
(self.d.oldx+int(self.width*0.6), self.d.oldy+self.height),\
(self.d.oldx, self.d.oldy+int(self.height*0.75)),\
(self.d.oldx-int(self.width*0.6), self.d.oldy+self.height),\
(self.d.oldx-int(self.width*0.35), self.d.oldy+int(self.height*0.6)),\
(self.d.oldx-int(self.width),self.d.oldy+int(self.height*0.4)),\
(self.d.oldx-int(self.width*0.25), self.d.oldy+int(self.height*0.4))]
        self.d.pixmap_temp.draw_drawable(self.d.gc,self.d.pixmap,  0 , 0 ,0,0, WIDTH, HEIGHT)
        self.d.pixmap_temp.draw_polygon(self.d.gc, True, self.points)
        self.d.pixmap_temp.draw_polygon(self.d.gc_line, False, self.points)

    
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

            self.d.enableUndo(widget)
            
            widget.queue_draw()

    def loadImage(self, name, widget):
        """Load an image.

        Keyword arguments:
        self -- Desenho.Desenho instance
        name -- string (image file path)

        """
        pixbuf = gtk.gdk.pixbuf_new_from_file(name) 
        self.d.pixmap.draw_pixbuf(self.d.gc, pixbuf, 0, 0, 0, 0, width=-1, height=-1, dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
        self.d.pixmap_temp.draw_pixbuf(self.d.gc, pixbuf, 0, 0, 0, 0, width=-1, height=-1, dither=gtk.gdk.RGB_DITHER_NORMAL, x_dither=0, y_dither=0)
        
        self.d.enableUndo(widget)
        
        self.d.queue_draw()

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
        self.d.pixmap_temp.draw_rectangle(self.d.gc_selection1, False, \
                                        self.d.newx-1,self.d.newy-1,self.d.newx_+2,self.d.newy_+2)
        
    def moveSelection(self, widget, coords):
        """Move the selection.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """ 
        self.d.pixmap_temp.draw_rectangle(self.d.get_style().white_gc, True,0, 0, WIDTH, HEIGHT)
        self.d.pixmap_temp.draw_drawable(self.d.gc,self.d.pixmap,  0 , 0 ,0,0, WIDTH, HEIGHT)
        
        self.d.pixmap_sel.draw_rectangle(self.d.get_style().white_gc, True,0, 0, WIDTH, HEIGHT)
        self.d.pixmap_sel.draw_drawable(self.d.gc,self.d.pixmap,  0 , 0 ,0,0, WIDTH, HEIGHT)   
        
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
        
        self.d.pixmap_sel.draw_rectangle(self.d.get_style().white_gc, True, x0, x1, w, h)
        self.d.pixmap_sel.draw_drawable(self.d.gc, self.d.pixmap, x0, x1, coords[0] - w/2, coords[1]- h/2, w, h)
	    #to draw the selection black and white line rectangle
        self.d.pixmap_sel.draw_rectangle(self.d.gc_selection, False ,coords[0] - w/2, coords[1]- h/2, w, h)
        self.d.pixmap_sel.draw_rectangle(self.d.gc_selection1, False ,coords[0] - w/2-1, coords[1]- h/2-1, w+2, h+2)
        
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

