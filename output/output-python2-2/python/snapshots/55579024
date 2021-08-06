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

class INTERFACEMODE:
    DEFAULT = 0
    DRAW = 1
    PASTE = 2

class TrackInterface( gtk.EventBox ):
    
    def __init__( self, onNoteDrag ):
        gtk.EventBox.__init__( self )

        self.drawingArea = gtk.DrawingArea()
        self.drawingAreaDirty = False # is the drawingArea waiting to draw?
        self.add( self.drawingArea )
        self.dirtyRectToAdd = gtk.gdk.Rectangle() # used by the invalidate_rect function
        
        self.fullWidth = 1 # store the maximum allowed width
        self.width = 1
        self.height = 1

        self.interfaceMode = INTERFACEMODE.DRAW
        
        self.note = {}          # list of pages, tracks, and notes: self.note[pageId][trackId][noteId]
        self.pageBeatCount = {} # keep track of the beat count for each page
        self.pageNoteCount = {} # keep track of how many notes are on a page (so we can get rid of them when they're empty)
        self.noteMap = {}       # maps note ids to self.note[p][t][i]s

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
        
        self.playheadX = 0

        self.cursor = { \
            "default":          None, \
            "drag-onset":       gtk.gdk.Cursor(gtk.gdk.SB_RIGHT_ARROW), \
            "drag-pitch":       gtk.gdk.Cursor(gtk.gdk.SB_V_DOUBLE_ARROW), \
            "drag-duration":    gtk.gdk.Cursor(gtk.gdk.SB_H_DOUBLE_ARROW), \
            "drag-playhead":    gtk.gdk.Cursor(gtk.gdk.LEFT_SIDE), \
            "pencil":           gtk.gdk.Cursor(gtk.gdk.PENCIL), \
            "error":            None }

        self.add_events(gtk.gdk.POINTER_MOTION_MASK|gtk.gdk.POINTER_MOTION_HINT_MASK)

        self.drawingArea.connect( "expose-event", self.draw )
        self.connect( "button-press-event", self.handleButtonPress )
        self.connect( "button-release-event", self.handleButtonRelease )
        self.connect( "motion-notify-event", self.handleMotion )

        self.onNoteDrag = onNoteDrag

    #=======================================================
    #  Module Interface

    def addNotes( self, noteParams, noteCount ):
        at = {}
        
        for i in range(noteCount):
            p = noteParams["page"][i]
            t = noteParams["track"][i]
            if p not in at:
                at[p] = [0] * Constants.NUMBER_OF_TRACKS
                #at[p] = []
                #for j in range(Constants.NUMBER_OF_TRACKS): at[p].append(0)
            if p not in self.note: 
                self.note[p] = map(lambda x:[], range(Constants.NUMBER_OF_TRACKS))
                #self.note[p] = []
                #for j in range(Constants.NUMBER_OF_TRACKS):
                    #self.note[p].append( [] )
                self.pageBeatCount[p] = noteParams["beatCount"][i]
                self.pageNoteCount[p] = 0
            csnote = noteParams["csnote"][i]
            note = NoteInterface( self, p, noteParams["track"][i], noteParams["note"][i], \
                                  csnote.pitch, csnote.onset, csnote.duration, csnote.amplitude )
            while at[p][t] > 0:
                if self.note[p][t][at[p][t]-1].getStartTick() < csnote.onset: break
                at[p][t] -= 1
            last = len(self.note[p][t])
            while at[p][t] < last:
                if self.note[p][t][at[p][t]].getStartTick() > csnote.onset: break
                at[p][t] += 1
            self.note[p][t].insert( at[p][t], note )
            self.pageNoteCount[p] += 1
            at[p][t] += 1 # assume the next note will fall after this one
            
        for page in at:
            self.updateNoteMap( page )

    def updateNotes( self, noteParams, noteCount ):
        map( lambda page, track, id, csnote: \
            self.note[page][track][self.noteMap[page][id]].updateParams( csnote.pitch, csnote.onset, csnote.duration, csnote.amplitude ), \
            noteParams["page"], noteParams["track"], noteParams["note"], noteParams["csnote"] )        
        # assume that the note order will not have changed!

    # noteParams: { "page":pagelist, "track":tracklist, "note":noteIDlist }
    def deleteNotes( self, noteParams, noteCount ):
        modified = {}
        for i in range(noteCount):
            p = noteParams["page"][i]
            t = noteParams["track"][i]
            id = noteParams["note"][i]
            if not p in modified: modified[p] = True
            if p == self.curPage and self.note[p][t][self.noteMap[p][id]].getSelected():
                self.deselectNotes( { t: [ self.note[p][t][self.noteMap[p][id]] ] } )
            self.note[p][t][self.noteMap[p][id]].destroy()
            self.note[p][t][self.noteMap[p][id]] = None # flag for removal
            self.pageNoteCount[p] -= 1
            if self.pageNoteCount[p] == 0:
                del self.note[p]
                del self.pageNoteCount[p]
                del self.noteMap[p]
                del modified[p]
        
        #James->Adrian: is it ok that the previous loop called del modified[p] on any pages whose counts dropped to 0?
        for page in modified:
            for i in range(Constants.NUMBER_OF_TRACKS):
                j = len(self.note[page][i])-1
                while j >= 0:
                    if self.note[page][i][j] == None: del self.note[page][i][j]
                    j -= 1
            self.updateNoteMap( page )
        
    def displayPage( self, page, beatCount ):
        if page == self.curPage and self.beatCount == beatCount: return
        
        if self.curPage >= 0 and self.curPage != page: self.clearSelectedNotes()
        
        self.curPage = page
        
        if page not in self.note: # create a blank page if the page doesn't already exist
            self.note[page] = []
            for i in range(Constants.NUMBER_OF_TRACKS):
                self.note[page].append( [] )
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
            for i in range(Constants.NUMBER_OF_TRACKS):
                track = self.note[self.curPage][i]
                map( lambda note:note.updateTransform( True ), track )
        
        if self.drawingArea.window != None:
            self.invalidate_rect( 0, 0, self.fullWidth, self.height )
            
    def setPlayhead( self, ticks ):
        self.invalidate_rect( self.playheadX, 0, GUIConstants.PLAYHEAD_SIZE, self.height )
        self.playheadX = self.ticksToPixels( ticks ) + GUIConstants.BORDER_SIZE
        self.invalidate_rect( self.playheadX, 0, GUIConstants.PLAYHEAD_SIZE, self.height )

    def getSelectedTracks( self ):
        r = []
        for i in range( len(self.trackSelected) ):
            if self.trackSelected[i]: r.append( i )
        return r
        
    # private
    def updateNoteMap( self, page ):
        self.noteMap[page] = {}
        for i in range(Constants.NUMBER_OF_TRACKS):
            for j in range(len(self.note[page][i])):
                self.noteMap[page][self.note[page][i][j].getId()] = j

    #=======================================================
    #  Event Callbacks

    def set_size_request( self, width, height ):
        self.drawingArea.set_size_request( width, height )
        
        self.trackHeight = (height - (Constants.NUMBER_OF_TRACKS-1)*GUIConstants.TRACK_SPACING) / Constants.NUMBER_OF_TRACKS 
        self.height = self.trackHeight*Constants.NUMBER_OF_TRACKS + GUIConstants.TRACK_SPACING*(Constants.NUMBER_OF_TRACKS-1)
        self.trackLimits = []
        self.trackOrigin = []
        for i in range(Constants.NUMBER_OF_TRACKS):
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
            for i in range(Constants.NUMBER_OF_TRACKS):
                track = self.note[page][i]
                map( lambda note:note.updateTransform( False ), track )

        if self.drawingArea.window != None:
            self.invalidate_rect( 0, 0, width, height )

    def handleButtonPress( self, drawingArea, event ):

        TP.ProfileBegin( "TI::handleButtonPress" )

        if event.type == gtk.gdk._2BUTTON_PRESS:   self.buttonPressCount = 2
        elif event.type == gtk.gdk._3BUTTON_PRESS: self.buttonPressCount = 3
        else:                                      self.buttonPressCount = 1

        self.clickLoc = [ event.x, event.y ]
        
        # check if we clicked on the playhead
        if event.x >= self.playheadX and event.x <= self.playheadX + GUIConstants.PLAYHEAD_SIZE:
            self.setCurrentAction( "playhead-drag", self )
            TP.ProfileEnd( "TI::handleButtonPress" )
            return 

        for i in range(Constants.NUMBER_OF_TRACKS):
            if self.trackLimits[i][0] > event.y: break
            if self.trackLimits[i][1] < event.y: continue
            
            handled = 0
            notes = self.note[self.curPage][i]
            for n in range(len(notes)):
                handled = notes[n].handleButtonPress( self, event )
                if handled == 0:
                    continue
                elif handled == 1:
                    if not self.curAction: self.curAction = True # it was handled but no action was declared, set curAction to True anyway
                    TP.ProfileEnd( "TI::handleButtonPress" )
                    return
                else:      # all other options mean we can stop looking
                    break
            
            if self.interfaceMode == INTERFACEMODE.DRAW:
                if handled == -1:  # event occured before this note and didn't overlap with the previous note, so we can draw
                    print "draw a note"

        if event.button == 3:
            print "Should bring up some note parameters or something!"
            #self.noteParameters = NoteParametersWindow( self.trackDictionary, self.getNoteParameters )
            #self.setCurrentAction( "noteParameters", False )

        TP.ProfileEnd( "TI::handleButtonPress" )


    def handleButtonRelease( self, drawingArea, event ):
        TP.ProfileBegin( "TI::handleButtonRelease" )

        if not self.curAction: #do track selection stuff here so that we can also handle marquee selection
            for i in range(Constants.NUMBER_OF_TRACKS):
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
            self.updateTooltip( event )
        else:
            # we're doing the action ourselves
            if self.curAction == "marquee":         self.doneMarquee( event )
            elif self.curAction == "playhead-drag": self.donePlayhead( event )
            self.updateTooltip( event )


        TP.ProfileEnd( "TI::handleButtonRelease" )
        return

    def handleMotion( self, drawingArea, event ):
        TP.ProfileBegin( "TI::handleMotion::Common" )


        if event.is_hint:
            x, y, state = self.window.get_pointer()
            event.x = float(x)
            event.y = float(y)
            event.state = state

        TP.ProfileEnd( "TI::handleMotion::Common" )
            
        if event.state & gtk.gdk.BUTTON1_MASK:
            TP.ProfileBegin( "TI::handleMotion::Drag" )
            
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
        
            elif self.curAction == "playhead-drag":
                self.updatePlayhead( event )
            
            TP.ProfileEnd( "TI::handleMotion::Drag" )
        else:
            TP.ProfileBegin( "TI::handleMotion::Hover" )
            self.updateTooltip( event )
            TP.ProfileEnd( "TI::handleMotion::Hover" )
            
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
            for i in range(Constants.NUMBER_OF_TRACKS):
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
            map( lambda note:note.setSelected( True ), track )
            self.selectedNotes[trackN] = []
            map( lambda note:self.selectedNotes[trackN].append(note), track )
        elif mode == SELECTNOTES.NONE:
            if self.note.has_key(self.curPage):
                track = self.note[self.curPage][trackN]
                map( lambda note:note.setSelected( False ), track )
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
            notes = self.note[self.curPage][trackN]
            for n in range(len(notes)):
                if notes[n] in which: 
                    if notes[n].setSelected( True ):
                        self.selectedNotes[trackN].append( notes[n] )
                else: 
                    if notes[n].setSelected( False ):
                        self.selectedNotes[trackN].remove( notes[n] )

    def selectNotesByBar( self, trackN, start, stop ):
        for i in range(Constants.NUMBER_OF_TRACKS):
            if i == trackN: 
                notes = []
                track = self.note[self.curPage][trackN]
                for n in range(len(track)):
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
            for i in range(Constants.NUMBER_OF_TRACKS):
                if i == trackN: self.applyNoteSelection( SELECTNOTES.ALL, trackN, [] )
                else:           self.applyNoteSelection( SELECTNOTES.NONE, i, [] )
        self.selectionChanged()

    def selectNotes( self, noteDic ):
        if ModKeys.ctrlDown:
            for i in noteDic:
                self.applyNoteSelection( SELECTNOTES.FLIP, i, noteDic[i] )
        else:
            for i in range(Constants.NUMBER_OF_TRACKS):
                if i in noteDic: self.applyNoteSelection( SELECTNOTES.EXCLUSIVE, i, noteDic[i] )
                else:            self.applyNoteSelection( SELECTNOTES.NONE, i, [] )
        self.selectionChanged()

    def deselectNotes( self, noteDic ):
        for i in noteDic: 
            self.applyNoteSelection( SELECTNOTES.REMOVE, i, noteDic[i] )
        self.selectionChanged()

    def clearSelectedNotes( self ):
        for i in range(Constants.NUMBER_OF_TRACKS):
            self.applyNoteSelection( SELECTNOTES.NONE, i, [] )
        self.selectionChanged()

    def updateDragLimits( self ):
        self.dragLimits = [ [-9999,9999], [-9999,9999], [-9999,9999] ] # initialize to big numbers!
        maxRightBound = self.beatCount * Constants.TICKS_PER_BEAT
            
        for i in range(Constants.NUMBER_OF_TRACKS):
            if not len(self.selectedNotes[i]): continue  # no selected notes here

            track = self.note[self.curPage][i]
            leftBound = 0
            skip = True # skip the first note
            for n in range(len(track)):
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
        
        for i in range(Constants.NUMBER_OF_TRACKS):
            self.onNoteDrag( [ note.noteDrag(self, do, dp, dd) for note in self.selectedNotes[i] ] )

    def noteDragDuration( self, event ):
        do = 0
        dp = 0
        dd = self.pixelsToTicks( event.x - self.clickLoc[0] )
        dd = min( self.dragLimits[2][1], max( self.dragLimits[2][0], dd ) )

        for i in range(Constants.NUMBER_OF_TRACKS):
            self.onNoteDrag( [ note.noteDrag(self, do, dp, dd) for note in self.selectedNotes[i] ] )

    def noteDragPitch( self, event ):
        do = 0
        dp = self.pixelsToPitch( event.y - self.clickLoc[1] )
        dp = min( self.dragLimits[1][1], max( self.dragLimits[1][0], dp ) )
        dd = 0

        for i in range(Constants.NUMBER_OF_TRACKS):
            self.onNoteDrag( [ note.noteDrag(self, do, dp, dd) for note in self.selectedNotes[i] ] )

    def doneNoteDrag( self ):
        for i in range(Constants.NUMBER_OF_TRACKS):
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
        self.invalidate_rect( x-1, y-1, width+2, height+2 ) # increase by 1 to handle switching quadrants

    def doneMarquee( self, event ):                
        if self.marqueeLoc:
            stop =  [ self.marqueeRect[0][0] + self.marqueeRect[1][0], self.marqueeRect[0][1] + self.marqueeRect[1][1] ]

            select = {}
            
            for i in range(Constants.NUMBER_OF_TRACKS):
                intersectionY = [ max(self.marqueeRect[0][1],self.trackLimits[i][0]), min(stop[1],self.trackLimits[i][1]) ]
                if intersectionY[0] > intersectionY[1]:
                    continue
                
                notes = []
                track = self.note[self.curPage][i]
                for n in range(len(track)):
                    hit = track[n].handleMarqueeSelect( self, 
                                      [ self.marqueeRect[0][0], intersectionY[0] ], \
                                      [ stop[0], intersectionY[1] ] )           
                    if hit: notes.append(track[n])

                if len(notes): select[i] = notes
            
            self.selectNotes( select )

        self.marqueeLoc = False        
        self.doneCurrentAction()
        
        self.invalidate_rect( self.marqueeRect[0][0]-1, self.marqueeRect[0][1]-1, self.marqueeRect[1][0]+2, self.marqueeRect[1][1]+2 )
    
    def updatePlayhead( self, event ):
        x = min( self.width - GUIConstants.BORDER_SIZE_MUL2 - self.pixelsPerTick, max( GUIConstants.BORDER_SIZE, event.x ) )
        self.setPlayhead( self.pixelsToTicks( x ) )
        
        
    def donePlayhead( self, event ):
        x = min( self.width - GUIConstants.BORDER_SIZE_MUL2, max( GUIConstants.BORDER_SIZE, event.x ) )
        ticks = self.pixelsToTicks( x )
        print "set playhead to %d ticks" % (ticks)
        
    def updateTooltip( self, event ):
        
        # check clicked the playhead
        if event.x >= self.playheadX and event.x <= self.playheadX + GUIConstants.PLAYHEAD_SIZE:
            self.setCursor("drag-playhead")
            return 
        
        for i in range(Constants.NUMBER_OF_TRACKS):
            if self.trackLimits[i][0] > event.y: break
            if self.trackLimits[i][1] < event.y: continue
            
            notes = self.note[self.curPage][i]
            handled = 0
            for n in range(len(notes)):
                handled = notes[n].updateTooltip( self, event )
                if handled == 0:   continue
                elif handled == 1: return   # event was handled
                else:              break
                
            # note wasn't handled, could potentially draw a note
            if self.interfaceMode == INTERFACEMODE.DRAW:
                if handled == -2: # event X overlapped with a note
                    self.setCursor("default")
                    return
                
                self.setCursor("pencil")
                return
            
            break
            
        self.setCursor("default")
        
    def setCursor( self, cursor ):
        self.window.set_cursor(self.cursor[cursor])
            
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

        for i in range( Constants.NUMBER_OF_TRACKS):
            if startY > self.trackLimits[i][1]: continue
            if stopY < self.trackLimits[i][0]: break

            if False:
                context.set_line_width( GUIConstants.BORDER_SIZE )    

                context.move_to( GUIConstants.BORDER_SIZE_DIV2, self.trackLimits[i][0] + GUIConstants.BORDER_SIZE_DIV2 )
                context.rel_line_to( self.width - GUIConstants.BORDER_SIZE, 0 )
                context.rel_line_to( 0, self.trackHeight - GUIConstants.BORDER_SIZE )
                context.rel_line_to( -self.width + GUIConstants.BORDER_SIZE, 0 )
                context.close_path()

                #draw background
                context.set_source_rgb( 0.75, 0.75, 0.75 )
                context.fill_preserve()
        
            else:
                context.rectangle(GUIConstants.BORDER_SIZE_DIV2, self.trackLimits[i][0] + GUIConstants.BORDER_SIZE_DIV2 , self.width, self.height)
                context.set_source_rgb( 0.75, 0.75, 0.75 )
                context.fill()

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
            notes = self.note[self.curPage][i]
            for n in range(len(notes)):
                if not notes[n].draw( context, startX, stopX ): break
                
        # draw playhead
        context.set_line_width( GUIConstants.PLAYHEAD_SIZE )
        context.move_to( self.playheadX + GUIConstants.PLAYHEAD_SIZE_DIV2, 0 )
        # do some fancy shit here to grey out muted tracks!?
        context.rel_line_to( 0, self.height )
        context.set_source_rgb( 0, 0, 0 )
        context.stroke()
        
        if self.marqueeLoc:                 # draw the selection rect
            context.set_line_width( 1 )     
            context.move_to( self.marqueeRect[0][0] + 0.5, self.marqueeRect[0][1] + 0.5 )
            context.rel_line_to( self.marqueeRect[1][0] - 1, 0 )
            context.rel_line_to( 0, self.marqueeRect[1][1] - 1 )
            context.rel_line_to( -self.marqueeRect[1][0] + 1, 0 )
            context.close_path()
            context.set_source_rgb( 1, 1, 1 )
            context.stroke()    
            
        self.drawingAreaDirty = False

        TP.ProfileEnd( "TrackInterface::draw" )        
          
    def invalidate_rect( self, x, y, width, height ):
        self.dirtyRectToAdd.x = x
        self.dirtyRectToAdd.y = y
        self.dirtyRectToAdd.width = width
        self.dirtyRectToAdd.height = height
        self.drawingArea.window.invalidate_rect( self.dirtyRectToAdd, True )
        self.drawingAreaDirty = True
        #self.queue_draw()

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
