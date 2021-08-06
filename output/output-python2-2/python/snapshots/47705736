import logging
import cairo
import gobject
import gtk
import random

import game


logger = logging.getLogger('PlayGo.boardwidget')


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
        self.lastDat = 0
        self.lastX = 0 
        self.lastY = 0
        self.lastCmap = None
        self.myBoard = aBoard
        self.myGame = None
        
        self.lastColor = 1 

        # get the bitmap for genuine simulated wooden board
#        input = open("./images/board.gif")
#        imagebuf = input.read()
#        pixbufloader = gtk.gdk.PixbufLoader()
#        pixbufloader.write(imagebuf)
#        pixbufloader.close()
#        self.pixBoard = pixbufloader.get_pixbuf()

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
        
        logger.debug( "baord widget starts" )
        
#        for x in range( 19 ):
#            for y in range( 19 ):
#                self.myBoard.setPointi(x, y, random.randint( 0, 3 ) ) 
                

            
    def check_coord (self, i, j):
        return i >= 0 and i < self.rows and j >= 0 and j < self.columns

    def insert(self, dat, value):
        """Return:
            None : no winner
            0, 1: player 0/1 wins the game
        """
        color = dat >> 16
        y = dat & 0xff
        x = ( dat >> 8 ) & 0xff
        
        logger.debug( 'stone event x=%d y=%d col=%d value=%d', x,y, dat, color )
        
        assert x < self.myBoard.size
        assert y < self.myBoard.size
        
        self.myBoard.setPointi( x, y, color )


        return None


    def remove(self, column, value):
        """Return:
            None : no winner
            0, 1: player 0/1 wins the game
        """
        y = column & 0xff
        x = column >> 8
        
        logger.debug( 'remove stone event x=%d y=%d col=%d value=%d', x,y,column, value )
        
        assert x < self.myBoard.size
        assert y < self.myBoard.size
        
        self.myBoard.setPointi( x, y, 0 )
        return None


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
            ct.set_source_pixbuf(self.pixBlackSized, unit*x - unit/2, unit*y - unit/2 )
        else :
            ct.set_source_pixbuf(self.pixWhiteSized, unit*x - unit/2, unit*y - unit/2 )
            
        ctx.paint()
        

    def draw_stones( self, ctx ):

        for x in self.myBoard.status.keys() :
            if self.myBoard.status[x] == 'B' :
                self.draw_stone( x[0], x[1], 1, self.lastUnit, ctx )
            else :
                self.draw_stone( x[0], x[1], 0, self.lastUnit, ctx )
                    
        ctx.stroke()

    def get_mouse_event_col(self, event):
        
        unit, x0, y0 = self.get_coordinates(self.get_allocation())
        col = ( ( event.x - x0 ) / unit ) - 0.5
        row = ( ( event.y - y0 ) / unit ) - 0.5
        return int(row), int(col)

    def motion_cb(self, event):

        y, x = self.get_mouse_event_col(event)
        dat = ( y << 8 ) + x
        
        if dat == self.lastDat :
            return
        
        if self.lastX :
            self.myWidget.window.clear_area( int(self.lastX - self.lastUnit/2), int(self.lastY - self.lastUnit/2), int(self.lastUnit), int(self.lastUnit) )
            
        if self.myBoard.status.has_key( (x,y) ) :
            self.lastX = 0
            return

        x += 1
        y += 1
        dat = self.lastDat
        ctx = self.myWidget.window.cairo_create()
        

        if ( ( self.myGame is None ) and ( self.lastColor == 2 ) ) or \
           ( self.myGame  and not self.myGame.is_initiator ) :
            ctx.set_source_rgba(0, 0, 0, .5 )
        else :
            ctx.set_source_rgba(0xff, 0xff, 0xff, .5 )
            
        
        self.lastX = self.lastUnit * x
        self.lastY = self.lastUnit * y 
        ctx.arc( self.lastX, self.lastY, 16, 0, -1e-10)
        ctx.fill_preserve()
        ctx.stroke()
        del ctx

    def button_release_cb(self, event):

        x, y = self.get_mouse_event_col(event)
        dat = ( y << 8 ) + x
        
        if self.myGame is None :
            
            if event.button != 3 :
                if self.lastColor is 1:
                    dat = dat | 0x10000
                    self.lastColor = 2;
                else :
                    dat = dat | 0x20000
                    self.lastColor = 1;
                    
            self.lastX = 0;    
            self.insert( dat, 1 )
            
        else:
            
            if event.button != 3 :
                if self.myGame.is_initiator : 
                    dat = dat | 0x10000
                else :
                    dat = dat | 0x20000
                    
            self.lastX = 0;    
            self.emit('insert-requested', dat )

        logger.debug( 'mouse up button event x=%d   y=%d     row=%d col=%d   value=%x', event.x, event.y, x, y, dat )
            
        self.window.invalidate_rect(self.get_allocation(), True)

    def queue_draw(self):
        self.output.queue_draw()

    def get_coordinates(self, rect):
        """Returns tuple (unit size, origin x, origin y) suitable for drawing
        a grid within @rect."""

        rect = self.get_allocation()
        
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
         
        # now shrink the size for a 1 unit border   
        unit = unit - unit / self.rows

        #return unit, x0, y0
        return unit, 0, 0
    
    def draw(self, rect, ctx, win):
        """Draw a board using the cairo context @ctx within the rectangle
        @rect."""

        #ctx.save()
        ctx.set_line_cap(cairo.LINE_CAP_ROUND)
        unit, x0, y0 = self.get_coordinates(rect)
        
        # I could not find the resize event so...
        if self.lastUnit != unit :
            
            pixbuf = gtk.gdk.pixbuf_new_from_file_at_size("./images/board.gif", rect.width, rect.height )
            pixmap, mask = pixbuf.render_pixmap_and_mask()
            del pixbuf
            
            pctx = pixmap.cairo_create()
            self.draw_lines( rect, unit, pctx )
            win.set_back_pixmap(pixmap, False )
            del pixmap
            del pctx
            
            # now resize the stones
            self.pixBlackSized = self.pixBlack.scale_simple( int(unit), int(unit), gtk.gdk.INTERP_BILINEAR )
            self.pixWhiteSized = self.pixWhite.scale_simple( int(unit), int(unit), gtk.gdk.INTERP_BILINEAR )
            self.lastUnit = unit

            # redraw it all
            win.invalidate_rect( rect, False )
#            bx, by = self.pixBoard.get_size()
#            if rect.height > by :
#                self.pixBoard = self.pixBoard.scaleSimple( bx, by, gtk.gdk.INTERP_BILINEAR ) 
        
        ctx.translate( x0, y0 )
        self.draw_stones( ctx )       
    

    def expose_cb(self, widget, event):

        rect = self.get_allocation()
        if rect.height != rect.width :
            if rect.height > rect.width :
                widget.window.resize( rect.width, rect.width )
            else :
                widget.window.resize( rect.height, rect.height )
#                
        self.myWidget = widget
        ctx = widget.window.cairo_create()
        ctx.save()
        rect = self.get_allocation()
        self.draw(rect, ctx, widget.window )
        ctx.restore()
