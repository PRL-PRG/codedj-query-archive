
import pygtk
pygtk.require( '2.0' )
import gtk

import Config

from gettext import gettext as _

from Jam import Block
from Jam import Popup

class Desktop( gtk.EventBox ):

    def __init__( self, owner ):
        gtk.EventBox.__init__( self )

        self.owner = owner

        self.drawingArea = gtk.DrawingArea()
        self.add( self.drawingArea )

        # take drawing setup from owner
        self.gc = owner.gc
        self.colors = owner.colors
        self.blockMask = owner.blockMask

        self.noteDB = owner.noteDB

        self.dirtyRectToAdd = gtk.gdk.Rectangle() # used by the invalidate_rect function
        self.screenBuf = None
        self.screenBufDirty = False
        self.screenBufDirtyRect = gtk.gdk.Rectangle()

        self.blocks = [] # items on the desktop
        self.activeInstrument = None
        self.activeDrum = None

        self.loops = {} # dict of playing loops by loop root 

        self.add_events(gtk.gdk.POINTER_MOTION_MASK|gtk.gdk.POINTER_MOTION_HINT_MASK)
        
        self.connect( "size-allocate", self.on_size_allocate )
        self.connect( "button-press-event", self.on_button_press )
        self.connect( "button-release-event", self.on_button_release )
        self.connect( "motion-notify-event", self.on_motion_notify )
        self.drawingArea.connect( "expose-event", self.on_expose )

        self.clickedBlock = None
        self.possibleParent = None
        self.possibleSubstitute = None
        self.dragging = False
        self.possibleDelete = False

        #-- Popups --------------------------------------------
        self.rightClicked = False
        
        self.popup = {}
        self.popup[Popup.Instrument] = Popup.Instrument( _("Instrument Properties"), self.owner )
        self.popup[Popup.Drum] = Popup.Drum( _("Drum Kit Properties"), self.owner )
        self.popup[Popup.Shortcut] = Popup.Shortcut( _("Assign Key"), self.owner )

    def dumpToStream( self, ostream ):
        for b in self.blocks:
            b.dumpToStream( ostream )

    def on_size_allocate( self, widget, allocation ):
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
        
        block = blockClass( self, blockData )

        if loc[0] != -1: x = loc[0]
        else:            x = self.alloc.width//2 
        if loc[1] != -1: y = loc[1]
        elif drag:       y = self.alloc.height - block.height//2
        else:            y = self.alloc.height//2

        if drag:
            win = gtk.gdk.get_default_root_window()
            display = win.get_display()
            screen = display.get_default_screen()
            display.warp_pointer( screen, int(self.absoluteLoc[0] + x), int(self.absoluteLoc[1] + y) )
            self._beginDrag( block )
            block.setLoc( x - block.width//2, y - block.height//2 )
        else:
            self.blocks.append( block )
            block.setLoc( x - block.width//2, y - block.height//2 )

        if blockClass == Block.Instrument:
            pass
        elif blockClass == Block.Drum:
            pass
        elif blockClass == Block.Loop:
            pass

        return block

    def deleteBlock( self, block ):
        if block.type == Block.Instrument:
            if block == self.activeInstrument:
                self.activeInstrument = None
                for i in range(len(self.blocks)-1, -1, -1):
                    if self.blocks[i].type == Block.Instrument:
                        self.activateInstrument( self.blocks[i] )
                        break

        elif block.type == Block.Drum:
            if block == self.activeDrum:
                self.deactivateDrum()

        elif block.type == Block.Loop:
            pass

        if block in self.blocks:
            block.invalidate_rect( True )
            self.blocks.remove( block )

        block.destroy()
 
    def _clearDesktop( self ):
        for i in range( len(self.blocks)-1, -1, -1 ):
            self.deleteBlock( self.blocks[i] )
 
    def getInstrumentImage( self, id, active = False ):
        return self.owner.getInstrumentImage( id, active )

    def getLoopImage( self, id, active = False ):
        return self.owner.getLoopImage( id, active )

    def updateLoopImage( self, id ):
        self.owner.updateLoopImage( id )

    #==========================================================
    # State

    def activateInstrument( self, block ):
        if self.activeInstrument:
            self.activeInstrument.setActive( False )

        block.setActive( True )
        self.activeInstrument = block
        
        self.updateInstrument( block )
    
    def updateInstrument( self, block ):
        data = block.data
        self.owner._updateInstrument( data["id"], data["volume"], data["pan"], data["reverb"] )

    def activateDrum( self, block ):
        if self.activeDrum:
            self.activeDrum.setActive( False )
            self.owner._stopDrum()

        block.setActive( True )
        self.activeDrum = block

        self.updateDrum()

    def deactivateDrum( self ):
        if not self.activeDrum:
            return

        self.activeDrum.setActive( False )
        self.activeDrum = None
        self.owner._stopDrum()

    def updateDrum( self ):
        data = self.activeDrum.data
        self.owner._playDrum( data["id"], data["volume"], data["reverb"], data["beats"], data["regularity"], data["seed"] ) 

    def activateLoop( self, block ):
        block.setActive( True )

        inst = block.parent.data

        tune = []
        itr = block
        while itr != None:
            tune.append( itr.data["id"] )
            itr = itr.child

        self.loops[block] = self.owner._playLoop( inst["id"], inst["volume"], tune ) 
        
    def deactivateLoop( self, block ):
        block.setActive( False )

        self.owner._stopLoop( self.loops[block] )
        del self.loops[block]

    def updateLoop( self, block ):
        inst = block.parent.data

        tune = []
        itr = block
        while itr != None:
            tune.append( itr.data["id"] )
            itr = itr.child

        self.loops[block] = self.owner._playLoop( inst["id"], inst["volume"], tune, self.loops[block] )

    #==========================================================
    # Mouse

    def on_button_press( self, widget, event ):
        
        if event.button == 3:
            self.rightClicked = True

        hit = False
        for i in range(len(self.blocks)-1, -1, -1):
            hit = self.blocks[i].button_press( event )
            if hit:
                self.clickedBlock = hit
                break

    def on_button_release( self, widget, event ):

        if event.button == 3: # Right Click
            if self.clickedBlock:
                if self.clickedBlock.type == Block.Instrument:
                    self.popup[Popup.Instrument].setBlock( self.clickedBlock )
                    if self.popup[Popup.Instrument].is_up():
                        self.popup[Popup.Instrument].updatePosition()
                    else:
                        self.popup[Popup.Instrument].popup( True ) 

                elif self.clickedBlock.type == Block.Drum:
                    #self.popup[Popup.Drum].setBlock( self.clickedBlock )
                    if self.popup[Popup.Drum].is_up():
                        self.popup[Popup.Drum].updatePosition()
                    else:
                        self.popup[Popup.Drum].popup( True ) 

                self.clickedBlock = None
            self.rightClicked = False
            return

        if self.possibleDelete:
            self.possibleDelete = False
            self.deleteBlock( self.clickedBlock )
            self.clickedBlock = None
            self.possibleParent = None
            self.possibleSubstitute = None
            self.dragging = False

        if self.dragging:
            self.dragging = False
            
            if self.possibleParent:
                self.possibleParent.addChild( self.clickedBlock )
                root = self.possibleParent.getRoot()
                self.blocks.remove(root)
                self.blocks.append(root)
                root.invalidateBranch( True )
                self.possibleParent = None
            elif self.possibleSubstitute:
                self.possibleSubstitute.substitute( self.clickedBlock )
                if self.clickedBlock.isPlaced():
                    if self.clickedBlock.resetLoc():
                        self.blocks.append( self.clickedBlock )
                else:
                    self.deleteBlock( self.clickedBlock )
                    self.clickedBlock = None
                    if self.possibleSubstitute.type == Block.Instrument:
                        self.activateInstrument( self.possibleSubstitute )
                self.possibleSubstitute = None
            else:
                self.blocks.append( self.clickedBlock )

        if self.clickedBlock:
            self.clickedBlock.button_release( event )
            self.clickedBlock = None
            

    def on_motion_notify( self, widget, event ):

        if not self.clickedBlock or self.rightClicked:
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

        blockCount = len(self.blocks)
        if self.clickedBlock.canChild and blockCount:
            for i in range(blockCount-1, -1, -1):
                handled = self.blocks[i].testChild( self.clickedBlock.getParentAnchor() )
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

        if self.clickedBlock.canSubstitute and blockCount:
            for i in range(blockCount-1, -1, -1):
                handled = self.blocks[i].testSubstitute( self.clickedBlock )
                if handled:
                    if self.possibleSubstitute != handled:
                        if self.possibleSubstitute:
                            self.possibleSubstitute.invalidate_rect( False )
                        self.possibleSubstitute = handled
                        self.possibleSubstitute.invalidate_rect( False )
                    break
            if not handled and self.possibleSubstitute:
                self.possibleSubstitute.invalidate_rect( False )
                self.possibleSubstitute = None

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
        self.gc.set_clip_mask( self.blockMask )
        for block in self.blocks:
            block.draw( startX, startY, stopX, stopY, self.screenBuf )

        self.screenBufDirty = False

    def on_expose( self, DA, event ):

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

        self.gc.set_clip_mask( self.blockMask )

        # draw possible parent
        if self.possibleParent:
            self.possibleParent.drawHighlight( startX, startY, stopX, stopY, DA.window )

        # draw dragged objects
        if self.dragging:
            self.clickedBlock.draw( startX, startY, stopX, stopY, DA.window )

        # draw possible substitute
        if self.possibleSubstitute:
            self.possibleSubstitute.drawHighlight( startX, startY, stopX, stopY, DA.window )

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

