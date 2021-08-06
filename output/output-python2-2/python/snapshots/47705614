# -*- coding: UTF-8 -*-
# Copyright 2007-2008 One Laptop Per Child
# Copyright 2007 Gerard J. Cerchio <www.circlesoft.com>
# Copyright 2008 Andrés Ambrois <andresambrois@gmail.com>
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program; if not, write to the Free Software
# Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

import gobject
import gtk

import logging

logger = logging.getLogger('PlayGo.GoBoardWidget')

class GoBoardWidget(gtk.Widget):
    ''' A Go Board Widget '''
    
    __gsignals__ =  {
            'insert-requested': (gobject.SIGNAL_RUN_FIRST, gobject.TYPE_NONE, (gobject.TYPE_INT, gobject.TYPE_INT)),
        }
        
    def __init__(self, status, size=19):
        gtk.Widget.__init__(self)

        self.status = status
        self.size = size
        
        self.lastX = -1
        self.lastY = -1
        

    def do_realize(self):
        """Called when the widget should create all of its 
        windowing resources.  Create our gtk.gdk.Window
        and load our pixmaps."""
        
        # First set an internal flag telling that we're realized
        self.set_flags(self.flags() | gtk.REALIZED)
        
        # Create our window and set the event masks we need
        self.window = gtk.gdk.Window(
                                     self.get_parent_window(), 
                                     width=self.allocation.width, 
                                     height=self.allocation.height,
                                    window_type=gtk.gdk.WINDOW_CHILD, 
                                   wclass=gtk.gdk.INPUT_OUTPUT, 
                                  event_mask=self.get_events() | gtk.gdk.EXPOSURE_MASK 
                                  | gtk.gdk.BUTTON1_MOTION_MASK | gtk.gdk.BUTTON_PRESS_MASK
                                  | gtk.gdk.POINTER_MOTION_MASK | gtk.gdk.POINTER_MOTION_HINT_MASK
                                  | gtk.gdk.BUTTON_RELEASE_MASK)
                                  
        # Asociate ourselves with this window
        self.window.set_user_data(self)
        
        # Set this window's style
        self.style.attach(self.window)
        
        # The default color of the background should be what
        # the style (theme engine) tells us.
        self.style.set_background(self.window, gtk.STATE_NORMAL)
        self.window.move_resize(*self.allocation)
        
        # Load the board pixmap
        pixbuf = gtk.gdk.pixbuf_new_from_file("./images/board.gif")
        self.BoardPixmap, mask = pixbuf.render_pixmap_and_mask()
        del pixbuf
        
        # Load the white stone pixmap
        self.WhitePixbuf = gtk.gdk.pixbuf_new_from_file("./images/white.gif")
        
        # Load the black stone pixmap
        self.BlackPixbuf = gtk.gdk.pixbuf_new_from_file("./images/black.gif")
        
        self.gc = self.style.fg_gc[gtk.STATE_NORMAL]
        
        self.connect('button-release-event', self.button_release_cb)

    def draw_lines(self):
        ctx = self.window.cairo_create()
        
        # Single width black lines
        ctx.set_line_width(1)
        ctx.set_source_rgba(0, 0, 0, 1)
        
        # Horizontal lines
        for i in xrange(1, self.size + 1):
            ctx.move_to( self.unit, i * self.unit)
            ctx.line_to(self.size * self.unit, i * self.unit )

        # Vertical lines
        for i in xrange(1, self.size + 1):
            ctx.move_to(i * self.unit, self.unit )
            ctx.line_to(i * self.unit, self.size * self.unit)
        
        ctx.stroke()
        
        # star point coords per board size
        if self.size == 19 :
            seq = [ 4, 10, 16 ]
        elif self.size == 13 :
            seq = [ 4, 7, 10 ]
        elif self.size == 9 :
            seq = [ 3, 7 ]
            # set the middle singleton
            ctx.arc( self.unit * 5, self.unit * 5, 3, 0, -1e-10)
            ctx.fill_preserve()
            ctx.stroke()
        else :
            seq = []
        
        # stroke in the star points
        #TODO: adjust size for teeny boards
        for x in seq :
            for y in seq :
                ctx.arc( self.unit * x, self.unit * y, 3, 0, -1e-10)
                ctx.fill_preserve()
                ctx.stroke()   
                
        
    def do_unrealize(self):
        # The do_unrealized method is responsible for freeing the GDK resources
        # De-associate the window we created in do_realize with ourselves
        self.window.destroy()
        
    def do_size_request(self, requisition):
        """From Widget.py: The do_size_request method Gtk+ is calling
         on a widget to ask it the widget how large it wishes to be. 
         It's not guaranteed that gtk+ will actually give this size 
         to the widget.  So we will send gtk+ an appropiate minimum size"""
        
        requisition.height = 500
        requisition.width = 500

    def do_size_allocate(self, allocation):
        """The do_size_allocate is called by when the actual 
        size is known and the widget is told how much space 
        could actually be allocated Save the allocated space
        self.allocation = allocation."""
        
        logger.debug('Allocating %s x %s for widget', allocation.height, allocation.width)
        self.allocation = allocation
        if self.flags() & gtk.REALIZED:
                self.window.move_resize(*allocation)

    def do_expose_event(self, event=None):
        """This is where the widget must draw itself."""

        #Scale everything
        self.unit = (min(self.allocation.height, self.allocation.width)+10)/(self.size + 1) 
        self.ScaledBlackPixbuf = self.BlackPixbuf.scale_simple( int(self.unit), int(self.unit), gtk.gdk.INTERP_BILINEAR )
        self.ScaledWhitePixbuf = self.WhitePixbuf.scale_simple( int(self.unit), int(self.unit), gtk.gdk.INTERP_BILINEAR )
        #Draw the board
        self.window.draw_drawable(self.gc, self.BoardPixmap, 0, 0, 0, 0, self.allocation.width, self.allocation.height)
        #Draw the lines
        self.draw_lines()
        #Draw the stones
        self.draw_stones(self.status)
        
    def get_mouse_event_xy(self, event):
        """
        calculate the x and y position on the board given pixel address
        """
        
        x0 = 0 #self.get_allocation().x
        y0 = 0 #self.get_allocation().y
        x = int(( ( event.x - x0 ) / self.unit ) - 0.5)
        y = int(( ( event.y - y0 ) / self.unit ) - 0.5)
        if x > self.size - 1: x = self.size - 1
        if y > self.size - 1: y = self.size - 1
        return x, y
        
    def draw_ghost_stone(self, x, y, color):
        x, y = self.get_pixel_from_coordinates(x, y)
        if x == self.lastX and y == self.lastY:
            return
            
        if self.lastX is not -1 :
            self.window.invalidate_rect(gtk.gdk.Rectangle(int(self.lastX - self.unit/2), int(self.lastY - self.unit/2), int(self.unit), int(self.unit)), False)
        
        self.lastX = x
        self.lastY = y
        
        ctx = self.window.cairo_create()
        if color is 'B':
            ctx.set_source_rgba(0, 0, 0, .5 )
        else:
            ctx.set_source_rgba(0xff, 0xff, 0xff, .5 )
        
        ctx.arc( self.lastX, self.lastY, self.unit/2 -4, 0, -1e-10)
        ctx.fill_preserve()
        ctx.stroke()
        del ctx
        
    def button_release_cb(self, widget, event):
        x, y = self.get_mouse_event_xy(event)
        self.emit('insert-requested', x, y)
        
    def draw_stone(self, x, y, color, widget):
        """
        paint a single stone on a point
        """
        x = x + 1
        y = y + 1
        ctx = self.window.cairo_create()
        ct = gtk.gdk.CairoContext(ctx)
        if  color == 'B': 
            ct.set_source_pixbuf(self.ScaledBlackPixbuf, self.unit*x - self.unit/2, self.unit*y - self.unit/2 )
        else:
            ct.set_source_pixbuf(self.ScaledWhitePixbuf, self.unit*x - self.unit/2, self.unit*y - self.unit/2 )
        ctx.paint()
    
    def draw_stones(self, status):
        for x in status.keys():
            self.draw_stone(x[0], x[1], status[x], self)
            
    def redraw_area(self, x, y):
        x, y = self.get_pixel_from_coordinates(x, y)
        self.window.invalidate_rect(gtk.gdk.Rectangle(int(x - self.unit/2), int(y - self.unit/2), int(self.unit), int(self.unit)), False)
        
    def get_pixel_from_coordinates(self, x, y):
        if x > self.size - 1: x = self.size - 1
        if y > self.size - 1: y = self.size - 1
        x = (x+1) * self.unit
        y = (y+1) * self.unit
        return x, y

    def clear(self):
        self.lastX = -1
        self.lastY = -1
        self.do_expose_event()
