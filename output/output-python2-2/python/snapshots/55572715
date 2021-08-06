
import pygtk
pygtk.require( '2.0' )
import gtk

import Config

import Jam.Block as Block

class Desktop( gtk.EventBox ):

    def __init__( self, owner ):
        gtk.EventBox.__init__( self )

        self.owner = owner

        self.drawingArea = gtk.DrawingArea()
        self.add( self.drawingArea )

        win = gtk.gdk.get_default_root_window()
        self.gc = gtk.gdk.GC( win )
        colormap = self.drawingArea.get_colormap()
        self.colors = { "bg":                 colormap.alloc_color( Config.BG_COLOR, True, True ), \
                        "Border_Active":      colormap.alloc_color( "#FF6000", True, True ), \
                        "Border_Inactive":    colormap.alloc_color( "#5D5D5D", True, True ), \
                        "Border_Highlight":   colormap.alloc_color( "#FFFFFF", True, True ), \
                        "Bg_Active":          colormap.alloc_color( "#9400BE", True, True ), \
                        "Bg_Inactive":        colormap.alloc_color( "#DBDBDB", True, True ), \
                        "tempWhite":     colormap.alloc_color( "#FFFFFF", True, True ), \
                        "tempBlock1":    colormap.alloc_color( "#227733", True, True ), \
                        "tempBlock2":    colormap.alloc_color( "#837399", True, True ), \
                        "tempBlock3":    colormap.alloc_color( "#111177", True, True ), \
                        "tempBlock4":    colormap.alloc_color( "#99AA22", True, True ), \
                        "tempBlock5":    colormap.alloc_color( "#449977", True, True ) }

        if True: # load clipmask
            pix = gtk.gdk.pixbuf_new_from_file(Config.IMAGE_ROOT+'jam-blockMask.png')
            pixels = pix.get_pixels()
            stride = pix.get_rowstride()
            channels = pix.get_n_channels()
            bitmap = ""
            byte = 0
            shift = 0
            for j in range(pix.get_height()):
                offset = stride*j
                for i in range(pix.get_width()):
                    r = pixels[i*channels+offset]
                    if r != "\0": byte += 1 << shift
                    shift += 1
                    if shift > 7:
                        bitmap += "%c" % byte
                        byte = 0
                        shift = 0
                if shift > 0:
                    bitmap += "%c" % byte
                    byte = 0
                    shift = 0
            self.clipMask = gtk.gdk.bitmap_create_from_data( None, bitmap, pix.get_width(), pix.get_height() )

        self.dirtyRectToAdd = gtk.gdk.Rectangle() # used by the invalidate_rect function
        self.screenBuf = None
        self.screenBufDirty = False
        self.screenBufDirtyRect = gtk.gdk.Rectangle()

        self.blocks = [] # items on the desktop

        # TEMP
        self.addBlock( Block.Instrument, [], ( 100, 100 ) )

        self.add_events(gtk.gdk.POINTER_MOTION_MASK|gtk.gdk.POINTER_MOTION_HINT_MASK)
        
        self.connect( "size-allocate", self.size_allocate )
        self.connect( "button-press-event", self.button_press )
        self.connect( "button-release-event", self.button_release )
        self.connect( "motion-notify-event", self.motion_notify )
        self.drawingArea.connect( "expose-event", self.expose )

        self.clickedBlock = None
        self.possibleParent = None
        self.dragging = False
        self.possibleDelete = False
 
    def size_allocate( self, widget, allocation ):
        if self.screenBuf == None or self.alloc.width != allocation.width or self.alloc.height != allocation.height:
            win = gtk.gdk.get_default_root_window()
            self.screenBuf = gtk.gdk.Pixmap( win, allocation.width, allocation.height )
            self.invalidate_rect( 0, 0, allocation.width, allocation.height )
        self.alloc = allocation
        self.absoluteLoc = [0,0]
        parent = self.get_parent()
        while parent:
            alloc = parent.get_allocation()
            self.absoluteLoc[0] += alloc.x
            self.absoluteLoc[1] += alloc.y
            parent = parent.get_parent()
        return False

    #==========================================================
    # Blocks

    def addBlock( self, blockClass, blockData, loc = (-1,-1), drag = False ):
        
        block = blockClass( self, self.gc, blockData )

        if loc[0] != -1: x = loc[0]
        else:            x = self.alloc.width//2 
        if loc[1] != -1: y = loc[1]
        elif drag:       y = self.alloc.height - block.height//2
        else:            y = self.alloc.height//2

        if drag:
            win = gtk.gdk.get_default_root_window()
            display = win.get_display()
            screen = display.get_default_screen()
            display.warp_pointer( screen, self.absoluteLoc[0] + x, self.absoluteLoc[1] + y )
            self._beginDrag( block )
            block.setLoc( x - block.width//2, y - block.height//2 )
        else:
            self.blocks.append( block )
            block.setLoc( x - block.width//2, y - block.height//2 )
 
        

    #==========================================================
    # Mouse

    def button_press( self, widget, event ):
        
        hit = False
        for i in range(len(self.blocks)-1, -1, -1):
            hit = self.blocks[i].button_press( event )
            if hit:
                self.clickedBlock = hit
                break

    def button_release( self, widget, event ):

        if self.possibleDelete:
            self.possibleDelete = False
            self.clickedBlock.destroy()
            self.clickedBlock = None
            self.possibleParent = None
            self.dragging = False

        if self.dragging:
            self.dragging = False
            
            if self.possibleParent:
                self.possibleParent.invalidate_rect( False )
                self.possibleParent.addChild( self.clickedBlock )
                root = self.possibleParent.getRoot()
                self.blocks.remove(root)
                self.blocks.append(root)
                self.possibleParent = None
            else:
                self.blocks.append( self.clickedBlock )

        if self.clickedBlock:
            self.clickedBlock.button_release( event )
            self.clickedBlock = None
            

    def motion_notify( self, widget, event ):
        
        if not self.clickedBlock:
            return

        if event.is_hint or widget != self:
            x, y, state = self.window.get_pointer()
            event.x = float(x)
            event.y = float(y)
            event.state = state
        
        self.dragging = True

        if self.clickedBlock.motion_notify( event ): # first drag of root block, remove from self.blocks
            self.blocks.remove( self.clickedBlock )

        if event.y < 0 or event.y > self.alloc.height:
            self.possibleDelete = True
            return 
        else:
            self.possibleDelete = False

        if self.clickedBlock.canChild and len(self.blocks):
            for i in range(len(self.blocks)-1, -1, -1):
                handled = self.blocks[i].testChild( self.clickedBlock.getAttachLoc() )
                if handled:
                    if self.possibleParent != handled:
                        if self.possibleParent:
                            self.possibleParent.invalidate_rect( False )
                        self.possibleParent = handled
                        self.possibleParent.invalidate_rect( False )
                    break
            if not handled and self.possibleParent:
                self.possibleParent.invalidate_rect( False )
                self.possibleParent = None

    def _beginDrag( self, block ):
        block._beginDrag()
        self.clickedBlock = block
        self.dragging = True

    #==========================================================
    # Drawing

    def draw( self ):

        startX = self.screenBufDirtyRect.x
        startY = self.screenBufDirtyRect.y
        stopX = startX + self.screenBufDirtyRect.width
        stopY = startY + self.screenBufDirtyRect.height

        self.gc.set_clip_rectangle( self.screenBufDirtyRect )

        # draw background
        self.gc.foreground = self.colors["bg"]
        self.screenBuf.draw_rectangle( self.gc, True, startX, startY, self.screenBufDirtyRect.width, self.screenBufDirtyRect.height )

        # draw blocks
        self.gc.set_clip_mask( self.clipMask )
        for block in self.blocks:
            block.draw( startX, startY, stopX, stopY, self.screenBuf )

        self.screenBufDirty = False

    def expose( self, DA, event ):

        if self.screenBufDirty:
            self.draw()

        self.drawingAreaDirty = False

        startX = event.area.x
        startY = event.area.y
        stopX = event.area.x + event.area.width
        stopY = event.area.y + event.area.height

        self.gc.set_clip_rectangle( event.area )

        # draw base
        DA.window.draw_drawable( self.gc, self.screenBuf, startX, startY, startX, startY, event.area.width, event.area.height )

        if self.possibleDelete:
            return

        self.gc.set_clip_mask( self.clipMask )

        # draw possible parent
        if self.possibleParent:
            self.possibleParent.drawHighlight( startX, startY, stopX, stopY, DA.window )

        # draw dragged objects
        if self.dragging:
            self.clickedBlock.draw( startX, startY, stopX, stopY, DA.window )

    def invalidate_rect( self, x, y, width, height, base = True ):
        self.dirtyRectToAdd.x = x
        self.dirtyRectToAdd.y = y
        self.dirtyRectToAdd.width = width
        self.dirtyRectToAdd.height = height

        #print "dirty %d %d %d %d %d %d" % (x, y, width, height, x+width, y+height)
        if base: # the base image has been dirtied
            if not self.screenBufDirty:
                self.screenBufDirtyRect.x = x
                self.screenBufDirtyRect.y = y
                self.screenBufDirtyRect.width = width
                self.screenBufDirtyRect.height = height
            else:
                self.screenBufDirtyRect = self.screenBufDirtyRect.union( self.dirtyRectToAdd )
            self.screenBufDirty = True
        if self.drawingArea.window != None:
            self.drawingArea.window.invalidate_rect( self.dirtyRectToAdd, True )
        self.drawingAreaDirty = True
 
