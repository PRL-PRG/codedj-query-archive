
import pygtk
pygtk.require( '2.0' )
import gtk

import Config

import random
    
class Block():

    WIDTH = 100
    HEIGHT = 100

    WIDTH_DIV2 = WIDTH//2
    HEIGHT_DIV2 = HEIGHT//2
   
    def __init__( self, owner, graphics_context ):
        self.owner = owner
        self.gc = graphics_context

        self.type = Block

        self.parent = None
        self.canChild = False
        self.child = None
        self.canParent = False

        self.dragging = False 

        self.placed = False
        self.x = -1 
        self.y = -1

        # TEMP
        self.color = random.choice( [ "tempBlock1", "tempBlock2", "tempBlock3", "tempBlock4", "tempBlock5" ] )

    def destroy( self ):
        if self.child:
            self.child.destroy()
            self.child = None

    def getLoc( self ):
        return ( self.x, self.y )

    def setLoc( self, x, y ):
        if x == self.x and y == self.y: return

        if self.placed:
            self.invalidate_rect( not self.dragging )
        else:
            self.placed = True
        
        self.x = x
        self.y = y
        self.endX = x + self.type.WIDTH
        self.endY = y + self.type.HEIGHT

        self.invalidate_rect( not self.dragging )

        if self.child:
            self.child.setLoc( self.endX, y )

    def testChild( self, loc ):

        if not self.canParent:
            return False

        if self.child:
            handled = self.child.testChild( loc )
            if handled: return handled
        elif abs( self.endX - loc[0] ) < 10 and abs( self.y - loc[1] ) < 10:
            return self

        return False

    def addChild( self, child ):
        self.child = child
        child._addParent( self )
        child.setLoc( self.endX, self.y )

    def removeChild( self ):
        self.child._removeParent()
        self.child = None

    def _addParent( self, parent ):
        self.parent = parent

    def _removeParent( self ):
        self.parent = None

    def getRoot( self ):
        if self.parent: return self.parent.getRoot()
        return self

    def button_press( self, event ):
        
        if event.y < self.y or event.y > self.endY:
            return False

        return self._button_pressB( event )

    def _button_pressB( self, event ):
        
        if event.x < self.x:
            return False

        if self.child:
            handled = self.child._button_pressB( event )
            if handled: return handled

        if event.x > self.endX:
            return False

        self.dragOffset = ( event.x - self.x, event.y - self.y )

        self._doButtonPress( event )

        return self

    def _doButtonPress( self, event ):
        pass # override in subclasses

    def button_release( self, event ):
        if self.dragging:
            self.dragging = False
            self.invalidateBranch()

    def motion_notify( self, event ):
        
        removeFromBlocks = not self.dragging and not self.parent

        if not self.dragging:
            self.dragging = True
            self.invalidate_rect()

        if self.parent:
            self.parent.removeChild()
        
        self.setLoc( event.x - self.dragOffset[0], event.y - self.dragOffset[1] )

        return removeFromBlocks

    def _beginDrag( self ):
        self.dragging = True
        self.dragOffset = ( self.type.WIDTH_DIV2, self.type.HEIGHT_DIV2 )

    def invalidateBranch( self, base = True ):
        self.invalidate_rect( base )
        if self.child:
            self.child.invalidateBranch( base )

    def invalidate_rect( self, base = True ):
        self.owner.invalidate_rect( self.x, self.y, self.type.WIDTH, self.type.HEIGHT, base )

    def draw( self, startX, startY, stopX, stopY, pixmap ):
        if stopY < self.y or startY > self.endY:
            return False

        self._drawB( startX, startY, stopX, stopY, pixmap )

    def _drawB( self, startX, startY, stopX, stopY, pixmap ):

        if stopX < self.x:
            return False

        if self.child:
            self.child._drawB( startX, startY, stopX, stopX, pixmap )

        if startX > self.endX:
            return False

        self._doDraw( startX, startY, stopX, stopY, pixmap )

        return True

    def _doDraw( self, startX, startY, stopX, stopY, pixmap ):
        # TEMP
        self.gc.foreground = self.owner.colors[self.color]
        pixmap.draw_rectangle( self.gc, True, self.x, self.y, self.type.WIDTH, self.type.HEIGHT )
        pass # override in subclasses

    def drawHighlight( self, startX, startY, stopX, stopY, pixmap ):
        # TEMP
        self.gc.foreground = self.owner.colors["tempWhite"]
        pixmap.draw_rectangle( self.gc, False, self.x, self.y, self.type.WIDTH-1, self.type.HEIGHT-1 )
        

class Instrument(Block):
    
    WIDTH = Block.WIDTH 
    HEIGHT = Block.HEIGHT

    WIDTH_DIV2 = WIDTH//2
    HEIGHT_DIV2 = HEIGHT//2
   
    def __init__( self, owner, graphics_context ):
        Block.__init__( self, owner, graphics_context )

        self.type = Instrument

        self.canParent = True
        self.canChild = True

    def _doButtonPress( self, event ): # we were hit with a button press
        pass
        
