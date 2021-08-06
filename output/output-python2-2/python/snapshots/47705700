import logging
from gettext import gettext as _
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

    def __init__( self, aBoard, activity ):
        """
        Startup the board widget
          1. setup our signals: expose, insert, button and mouse movement
          2. initialize the tracking variables
          3. read in the stone source bitmaps 
        """
        
        gtk.EventBox.__init__( self )

        self.output = gtk.DrawingArea()
        self.set_property('child', self.output)

        self.output.connect('expose-event', self.expose_cb)
        self.add_events(gtk.gdk.POINTER_MOTION_MASK)
        
        self.connect('button-release-event', self.__class__.button_release_cb)
        self.connect('motion-notify-event', self.__class__.motion_cb)
        #self.connect('size-request', self.__class__.sizeRequest_cb)

        self.activity = activity
        self.drawCoords = 1
        self.size = aBoard.size
        self.lastUnit = 0
        self.lastDat = -1
        self.lastX = 0 
        self.lastY = 0
        self.myBoard = aBoard
        self.myGame = None
        
        self.lastColor = 2
        
        logger.setLevel( logging.DEBUG )

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
        
        logger.debug( "baord widget started" )


    def sizeRequest_cb(self, requistion ):
        requistion.width = 8
        requistion.height = 8

            
    def check_coord (self, x, y):
        """
        check to see if x and y are within the board grid
        """
        return x >= 0 and x < self.size and y >= 0 and y < self.size


    def insert(self, dat, value):
        """
        Perform a Stone placement, this is called directly from self
        when playing a local game  or from the tube to a remote player
        managed in game.py
        """
        color = dat >> 16
        x = dat & 0xff
        y = ( dat >> 8 ) & 0xff
        
        logger.debug( 'stone event x=%d y=%d col=%d value=%d', x,y, dat, color )
        
        assert x < self.myBoard.size
        assert y < self.myBoard.size
        
        self.myBoard.setPointi( x, y, color )
        self.update_score(self.myBoard.score)
        return None
        
    def update_score(self,  score): 
        if score == 0: 
            return 0
        self.activity.info_panel.show_score(_("Score is: Whites %(W)d - Blacks %(B)d" % score))

