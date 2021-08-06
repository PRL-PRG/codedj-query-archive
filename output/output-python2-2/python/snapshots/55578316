import pygtk
pygtk.require( '2.0' )
import gtk

from math import floor

from Framework.Constants import Constants
from GUI.GUIConstants import GUIConstants
from GUI.GUIConstants import ModKeys
from GUI.Core.NoteInterface import NoteInterface
from GUI.Core.MainWindow import ModKeys
#from GUI.Core.NoteParametersWindow import NoteParametersWindow

from Framework.Core.Profiler import TP

class SELECTNOTES:
    ALL = -1
    NONE = 0
    ADD = 1
    REMOVE = 2
    FLIP = 3
    EXCLUSIVE = 4

class TrackInterface( gtk.EventBox ):
    
    def __init__( self ):
        gtk.EventBox.__init__( self )

        self.drawingArea = gtk.DrawingArea()
        self.add( self.drawingArea )
        self.dirtyRectToAdd = gtk.gdk.Rectangle() # used by the invalidate_rect function
        
        self.fullWidth = 1 # store the maximum allowed width
        self.width = 1
        self.height = 1

        self.note = {}          # list of pages, tracks, and notes: self.note[pageId][trackId][noteId]
        self.pageBeatCount = {} # keep track of the beat count for each page
        self.pageNoteCount = {} # keep track of how many notes are on a page (so we can get rid of them when they're empty)

        self.curPage = -1   # this isn't a real page at all!
        self.beatCount = 4

        self.trackSelected = []
        self.selectedNotes = []
        for i in range(0,Constants.NUMBER_OF_TRACKS):
            self.trackSelected.insert( 0, False )
            self.selectedNotes.insert( 0, [] )

        self.curAction = False          # stores the current mouse action
        self.curActionObject = False    # stores the object that in handling the action

        self.buttonPressCount = 1   # used on release events to indicate double/triple releases
        self.clickLoc = [0,0]       # location of the last click
        self.marqueeLoc = False     # current drag location of the marquee
        self.marqueeRect = [[0,0],[0,0]]

        self.drawingArea.connect( "expose-event", self.draw )
        self.connect( "button-press-event", self.handleButtonPress )
        self.connect( "button-release-event", self.handleButtonRelease )
        self.connect( "motion-notify-event", self.handleMotion )

    #=======================================================
    #  Module Interface

    def addNotes( self, noteParams, noteCount ):
        for i in range(0,noteCount):
            if noteParams["page"][i] not in self.note: 
                self.note[noteParams["page"][i]] = []
                for j in range(0,Constants.NUMBER_OF_TRACKS):
                    self.note[noteParams["page"][i]].insert(0, {})
                self.pageBeatCount[noteParams["page"][i]] = noteParams["beatCount"][i]
                self.pageNoteCount[noteParams["page"][i]] = 0
            csnote = noteParams["csnote"][i]
            self.note[noteParams["page"][i]][noteParams["track"][i]][noteParams["note"][i]] \
                = NoteInterface( self, noteParams["page"][i], noteParams["track"][i], \
                                 csnote.pitch, csnote.onset, csnote.duration, csnote.amplitude )
            self.pageNoteCount[noteParams["page"][i]] += 1

    def updateNotes( self, noteParams, noteCount ):
        map( lambda page, track, id, csnote: \
            self.note[page][track][id].updateParams( csnote.pitch, csnote.onset, csnote.duration, csnote.amplitude ), \
            noteParams["page"], noteParams["track"], noteParams["note"], noteParams["csnote"] )        

    def deleteNotes( self, noteParams, noteCount ):
        for i in range(0,noteCount):
            self.note[noteParams["page"][i]][noteParams["track"][i]][noteParams["note"][i]].destroy()
            del self.note[noteParams["page"][i]][noteParams["track"][i]][noteParams["note"][i]]
            self.pageNoteCount[noteParams["page"][i]] -= 1
            if self.pageNoteCount[noteParams["page"][i]] == 0:
                del self.note[noteParams["page"][i]]
                del self.pageNoteCount[noteParams["page"][i]]
        
    def displayPage( self, page, beatCount ):
        if page == self.curPage and self.beatCount == beatCount: return
        
        oldPage = self.curPage
        self.curPage = page        

        if oldPage >= 0 and oldPage != page: self.clearSelectedNotes()
        
        if page not in self.note: # create a blank page if the page doesn't already exist
            self.note[page] = []
            for i in range(0,Constants.NUMBER_OF_TRACKS):
                self.note[page].insert(0, {})
            self.pageBeatCount[page] = beatCount
            self.pageNoteCount[page] = 0
        
        self.updateBeatCount( beatCount )

    def updateBeatCount( self, beatCount ):
        self.beatCount = beatCount
        
        # make sure this matches the calculation in set_size_request
        self.beatSpacing = (self.fullWidth - GUIConstants.BORDER_SIZE_MUL2 + GUIConstants.BEAT_LINE_SIZE)/self.beatCount
        self.width = self.beatSpacing * self.beatCount + GUIConstants.BORDER_SIZE_MUL2        
        self.ticksPerPixel = float(self.beatCount * Constants.TICKS_PER_BEAT) / (self.width-2*GUIConstants.BORDER_SIZE)
        self.pixelsPerTick = 1/self.ticksPerPixel

        if self.pageBeatCount[self.curPage] != beatCount:
            self.pageBeatCount[self.curPage] = beatCount
            for i in range(0,Constants.NUMBER_OF_TRACKS):
                track = self.note[self.curPage][i]
                map( lambda n:track[n].updateTransform( True ), track )
        
        if self.drawingArea.window != None:
            self.invalidate_rect( 0, 0, self.fullWidth, self.height )

    #=======================================================
    #  Event Callbacks

    def set_size_request( self, width, height ):
        self.drawingArea.set_size_request( width, height )
        
        self.trackHeight = (height - (Constants.NUMBER_OF_TRACKS-1)*GUIConstants.TRACK_SPACING) / Constants.NUMBER_OF_TRACKS 
        self.height = self.trackHeight*Constants.NUMBER_OF_TRACKS + GUIConstants.TRACK_SPACING*(Constants.NUMBER_OF_TRACKS-1)
        self.trackLimits = []
        self.trackOrigin = []
        for i in range(0,Constants.NUMBER_OF_TRACKS):
            start = i*(self.trackHeight+GUIConstants.TRACK_SPACING)
            self.trackLimits.insert( i, (start,start+self.trackHeight) )
            self.trackOrigin.insert( i, (GUIConstants.BORDER_SIZE,start+GUIConstants.BORDER_SIZE) )

        self.fullWidth = width - 2 # cut off 2 pixels cause otherwise we try to draw on an area that gets cut off!?

        # make sure this matches the calculations in updateBeatCount
        self.beatSpacing = (self.fullWidth - GUIConstants.BORDER_SIZE_MUL2 + GUIConstants.BEAT_LINE_SIZE)/self.beatCount
        self.width = self.beatSpacing * self.beatCount + GUIConstants.BORDER_SIZE_MUL2
        self.ticksPerPixel = float(self.beatCount * Constants.TICKS_PER_BEAT) / (self.width-2*GUIConstants.BORDER_SIZE)
        self.pixelsPerTick = 1/self.ticksPerPixel

        self.pitchPerPixel = float(Constants.NUMBER_OF_POSSIBLE_PITCHES-1) / (self.trackHeight-2*GUIConstants.BORDER_SIZE-GUIConstants.NOTE_HEIGHT)
        self.pixelsPerPitch = float(self.trackHeight-2*GUIConstants.BORDER_SIZE-GUIConstants.NOTE_HEIGHT)/(Constants.MAXIMUM_PITCH - Constants.MINIMUM_PITCH)

        # this could potentially take a loooong time, make sure they don't resize the window very often
        for page in self.note:
            for i in range(0,Constants.NUMBER_OF_TRACKS):
                track = self.note[page][i]
                map( lambda n:track[n].updateTransform( False ), track )

        if self.drawingArea.window != None:
            self.invalidate_rect( 0, 0, width, height )

    def handleButtonPress( self, drawingArea, event ):

        TP.ProfileBegin( "TI::handleButtonPress" )

        if event.type == gtk.gdk._2BUTTON_PRESS:   self.buttonPressCount = 2
        elif event.type == gtk.gdk._3BUTTON_PRESS: self.buttonPressCount = 3
        else:                                      self.buttonPressCount = 1

        self.clickLoc = [ event.x, event.y ]

        handled = False
        for i in range(0,Constants.NUMBER_OF_TRACKS):
            if self.trackLimits[i][0] > event.y: break
            if self.trackLimits[i][1] < event.y: continue
            
            track = self.note[self.curPage][i]
            for n in track:
                handled = track[n].handleButtonPress( self, event )
                if handled: 
                    if not self.curAction: self.curAction = True # it was handled maybe no action was declared, set curAction to True anyway
                    TP.ProfileEnd( "TI::handleButtonPress" )
                    return 

        if event.button == 3:
            print "Should bring up some note parameters or something!"
            #self.noteParameters = NoteParametersWindow( self.trackDictionary, self.getNoteParameters )
            #self.setCurrentAction( "noteParameters", False )

        TP.ProfileEnd( "TI::handleButtonPress" )


    def handleButtonRelease( self, drawingArea, event ):
        TP.ProfileBegin( "TI::handleButtonRelease" )

        if not self.curAction: #do track selection stuff here so that we can also handle marquee selection
            for i in range(0,Constants.NUMBER_OF_TRACKS):
                if self.trackLimits[i][0] > event.y: break
                if self.trackLimits[i][1] < event.y: continue    
                if event.button == 1:
                    if self.buttonPressCount == 1: self.toggleTrack( i, False )
                    else:                          self.toggleTrack( i, True )
                break
            
            TP.ProfileEnd( "TI::handleButtonRelease" )
            return

        if not self.curActionObject: # there was no real action to carry out
            self.curAction = False
            TP.ProfileEnd( "TI::handleButtonRelease" )
            return

        if self.curActionObject != self:
            self.curActionObject.handleButtonRelease( self, event, self.buttonPressCount )
        else:
            # we're doing the action ourselves
            if self.curAction == "marquee": self.doneMarquee( event )

        TP.ProfileEnd( "TI::handleButtonRelease" )
        return

    def handleMotion( self, drawingArea, event ):
        TP.ProfileBegin( "TI::handleMotion" )

        if not self.curAction: # no action is in progress yet we're dragging, start a marquee
            self.setCurrentAction( "marquee", self )

        if self.curAction == "note-drag-onset": 
            self.noteDragOnset( event )

        elif self.curAction == "note-drag-duration": 
            self.noteDragDuration( event )

        elif self.curAction == "note-drag-pitch": 
            self.noteDragPitch( event )
     
        elif self.curAction == "marquee": 
            self.updateMarquee( event )

        TP.ProfileEnd( "TI::handleMotion" )
        return

    #=======================================================
    #  Actions

    def setCurrentAction( self, action, obj ):
        if self.curAction:
            print "BackgroundView - Action already in progress!"

        self.curAction = action
        self.curActionObject = obj

        if   action == "note-drag-onset":    self.updateDragLimits()
        elif action == "note-drag-duration": self.updateDragLimits()
        elif action == "note-drag-pitch":    self.updateDragLimits()

    def doneCurrentAction( self ):
        if   self.curAction == "note-drag-onset":    self.doneNoteDrag()
        elif self.curAction == "note-drag-duration": self.doneNoteDrag()
        elif self.curAction == "note-drag-pitch":    self.doneNoteDrag()

        self.curAction = False
        self.curActionObject = False

    def toggleTrack( self, trackN, exclusive ):
        if exclusive:
            for i in range(0,Constants.NUMBER_OF_TRACKS):
                self.trackSelected[i] = False
            self.trackSelected[trackN] = True
            self.invalidate_rect( 0, 0, self.width, self.height )
        else:
            self.trackSelected[trackN] = not self.trackSelected[trackN]
            self.invalidate_rect( 0, self.trackLimits[trackN][0], self.width, self.trackLimits[trackN][1]-self.trackLimits[trackN][0] )

    def selectionChanged( self ):
        if   self.curAction == "note-drag-onset":    self.updateDragLimits()
        elif self.curAction == "note-drag-duration": self.updateDragLimits()
        elif self.curAction == "note-drag-pitch":    self.updateDragLimits()

    def applyNoteSelection( self, mode, trackN, which ):
        if mode == SELECTNOTES.ALL:
            track = self.note[self.curPage][trackN]
            map( lambda n:track[n].setSelected( True ), track )
            self.selectedNotes[trackN] = []
            map( lambda n:self.selectedNotes[trackN].append(track[n]), track )
        elif mode == SELECTNOTES.NONE:
            track = self.note[self.curPage][trackN]
            map( lambda n:track[n].setSelected( False ), track )
            self.selectedNotes[trackN] = []
        elif mode == SELECTNOTES.ADD:
            for note in which: 
                if note.setSelected( True ): 
                    self.selectedNotes[trackN].append( note )
        elif mode == SELECTNOTES.REMOVE:
            for note in which: 
                if note.setSelected( False ):
                    self.selectedNotes[trackN].remove( note )
        elif mode == SELECTNOTES.FLIP:
            for note in which:
                if note.getSelected():
                    note.setSelected( False )
                    self.selectedNotes[trackN].remove( note )
                else:
                    note.setSelected( True )
                    self.selectedNotes[trackN].append( note )
        elif mode == SELECTNOTES.EXCLUSIVE:
            track = self.note[self.curPage][trackN]
            for n in track:
                if track[n] in which: 
                    if track[n].setSelected( True ):
                        self.selectedNotes[trackN].append( track[n] )
                else: 
                    if track[n].setSelected( False ):
                        self.selectedNotes[trackN].remove( track[n] )

    def selectNotesByBar( self, trackN, start, stop ):
        for i in range(0,Constants.NUMBER_OF_TRACKS):
            if i == trackN: 
                notes = []
                track = self.note[self.curPage][trackN]
                for n in track:
                    if track[n].testOnset( start, stop ): notes.append(track[n])
                if not ModKeys.ctrlDown: self.applyNoteSelection( SELECTNOTES.EXCLUSIVE, trackN, notes )
                else:                    self.applyNoteSelection( SELECTNOTES.ADD, trackN, notes )
            else:
                if not ModKeys.ctrlDown: self.applyNoteSelection( SELECTNOTES.NONE, i, [] )
        self.selectionChanged()
        
    def selectNotesByTrack( self, trackN ):
        if ModKeys.ctrlDown:
            self.applyNoteSelection( SELECTNOTES.ALL, trackN, [] )
        else:
            for i in range(0,Constants.NUMBER_OF_TRACKS):
                if i == trackN: self.applyNoteSelection( SELECTNOTES.ALL, trackN, [] )
                else:           self.applyNoteSelection( SELECTNOTES.NONE, i, [] )
        self.selectionChanged()

    def selectNotes( self, noteDic ):
        if ModKeys.ctrlDown:
            for i in noteDic:
                self.applyNoteSelection( SELECTNOTES.FLIP, i, noteDic[i] )
        else:
            for i in range(0,Constants.NUMBER_OF_TRACKS):
                if i in noteDic: self.applyNoteSelection( SELECTNOTES.EXCLUSIVE, i, noteDic[i] )
                else:            self.applyNoteSelection( SELECTNOTES.NONE, i, [] )
        self.selectionChanged()

    def deselectNotes( self, noteDic ):
        for i in noteDic: 
            self.applyNoteSelection( SELECTNOTES.REMOVE, i, noteDic[i] )
        self.selectionChanged()

    def clearSelectedNotes( self ):
        for i in range(0,Constants.NUMBER_OF_TRACKS):
            self.applyNoteSelection( SELECTNOTES.NONE, i, [] )
        self.selectionChanged()

    def updateDragLimits( self ):
        self.dragLimits = [ [-9999,9999], [-9999,9999], [-9999,9999] ] # initialize to big numbers!
        maxRightBound = self.beatCount * Constants.TICKS_PER_BEAT
            
        for i in range(0,Constants.NUMBER_OF_TRACKS):
            if not len(self.selectedNotes[i]): continue  # no selected notes here

            track = self.note[self.curPage][i]
            leftBound = 0
            skip = True # skip the first note
            for n in track:
                if skip:
                    skip = False
                    thisNote = track[n]
                    continue
                nextNote = track[n]
                if not thisNote.getSelected(): 
                    leftBound = thisNote.getEndTick()
                else:
                    if not nextNote.getSelected(): 
                        rightBound = min( nextNote.getStartTick(), maxRightBound )
                        widthBound = rightBound
                    else:
                        rightBound = maxRightBound
                        widthBound = min( nextNote.getStartTick(), maxRightBound )
                    thisNote.updateDragLimits( self.dragLimits, leftBound, rightBound, widthBound )
                thisNote = nextNote
            # do the last note
            if thisNote.getSelected(): 
                thisNote.updateDragLimits( self.dragLimits, leftBound, maxRightBound, maxRightBound )

    def noteDragOnset( self, event ):
        do = self.pixelsToTicks( event.x - self.clickLoc[0] )
        do = min( self.dragLimits[0][1], max( self.dragLimits[0][0], do ) )
        dp = 0
        dd = 0
        
        for i in range(0,Constants.NUMBER_OF_TRACKS):
            for note in self.selectedNotes[i]:
                note.noteDrag( self, do, dp, dd )

    def noteDragDuration( self, event ):
        do = 0
        dp = 0
        dd = self.pixelsToTicks( event.x - self.clickLoc[0] )
        dd = min( self.dragLimits[2][1], max( self.dragLimits[2][0], dd ) )

        for i in range(0,Constants.NUMBER_OF_TRACKS):
            for note in self.selectedNotes[i]:
                note.noteDrag( self, do, dp, dd )

    def noteDragPitch( self, event ):
        do = 0
        dp = self.pixelsToPitch( event.y - self.clickLoc[1] )
        dp = min( self.dragLimits[1][1], max( self.dragLimits[1][0], dp ) )
        dd = 0

        for i in range(0,Constants.NUMBER_OF_TRACKS):
            for note in self.selectedNotes[i]:
                note.noteDrag( self, do, dp, dd )

    def doneNoteDrag( self ):
        for i in range(0,Constants.NUMBER_OF_TRACKS):
            for note in self.selectedNotes[i]:
                note.doneNoteDrag( self )

    def updateMarquee( self, event ):
        if self.marqueeLoc:
            oldX = self.marqueeRect[0][0]
            oldEndX = self.marqueeRect[0][0] + self.marqueeRect[1][0]
            oldY = self.marqueeRect[0][1]
            oldEndY = self.marqueeRect[0][1] + self.marqueeRect[1][1]
        else:
            oldX = oldEndX = self.clickLoc[0]
            oldY = oldEndY = self.clickLoc[1]

        self.marqueeLoc = [ event.x, event.y ]  
        if self.marqueeLoc[0] < 0: self.marqueeLoc[0] = 0
        elif self.marqueeLoc[0] > self.width: self.marqueeLoc[0] = self.width
        if self.marqueeLoc[1] < 0: self.marqueeLoc[1] = 0
        elif self.marqueeLoc[1] > self.height: self.marqueeLoc[1] = self.height

        if self.marqueeLoc[0] > self.clickLoc[0]:
            self.marqueeRect[0][0] = self.clickLoc[0]
            self.marqueeRect[1][0] = self.marqueeLoc[0] - self.clickLoc[0]
        else:
            self.marqueeRect[0][0] = self.marqueeLoc[0]
            self.marqueeRect[1][0] = self.clickLoc[0] - self.marqueeLoc[0]
        if self.marqueeLoc[1] > self.clickLoc[1]:
            self.marqueeRect[0][1] = self.clickLoc[1]
            self.marqueeRect[1][1] = self.marqueeLoc[1] - self.clickLoc[1]
        else:
            self.marqueeRect[0][1] = self.marqueeLoc[1]
            self.marqueeRect[1][1] = self.clickLoc[1] - self.marqueeLoc[1]
        
        x = min( self.marqueeRect[0][0], oldX )
        width = max( self.marqueeRect[0][0] + self.marqueeRect[1][0], oldEndX ) - x
        y = min( self.marqueeRect[0][1], oldY )
        height = max( self.marqueeRect[0][1] + self.marqueeRect[1][1], oldEndY ) - y
        self.invalidate_rect( x-1, y-1, width+2, height+2 ) # increase by 1 to handle whiching quadrants

    def doneMarquee( self, event ):                
        if self.marqueeLoc:
            stop =  [ self.marqueeRect[0][0] + self.marqueeRect[1][0], self.marqueeRect[0][1] + self.marqueeRect[1][1] ]

            select = {}
            
            for i in range(0,Constants.NUMBER_OF_TRACKS):
                intersectionY = [ max(self.marqueeRect[0][1],self.trackLimits[i][0]), min(stop[1],self.trackLimits[i][1]) ]
                if intersectionY[0] > intersectionY[1]:
                    continue
                
                notes = []
                for n in self.note[self.curPage][i]:
                    hit = self.note[self.curPage][i][n].handleMarqueeSelect( self, 
                                      [ self.marqueeRect[0][0], intersectionY[0] ], \
                                      [ stop[0], intersectionY[1] ] )           
                    if hit: notes.insert(0,self.note[self.curPage][i][n])

                if len(notes): select[i] = notes
            
            self.selectNotes( select )

        self.marqueeLoc = False        
        self.doneCurrentAction()
        
        self.invalidate_rect( self.marqueeRect[0][0]-1, self.marqueeRect[0][1]-1, self.marqueeRect[1][0]+2, self.marqueeRect[1][1]+2 )
    
    #=======================================================
    #  Drawing

    def draw( self, drawingArea, event ):    
        TP.ProfileBegin( "TrackInterface::draw" )

        startX = event.area.x
        startY = event.area.y
        stopX = event.area.x + event.area.width
        stopY = event.area.y + event.area.height

        context = drawingArea.window.cairo_create()
        context.set_antialias(0) # I don't know what to set this to to turn it off, and it doesn't seem to work anyway!?

        for i in range(0,Constants.NUMBER_OF_TRACKS):
            if startY > self.trackLimits[i][1]: continue
            if stopY < self.trackLimits[i][0]: break

            context.set_line_width( GUIConstants.BORDER_SIZE )    

            context.move_to( GUIConstants.BORDER_SIZE_DIV2, self.trackLimits[i][0] + GUIConstants.BORDER_SIZE_DIV2 )
            context.rel_line_to( self.width - GUIConstants.BORDER_SIZE, 0 )
            context.rel_line_to( 0, self.trackHeight - GUIConstants.BORDER_SIZE )
            context.rel_line_to( -self.width + GUIConstants.BORDER_SIZE, 0 )
            context.close_path()

            #draw background
            context.set_source_rgb( 0.75, 0.75, 0.75 )
            context.fill_preserve()
    
            # draw border
            if self.trackSelected[i]: context.set_source_rgb( 1, 1, 1 )
            else:                     context.set_source_rgb( 0, 0, 0 )
            context.stroke()   
            
            # draw beat lines
            context.set_line_width( GUIConstants.BEAT_LINE_SIZE )
            beatStart = GUIConstants.BORDER_SIZE + GUIConstants.BEAT_LINE_SIZE_DIV2
            context.set_source_rgb( 0.4, 0.4, 0.4 )
            for j in range(1,self.beatCount):
                context.move_to( beatStart + j*self.beatSpacing, self.trackLimits[i][0] + GUIConstants.BORDER_SIZE )
                context.rel_line_to( 0, self.trackHeight - GUIConstants.BORDER_SIZE_MUL2 )
                context.stroke()

            # draw notes
            track = self.note[self.curPage][i]
            for n in track:
                if not track[n].draw( context, startX, stopX ): break
        
        if self.marqueeLoc:                 # draw the selection rect
            context.set_line_width( 1 )     
            context.move_to( self.marqueeRect[0][0] + 0.5, self.marqueeRect[0][1] + 0.5 )
            context.rel_line_to( self.marqueeRect[1][0] - 1, 0 )
            context.rel_line_to( 0, self.marqueeRect[1][1] - 1 )
            context.rel_line_to( -self.marqueeRect[1][0] + 1, 0 )
            context.close_path()
            context.set_source_rgb( 1, 1, 1 )
            context.stroke()    

        TP.ProfileEnd( "TrackInterface::draw" )        
          
    def invalidate_rect( self, x, y, width, height ):
        self.dirtyRectToAdd.x = x
        self.dirtyRectToAdd.y = y
        self.dirtyRectToAdd.width = width
        self.dirtyRectToAdd.height = height
        self.drawingArea.window.invalidate_rect( self.dirtyRectToAdd, True )
        #self.queue_draw()

    def dirty( self ):
        print "should never by called!"
    
    def getTrackOrigin( self, track ):
        return self.trackOrigin[track]

    def ticksToPixels( self, ticks ):
        return int(round( ticks * self.pixelsPerTick ))
    def pixelsToTicks( self, pixels ):
        return int(round( pixels * self.ticksPerPixel ))
    def pitchToPixels( self, pitch ):
        return int(round(  ( Constants.MAXIMUM_PITCH - pitch ) * self.pixelsPerPitch ))
    def pixelsToPitch( self, pixels ):
        return int(round(-pixels*self.pitchPerPixel))
