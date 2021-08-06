import pygtk
pygtk.require( '2.0' )
import gtk

import Config

from Util.Profiler import TP
from Edit.MainWindow import CONTEXT

from Util.NoteDB import PARAMETER

class TuneInterfaceParasite:

    def __init__( self, noteDB, owner, note ):
        self.noteDB = noteDB
        self.owner = owner
        self.note = note

        self.x = self.y = self.width = -1

    def attach( self ):
        self.updateParameter( None, None )
        return self

    def destroy( self ):
        self.owner.invalidate_thumbnail( self.note.page, self.x, self.y, self.width, 1 )

    def updateParameter( self, parameter, value ):
        if parameter == PARAMETER.AMPLITUDE: return
        x = 2 + Config.THUMBNAIL_TRACK_RECT[self.note.track][0] + self.owner.ticksToPixels( self.noteDB.getPage( self.note.page).beats, self.note.cs.onset )
        if self.note.track == Config.NUMBER_OF_TRACKS-1: # drum track
            y = Config.THUMBNAIL_TRACK_RECT[self.note.track][1] + self.owner.pitchToPixelsDrum( self.note.cs.pitch )
            if x != self.x or y != self.y:
                if parameter != None: # not the first update
                    xx = min( self.x, x )
                    yy = min( self.y, y )
                    endxx = max( self.endx, x + 1 )
                    endyy = max( self.y, y ) + 1
                    self.x = x
                    self.endx = x + 1
                    self.y = y
                    self.owner.invalidate_thumbnail( self.note.page, xx, yy, endxx-xx, endyy-yy )
                else:
                    self.x = x
                    self.endx = x + 1
                    self.y = y
                    self.owner.invalidate_thumbnail( self.note.page, x, y, 1, 1 )
        else:
            y = Config.THUMBNAIL_TRACK_RECT[self.note.track][1] + self.owner.pitchToPixels( self.note.cs.pitch )
            width = max( 1, self.owner.ticksToPixels( self.noteDB.getPage( self.note.page).beats, self.note.cs.duration ) )
            if x != self.x or y != self.y or width != self.width:
                if parameter != None: # not the first update
                    xx = min( self.x, x )
                    yy = min( self.y, y )
                    endxx = max( self.endx, x + width )
                    endyy = max( self.y, y ) + 1
                    self.x = x
                    self.endx = x + width
                    self.y = y
                    self.width = width
                    self.owner.invalidate_thumbnail( self.note.page, xx, yy, endxx-xx, endyy-yy )
                else:
                    self.x = x
                    self.endx = x + width
                    self.y = y
                    self.width = width
                    self.owner.invalidate_thumbnail( self.note.page, x, y, width, 1 )

    def draw( self, win, gc, startX, stopX ):
        if stopX < self.x: return False     # we don't need to draw and no one after us will draw
        if startX > self.endx: return True  # we don't need to draw, but maybe a later note does

        win.draw_line( gc, self.x, self.y, self.endx, self.y )

        return True # we drew something


