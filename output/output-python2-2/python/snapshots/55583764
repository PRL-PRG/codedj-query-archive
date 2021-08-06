# -*- coding: utf-8 -*-
"""
@namespace Desenho

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

UI Design (OLPC):
Eben Eliason                        (eben@laptop.org)

Project Coordinator (OLPC):
Manusheel Gupta                     (manu@laptop.org)

Project Advisor (OLPC):
Walter Bender                       (walter@laptop.org)

"""


import  pygtk
pygtk.require('2.0')
import gtk
import sys, gobject, logging
from gtk import gdk
import math
import pango
import gc


##Pixmap manipulation
class Desenho:
    def __init__(self, widget):
        """Initialize Desenho object.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)

        """
        #self.d = widget
        
    def line(self, widget, coords):
        """Draw line.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)
            @param  coords -- Two value tuple

        """
        width, height = widget.window.get_size()
        widget.pixmap_temp.draw_drawable(widget.gc,widget.pixmap,  0 , 0 ,0,0, width, height)
        widget.pixmap_temp.draw_line(widget.gc_line,widget.oldx,widget.oldy,coords[0],coords[1])
        #widget.newx = coords[0] 
        #widget.newy = coords[1]
        widget.queue_draw()
    
    def eraser(self, widget, coords, last, size = 30, shape = 'circle'):
        """Erase part of the drawing.

            @param  self -- Desenho.Desenho instance
            @param  last -- last of oldx
            @param  widget -- Area object (GtkDrawingArea)
            @param  coords -- Two value tuple
            @param  size -- integer (default 30)
            @param  shape -- string (default 'circle')

        """
        widget.desenha = False
        if(shape == 'circle'):
            widget.pixmap.draw_arc(widget.gc_eraser, True, coords[0], coords[1], size, size, 0, 360*64)
            if last:
                widget.gc_eraser.set_line_attributes(size, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)
                widget.pixmap.draw_line(widget.gc_eraser,last[0]+size/2,last[1]+size/2,coords[0]+size/2,coords[1]+size/2)
                widget.gc_eraser.set_line_attributes(0, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)
        if(shape == 'square'):
            widget.pixmap.draw_rectangle(widget.gc_eraser, True, coords[0], coords[1], size, size)
            if last:
                points = [coords, last, (last[0]+size,last[1]+size), (coords[0]+size,coords[1]+size)]
                widget.pixmap.draw_polygon(widget.gc_eraser,True,points)
                points = [(last[0]+size,last[1]), (coords[0]+size,coords[1]), (coords[0],coords[1]+size), (last[0],last[1]+size)]
                widget.pixmap.draw_polygon(widget.gc_eraser,True,points)
        widget.queue_draw()
        
    def brush(self, widget, coords, last, size = 5, shape = 'circle'):
        """Paint with brush.

            @param  self -- Desenho.Desenho instance
            @param  last -- last of oldx
            @param  widget -- Area object (GtkDrawingArea)
            @param  coords -- Two value tuple
            @param  size -- integer (default 30)
            @param  shape -- string (default 'circle')

        """
        widget.desenha = False
        if(shape == 'circle'):
            widget.pixmap.draw_arc(widget.gc_brush, True, coords[0], coords[1], size, size, 0, 360*64)
            if last:
                widget.gc_brush.set_line_attributes(size, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)
                widget.pixmap.draw_line(widget.gc_brush,last[0]+size/2,last[1]+size/2,coords[0]+size/2,coords[1]+size/2)
                widget.gc_brush.set_line_attributes(0, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)
        if(shape == 'square'):
            widget.pixmap.draw_rectangle(widget.gc_brush, True, coords[0], coords[1], size, size)
            if last:
                points = [coords, last, (last[0]+size,last[1]+size), (coords[0]+size,coords[1]+size)]
                widget.pixmap.draw_polygon(widget.gc_brush,True,points)
                points = [(last[0]+size,last[1]), (coords[0]+size,coords[1]), (coords[0],coords[1]+size), (last[0],last[1]+size)]
                widget.pixmap.draw_polygon(widget.gc_brush,True,points)
        widget.queue_draw()

    def rainbow(self, widget, coords, last, color, size = 5, shape = 'circle'):
        """Paint with rainbow.

            @param  self -- Desenho.Desenho instance
            @param  last -- last of oldx
            @param  widget -- Area object (GtkDrawingArea)
            @param  color -- select the color adress
            @param  coords -- Two value tuple
            @param  size -- integer (default 30)
            @param  shape -- string (default 'circle')

        """
        colormap = widget.get_colormap()
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

        widget.gc_rainbow.set_foreground(rainbow_colors[color])
        widget.desenha = False
        if(shape == 'circle'):
            widget.pixmap.draw_arc(widget.gc_rainbow, True, coords[0], coords[1], size, size, 0, 360*64)
            if last:
                widget.gc_rainbow.set_line_attributes(size, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)
                widget.pixmap.draw_line(widget.gc_rainbow,last[0]+size/2,last[1]+size/2,coords[0]+size/2,coords[1]+size/2)
                widget.gc_rainbow.set_line_attributes(0, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_ROUND)
        if(shape == 'square'):
            if last:
                widget.pixmap.draw_rectangle(widget.gc_rainbow, True, last[0], last[1], size, size)
                points = [coords, last, (last[0]+size,last[1]+size), (coords[0]+size,coords[1]+size)]
                widget.pixmap.draw_polygon(widget.gc_rainbow,True,points)
                points = [(last[0]+size,last[1]), (coords[0]+size,coords[1]), (coords[0],coords[1]+size), (last[0],last[1]+size)]
                widget.pixmap.draw_polygon(widget.gc_rainbow,True,points)
            widget.pixmap.draw_rectangle(widget.gc_rainbow, True, coords[0], coords[1], size, size)
        widget.queue_draw()

    
    def square(self, widget, event, coords, temp, fill):
        """Draw a square.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)
            @param  coords -- Two value tuple
            @param  temp -- switch between pixmap and pixmap_temp
            @param  fill -- Fill object
        """
        if temp == True:
            pixmap = widget.pixmap_temp
        else:
            pixmap = widget.pixmap
        width, height = widget.window.get_size()
        
        x, y, dx, dy, = self.adjust(widget,coords)
        
        pixmap.draw_drawable(widget.gc,widget.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_rectangle(widget.gc,True,x,y,dx,dy)
        pixmap.draw_rectangle(widget.gc_line,False,x,y,dx,dy)
        widget.queue_draw()
    
    def triangle(self, widget, coords, temp, fill):
        """Draw a triangle.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)
            @param  coords -- Two value tuple
            @param  temp -- switch between pixmap and pixmap_temp
            @param  fill -- Fill object

        """

        if temp == True:
            pixmap = widget.pixmap_temp
        else:
            pixmap = widget.pixmap
        width, height = widget.window.get_size()
        
        points = [(widget.oldx,widget.oldy), (widget.oldx+int((coords[0]-widget.oldx)/2),coords[1]), (coords[0],widget.oldy)]
        pixmap.draw_drawable(widget.gc,widget.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_polygon(widget.gc,True,points)
        pixmap.draw_polygon(widget.gc_line,False,points)
        widget.queue_draw()


    def trapezoid(self, widget, coords, temp, fill):
        """Draw a trapezoid.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)
            @param  coords -- Two value tuple
            @param  temp -- switch between pixmap and pixmap_temp
            @param  fill -- Fill object

        """

        if temp == True:
            pixmap = widget.pixmap_temp
        else:
            pixmap = widget.pixmap
        width, height = widget.window.get_size()

        dif = int((coords[0] - widget.oldx)/4)
        points = [(widget.oldx, widget.oldy), (widget.oldx+dif, coords[1]), (coords[0]-dif, coords[1]) , (coords[0],widget.oldy)]                
        pixmap.draw_drawable(widget.gc,widget.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_polygon(widget.gc, True, points)
        pixmap.draw_polygon(widget.gc_line,False,points)
        widget.queue_draw()
        
        
    def arrow(self, widget, coords, temp, fill):
        """Draw a arrow.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)
            @param  coords -- Two value tuple
            @param  temp -- switch between pixmap and pixmap_temp
            @param  fill -- Fill object

        """
        if temp == True:
            pixmap = widget.pixmap_temp
        else:
            pixmap = widget.pixmap
        width, height = widget.window.get_size()

        x = coords[0] - widget.oldx
        y = coords[1] - widget.oldy
        points = [(widget.oldx,widget.oldy),\
(widget.oldx+int(x/6),widget.oldy+y),\
(widget.oldx+int(x/6),widget.oldy+int(y/3)),\
(widget.oldx+x,widget.oldy+int(y/3)),\
(widget.oldx+x,widget.oldy-int(y/3)),\
(widget.oldx+int(x/6),widget.oldy-int(y/3)),\
(widget.oldx+int(x/6),widget.oldy-y)]
        pixmap.draw_drawable(widget.gc,widget.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_polygon(widget.gc,True,points)
        pixmap.draw_polygon(widget.gc_line,False,points)
        widget.queue_draw()
        
        
    def parallelogram(self, widget, coords, temp, fill):
        """Draw a parallelogram.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)
            @param  coords -- Two value tuple
            @param  temp -- switch between pixmap and pixmap_temp
            @param  fill -- Fill object
        """
        if temp == True:
            pixmap = widget.pixmap_temp
        else:
            pixmap = widget.pixmap
        width, height = widget.window.get_size()

        x = int((coords[0] - widget.oldx)/4)
        points = [(widget.oldx,widget.oldy), (coords[0]-x, widget.oldy), (coords[0],coords[1]), (widget.oldx+x,coords[1])]
        pixmap.draw_drawable(widget.gc,widget.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_polygon(widget.gc,True,points)
        pixmap.draw_polygon(widget.gc_line,False,points)
        widget.queue_draw()


    def star(self, widget, coords, n, temp, fill):
        """Draw polygon with n sides.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)
            @param  coords -- Two value tuple
            @param  n -- number of sides
            @param  temp -- switch between pixmap and pixmap_temp
            @param  fill -- Fill object

        """
        if temp == True:
            pixmap = widget.pixmap_temp
        else:
            pixmap = widget.pixmap
        width, height = widget.window.get_size()
        
        x = coords[0] - widget.oldx
        y = coords[1] - widget.oldy
        A = math.atan2(y,x)
        dA = 2*math.pi/n
        r = math.hypot(y,x)
        p = [(widget.oldx+int(r*math.cos(A)),widget.oldy+int(r*math.sin(A))),\
        (widget.oldx+int(0.4*r*math.cos(A+dA/2)),widget.oldy+int(0.4*r*math.sin(A+dA/2)))]
        for i in range(n-1):
            A = A+dA
            p.append((widget.oldx+int(r*math.cos(A)),widget.oldy+int(r*math.sin(A))))
            p.append((widget.oldx+int(0.4*r*math.cos(A+dA/2)),widget.oldy+int(0.4*r*math.sin(A+dA/2))))
        tp = tuple(p)
        
        pixmap.draw_drawable(widget.gc,widget.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_polygon(widget.gc,True,tp)
        pixmap.draw_polygon(widget.gc_line,False,tp)
        widget.queue_draw()


    def polygon_regular(self, widget, coords, n, temp, fill):
        """Draw polygon with n sides.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)
            @param  coords -- Two value tuple
            @param  n -- number of sides
            @param  temp -- switch between pixmap and pixmap_temp
            @param  fill -- Fill object

        """
        if temp == True:
            pixmap = widget.pixmap_temp
        else:
            pixmap = widget.pixmap
        width, height = widget.window.get_size()
        
        x = coords[0] - widget.oldx
        y = coords[1] - widget.oldy
        A = math.atan2(y,x)
        dA = 2*math.pi/n
        r = math.hypot(y,x)
        p = [(widget.oldx+int(r*math.cos(A)),widget.oldy+int(r*math.sin(A)))]
        for i in range(n-1):
            A = A+dA
            p.append((widget.oldx+int(r*math.cos(A)),widget.oldy+int(r*math.sin(A))))
        tp = tuple(p)
        
        pixmap.draw_drawable(widget.gc,widget.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_polygon(widget.gc,True,tp)
        pixmap.draw_polygon(widget.gc_line,False,tp)
        widget.queue_draw()


    def heart(self, widget, coords, temp, fill):
        """Draw polygon with n sides.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)
            @param  coords -- Two value tuple
            @param  temp -- switch between pixmap and pixmap_temp
            @param  fill -- Fill object

        """
        if temp == True:
            pixmap = widget.pixmap_temp
        else:
            pixmap = widget.pixmap
        width, height = widget.window.get_size()

        if coords[0] < widget.oldx:
            x = coords[0]
        else:
            x = widget.oldx
        if coords[1] < widget.oldy:
            y = coords[1]
        else:
            y = widget.oldy
        
        dx = math.fabs(coords[0] - widget.oldx)
        dy = math.fabs(coords[1] - widget.oldy)
        
        w=int(4*dx)
        e=int(4*dx/math.sqrt(3))

        pixmap.draw_drawable(widget.gc,widget.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_arc(widget.gc,True,int(widget.oldx-dx),int(widget.oldy-e/2),w,e,180*64,60*64)
            pixmap.draw_arc(widget.gc,True,int(widget.oldx-3*dx),int(widget.oldy-e/2),w,e,300*64,60*64)
            pixmap.draw_arc(widget.gc,True,int(widget.oldx-dx*0.2),int(widget.oldy-0.6*dx+2),int(1.2*dx),int(1.2*dx),0,180*64)
            pixmap.draw_arc(widget.gc,True,int(widget.oldx-dx),int(widget.oldy-0.6*dx+2),int(1.2*dx),int(1.2*dx),0,180*64)
        pixmap.draw_arc(widget.gc_line,False,int(widget.oldx-dx),int(widget.oldy-e/2),w,e,180*64,60*64)
        pixmap.draw_arc(widget.gc_line,False,int(widget.oldx-dx-w/2),int(widget.oldy-e/2),w,e,300*64,60*64)
        pixmap.draw_arc(widget.gc_line,False,int(widget.oldx-dx*0.2),int(widget.oldy-0.6*dx+2),int(1.2*dx),int(1.2*dx),0,132*64)
        pixmap.draw_arc(widget.gc_line,False,int(widget.oldx-dx),int(widget.oldy-0.6*dx+2),int(1.2*dx),int(1.2*dx),48*64,132*64)
        
        widget.queue_draw()

 
    def circle(self, widget, coords, temp, fill):
        """Draw a circle.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)
            @param  coords -- Two value tuple
            @param  temp -- switch between pixmap and pixmap_temp
            @param  fill -- Fill object

        """

        if temp == True:
            pixmap = widget.pixmap_temp
        else:
            pixmap = widget.pixmap
        width, height = widget.window.get_size()

        x, y, dx, dy = self.adjust(widget,coords)
        
        pixmap.draw_drawable(widget.gc,widget.pixmap,0,0,0,0,width,height)
        if fill == True:
            pixmap.draw_arc(widget.gc,True,x,y,dx,dy,0,360*64)
        pixmap.draw_arc(widget.gc_line,False,x,y,dx,dy,0,360*64)     
        widget.queue_draw()


    def pencil(self, widget, coords):
        """Draw a pencil.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)
            @param  coords -- Two value tuple

        """
        width, height = widget.window.get_size()
        widget.pixmap_temp.draw_drawable(widget.gc,widget.pixmap,  0 , 0 ,0,0, width, height)
        widget.pixmap.draw_line(widget.gc_line,widget.oldx,widget.oldy,coords[0],coords[1]) 
        widget.oldx = coords[0]
        widget.oldy = coords[1]
        widget.queue_draw()

    def clear(self, widget):
        """Clear the drawing.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)
        """
        logging.debug('Desenho.clear')
        
        widget.desenho = []
        widget.textos = []
        
        # try to clear a selected area first
        if widget.is_selected():
            try:
                width, height = widget.pixmap_sel.get_size()
                
                # Clear the selected area
                widget.pixmap_sel.draw_rectangle(widget.get_style().white_gc, True,0, 0, width, height)
                # Draw the selected area in the displayed pixmap
                widget.pixmap_temp.draw_drawable(widget.gc,widget.pixmap_sel,0,0,widget.orig_x,widget.orig_y, width, height)
                # Draw the selection rectangle
                widget.pixmap_temp.draw_rectangle(widget.gc_selection, False, widget.orig_x,widget.orig_y, width, height)
                widget.pixmap_temp.draw_rectangle(widget.gc_selection1,False, widget.orig_x-1,widget.orig_y-1, width+2, height+2)
                
            except NameError, message:
                logging.debug(message)
            except Exception, message:
                logging.debug('Unexpected error: %s', message)
        else:
            width, height = widget.window.get_size()
            widget.pixmap.draw_rectangle(widget.get_style().white_gc, True,0, 0, width, height)
            widget.pixmap_temp.draw_rectangle(widget.get_style().white_gc, True,0, 0, width, height)
        widget.queue_draw() 
    
    def text(self,widget,event):
        """Display and draw text in the drawing area.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)
            @param  event -- GdkEvent

        """
        
        if widget.estadoTexto == 0:
            widget.estadoTexto = 1
            
            #widget.janela.fixed.move(widget.janela.textview, int(event.x)+200, int(event.y)+100)
            # Area size has changed...
            widget.janela.fixed.move(widget.janela.textview, int(event.x), int(event.y))
            widget.janela.textview.show()
            widget.janela.textview.grab_focus()
            
        else:   
            widget.estadoTexto = 0  
            
            try:
            # This works for a gtk.Entry
                text = widget.janela.textview.get_text()
            except AttributeError:
            # This works for a gtk.TextView
                buf = widget.janela.textview.get_buffer()
                start, end = buf.get_bounds()
                text = buf.get_text(start, end)
            
            layout = widget.create_pango_layout(text)
            #layout.set_font_description(widget.font)
            
            widget.pixmap.draw_layout(widget.gc, widget.oldx, widget.oldy, layout)
            widget.pixmap_temp.draw_layout(widget.gc, widget.oldx, widget.oldy, layout)
            widget.janela.textview.hide()
            
            try:
                widget.janela.textview.set_text('')
            except AttributeError:
                buf.set_text('')

            widget.enableUndo(widget)
            
            widget.queue_draw()
            

    def selection(self, widget, coords, temp=True):
        """Make a selection.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)
            @param  coords -- Two value tuple
            @param  temp -- switch between pixmap and pixmap_temp
            @param  fill -- Fill object
            
            @return (x0,y0,x1,y1) -- coords of corners 
        """ 
      
        width, height = widget.window.get_size()

        x, y, dx, dy = self.adjust(widget,coords,True)

        widget.pixmap_temp.draw_drawable(widget.gc,widget.pixmap,0,0,0,0,width,height)
        if temp:
            widget.pixmap_temp.draw_rectangle(widget.gc_selection,False,x,y,dx,dy)
            widget.pixmap_temp.draw_rectangle(widget.gc_selection1,False,x-1,y-1,dx+2,dy+2)
        else:
            try:
                del (widget.pixmap_sel)
            except: pass
            widget.pixmap_sel = gtk.gdk.Pixmap(widget.window,dx,dy,-1)
            widget.pixmap_sel.draw_drawable(widget.gc,widget.pixmap,x,y,0,0,dx,dy)
            widget.pixmap.draw_rectangle(widget.get_style().white_gc,True,x,y,dx,dy)
            widget.orig_x = x
            widget.orig_y = y
            widget.pixmap_temp.draw_rectangle(widget.gc_selection,False,x,y,dx,dy)
            widget.pixmap_temp.draw_rectangle(widget.gc_selection1,False,x-1,y-1,dx+2,dy+2)

        widget.queue_draw()
        
    def moveSelection(self, widget, coords):
        """Move the selection.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)
            @param  coords -- Two value tuple
            @param  mvcopy -- Copy or Move
            @param  pixbuf_copy -- For import image

        """

        width, height = widget.window.get_size()

        widget.pixmap_temp.draw_drawable(widget.gc,widget.pixmap,0,0,0,0, width, height)

        dx = int(coords[0]-widget.oldx)
        dy = int(coords[1]-widget.oldy)
        
        size = widget.pixmap_sel.get_size()

        widget.pixmap_temp.draw_drawable(widget.gc,widget.pixmap_sel,0,0,widget.orig_x+dx,widget.orig_y+dy, size[0], size[1])
        
        widget.pixmap_temp.draw_rectangle(widget.gc_selection, False ,widget.orig_x+dx,widget.orig_y+dy, size[0], size[1])
        widget.pixmap_temp.draw_rectangle(widget.gc_selection1,False,widget.orig_x+dx-1,widget.orig_y+dy-1,size[0]+2,size[1]+2)

        widget.queue_draw()

    def resizeSelection(self, widget, width_percent, height_percent):
        """Resize the selection.

            @param  self -- Desenho.Desenho instance
            @param  width_percent -- Percent of x scale
            @param  height_percent -- Percent of y scale

        """ 
        width, height = widget.window.get_size()
        widget.desenha = True
        widget.selmove = True
        
        gc.collect()
        #Create the pixbuf for future resizes
        try:
            self.pixbuf_resize
        except:
            size = widget.pixmap_sel.get_size()
            self.pixbuf_resize = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB,False,8,size[0],size[1])
            self.pixbuf_resize.get_from_drawable(widget.pixmap_sel,gtk.gdk.colormap_get_system(),0,0,0,0,size[0],size[1])

        w = self.pixbuf_resize.get_width()
        h = self.pixbuf_resize.get_height()
        wr = int(w*width_percent)
        hr = int(h*height_percent)
        
        resized = self.pixbuf_resize.scale_simple(wr,hr,gtk.gdk.INTERP_HYPER)

        #Copy the resized picture to pixmap_sel
        try:
            del (widget.pixmap_sel)
        except: pass
        widget.pixmap_sel = gtk.gdk.Pixmap(widget.window,wr,hr,-1)
        widget.pixmap_sel.draw_pixbuf(widget.get_style().white_gc,resized,0,0,0,0,wr,hr)
        
        #Draw the new pixmap_sel
        widget.pixmap_temp.draw_drawable(widget.gc,widget.pixmap,0,0,0,0,width,height)
        widget.pixmap_temp.draw_drawable(widget.gc,widget.pixmap_sel,0,0,widget.orig_x,widget.orig_y,wr,hr)
        widget.pixmap_temp.draw_rectangle(widget.gc_selection,False,widget.orig_x,widget.orig_y,wr,hr)
        widget.pixmap_temp.draw_rectangle(widget.gc_selection1,False,widget.orig_x-1,widget.orig_y-1,wr+2,hr+2)

        widget.queue_draw()
        gc.collect()
        
    def polygon(self, widget, coords, temp, fill, param = None):
        """Draw polygon.

            @param  self -- Desenho.Desenho instance
            @param  widget -- Area object (GtkDrawingArea)
            @param  coords -- Two value tuple
            @param  temp -- switch between pixmap and pixmap_temp
            @param  fill -- Fill object

        """

        if temp == True:
            pixmap = widget.pixmap_temp
        else:
            pixmap = widget.pixmap
        width, height = widget.window.get_size()

        pixmap.draw_drawable(widget.gc, widget.pixmap, 0, 0, 0, 0, width, height)

        if param == "moving":
            # mouse not pressed moving
            if not widget.polygon_start:
                pixmap.draw_line(widget.gc_line,widget.last[0],widget.last[1], coords[0], coords[1])
        elif widget.polygon_start == True: # Starting a new polygon ?
            if param == "motion":
                # first press
                try:
                    pixmap.draw_line(widget.gc_line,widget.last[0],widget.last[1], coords[0], coords[1])
                    widget.pixmap.draw_line(widget.gc_line,widget.last[0],widget.last[1], coords[0], coords[1])
                    widget.points.append(coords)
                except:
                    pixmap.draw_line(widget.gc_line,widget.oldx,widget.oldy, coords[0], coords[1])
                    widget.pixmap.draw_line(widget.gc_line,widget.oldx,widget.oldy, coords[0], coords[1])
                    widget.first = widget.oldx, widget.oldy
                    widget.points = [widget.first, coords]
                widget.enableUndo(widget)
                widget.last = coords                    
            else: # param == "release"
                # first release
                try:
                    widget.first
                    widget.points.append(coords)
                    widget.pixmap.draw_line(widget.gc_line,widget.last[0],widget.last[1], coords[0], coords[1])
                except:
                    widget.first = widget.oldx, widget.oldy
                    widget.points = [widget.first, coords]                
                    widget.pixmap.draw_line(widget.gc_line,widget.oldx,widget.oldy, coords[0], coords[1])
                widget.enableUndo(widget)
                widget.last = coords
            widget.polygon_start = False
        else:
            if param == "motion":
     #           print "press"
                pixmap.draw_line(widget.gc_line,widget.last[0],widget.last[1],coords[0],coords[1])
                widget.pixmap.draw_line(widget.gc_line,widget.last[0],widget.last[1], coords[0], coords[1])
                widget.enableUndo(widget)
                widget.last = coords
                widget.points.append(coords)
            elif param == "release":
         #       print "release"
                x = coords[0] - widget.first[0]
                y = coords[1] - widget.first[1]
                d = math.hypot(x,y)
                if d > 20: # close the polygon ?
                    pixmap.draw_line(widget.gc_line,widget.last[0],widget.last[1],coords[0],coords[1])
                    widget.last = coords
                    widget.points.append(coords)
                else:
                    tp = tuple(widget.points)
                    if fill == True:
                        pixmap.draw_polygon(widget.gc, True, tp)
                    pixmap.draw_polygon(widget.gc_line, False, tp)
                    widget.last = []
                    widget.polygon_start = True
                    widget.undo_times -= 1#destroy the undo screen of polygon start 
                    widget.enableUndo(widget)
            elif param == "bug":
                tp = tuple(widget.points)
                if fill == True:
                    pixmap.draw_polygon(widget.gc, True, tp)
                pixmap.draw_polygon(widget.gc_line, False, tp)
                widget.last = []
                widget.polygon_start = True
                widget.undo_times -= 1#destroy the undo screen of polygon start 
                widget.enableUndo(widget)
        widget.queue_draw()
        
    def adjust(self, widget, coords, locked=False):
        width, height = widget.window.get_size()
        if widget.oldx > int(coords[0]):
            xi = int(coords[0])
            xf = widget.oldx
        else:
            xi = widget.oldx
            xf = int(coords[0])

        if widget.oldy > int(coords[1]):
            yi = int(coords[1])
            yf = widget.oldy
        else:
            yi = widget.oldy
            yf = int(coords[1])
        
        if locked == True:
            if xi < 0:
                xi = 0
            if yi < 0:
                yi = 0
            if xf > width:
                xf = width
            if yf > height:
                yf = height

        dx = xf-xi
        dy = yf-yi
        return xi, yi, dx, dy
       

