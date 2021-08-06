import pygtk
pygtk.require( '2.0' )
import gtk

import Config

from Util.Profiler import TP

class TuneInterface( gtk.EventBox ):
    
    DRAG_SELECT = 1
    DRAG_DESELECT = 2
    DRAG_MOVE = 3
    
    def __init__( self, mainWindow ):
        gtk.EventBox.__init__( self )
        
        self.mainWindow = mainWindow
        
        self.drawingArea = gtk.DrawingArea()
        self.drawingAreaDirty = False # is the drawingArea waiting to draw?
        self.add( self.drawingArea )
        self.dirtyRectToAdd = gtk.gdk.Rectangle() # used by the invalidate_rect function

        self.pages = []
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
            self.mainWindow.scrollTune( self.scrollTo )
            self.scrollTo = -1
            
        self.waitingForAlloc = False

    def updateSize( self ):
        if not self.alloced: return
        width  = len(self.pages)*(Config.PAGE_THUMBNAIL_WIDTH + Config.PAGE_THUMBNAIL_PADDING_MUL2) + Config.PAGE_THUMBNAIL_PADDING_MUL2
        self.waitingForAlloc = True
        self.set_size_request( max( self.baseWidth, width), -1 )

    def handleButtonPress( self, widget, event ):
    	ind = int(event.x-self.pageOffset)//self.pageSpacing
    	if ind >= len(self.pages): return
    	if ind < 0: ind = 0
    	
    	self.clickX = event.x

        id = self.pages[ ind ]
   	
        if event.type == gtk.gdk._2BUTTON_PRESS: # double click -> exclusive select
            self.selectPage( id )
            self.mainWindow.displayPage( id )
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
                self.mainWindow.displayPage( id )
            else:								 # click, unselected page -> exclusive select
                self.selectPage( id )
                self.mainWindow.displayPage( id )
    
    def handleButtonRelease( self, widget, event ):
    
        if self.dragMode == self.DRAG_MOVE:
            self.invalidate_rect( 0, 0, self.width, self.height ) # drop head
            self.mainWindow.movePages( self.dropAt, self.selectedIds )
            self.dropAt = -1
    
        self.dragMode = None
        
    def handleMotion( self, widget, event ):
        
        if Config.ModKeys.ctrlDown and (self.dragMode == None or self.dragMode == self.DRAG_MOVE): 
            self.dropAt = -1
            self.dragMode = self.DRAG_SELECT
        
        if self.dragMode == self.DRAG_SELECT:     # select on drag
            ind = int(event.x-self.pageOffset)//self.pageSpacing
            if ind < len(self.pages): self.selectPage( self.pages[ind], False )
        elif self.dragMode == self.DRAG_DESELECT: # deselect on drag    
            ind = int(event.x-self.pageOffset)//self.pageSpacing
            if ind < len(self.pages): self.deselectPage( self.pages[ind] )
        elif self.dragMode == None and abs(self.clickX-event.x) > 20:					                  # drag and drop
            self.dragMode = self.DRAG_MOVE
        
        if self.dragMode == self.DRAG_MOVE:
            self.dropAt = int(event.x-self.pageOffset+Config.PAGE_THUMBNAIL_WIDTH_DIV2)//self.pageSpacing
            if self.dropAt > len(self.pages): self.dropAt = len(self.pages)
            self.invalidate_rect( 0, 0, self.width, self.height )            	
    
    def displayPage( self, id, scroll ):
        if self.displayedPage == id: return -1
        
    	self.displayedPage = id
    	
    	for ind in range(len(self.pages)): 
    	    if self.pages[ind] == id: break
    	
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
    
    def insertPages( self, ids, insert, select = True, exclusive = True ):
        if exclusive:
            self.clearSelection()
        for id in ids: 
            self.insertPage( id, insert, select, False, False )
            insert += 1
            
        self.updateSize()        
    	
    def insertPage( self, id, insert, select = True, exclusive = True, resize = True ):
        self.pages.insert( insert, id )
    	
        if select: self.selectPage( id, exclusive )
        else: self.invalidate_rect( 0, 0, self.width, self.height )
        
        if resize: self.updateSize()  
        
    def removePages( self, ids ):
        for id in ids:
            self.removePage( id, False )
            
        self.updateSize()  
        
    def removePage( self, id, resize = True ):
        
        if id in self.selectedIds: self.selectedIds.remove(id)
        
        self.pages.remove(id)
        
        self.invalidate_rect( 0, 0, self.width, self.height )
        
        if resize: self.updateSize()
        
    def movePage( self, remove, insert ):
        if remove == insert: return
        elif remove < insert:
          	insert -= 1
          	if remove == insert: return

        id = self.pages[remove]
        self.pages.pop(remove)
        self.pages.insert( insert, id )

        self.invalidate_rect( 0, 0, self.width, self.height )

    def selectPage( self, id, exclusive = True ):
        if exclusive: self.selectedIds = []
        
        if id in self.selectedIds: return False # no change
        
        l = len(self.selectedIds)
        j = 0
        for i in range(len(self.pages)):
            if j == l: break
            if self.pages[i] == self.selectedIds[j]: j += 1
            elif id == self.pages[i]: break
            
        self.selectedIds.insert( j, id )
        
        self.invalidate_rect( 0, 0, self.width, self.height )
        
        return True # page added to selection

    def deselectPage( self, id, force = False ):
        if not id in self.selectedIds: return False # page isn't selected
    
        if not force:
            if len(self.selectedIds) <= 1: return False # don't deselect the last page

            if self.displayedPage == id:
                for i in range(len(self.selectedIds)):
                    if self.selectedIds[i] == id:
                        if i == 0: self.mainWindow.displayPage( self.selectedIds[1] )
                        else: self.mainWindow.displayPage( self.selectedIds[i-1] )
                        break
                        
        self.selectedIds.remove( id ) 
        self.invalidate_rect( 0, 0, self.width, self.height )
        
        return True # page removed from the selection
    	        
    def clearSelection( self ):
        self.selectedIds = []
	
        self.invalidate_rect( 0, 0, self.width, self.height )
    	
    def getSelectedIds( self ):
    	return self.selectedIds

    def getAllIds( self ):
        return self.pages
    	
    def getLastSelected( self ):
        if len(self.selectedIds): 
            id = self.selectedIds[-1]
            i = 0
            for pageId in self.pages:
            	if pageId == id: break
            	i += 1
    	    return i
    	else: return 0
    	
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
        for pageId in self.pages:
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
        #self.queue_draw()
