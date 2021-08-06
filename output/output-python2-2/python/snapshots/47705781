import logging
import cairo
import gobject
import gtk

import game


logger = logging.getLogger('PlayGo-activity.gridwidget')


class BoardWidget(gtk.EventBox):
    "Gtk widget for drawing the graphical board."""

    __gsignals__ =  {
            'insert-requested': (gobject.SIGNAL_RUN_FIRST, None, [int]),
        }

    def __init__( self, aBoard ):
        
        gtk.EventBox.__init__( self )

        self.output = gtk.DrawingArea()
        self.set_property('child', self.output)

        self.output.connect('expose-event', self.expose_cb)
        self.add_events(gtk.gdk.POINTER_MOTION_MASK)
        
        self.connect('button-release-event', self.__class__.button_release_cb)
        self.connect('motion-notify-event', self.__class__.motion_cb)

        self.drawCoords = 1
        self.columns = aBoard.size
        self.rows = aBoard.size
        self.lastUnit = 0
        self.myBoard = aBoard
        
        self.lastColor = 1 

        # get the bitmap for genuine simulated wooden board
        input = open("./images/board.gif")
        imagebuf = input.read()
        pixbufloader = gtk.gdk.PixbufLoader()
        pixbufloader.write(imagebuf)
        pixbufloader.close()
        self.pixBoard = pixbufloader.get_pixbuf()

        # get the bitmap for genuine simulated white stone
        input = open("./images/white.gif")
        imagebuf = input.read()
        pixbufloader = gtk.gdk.PixbufLoader()
        pixbufloader.write(imagebuf)
        pixbufloader.close()
        self.pixWhite = pixbufloader.get_pixbuf()
        
        # get the bitmap for genuine simulated black stone
        input = open("./images/black.gif")
        imagebuf = input.read()
        pixbufloader = gtk.gdk.PixbufLoader()
        pixbufloader.write(imagebuf)
        pixbufloader.close()
        self.pixBlack = pixbufloader.get_pixbuf()
            
    def check_coord (self, i, j):
        return i >= 0 and i < self.rows and j >= 0 and j < self.columns

    def insert(self, column, value):
        """Return:
            None : no winner
            0, 1: player 0/1 wins the game
        """
        discs = [row[column] for row in self.grid]

        if -1 not in discs:
            raise ValueError('Column is full')

        row = self.rows - list(reversed(discs)).index(-1) - 1
        self.grid[row][column] = value

        return self.check_winner(row, column, value)


    def draw_background(self, rect, unit, ctx):

        ct = gtk.gdk.CairoContext(ctx)
        ct.set_source_pixbuf(self.pixBoard,0,0)
        ctx.paint()
        ctx.stroke()
                

    def draw_lines(self, rect, unit, ctx):
        
        # single width balck lines
        ctx.set_line_width(1)
        ctx.set_source_rgba(0, 0, 0, 1)

        for i in xrange(self.rows + 1):
            ctx.move_to( unit, i * unit)
            ctx.line_to(self.columns * unit, i * unit )

        for i in xrange(self.columns + 1):
            ctx.move_to(i * unit, unit )
            ctx.line_to(i * unit, self.rows * unit)

        ctx.stroke()
        
        # star point coords per board size
        if self.columns == 19 :
            seq = [ 4, 10, 16 ]
        elif self.columns == 13 :
            seq = [ 4, 7, 10 ]
        elif self.columns == 9 :
            seq = [ 3, 7 ]
            # set the middle singleton
            ctx.arc( unit * 5, unit * 5, 3, 0, -1e-10)
            ctx.fill_preserve()
            ctx.stroke()
        
        # stroke in the star points
        #TODO: adjust size for teeny boards
        for x in seq :
            for y in seq :
                ctx.arc( unit * x, unit * y, 3, 0, -1e-10)
                ctx.fill_preserve()
                ctx.stroke()

            

    def draw_stone(self, x, y, color, unit, ctx):
        
        x = x + 1
        y = y + 1
        ct = gtk.gdk.CairoContext(ctx)
        if  color == 0 : 
            ct.set_source_pixbuf(self.pixBlackSized, unit*x - unit/2, unit*y - unit/2, )
        else :
            ct.set_source_pixbuf(self.pixWhiteSized, unit*x - unit/2, unit*y - unit/2, )
            
        ctx.paint()

    def draw_stones( self, ctx ):
            
        for x in xrange(self.rows):
            for y in xrange(self.columns):
                
                point =  self.myBoard.getPoint( x, y )
                
                if ( point == 1 ) :
                    self.draw_stone( x, y, 1, self.lastUnit, ctx )
                elif ( point == 2 ) :
                    self.draw_stone( x, y, 0, self.lastUnit, ctx )
                    
        ctx.stroke()

    def get_mouse_event_col(self, event):
        
        unit, x0, y0 = self.get_coordinates(self.get_allocation())
        col = ( event.x - x0 ) / unit
        row = ( event.y - y0 ) / unit
        return int(row), int(col)

    def motion_cb(self, event):

        col = self.get_mouse_event_col(event)

    def button_release_cb(self, event):

        self.motion_cb(event)
        row, col = self.get_mouse_event_col(event)
        
        self.myBoard.setPointi( col, row, self.lastColor )
        if  self.lastColor == 1:
            self.lastColor = 2
        else :
            self.lastColor = 1
            
        self.window.invalidate_rect(self.get_allocation(), True)
        #self.emit('insert-requested', col)

    def queue_draw(self):
        self.output.queue_draw()

    def get_coordinates(self, rect):
        """Returns tuple (unit size, origin x, origin y) suitable for drawing
        a grid within @rect."""

        if rect.height / float(self.rows) < rect.width / float(self.columns):
            # wide
            unit = rect.height / float(self.rows)
            x0 = rect.x + (rect.width - self.columns * unit) / 2.0
            y0 = rect.y
        else:
            # narrow
            unit = rect.width / float(self.columns)
            x0 = rect.x
            y0 = rect.y + (rect.height - self.rows * unit) / 2.0
         
        # now shrink the size for a 1 unit boarder   
        unit = unit - unit / self.rows

        return unit, x0, y0

    def draw(self, rect, ctx):
        """Draw a grid using the cairo context @ctx within the rectangle
        @rect."""

        ctx.save()
        ctx.set_line_cap(cairo.LINE_CAP_ROUND)
        unit, x0, y0 = self.get_coordinates(rect)
        
        # I could not find the resize event so...
        if self.lastUnit != unit :
            self.pixBlackSized = self.pixBlack.scale_simple( int(unit), int(unit), gtk.gdk.INTERP_BILINEAR )
            self.pixWhiteSized = self.pixWhite.scale_simple( int(unit), int(unit), gtk.gdk.INTERP_BILINEAR )
            self.lastUnit = unit
#            bx, by = self.pixBoard.get_size()
#            if rect.height > by :
#                self.pixBoard = self.pixBoard.scaleSimple( bx, by, gtk.gdk.INTERP_BILINEAR ) 
        
        ctx.translate( x0, y0 )
        self.draw_background( rect, unit, ctx )
        self.draw_lines( rect, unit, ctx )
        self.draw_stones( ctx )        
        ctx.restore()


    def expose_cb(self, widget, event):

        ctx = widget.window.cairo_create()
        rect = self.get_allocation()
        self.draw(rect, ctx)