#    def remove(self, column, value):
#        """Return:
#            None : no winner
#            0, 1: player 0/1 wins the game
#        """
#        y = column & 0xff
#        x = column >> 8
#        
#        logger.debug( 'remove stone event x=%d y=%d col=%d value=%d', x,y,column, value )
#        
#        assert x < self.myBoard.size
#        assert y < self.myBoard.size
#        
#        self.myBoard.setPointi( x, y, 0 )
#        return None


    def draw_background(self, rect, unit, ctx):
        """
        set the board window's background to the board image
        """

        ct = gtk.gdk.CairoContext(ctx)
        ct.set_source_pixbuf(self.pixBoard,0,0)
        ctx.paint()
        ctx.stroke()
                

    def draw_lines(self, rect, unit, ctx):
        """
        draw the grid and star points onto the board bitmap
        """
        
        # single width balck lines
        ctx.set_line_width(1)
        ctx.set_source_rgba(0, 0, 0, 1)

        for i in xrange(self.size + 1):
            ctx.move_to( unit, i * unit)
            ctx.line_to(self.size * unit, i * unit )

        for i in xrange(self.size + 1):
            ctx.move_to(i * unit, unit )
            ctx.line_to(i * unit, self.size * unit)

        ctx.stroke()
        
        # star point coords per board size
        if self.size == 19 :
            seq = [ 4, 10, 16 ]
        elif self.size == 13 :
            seq = [ 4, 7, 10 ]
        elif self.size == 9 :
            seq = [ 3, 7 ]
            # set the middle singleton
            ctx.arc( unit * 5, unit * 5, 3, 0, -1e-10)
            ctx.fill_preserve()
            ctx.stroke()
        else :
            seq = []
        
        # stroke in the star points
        #TODO: adjust size for teeny boards
        for x in seq :
            for y in seq :
                ctx.arc( unit * x, unit * y, 3, 0, -1e-10)
                ctx.fill_preserve()
                ctx.stroke()     

    def draw_stone(self, x, y, color, unit, ctx):
        """
        paint a single stone on a point
        """
        x = x + 1
        y = y + 1
        ct = gtk.gdk.CairoContext(ctx)
        if  color == 0 : 
            ct.set_source_pixbuf(self.pixBlackSized, unit*x - unit/2, unit*y - unit/2 )
        else :
            ct.set_source_pixbuf(self.pixWhiteSized, unit*x - unit/2, unit*y - unit/2 )
            
        ctx.paint()
        

    def draw_stones( self, ctx ):
        """
        paint all the stones on the board
        """

        for x in self.myBoard.status.keys() :
            if self.myBoard.status[x] == 'B' :
                self.draw_stone( x[0], x[1], 1, self.lastUnit, ctx )
            else :
                self.draw_stone( x[0], x[1], 0, self.lastUnit, ctx )
                    
        ctx.stroke()

    def get_mouse_event_col(self, event):
        """
        calculate the x and y position on the board given pixel address
        """
        
        unit, x0, y0 = self.get_coordinates(self.get_allocation())
        x = ( ( event.x - x0 ) / unit ) - 0.5
        y = ( ( event.y - y0 ) / unit ) - 0.5
        return int(x), int(y)


    def legal(self, x, y ):
        """
        boolean check if the stone play is legal
        """
        if  self.myBoard.status.has_key( (x,y) ) :
            self.activity.info_panel.show(_("There already is a stone there!"))
            return False
        
        c = 'W'
        if self.lastColor is 1 :
            c = 'B'
            
        if  not self.myBoard.legal( (x,y), c ) :
            self.activity.info_panel.show(_("Illegal move"))
            return False
        
        if self.myBoard.checkKo( (x, y), c):
            self.activity.info_panel.show(_("Ko violation!"))
            return False
        
        logger.debug( " returning legal ")
        return True

    def motion_cb(self, event):
        """
        When the mouse moves, find out if it is a legal point
        if it is a legal point place the transparent stone on 
        the point and erase the previously placed transparent 
        stone
        """

        x, y = self.get_mouse_event_col(event)
        dat = ( y << 8 ) + x
        
        if  not self.check_coord( x, y ) :
            return

        if dat == self.lastDat :
            return
        
        if self.lastX is not -1 :
            self.myWidget.window.clear_area( int(self.lastX - self.lastUnit/2), int(self.lastY - self.lastUnit/2), int(self.lastUnit), int(self.lastUnit) )
            
        if self.myBoard.status.has_key( (x,y) ) :
            self.lastX = -1
            return

        # the board is zero based and there is a 1 unit border so bump x&y
        x += 1
        y += 1
        
        # we need a cario context for drawing on top of the board bitmap
        ctx = self.myWidget.window.cairo_create()
        
        # decide whether black or white and set transparent
        if ( ( self.myGame is None ) and ( self.lastColor == 2 ) ) or \
           ( self.myGame  and not self.myGame.is_initiator ) :
            ctx.set_source_rgba(0, 0, 0, .5 )
        else :
            ctx.set_source_rgba(0xff, 0xff, 0xff, .5 )
            
        # rember our state 
        self.lastDat = dat
        self.lastX = self.lastUnit * x
        self.lastY = self.lastUnit * y
        
        #  now draw a transparent stone a 4 pixels smaller than the unit 
        ctx.arc( self.lastX, self.lastY, self.lastUnit/2 -4, 0, -1e-10)
        ctx.fill_preserve()
        ctx.stroke()
        del ctx


    def button_release_cb(self, event):
        """
        When the mouse button is released drop a stone on the board
        """

        x, y = self.get_mouse_event_col(event)
        dat = ( y << 8 ) + x
        
        logger.debug( 'Button release event x=%d y=%d, pixx=%d pixy=%d', x,y, event.x, event.y )
        
        if self.myGame is None :
                            
            if ( event.button != 3 ) :
                #if  self.myBoard.status.has_key( (x,y) ):
                if not self.legal( x, y ) :
                    return
                else :
                    if self.lastColor is 1:
                        dat = dat | 0x10000
                        self.lastColor = 2;
                        self.activity.info_panel.show(_("Black's turn "))
                    else :
                        dat = dat | 0x20000
                        self.lastColor = 1;
                        self.activity.info_panel.show(_("White's turn "))
                        
                    self.lastX = 0;    
                    self.insert( dat, 1 )
                    
        else:
            
            if event.button != 3 :
                if self.myGame.is_initiator : 
                    dat = dat | 0x10000
                else :
                    dat = dat | 0x20000
                    
            self.lastX = -1;    
            self.emit('insert-requested', dat )

        self.window.invalidate_rect(self.get_allocation(), True)

    def queue_draw(self):
        self.output.queue_draw()

    def get_coordinates(self, rect):
        """Returns tuple (unit size, origin x, origin y) suitable for drawing
        a grid within @rect."""

        rect = self.get_allocation()

        unit = rect.height / float(self.size )
        x0 = rect.x + (rect.width - self.size * unit) / 2.0
        y0 = rect.y
        
#        if rect.height / float(self.rows) < rect.width / float(self.size):
#            # wide
#            unit = rect.height / float(self.rows)
#            x0 = rect.x + (rect.width - self.size * unit) / 2.0
#            y0 = rect.y
#        else:
#            # narrow
#            unit = rect.width / float(self.size)
#            x0 = rect.x
#            y0 = rect.y + (rect.height - self.rows * unit) / 2.0
#         
        # now shrink the size for a 1 unit border   
        unit = unit - unit / self.size

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
        unit, x0, y0 = self.get_coordinates(rect)
        if self.lastUnit != unit :
            logger.debug( 'resizing the window to %d', rect.height )
            self.lastUnit = -1
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
