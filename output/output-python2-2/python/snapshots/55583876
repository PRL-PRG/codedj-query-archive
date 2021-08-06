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
        width, height = self.d.window.get_size()
        self.d.pixmap_temp.draw_drawable(self.d.gc,self.d.pixmap,  0 , 0 ,0,0, width, height)
        self.d.pixmap_temp.draw_line(self.d.gc_line,self.d.oldx,self.d.oldy,coords[0],coords[1])
        #self.d.newx = coords[0] 
        #self.d.newy = coords[1]
        widget.queue_draw()
    
    def eraser(self, widget, coords, last, size = 30, shape = 'circle'):
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
            if last[0] != -1:
                self.d.gc_eraser.set_line_attributes(size, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)
                self.d.pixmap.draw_line(self.d.gc_eraser,last[0]+size/2,last[1]+size/2,coords[0]+size/2,coords[1]+size/2)
                self.d.gc_eraser.set_line_attributes(0, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)
        if(shape == 'square'):
            self.d.pixmap.draw_rectangle(self.d.gc_eraser, True, coords[0], coords[1], size, size)
            if last[0] != -1:
                points = [coords, last, (last[0]+size,last[1]+size), (coords[0]+size,coords[1]+size)]
                self.d.pixmap.draw_polygon(self.d.gc_eraser,True,points)
                points = [(last[0]+size,last[1]), (coords[0]+size,coords[1]), (coords[0],coords[1]+size), (last[0],last[1]+size)]
                self.d.pixmap.draw_polygon(self.d.gc_eraser,True,points)
        widget.queue_draw()
        
    def brush(self, widget, coords, last, size = 5, shape = 'circle'):
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
            if last[0] != -1:
                self.d.gc_brush.set_line_attributes(size, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)
                self.d.pixmap.draw_line(self.d.gc_brush,last[0]+size/2,last[1]+size/2,coords[0]+size/2,coords[1]+size/2)
                self.d.gc_brush.set_line_attributes(0, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)
        if(shape == 'square'):
            self.d.pixmap.draw_rectangle(self.d.gc_brush, True, coords[0], coords[1], size, size)
            if last[0] != -1:
                points = [coords, last, (last[0]+size,last[1]+size), (coords[0]+size,coords[1]+size)]
                self.d.pixmap.draw_polygon(self.d.gc_brush,True,points)
                points = [(last[0]+size,last[1]), (coords[0]+size,coords[1]), (coords[0],coords[1]+size), (last[0],last[1]+size)]
                self.d.pixmap.draw_polygon(self.d.gc_brush,True,points)
        widget.queue_draw()

    def rainbow(self, widget, coords, last, color, size = 5, shape = 'circle'):
        """Paint with rainbow.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple
        size -- integer (default 30)
        shape -- string (default 'circle')

        """
        colormap = self.d.get_colormap()
        rainbow_colors = [
        colormap.alloc_color('#ff0000', True, True), # vermelho
        colormap.alloc_color('#ff8000', True, True), # laranja
        colormap.alloc_color('#ffff00', True, True), # amarelo
        colormap.alloc_color('#80ff00', True, True), # verde lima
        colormap.alloc_color('#00ff00', True, True), # verde
        colormap.alloc_color('#00ff80', True, True), # verde agua
        colormap.alloc_color('#00ffff', True, True), # azul claro
        colormap.alloc_color('#007fff', True, True), # quase azul
        colormap.alloc_color('#0000ff', True, True), # azul
        colormap.alloc_color('#8000ff', True, True), # anil
        colormap.alloc_color('#ff00ff', True, True), # rosa violeta
        colormap.alloc_color('#ff0080', True, True), # violeta
        ]

        self.d.gc_rainbow.set_foreground(rainbow_colors[color])
        self.d.desenha = False
        if(shape == 'circle'):
            self.d.pixmap.draw_arc(self.d.gc_rainbow, True, coords[0], coords[1], size, size, 0, 360*64)
            if last[0] != -1:
                self.d.gc_rainbow.set_line_attributes(size, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)
                self.d.pixmap.draw_line(self.d.gc_rainbow,last[0]+size/2,last[1]+size/2,coords[0]+size/2,coords[1]+size/2)
                self.d.gc_rainbow.set_line_attributes(0, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)
        if(shape == 'square'):
            if last[0] != -1:
                self.d.pixmap.draw_rectangle(self.d.gc_rainbow, True, last[0], last[1], size, size)
                points = [coords, last, (last[0]+size,last[1]+size), (coords[0]+size,coords[1]+size)]
                self.d.pixmap.draw_polygon(self.d.gc_rainbow,True,points)
                points = [(last[0]+size,last[1]), (coords[0]+size,coords[1]), (coords[0],coords[1]+size), (last[0],last[1]+size)]
                self.d.pixmap.draw_polygon(self.d.gc_rainbow,True,points)
            self.d.pixmap.draw_rectangle(self.d.gc_rainbow, True, coords[0], coords[1], size, size)
        widget.queue_draw()

    
    def square(self, widget, coords, temp, fill):
        """Draw a square.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple
        """
        if temp == True:
            pixmap = self.d.pixmap_temp
        else:
            pixmap = self.d.pixmap
        width, height = self.d.window.get_size()
        
        dx = math.fabs(coords[0] - self.d.oldx)
        dy = math.fabs(coords[1] - self.d.oldy)
        
        if coords[0] < self.d.oldx:
            x = coords[0]
        else:
            x = self.d.oldx
        if coords[1] < self.d.oldy:
            y = coords[1]
        else:
            y = self.d.oldy
        
        pixmap.draw_drawable(self.d.gc,self.d.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_rectangle(self.d.gc,True,x,y,dx,dy)
        pixmap.draw_rectangle(self.d.gc_line,False,x,y,dx,dy)
        widget.queue_draw()
    
    def triangle(self, widget, coords, temp, fill):
        """Draw a triangle.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """

        if temp == True:
            pixmap = self.d.pixmap_temp
        else:
            pixmap = self.d.pixmap
        width, height = self.d.window.get_size()
        
        points = [(self.d.oldx,self.d.oldy), (self.d.oldx+int((coords[0]-self.d.oldx)/2),coords[1]), (coords[0],self.d.oldy)]
        pixmap.draw_drawable(self.d.gc,self.d.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_polygon(self.d.gc,True,points)
        pixmap.draw_polygon(self.d.gc_line,False,points)
        widget.queue_draw()


    def trapezoid(self, widget, coords, temp, fill):
        """Draw a trapezoid.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """

        if temp == True:
            pixmap = self.d.pixmap_temp
        else:
            pixmap = self.d.pixmap
        width, height = self.d.window.get_size()

        dif = int((coords[0] - self.d.oldx)/4)
        points = [(self.d.oldx, self.d.oldy), (self.d.oldx+dif, coords[1]), (coords[0]-dif, coords[1]) , (coords[0],self.d.oldy)]                
        pixmap.draw_drawable(self.d.gc,self.d.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_polygon(self.d.gc, True, points)
        pixmap.draw_polygon(self.d.gc_line,False,points)
        widget.queue_draw()
        
        
    def arrow(self, widget, coords, temp, fill):
        """Draw a arrow.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """
        if temp == True:
            pixmap = self.d.pixmap_temp
        else:
            pixmap = self.d.pixmap
        width, height = self.d.window.get_size()

        x = coords[0] - self.d.oldx
        y = coords[1] - self.d.oldy
        points = [(self.d.oldx,self.d.oldy),\
(self.d.oldx+int(x/6),self.d.oldy+y),\
(self.d.oldx+int(x/6),self.d.oldy+int(y/3)),\
(self.d.oldx+x,self.d.oldy+int(y/3)),\
(self.d.oldx+x,self.d.oldy-int(y/3)),\
(self.d.oldx+int(x/6),self.d.oldy-int(y/3)),\
(self.d.oldx+int(x/6),self.d.oldy-y)]
        pixmap.draw_drawable(self.d.gc,self.d.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_polygon(self.d.gc,True,points)
        pixmap.draw_polygon(self.d.gc_line,False,points)
        widget.queue_draw()
        
        
    def parallelogram(self, widget, coords, temp, fill):
        """Draw a parallelogram.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """
        if temp == True:
            pixmap = self.d.pixmap_temp
        else:
            pixmap = self.d.pixmap
        width, height = self.d.window.get_size()

        x = int((coords[0] - self.d.oldx)/4)
        points = [(self.d.oldx,self.d.oldy), (coords[0]-x, self.d.oldy), (coords[0],coords[1]), (self.d.oldx+x,coords[1])]
        pixmap.draw_drawable(self.d.gc,self.d.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_polygon(self.d.gc,True,points)
        pixmap.draw_polygon(self.d.gc_line,False,points)
        widget.queue_draw()


    def star(self, widget, coords, temp, fill):
        """Draw a arrow.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """
        if temp == True:
            pixmap = self.d.pixmap_temp
        else:
            pixmap = self.d.pixmap
        width, height = self.d.window.get_size()
        
        x = coords[0] - self.d.oldx
        y = coords[1] - self.d.oldy

        points = [(self.d.oldx,self.d.oldy),\
(self.d.oldx+int(x*0.25), self.d.oldy+int(y*0.4)),\
(self.d.oldx+int(x), self.d.oldy+int(y*0.4)),\
(self.d.oldx+int(x*0.35), self.d.oldy+int(y*0.6)),\
(self.d.oldx+int(x*0.6), self.d.oldy+y),\
(self.d.oldx, self.d.oldy+int(y*0.75)),\
(self.d.oldx-int(x*0.6), self.d.oldy+y),\
(self.d.oldx-int(x*0.35), self.d.oldy+int(y*0.6)),\
(self.d.oldx-int(x), self.d.oldy+int(y*0.4)),\
(self.d.oldx-int(x*0.25), self.d.oldy+int(y*0.4))]
        pixmap.draw_drawable(self.d.gc,self.d.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_polygon(self.d.gc,True,points)
        pixmap.draw_polygon(self.d.gc_line,False,points)
        widget.queue_draw()


    def polygon_regular(self, widget, coords, n, temp, fill):
        """Draw polygon with n sides.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple
        n -- number of sides
        temp -- switch between pixmap and pixmap_temp

        """
        if temp == True:
            pixmap = self.d.pixmap_temp
        else:
            pixmap = self.d.pixmap
        width, height = self.d.window.get_size()
        
        x = coords[0] - self.d.oldx
        y = coords[1] - self.d.oldy
        A = math.atan2(y,x)
        dA = 2*math.pi/n
        r = math.hypot(y,x)
        p = [(self.d.oldx+int(r*math.cos(A)),self.d.oldy+int(r*math.sin(A)))]
        for i in range(n-1):
            A = A+dA
            p.append((self.d.oldx+int(r*math.cos(A)),self.d.oldy+int(r*math.sin(A))))
        tp = tuple(p)
        
        pixmap.draw_drawable(self.d.gc,self.d.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_polygon(self.d.gc,True,tp)
        pixmap.draw_polygon(self.d.gc_line,False,tp)
        widget.queue_draw()


    def heart(self, widget, coords, temp, fill):
        """Draw polygon with n sides.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple
        n -- number of sides
        temp -- switch between pixmap and pixmap_temp

        """
        if temp == True:
            pixmap = self.d.pixmap_temp
        else:
            pixmap = self.d.pixmap
        width, height = self.d.window.get_size()

        if coords[0] < self.d.oldx:
            x = coords[0]
        else:
            x = self.d.oldx
        if coords[1] < self.d.oldy:
            y = coords[1]
        else:
            y = self.d.oldy
        
        dx = math.fabs(coords[0] - self.d.oldx)
        dy = math.fabs(coords[1] - self.d.oldy)
        
        w=int(4*dx)
        e=int(4*dx/math.sqrt(3))

        pixmap.draw_drawable(self.d.gc,self.d.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_arc(self.d.gc,True,int(self.d.oldx-dx),int(self.d.oldy-e/2),w,e,180*64,60*64)
            pixmap.draw_arc(self.d.gc,True,int(self.d.oldx-3*dx),int(self.d.oldy-e/2),w,e,300*64,60*64)
            pixmap.draw_arc(self.d.gc,True,int(self.d.oldx-dx*0.2),int(self.d.oldy-0.6*dx+2),int(1.2*dx),int(1.2*dx),0,180*64)
            pixmap.draw_arc(self.d.gc,True,int(self.d.oldx-dx),int(self.d.oldy-0.6*dx+2),int(1.2*dx),int(1.2*dx),0,180*64)
        pixmap.draw_arc(self.d.gc_line,False,int(self.d.oldx-dx),int(self.d.oldy-e/2),w,e,180*64,60*64)
        pixmap.draw_arc(self.d.gc_line,False,int(self.d.oldx-dx-w/2),int(self.d.oldy-e/2),w,e,300*64,60*64)
        pixmap.draw_arc(self.d.gc_line,False,int(self.d.oldx-dx*0.2),int(self.d.oldy-0.6*dx+2),int(1.2*dx),int(1.2*dx),0,132*64)
        pixmap.draw_arc(self.d.gc_line,False,int(self.d.oldx-dx),int(self.d.oldy-0.6*dx+2),int(1.2*dx),int(1.2*dx),48*64,132*64)
        
        widget.queue_draw()

 
    def circle(self, widget, coords, temp, fill):
        """Draw a circle.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """

        if temp == True:
            pixmap = self.d.pixmap_temp
        else:
            pixmap = self.d.pixmap
        width, height = self.d.window.get_size()

        if coords[0] < self.d.oldx:
            x = coords[0]
        else:
            x = self.d.oldx
        if coords[1] < self.d.oldy:
            y = coords[1]
        else:
            y = self.d.oldy
        
        dx = math.fabs(coords[0] - self.d.oldx)
        dy = math.fabs(coords[1] - self.d.oldy)
        
        pixmap.draw_drawable(self.d.gc,self.d.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_arc(self.d.gc,True,x,y,dx,dy,0,360*64)
        pixmap.draw_arc(self.d.gc_line,False,x,y,dx,dy,0,360*64)     
        widget.queue_draw()


    def pencil(self, widget, coords):
        """Draw a pencil.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """
        width, height = self.d.window.get_size()
        self.d.pixmap_temp.draw_drawable(self.d.gc,self.d.pixmap,  0 , 0 ,0,0, width, height)
        self.d.pixmap.draw_line(self.d.gc_line,self.d.oldx,self.d.oldy,coords[0],coords[1]) 
        self.d.oldx = coords[0]
        self.d.oldy = coords[1]
        widget.queue_draw()

    def clear(self):
        """Clear the drawing.

        Keyword arguments:
        self -- Desenho.Desenho instance

        """
        width, height = self.d.window.get_size()
        self.d.desenho = []
        self.d.textos = []      
        self.d.pixmap.draw_rectangle(self.d.get_style().white_gc, True,0, 0, width, height)
        self.d.pixmap_temp.draw_rectangle(self.d.get_style().white_gc, True,0, 0, width, height)
        self.d.queue_draw() 
    
    def text(self,widget,event):
        """Make a selection.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        event -- GdkEvent

        """
        #print self.d.estadoTexto
        if self.d.estadoTexto == 0:
            self.d.estadoTexto = 1
            
            self.d.janela._fixed.move(self.d.janela._textview, int(event.x)+200, int(event.y)+100)
            # Area size has changed...
            #self.d.janela._fixed.move(self.d.janela._textview, int(event.x), int(event.y))
            self.d.janela._textview.show()
            self.d.janela._textview.grab_focus()
            
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
            
        #print self.d.estadoTexto

    def selection(self, widget, coords, temp, fill):
        """Make a selection.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple
        """ 
        
        if temp == True:
            pixmap = self.d.pixmap_temp
        else:
            pixmap = self.d.pixmap
        width, height = self.d.window.get_size()
        
        dx = math.fabs(coords[0] - self.d.oldx)
        dy = math.fabs(coords[1] - self.d.oldy)
        
        if coords[0] < self.d.oldx:
            x = coords[0]
        else:
            x = self.d.oldx
        if coords[1] < self.d.oldy:
            y = coords[1]
        else:
            y = self.d.oldy
        
        pixmap.draw_drawable(self.d.gc,self.d.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_rectangle(self.d.gc,True,int(x),int(y),int(dx),int(dy))
            
        pixmap.draw_rectangle(self.d.gc_selection,False,int(x),int(y),int(dx),int(dy))
        widget.queue_draw()
        #return self.d.oldx, self.d.oldy, coords[0], coords[1]        
        return x,y,x+dx,y+dy
        
    def moveSelection(self, widget, coords):
        """Move the selection.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """ 
        width, height = self.d.window.get_size()
        
        self.d.pixmap_sel.draw_drawable(self.d.gc,self.d.pixmap,0,0,0,0, width, height)   
        
        if self.d.sx > self.d.oldx:
            x0 = self.d.oldx
        else:
            x0 = self.d.sx
            
        if self.d.sy > self.d.oldy:
            y0 = self.d.oldy
        else:
            y0 = self.d.sy
            
        w = self.d.sx - self.d.oldx
        if w < 0:
            w = - w
            
        h = self.d.sy - self.d.oldy         
        if h < 0:
            h = - h
            
        self.d._set_selection_bounds(coords[0]-w/2, coords[1]-h/2, coords[0]+w/2, coords[1]+h/2)
               
        
        self.d.pixmap_sel.draw_rectangle(self.d.get_style().white_gc, True, x0, y0, w, h)
        self.d.pixmap_sel.draw_drawable(self.d.gc, self.d.pixmap, x0, y0, coords[0] - w/2, coords[1]- h/2, w, h)
        self.d.pixmap_temp.draw_drawable(self.d.gc, self.d.pixmap_sel,0,0,0,0, width, height)
        
	    #to draw the selection black and white line rectangle
        self.d.pixmap_sel.draw_rectangle(self.d.gc_selection, False ,coords[0] - w/2, coords[1]- h/2, w, h)
        self.d.pixmap_sel.draw_rectangle(self.d.gc_selection1, False ,coords[0] - w/2-1, coords[1]- h/2-1, w+2, h+2)
       
        widget.queue_draw()


    def polygon(self, widget, coords, temp, fill):
        """Draw polygon.

        Keyword arguments:
        self -- Desenho.Desenho instance
        widget -- Area object (GtkDrawingArea)
        coords -- Two value tuple

        """

        if temp == True:
            pixmap = self.d.pixmap_temp
        else:
            pixmap = self.d.pixmap
        width, height = self.d.window.get_size()

        pixmap.draw_drawable(self.d.gc, self.d.pixmap, 0, 0, 0, 0, width, height)
        
        if self.d.polygon_start == True: # Starting a new polygon ?
            if temp == True:
                pixmap.draw_line(self.d.gc_line,self.d.oldx,self.d.oldy, coords[0], coords[1])
            else:
                pixmap.draw_line(self.d.gc_line,self.d.oldx,self.d.oldy, coords[0], coords[1])
                self.d.enableUndo(widget)
                self.d.last = coords
                self.d.first = self.d.oldx, self.d.oldy
                self.d.polygon_start = False
                self.d.points = [self.d.first, coords]
        else:
            if temp == True:
                pixmap.draw_line(self.d.gc_line,self.d.last[0],self.d.last[1],coords[0],coords[1])
            else:
                x = coords[0] - self.d.first[0]
                y = coords[1] - self.d.first[1]
                d = math.hypot(x,y)
                if d > 20: # close the polygon ?
                    pixmap.draw_line(self.d.gc_line,self.d.last[0],self.d.last[1],coords[0],coords[1])
                    self.d.last = coords
                    self.d.points.append(coords)
                else:
                    tp = tuple(self.d.points)
                    if fill == True:
                        pixmap.draw_polygon(self.d.gc, True, tp)
                    pixmap.draw_polygon(self.d.gc_line, False, tp)
                    self.d.last = -1, -1
                    self.d.polygon_start = True
                    self.d.undo_times -= 1#destroy the undo screen of polygon start 
                    self.d.enableUndo(widget)
        widget.queue_draw()
        
        

