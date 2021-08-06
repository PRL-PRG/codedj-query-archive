import pygtk
pygtk.require( '2.0' )
import gtk

import Config

from Util.Profiler import TP
from Edit.MainWindow import CONTEXT

class TuneInterfaceParasite:

    def __init__( self, noteDB, owner, note ):
        self.parasite = self
        self.noteDB = noteDB
        self.owner = owner
        self.note = note

    def attach( self ):
        return self

    def destroy( self ):
        return

    def updateParameter( self, parameter, value ):
        return

class TuneInterface( gtk.EventBox ):

    DRAG_BLOCK = -1 # block other drag events
    DRAG_SELECT = 1
    DRAG_DESELECT = 2
    DRAG_MOVE = 3

    def __init__( self, noteDB, owner ):
        gtk.EventBox.__init__( self )

        self.noteDB = noteDB
        self.owner = owner

        self.drawingArea = gtk.DrawingArea()
        self.drawingAreaDirty = False # is the drawingArea waiting to draw?
        self.add( self.drawingArea )
        self.dirtyRectToAdd = gtk.gdk.Rectangle() # used by the invalidate_rect function

        self.selectedIds = []
        self.displayedPage = -1

        self.alloced = False
        self.width = self.baseWidth = self.height = -1
        self.waitingForAlloc = True
        self.scrollTo = -1
        self.clickX = -1

        self.set_size_request( self.width, self.height )
        self.pageSpacing = Config.PAGE_THUMBNAIL_WIDTH + Config.PAGE_THUMBNAIL_PADDING_MUL2
        self.pageOffset = Config.PAGE_THUMBNAIL_PADDING + Config.PAGE_THUMBNAIL_PADDING_DIV2

        self.dragMode = None
        self.dropAt = -1

        self.add_events(gtk.gdk.POINTER_MOTION_MASK|gtk.gdk.POINTER_MOTION_HINT_MASK)

        self.connect( "size-allocate", self.size_allocated )
        self.drawingArea.connect( "expose-event", self.draw )
        self.connect( "button-press-event", self.handleButtonPress )
        self.connect( "button-release-event", self.handleButtonRelease )
        self.connect( "motion-notify-event", self.handleMotion )

    def size_allocated( self, widget, allocation ):
        if not self.alloced:
            self.baseWidth = allocation.width
            self.baseHeight = allocation.height
            self.alloced = True
    	self.width = allocation.width
    	self.height = allocation.height
        self.drawingArea.set_size_request( self.width, self.height )

        self.pageY = (self.height-Config.PAGE_THUMBNAIL_HEIGHT)//2

        if self.scrollTo >= 0:
            self.owner.scrollTune( self.scrollTo )
            self.scrollTo = -1

        self.waitingForAlloc = False

    def updateSize( self ):
        if not self.alloced: return
        width  = self.noteDB.getPageCount()*(Config.PAGE_THUMBNAIL_WIDTH + Config.PAGE_THUMBNAIL_PADDING_MUL2) + Config.PAGE_THUMBNAIL_PADDING_MUL2
        self.waitingForAlloc = True
        self.set_size_request( max( self.baseWidth, width), -1 )

    def handleButtonPress( self, widget, event ):
        if event.button != 1:
            # bring up properties or something
            return

    	ind = int(event.x-self.pageOffset)//self.pageSpacing
        if ind >= self.noteDB.getPageCount():
            if self.dragMode != self.DRAG_MOVE:
                self.dragMode = self.DRAG_BLOCK
            return
    	if ind < 0: ind = 0

    	self.clickX = event.x

        id = self.noteDB.getPageByIndex( ind )

        if event.type == gtk.gdk._3BUTTON_PRESS: # triple click -> select all
            self.selectAll()
            self.owner.displayPage( id )
        elif event.type == gtk.gdk._2BUTTON_PRESS: # double click -> exclusive select
            self.selectPage( id )
            self.owner.displayPage( id )
        else:
            if Config.ModKeys.ctrlDown:
                if id in self.selectedIds:		 # ctrl click, selected page -> remove page from selection
                    if self.deselectPage( id ):
                        self.dragMode = self.DRAG_DESELECT
                    else:
                        self.dragMode = self.DRAG_SELECT # special case, they clicked on the last selected page and it wasn't deselected
                else:                            # ctrl click, unselected page -> add page to selection (but don't display it)
                    self.selectPage( id, False )
                    self.dragMode = self.DRAG_SELECT
            elif id in self.selectedIds:		 # click, selected page -> display this page but don't change the selection
                self.owner.displayPage( id )
            else:								 # click, unselected page -> exclusive select
                self.selectPage( id )
                self.owner.displayPage( id )

        self.owner.setContext( CONTEXT.PAGE )

    def handleButtonRelease( self, widget, event ):
        if event.button != 1:
            return

        if self.dragMode == self.DRAG_MOVE:
            self.invalidate_rect( 0, 0, self.width, self.height ) # drop head

            if self.dropAt > 0: after = self.noteDB.getPageByIndex( self.dropAt-1 )
            else: after = False

            self.noteDB.movePages( self.selectedIds, after )

            self.dropAt = -1

            self.dragMode = None

    def handleMotion( self, widget, event ):

        if event.is_hint:
            x, y, state = self.window.get_pointer()
            event.x = float(x)
            event.y = float(y)
            event.state = state

        if event.state & gtk.gdk.BUTTON1_MASK: # clicking
            if Config.ModKeys.ctrlDown and (self.dragMode == None or self.dragMode == self.DRAG_MOVE):
                self.dropAt = -1
                self.dragMode = self.DRAG_SELECT

            if self.dragMode == self.DRAG_SELECT:     # select on drag
                ind = int(event.x-self.pageOffset)//self.pageSpacing
                if ind < self.noteDB.getPageCount(): self.selectPage( self.noteDB.getPageByIndex(ind), False )
            elif self.dragMode == self.DRAG_DESELECT: # deselect on drag
                ind = int(event.x-self.pageOffset)//self.pageSpacing
                if ind < self.noteDB.getPageCount(): self.deselectPage( self.noteDB.getPageByIndex(ind) )
            elif self.dragMode == None and abs(self.clickX-event.x) > 20:					                  # drag and drop
                self.dragMode = self.DRAG_MOVE

            if self.dragMode == self.DRAG_MOVE:
                self.dropAt = int(event.x-self.pageOffset+Config.PAGE_THUMBNAIL_WIDTH_DIV2)//self.pageSpacing
                c = self.noteDB.getPageCount()
                if self.dropAt > c: self.dropAt = c
                self.invalidate_rect( 0, 0, self.width, self.height )

        else: # hovering
            ind = int(event.x-self.pageOffset)//self.pageSpacing
            if 0 <= ind < self.noteDB.getPageCount():
                id = self.noteDB.getPageByIndex(ind)
                if id != self.displayedPage:
                    self.owner.predrawPage( id )


    def displayPage( self, id, scroll ):
        if self.displayedPage == id: return -1

    	self.displayedPage = id

        if id not in self.selectedIds:
            self.selectPage( id )

        ind = self.noteDB.getPageIndex( id )

    	startX = self.pageOffset + ind*self.pageSpacing
    	stopX = startX + self.pageSpacing

    	self.invalidate_rect( 0, 0, self.width, self.height )

    	if scroll > startX: scroll = startX
    	elif scroll + self.baseWidth < stopX:
            scroll = stopX - self.baseWidth
    	    if self.waitingForAlloc:
                self.scrollTo = scroll
                return -1
    	    else:
    	        scroll = stopX - self.baseWidth

    	return scroll

    def selectPage( self, id, exclusive = True ):
        if exclusive: self.selectedIds = []

        if id in self.selectedIds: return False # no change

        ind = self.noteDB.getPageIndex( id )
        l = len(self.selectedIds)
        i = 0 # in case len(self.selectedIds) == 0
        while i < l:
            if self.noteDB.getPageIndex( self.selectedIds[i] ) > ind: break
            i += 1

        self.selectedIds.insert( i, id )

        self.invalidate_rect( 0, 0, self.width, self.height )

        return True # page added to selection

    def deselectPage( self, id, force = False ):
        if not id in self.selectedIds: return False # page isn't selected

        if not force:
            if len(self.selectedIds) <= 1: return False # don't deselect the last page

            if self.displayedPage == id:
                i = self.selectedIds.index(id)
                if i == 0: self.owner.displayPage( self.selectedIds[1] )
                else: self.owner.displayPage( self.selectedIds[i-1] )

        self.selectedIds.remove( id )
        self.invalidate_rect( 0, 0, self.width, self.height )

        return True # page removed from the selection

    def selectAll( self ):
        self.selectedIds = self.noteDB.getTune()[:]
        self.invalidate_rect( 0, 0, self.width, self.height )

    def clearSelection( self ):
        self.selectedIds = []
        self.invalidate_rect( 0, 0, self.width, self.height )

    def getSelectedIds( self ):
    	return self.selectedIds

    def getLastSelected( self ):
       return self.selectedIds[-1]

    #=======================================================
    # NoteDB notifications

    def notifyPageAdd( self, id, at ):
        self.selectPage( id )
        self.updateSize()

    def notifyPageDelete( self, which, safe ):
        for id in self.selectedIds:
            if id in which: self.deselectPage( id, True )
        self.updateSize()

    def notifyPageDuplicate( self, new, at ):
        self.clearSelection()
        for k in new.keys():
            self.selectPage( new[k], False )
        self.updateSize()

    def notifyPageMove( self, which, low, high ):
        self.invalidate_rect( 0, 0, self.width, self.height )

    #=======================================================
    #  Drawing

    def draw( self, drawingArea, event ):

    	startX = event.area.x
        startY = event.area.y
        stopX = event.area.x + event.area.width
        stopY = event.area.y + event.area.height

        context = drawingArea.window.cairo_create()
        context.set_antialias(0) # I don't know what to set this to to turn it off, and it doesn't seem to work anyway!?
        context.set_line_width( 2 )

        context.move_to( 0, 0 )
        context.rel_line_to( self.width, 0 )
        context.rel_line_to( 0, self.height )
        context.rel_line_to( -self.width, 0 )
        context.close_path()

        #draw background
        context.set_source_rgb( 0.75, 0.05, 0.0 )
        context.fill_preserve()

        # draw pages
        x = Config.PAGE_THUMBNAIL_PADDING_MUL2 # double padding on first page!
        l = len(self.selectedIds)
        j = 0
        for pageId in self.noteDB.getTune():
            if pageId == self.displayedPage:              # displayed page border
                context.set_source_rgb( 1.0, 1.0, 1.0 )
                if j<l and self.selectedIds[j] == pageId: j += 1
            elif j<l and self.selectedIds[j] == pageId:   # selected page border
                context.set_source_rgb( 0.05, 0.75, 0.8 )
                j += 1
            else:                                         # normal border
        	    context.set_source_rgb( 0.05, 0.75, 0.0 )

            context.move_to( x, self.pageY )
            context.rel_line_to( Config.PAGE_THUMBNAIL_WIDTH, 0 )
            context.rel_line_to( 0, Config.PAGE_THUMBNAIL_HEIGHT )
            context.rel_line_to( -Config.PAGE_THUMBNAIL_WIDTH, 0 )
            context.close_path()
            context.stroke()

            x += self.pageSpacing

        # draw drop marker
        if self.dropAt >= 0:
            context.set_line_width( Config.PAGE_THUMBNAIL_PADDING )
            context.set_source_rgb( 0.0, 0.0, 0.0 )
            context.move_to( Config.PAGE_THUMBNAIL_PADDING + self.pageSpacing*self.dropAt, self.pageY - 4 )
            context.rel_line_to( 0, Config.PAGE_THUMBNAIL_HEIGHT + 8 )
            context.stroke()

    def invalidate_rect( self, x, y, width, height ):
        if self.alloced == False: return
        self.dirtyRectToAdd.x = x
        self.dirtyRectToAdd.y = y
        self.dirtyRectToAdd.width = width
        self.dirtyRectToAdd.height = height
        self.drawingArea.window.invalidate_rect( self.dirtyRectToAdd, True )
        self.drawingAreaDirty = True