class TuneInterface( gtk.EventBox ):

    DRAG_BLOCK = -1 # block other drag events
    DRAG_SELECT = 1
    DRAG_DESELECT = 2
    DRAG_MOVE = 3

    def __init__( self, noteDB, owner, adjustment ):
        gtk.EventBox.__init__( self )

        self.noteDB = noteDB
        self.owner = owner
        self.adjustment = adjustment
        #adjustment.connect( "changed", self.adjustmentChanged )
        adjustment.connect( "value-changed", self.adjustmentValue )

        self.drawingArea = gtk.DrawingArea()
        self.drawingAreaDirty = False # is the drawingArea waiting to draw?
        self.add( self.drawingArea )
        self.dirtyRectToAdd = gtk.gdk.Rectangle() # used by the invalidate_rect function

        self.selectedIds = []
        self.displayedPage = -1

        self.drumIndex = Config.NUMBER_OF_TRACKS-1

        self.trackRect = Config.THUMBNAIL_TRACK_RECT
        self.thumbnail = {}
        self.thumbnailDirty = {}
        self.thumbnailDirtyRect = {}
        self.defaultwin = gtk.gdk.get_default_root_window() # used when creating pixmaps
        self.gc = gtk.gdk.GC( self.defaultwin )
        colormap = self.drawingArea.get_colormap()
        self.bgColor = colormap.alloc_color( Config.BG_COLOR, True, True )
        self.lineColor = colormap.alloc_color( Config.THUMBNAIL_DRAG_COLOR, True, True )
        self.trackColor = colormap.alloc_color( Config.THUMBNAIL_TRACK_COLOR, True, True )
        self.displayedColor = colormap.alloc_color( Config.THUMBNAIL_DISPLAYED_COLOR, True, True )
        self.selectedColor = colormap.alloc_color( Config.THUMBNAIL_SELECTED_COLOR, True, True )

        # prepare thumbnail
        pix = gtk.gdk.pixbuf_new_from_file( Config.IMAGE_ROOT+"pageThumbnailBG.png" )
        self.thumbnailBG = gtk.gdk.Pixmap( self.defaultwin, Config.PAGE_THUMBNAIL_WIDTH, Config.PAGE_THUMBNAIL_HEIGHT )
        self.gc.foreground = self.bgColor
        self.thumbnailBG.draw_rectangle( self.gc, True, 0, 0, Config.PAGE_THUMBNAIL_WIDTH, Config.PAGE_THUMBNAIL_HEIGHT )
        self.thumbnailBG.draw_pixbuf( self.gc, pix, 0, 0, 0, 0, Config.PAGE_THUMBNAIL_WIDTH, Config.PAGE_THUMBNAIL_HEIGHT, gtk.gdk.RGB_DITHER_NONE )

        # load clipmask
        pix = gtk.gdk.pixbuf_new_from_file(Config.IMAGE_ROOT+'pageThumbnailMask.png')
        pixels = pix.get_pixels()
        stride = pix.get_rowstride()
        channels = pix.get_n_channels()
        bitmap = ""
        byte = 0
        shift = 0
        for j in range(pix.get_height()):
            offset = stride*j
            for i in range(pix.get_width()):
                if pixels[i*channels+offset] != "\0":
                    byte += 1 << shift
                shift += 1
                if shift > 7:
                    bitmap += "%c" % byte
                    byte = 0
                    shift = 0
            if shift:
                bitmap += "%c" % byte
                byte = 0
                shift = 0
        self.clipMask = gtk.gdk.bitmap_create_from_data( None, bitmap, pix.get_width(), pix.get_height() )
        self.clearMask = gtk.gdk.Rectangle( 0, 0, 1200, 800 )

        self.pageOffset = 5 # offset the first page by this
        self.dropWidth = 5      # line thickness of the drop head
        self.dropWidthDIV2 = self.dropWidth//2

        self.pixelsPerPitch = float(self.trackRect[0][3]-1)/(Config.MAXIMUM_PITCH - Config.MINIMUM_PITCH)
        self.pixelsPerPitchDrum = float(self.trackRect[self.drumIndex][3]-1)/(Config.MAXIMUM_PITCH_DRUM - Config.MINIMUM_PITCH_DRUM )
        self.pixelsPerTick = [0] + [ float(self.trackRect[0][2]-4)/(i*Config.TICKS_PER_BEAT) for i in range(1,Config.MAXIMUM_BEATS+1) ]

        self.alloced = False
        self.width = self.baseWidth = self.height = -1
        self.waitingForAlloc = True
        self.scrollTo = None
        self.clickX = -1

        self.set_size_request( self.width, self.height )

        self.button1Down = False
        self.dragMode = None
        self.dropAt = -1
        self.dropAtX = 0

        self.visibleX = 0
        self.visibleEndX = 0

        self.add_events(gtk.gdk.POINTER_MOTION_MASK|gtk.gdk.POINTER_MOTION_HINT_MASK)

        self.connect( "size-allocate", self.size_allocated )
        self.drawingArea.connect( "expose-event", self.draw )
        self.connect( "button-press-event", self.handleButtonPress )
        self.connect( "button-release-event", self.handleButtonRelease )
        self.connect( "motion-notify-event", self.handleMotion )

    def size_allocated( self, widget, allocation ):
        if not self.alloced:
            self.baseWidth = allocation.width
            self.visibleEndX = self.baseWidth
            self.baseHeight = allocation.height
            self.alloced = True
    	self.width = allocation.width
    	self.height = allocation.height
        self.drawingArea.set_size_request( self.width, self.height )
        self.clearMask.height = self.height
        self.clearMask.width = self.width

        self.pageY = (self.height-Config.PAGE_THUMBNAIL_HEIGHT)//2

        if self.scrollTo != None:
            if self.scrollTo >= 0: self.adjustment.set_value( self.scrollTo )
            else: self.adjustment.set_value( self.width - self.baseWidth )
            self.scrollTo = None

        self.waitingForAlloc = False

    def adjustmentValue( self, adj ):
        self.visibleX = int(adj.value)
        self.visibleEndX = self.visibleX + self.baseWidth

    def updateSize( self ):
        if not self.alloced: return
        width  = self.pageOffset + self.noteDB.getPageCount()*Config.PAGE_THUMBNAIL_WIDTH
        self.waitingForAlloc = True
        self.set_size_request( max( self.baseWidth, width), -1 )
        self.invalidate_rect( self.visibleX, 0, self.baseWidth, self.height )

    def handleButtonPress( self, widget, event ):
        if event.button != 1:
            # bring up properties or something
            return

        self.button1Down = True

        self.owner.abortPredrawPage()

        ind = int(event.x-self.pageOffset)//Config.PAGE_THUMBNAIL_WIDTH
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
                        self.dragLastInd = ind
                    else:
                        self.dragMode = self.DRAG_SELECT # special case, they clicked on the last selected page and it wasn't deselected
                        self.dragLastInd = ind
                else:                                    # ctrl click, unselected page -> add page to selection (but don't display it)
                    self.selectPage( id, False )
                    self.dragMode = self.DRAG_SELECT
                    self.dragLastInd = ind
            elif id in self.selectedIds:		 # click, selected page -> display this page but don't change the selection
                self.owner.displayPage( id )
            else:								 # click, unselected page -> exclusive select
                self.selectPage( id )
                self.owner.displayPage( id )


        self.owner.setContext( CONTEXT.PAGE )

    def handleButtonRelease( self, widget, event ):
        if event.button != 1:
            return

        self.button1Down = False

        if self.dragMode == self.DRAG_MOVE:
            self.invalidate_rect( self.dropAtX - self.dropWidthDIV2, 0, self.dropWidth, self.height ) # drop head

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

        if self.button1Down: # clicking
            if Config.ModKeys.ctrlDown and (self.dragMode == None or self.dragMode == self.DRAG_MOVE):
                self.dropAt = -1
                self.dragMode = self.DRAG_SELECT
                if event.x >= self.pageOffset: ind = int(event.x-self.pageOffset)//Config.PAGE_THUMBNAIL_WIDTH
                else: ind = 0
                self.dragLastInd = ind

            if self.dragMode == self.DRAG_SELECT:     # select on drag
                if event.x > self.pageOffset: ind = int(event.x-self.pageOffset)//Config.PAGE_THUMBNAIL_WIDTH
                else: ind = 0
                pageCount = self.noteDB.getPageCount()
                if ind >= pageCount: ind = pageCount-1
                for i in range( min(ind,self.dragLastInd), max(ind,self.dragLastInd)+1):
                    self.selectPage( self.noteDB.getPageByIndex(i), False )
                self.dragLastInd = ind
            elif self.dragMode == self.DRAG_DESELECT: # deselect on drag
                if event.x > self.pageOffset: ind = int(event.x-self.pageOffset)//Config.PAGE_THUMBNAIL_WIDTH
                else: ind = 0
                pageCount = self.noteDB.getPageCount()
                if ind >= pageCount: ind = pageCount-1
                for i in range( min(ind,self.dragLastInd), max(ind,self.dragLastInd)+1):
                    self.deselectPage( self.noteDB.getPageByIndex(i) )
                self.dragLastInd = ind
            elif self.dragMode == None and abs(self.clickX-event.x) > 20:					                  # drag and drop
                self.dragMode = self.DRAG_MOVE

            if self.dragMode == self.DRAG_MOVE:
                if self.dropAt >= 0: lastX = self.dropAtX
                else: lastX = -1
                if event.x > self.pageOffset: self.dropAt = int(event.x-self.pageOffset+Config.PAGE_THUMBNAIL_WIDTH_DIV2)//Config.PAGE_THUMBNAIL_WIDTH
                else: self.dropAt = 0
                c = self.noteDB.getPageCount()
                if self.dropAt > c: self.dropAt = c
                self.dropAtX = self.pageOffset + self.dropAt*Config.PAGE_THUMBNAIL_WIDTH - self.dropWidthDIV2 - 1
                if lastX >= 0 and lastX != self.dropAtX:
                    if lastX < self.dropAtX:
                        x = lastX - self.dropWidthDIV2
                        w = self.dropAtX - lastX + self.dropWidth
                    else:
                        x = self.dropAtX - self.dropWidthDIV2
                        w = lastX - self.dropAtX + self.dropWidth
                    self.invalidate_rect( x, 0, w, self.height )
                elif lastX == -1:
                    self.invalidate_rect( self.dropAtX-self.dropWidthDIV2, 0, self.dropWidth, self.height )

        else: # hovering
            ind = int(event.x-self.pageOffset)//Config.PAGE_THUMBNAIL_WIDTH
            if ind != self.lastPredrawInd and 0 <= ind < self.noteDB.getPageCount():
                id = self.noteDB.getPageByIndex(ind)
                if id != self.displayedPage:
                    self.owner.predrawPage( id )
                    self.lastPredrawInd = ind


    def trackToggled( self, i ):
        self.invalidate_rect( self.visibleX, 0, self.baseWidth, self.height )

    def displayPage( self, id ):
        if self.displayedPage == id: return -1

        self.lastPredrawInd = -1

        if self.displayedPage != -1:
            ind = self.noteDB.getPageIndex( self.displayedPage )
            self.invalidate_rect( self.pageOffset + ind*Config.PAGE_THUMBNAIL_WIDTH, 0, Config.PAGE_THUMBNAIL_WIDTH, self.height )

        if not self.thumbnail.has_key( id ):
            # premptive add
            self.thumbnail[id] = gtk.gdk.Pixmap( self.defaultwin, Config.PAGE_THUMBNAIL_WIDTH, Config.PAGE_THUMBNAIL_HEIGHT )
            self.thumbnailDirtyRect[id] = gtk.gdk.Rectangle( 0, 0, Config.PAGE_THUMBNAIL_WIDTH, Config.PAGE_THUMBNAIL_HEIGHT )
            self.thumbnailDirty[id] = True
            self.selectPage( id )
            self.updateSize()

    	self.displayedPage = id

        if id not in self.selectedIds:
            self.selectPage( id )

        ind = self.noteDB.getPageIndex( id )

        startX = self.pageOffset + ind*Config.PAGE_THUMBNAIL_WIDTH
        stopX = startX + Config.PAGE_THUMBNAIL_WIDTH

        if self.adjustment.value > startX:
            scroll = startX + Config.PAGE_THUMBNAIL_WIDTH + Config.PAGE_THUMBNAIL_WIDTH_DIV2 - self.baseWidth
            if scroll < 0: scroll = 0
            self.adjustment.set_value( scroll )
        elif self.adjustment.value + self.baseWidth < stopX:
            scroll = startX - Config.PAGE_THUMBNAIL_WIDTH_DIV2
            if scroll + self.baseWidth > self.width:
                if self.waitingForAlloc:
                    self.scrollTo = -1
                else:
                    self.adjustment.set_value( self.width - self.baseWidth )
            else:
                if self.waitingForAlloc:
                    self.scrollTo = scroll
    	        else:
                    self.adjustment.set_value( scroll )

        self.invalidate_rect( startX, 0, Config.PAGE_THUMBNAIL_WIDTH, self.height )

    def selectPage( self, id, exclusive = True ):
        if exclusive: 
            self.clearSelection()

        if id in self.selectedIds: return False # no change

        ind = self.noteDB.getPageIndex( id )
        l = len(self.selectedIds)
        i = 0 # in case len(self.selectedIds) == 0
        while i < l:
            if self.noteDB.getPageIndex( self.selectedIds[i] ) > ind: break
            i += 1

        self.selectedIds.insert( i, id )

        self.invalidate_rect( self.pageOffset + ind*Config.PAGE_THUMBNAIL_WIDTH, 0, Config.PAGE_THUMBNAIL_WIDTH, self.height )

        return True # page added to selection

    def deselectPage( self, id, force = False, skip_redraw = False ):
        if not id in self.selectedIds: return False # page isn't selected

        if not force:
            if len(self.selectedIds) <= 1: return False # don't deselect the last page

            if self.displayedPage == id:
                i = self.selectedIds.index(id)
                if i == 0: self.owner.displayPage( self.selectedIds[1] )
                else: self.owner.displayPage( self.selectedIds[i-1] )

        self.selectedIds.remove( id )
        if not skip_redraw:
            ind = self.noteDB.getPageIndex( id )
            self.invalidate_rect( self.pageOffset + ind*Config.PAGE_THUMBNAIL_WIDTH, 0, Config.PAGE_THUMBNAIL_WIDTH, self.height )

        return True # page removed from the selection

    def selectPages( self, which ):
        self.clearSelection()
        self.selectedIds += which

    def selectAll( self ):
        self.selectedIds = self.noteDB.getTune()[:]
        self.invalidate_rect( self.visibleX, 0, self.baseWidth, self.height )

    def clearSelection( self ):
        self.selectedIds = []
        self.invalidate_rect( self.visibleX, 0, self.baseWidth, self.height )

    def getSelectedIds( self ):
    	return self.selectedIds

    def getDisplayedIndex( self ):
        return self.selectedIds.index( self.displayedPage )

    def getFirstSelected( self ):
       return self.selectedIds[0]

    def getLastSelected( self ):
       return self.selectedIds[-1]

    #=======================================================
    # NoteDB notifications

    def notifyPageAdd( self, id, at ):
        if not self.thumbnail.has_key(id):
            self.thumbnail[id] = gtk.gdk.Pixmap( self.defaultwin, Config.PAGE_THUMBNAIL_WIDTH, Config.PAGE_THUMBNAIL_HEIGHT )
            self.thumbnailDirtyRect[id] = gtk.gdk.Rectangle( 0, 0, Config.PAGE_THUMBNAIL_WIDTH, Config.PAGE_THUMBNAIL_HEIGHT )
            self.thumbnailDirty[id] = True
            self.selectPage( id )
            self.updateSize()

    def notifyPageDelete( self, which, safe ):
        for id in self.selectedIds:
            if id in which: 
                self.deselectPage( id, True, True )
        for id in which:
            del self.thumbnail[id]
            del self.thumbnailDirtyRect[id]
            del self.thumbnailDirty[id]
        if self.displayedPage in which:
            self.displayedPage = -1
        self.updateSize()

    def notifyPageDuplicate( self, new, at ):
        for id in new:
            self.thumbnail[new[id]] = gtk.gdk.Pixmap( self.defaultwin, Config.PAGE_THUMBNAIL_WIDTH, Config.PAGE_THUMBNAIL_HEIGHT )
            self.thumbnailDirtyRect[new[id]] = gtk.gdk.Rectangle( 0, 0, Config.PAGE_THUMBNAIL_WIDTH, Config.PAGE_THUMBNAIL_HEIGHT )
            self.thumbnailDirty[new[id]] = True
        self.updateSize()

    def notifyPageMove( self, which, low, high ):
        self.invalidate_rect( self.visibleX, 0, self.baseWidth, self.height )

    #=======================================================
    #  Drawing

    def drawThumbnail( self, id, pixmap, rect ):
        startX = rect.x
        startY = rect.y
        stopX = rect.x + rect.width
        stopY = rect.y + rect.height

        # draw background
        pixmap.draw_drawable( self.gc, self.thumbnailBG, startX, startY, startX, startY, rect.width, rect.height+1 )

        # draw regular tracks
        self.gc.foreground = self.lineColor
        self.gc.set_line_attributes( 1, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_BUTT, gtk.gdk.JOIN_MITER )
        for i in range(self.drumIndex):
            if startY >= self.trackRect[i+1][1]: continue
            if stopY < self.trackRect[i][1]: break

            # draw notes
            notes = self.noteDB.getNotesByTrack( id, i, self )
            for n in range( len(notes) ):
                if not notes[n].draw( pixmap, self.gc, startX, stopX ): break
        # drum track
        if stopY > self.trackRect[self.drumIndex][0]:
            # draw notes
            notes = self.noteDB.getNotesByTrack( id, self.drumIndex, self )
            for n in range( len(notes) ):
                if not notes[n].draw( pixmap, self.gc, startX, stopX ): break

        self.thumbnailDirty[id] = False


    def draw( self, drawingArea, event ):

    	startX = event.area.x
        startY = event.area.y
        stopX = event.area.x + event.area.width
        stopY = event.area.y + event.area.height

        self.gc.set_clip_rectangle( self.clearMask )

        # draw background
        self.gc.foreground = self.bgColor
        drawingArea.window.draw_rectangle( self.gc, True, startX, startY, event.area.width, event.area.height )

        tracks = [ self.owner.getTrackSelected(i) for i in range(Config.NUMBER_OF_TRACKS) ]

        # draw pages
        self.gc.set_clip_mask( self.clipMask )

        x = self.pageOffset
        endx = x + Config.PAGE_THUMBNAIL_WIDTH
        for pageId in self.noteDB.getTune():
            if endx < startX:
                x = endx
                endx += Config.PAGE_THUMBNAIL_WIDTH
                continue
            if x > stopX: break

            # draw thumbnail
            if self.thumbnailDirty[pageId]:
                self.gc.set_clip_origin( 0, 0 )
                self.drawThumbnail( pageId, self.thumbnail[pageId], self.thumbnailDirtyRect[pageId] )
            self.gc.set_clip_origin( x, self.pageY )
            drawingArea.window.draw_drawable( self.gc, self.thumbnail[pageId], 0, 0, x, self.pageY, Config.PAGE_THUMBNAIL_WIDTH, Config.PAGE_THUMBNAIL_HEIGHT )

            # draw border if necessary
            if pageId == self.displayedPage:  # displayed page border
                self.gc.set_function( gtk.gdk.INVERT )
                for i in range(Config.NUMBER_OF_TRACKS):
                    if tracks[i]:
                        drawingArea.window.draw_rectangle( self.gc, True, x + self.trackRect[i][0], self.pageY + self.trackRect[i][1], self.trackRect[i][2], self.trackRect[i][3] )
                self.gc.set_function( gtk.gdk.COPY )
                self.gc.foreground = self.displayedColor
                self.gc.set_clip_origin( x - Config.PAGE_THUMBNAIL_WIDTH, self.pageY )
                drawingArea.window.draw_rectangle( self.gc, True, x, self.pageY, Config.PAGE_THUMBNAIL_WIDTH, Config.PAGE_THUMBNAIL_HEIGHT )
            elif pageId in self.selectedIds:  # selected page border
                self.gc.set_function( gtk.gdk.INVERT )
                for i in range(Config.NUMBER_OF_TRACKS):
                    if tracks[i]:
                        drawingArea.window.draw_rectangle( self.gc, True, x + self.trackRect[i][0], self.pageY + self.trackRect[i][1], self.trackRect[i][2], self.trackRect[i][3] )
                self.gc.set_function( gtk.gdk.COPY )
                self.gc.foreground = self.selectedColor
                self.gc.set_clip_origin( x - Config.PAGE_THUMBNAIL_WIDTH, self.pageY )
                drawingArea.window.draw_rectangle( self.gc, True, x, self.pageY, Config.PAGE_THUMBNAIL_WIDTH, Config.PAGE_THUMBNAIL_HEIGHT )

            x += Config.PAGE_THUMBNAIL_WIDTH

        # draw drop marker
        if self.dropAt >= 0:
            self.gc.set_line_attributes( self.dropWidth, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_ROUND, gtk.gdk.JOIN_MITER )
            self.gc.foreground = self.lineColor
            drawingArea.window.draw_line( self.gc, self.dropAtX, 2, self.dropAtX, Config.PAGE_THUMBNAIL_HEIGHT-4 )

    def invalidate_rect( self, x, y, width, height ):
        if self.alloced == False: return
        if x < self.visibleX: x = self.visibleX
        if x + width > self.visibleEndX: width = self.visibleEndX - x
        if width <= 0: return

        self.dirtyRectToAdd.x = x
        self.dirtyRectToAdd.y = y
        self.dirtyRectToAdd.width = width
        self.dirtyRectToAdd.height = height
        self.drawingArea.window.invalidate_rect( self.dirtyRectToAdd, True )
        self.drawingAreaDirty = True

    def invalidate_thumbnail( self, id, x, y, width, height ):
        if not self.thumbnailDirty[id]:
            self.thumbnailDirtyRect[id].x = x
            self.thumbnailDirtyRect[id].y = y
            self.thumbnailDirtyRect[id].width = width
            self.thumbnailDirtyRect[id].height = height
            self.thumbnailDirty[id] = True
        else:
            self.dirtyRectToAdd.x = x
            self.dirtyRectToAdd.y = y
            self.dirtyRectToAdd.width = width
            self.dirtyRectToAdd.height = height
            self.thumbnailDirtyRect[id] = self.thumbnailDirtyRect[id].union( self.dirtyRectToAdd )

        ind = self.noteDB.getPageIndex( id )
        self.invalidate_rect( self.pageOffset + ind*Config.PAGE_THUMBNAIL_WIDTH, 0, Config.PAGE_THUMBNAIL_WIDTH, self.height )

    def ticksToPixels( self, beats, ticks ):
        return int(round( ticks * self.pixelsPerTick[beats] ))
    def pitchToPixels( self, pitch ):
        return int(round( ( Config.MAXIMUM_PITCH - pitch ) * self.pixelsPerPitch ))
    def pitchToPixelsDrum( self, pitch ):
        return int(round( ( Config.MAXIMUM_PITCH_DRUM - pitch ) * self.pixelsPerPitchDrum ))
