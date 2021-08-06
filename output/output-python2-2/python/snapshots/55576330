import pygtk
pygtk.require( '2.0' )
import gtk

from math import floor
import time

import Config
from Edit.NoteInterface import NoteInterface
from Edit.HitInterface import HitInterface
from Edit.MainWindow import CONTEXT

from Util.NoteDB import PARAMETER

from Util.Profiler import TP

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
    PASTE_NOTES = 2
    PASTE_TRACKS = 3

class TrackInterfaceParasite:
    def __init__( self, noteDB, owner, note ):
        if note.track == Config.NUMBER_OF_TRACKS-1: # drum track
            self.parasite = HitInterface( noteDB, owner, note )
        else:
            self.parasite = NoteInterface( noteDB, owner, note )

    def attach( self ):
        return self.parasite

class TrackInterface( gtk.EventBox ):

    def __init__( self, noteDB, owner ):
        gtk.EventBox.__init__( self )

        self.noteDB = noteDB
        self.owner = owner

        self.drawingArea = gtk.DrawingArea()
        self.drawingAreaDirty = False # are we waiting to draw?
        self.add( self.drawingArea )
        self.dirtyRectToAdd = gtk.gdk.Rectangle() # used by the invalidate_rect function

        self.fullWidth = 1 # store the maximum allowed width
        self.width = 1
        self.height = 1

        self.interfaceMode = INTERFACEMODE.DEFAULT

        self.curPage = -1   # this isn't a real page at all!
        self.curBeats = 4

        self.selectedNotes = [ [] for i in range(Config.NUMBER_OF_TRACKS) ]

        self.curAction = False          # stores the current mouse action
        self.curActionObject = False    # stores the object that in handling the action

        self.buttonPressCount = 1   # used on release events to indicate double/triple releases
        self.clickLoc = [0,0]       # location of the last click
        self.marqueeLoc = False     # current drag location of the marquee
        self.marqueeRect = [[0,0],[0,0]]

        self.pasteTick = -1
        self.pasteTrack = -1
        self.pasteRect = False

        self.playheadX = Config.TRACK_SPACING_DIV2

        self.cursor = { \
            "default":          None, \
            "drag-onset":       gtk.gdk.Cursor(gtk.gdk.SB_RIGHT_ARROW), \
            "drag-pitch":       gtk.gdk.Cursor(gtk.gdk.BOTTOM_SIDE), \
            "drag-duration":    gtk.gdk.Cursor(gtk.gdk.RIGHT_SIDE), \
            "drag-playhead":    gtk.gdk.Cursor(gtk.gdk.SB_H_DOUBLE_ARROW), \
            "pencil":           gtk.gdk.Cursor(gtk.gdk.PENCIL), \
            "paste":            gtk.gdk.Cursor(gtk.gdk.CENTER_PTR), \
            "error":            None }

        self.add_events(gtk.gdk.POINTER_MOTION_MASK|gtk.gdk.POINTER_MOTION_HINT_MASK)

        self.connect( "size-allocate", self.size_allocate )

        self.drawingArea.connect( "expose-event", self.expose )
        self.connect( "button-press-event", self.handleButtonPress )
        self.connect( "button-release-event", self.handleButtonRelease )
        self.connect( "motion-notify-event", self.handleMotion )

        # prepare drawing stuff
        hexToInt = { "0":0, "1":1, "2":2, "3":3, "4":4, "5":5, "6":6, "7":7, "8":8, "9":9, "A":10, "B":11, "C":12, "D":13, "E":14, "F":15, "a":10, "b":11, "c":12, "d":13, "e":14, "f":15 }
        self.trackColors = []
        for i in Config.TRACK_COLORS:
            low = ( 256*(hexToInt[i[0][1]]*16+hexToInt[i[0][2]]), 256*(hexToInt[i[0][3]]*16+hexToInt[i[0][4]]), 256*(hexToInt[i[0][5]]*16+hexToInt[i[0][6]]) )
            high = ( 256*(hexToInt[i[1][1]]*16+hexToInt[i[1][2]]), 256*(hexToInt[i[1][3]]*16+hexToInt[i[1][4]]), 256*(hexToInt[i[1][5]]*16+hexToInt[i[1][6]]) )
            delta = ( high[0]-low[0], high[1]-low[1], high[2]-low[2] )
            self.trackColors.append( (low, delta) )

        colormap = self.drawingArea.get_colormap()
        self.beatColor = colormap.alloc_color( Config.BEAT_COLOR, True, True )
        self.playheadColor = colormap.alloc_color( Config.PLAYHEAD_COLOR, True, True )
        self.marqueeColor = colormap.alloc_color( Config.MARQUEE_COLOR, True, True )

        self.image = {}
        img = gtk.Image()
        win = gtk.gdk.get_default_root_window()
        self.gc = gtk.gdk.GC( win )

        def prepareDrawable( name ):
            img.set_from_file( Config.IMAGE_ROOT+name+".png" )
            pix = img.get_pixbuf()
            self.image[name] = gtk.gdk.Pixmap( win, pix.get_width(), pix.get_height() )
            self.image[name].draw_pixbuf( self.gc, pix, 0, 0, 0, 0, pix.get_width(), pix.get_height(), gtk.gdk.RGB_DITHER_NONE )
        def preparePixbuf( name ):
            newimg = gtk.Image()
            newimg.set_from_file( Config.IMAGE_ROOT+name+".png" )
            self.image[name] = newimg.get_pixbuf()

        prepareDrawable( "trackBG" )
        prepareDrawable( "trackBGSelected" )
        prepareDrawable( "trackBGDrum" )
        prepareDrawable( "trackBGDrumSelected" )
        preparePixbuf( "note" )
        preparePixbuf( "noteSelected" )
        preparePixbuf( "hit" )
        preparePixbuf( "hitSelected" )

        # define dimensions
        self.width = self.trackFullWidth = self.image["trackBG"].get_size()[0]
        self.trackWidth = self.width - Config.TRACK_SPACING
        self.trackFullHeight = self.image["trackBG"].get_size()[1]
        self.trackHeight = self.trackFullHeight - Config.TRACK_SPACING
        self.trackFullHeightDrum = self.image["trackBGDrum"].get_size()[1]
        self.trackHeightDrum = self.trackFullHeightDrum - Config.TRACK_SPACING
        self.height = self.trackHeight*(Config.NUMBER_OF_TRACKS-1) + self.trackHeightDrum + Config.TRACK_SPACING*Config.NUMBER_OF_TRACKS
        self.trackLimits = []
        self.trackRect = []
        self.drumIndex = Config.NUMBER_OF_TRACKS-1
        for i in range(self.drumIndex):
            start = i*(self.trackFullHeight)
            self.trackLimits.append( (start,start+self.trackFullHeight) )
            self.trackRect.append( gtk.gdk.Rectangle(Config.TRACK_SPACING_DIV2,start+Config.TRACK_SPACING_DIV2, self.trackWidth, self.trackHeight ) )
        self.trackLimits.append( ( self.height - self.trackFullHeightDrum, self.height ) )
        self.trackRect.append( gtk.gdk.Rectangle( Config.TRACK_SPACING_DIV2, self.height - self.trackFullHeightDrum + Config.TRACK_SPACING_DIV2, self.trackWidth, self.trackHeightDrum ) )

        self.pitchPerPixel = float(Config.NUMBER_OF_POSSIBLE_PITCHES-1) / (self.trackHeight - Config.NOTE_HEIGHT)
        self.pixelsPerPitch = float(self.trackHeight-Config.NOTE_HEIGHT)/(Config.MAXIMUM_PITCH - Config.MINIMUM_PITCH)
        self.pitchPerPixelDrum = float(Config.NUMBER_OF_POSSIBLE_PITCHES_DRUM-1)*Config.PITCH_STEP_DRUM / (self.trackHeightDrum - Config.HIT_HEIGHT)
        self.pixelsPerPitchDrum = float(self.trackHeightDrum-Config.HIT_HEIGHT)/(Config.MAXIMUM_PITCH_DRUM - Config.MINIMUM_PITCH_DRUM )

        self.pixelsPerTick = [0] + [ self.trackWidth//(i*Config.TICKS_PER_BEAT) for i in range(1,Config.MAXIMUM_BEATS+1) ]

        self.ticksPerPixel = [0] + [ 1.0/self.pixelsPerTick[i] for i in range(1,Config.MAXIMUM_BEATS+1) ]

        self.beatSpacing = [0] + [ self.pixelsPerTick[i]*Config.TICKS_PER_BEAT for i in range(1,Config.MAXIMUM_BEATS+1) ]

        # screen buffers
        self.screenBuf = [ gtk.gdk.Pixmap( win, self.width, self.height ), \
                           gtk.gdk.Pixmap( win, self.width, self.height ) ]
        self.screenBufPage = [ -1, -1 ]
        self.screenBufBeats = [ -1, -1 ]
        self.screenBufDirtyRect =  [ gtk.gdk.Rectangle(), gtk.gdk.Rectangle() ]
        self.screenBufDirty = [ False, False ]
        self.screenBufResume = [ [0,0], [0,0] ] # allows for stopping and restarting in the middle of a draw
        self.curScreen = 0
        self.preScreen = 1

    #-- private --------------------------------------------

    def _updateClipboardArea( self ):
        self.clipboardArea = self.owner.getClipboardArea( self.curPage )
        self.clipboardTrackTop = 0
        for t in range(self.drumIndex):
            if self.clipboardArea["tracks"][t]: break
            self.clipboardTrackTop += 1
        self.clipboardDrumTrack = self.clipboardArea["tracks"][self.drumIndex]

    #=======================================================
    #  Module Interface

    def getDrawingPackage( self, track ):
        if track == self.drumIndex:
            return ( self.image["hit"], self.image["hitSelected"], self.drawingArea.get_colormap(), self.trackColors[track] )
        else:
            return ( self.image["note"], self.image["noteSelected"], self.drawingArea.get_colormap(), self.trackColors[track] )

    def getActivePages( self ):
        return self.screenBufPage

    def setPredrawPage( self, page ):
        if self.screenBufPage[self.preScreen] != page:
            self.screenBufPage[self.preScreen] = page
            self.screenBufBeats[self.preScreen] = self.noteDB.getPage(page).beats
            self.invalidate_rect( 0, 0, self.width, self.height, page )
            return True
        return False

    def predrawPage( self, timeout ):
        return self.draw( self.preScreen, False, timeout )

    def displayPage( self, page, predraw = -1 ):
        if page == self.curPage:
            if predraw >= 0 and self.screenBufPage[self.preScreen] != predraw:
                self.screenBufPage[self.preScreen] = predraw
                self.screenBufBeats[self.preScreen] = self.noteDB.getPage(predraw).beats
                self.invalidate_rect( 0, 0, self.width, self.height, predraw )
            return

        if self.curPage >= 0 and self.curPage != page: clearNotes = True
        else: clearNotes = False

        oldPage = self.curPage
        self.curPage = page
        self.curBeats = self.noteDB.getPage(page).beats

        if self.screenBufPage[self.preScreen] == self.curPage: # we predrew this page, so smart!
            t = self.preScreen
            self.preScreen = self.curScreen
            self.curScreen = t
            self.invalidate_rect( 0, 0, self.width, self.height, self.curPage, False )
        else: # we need to draw this page from scratch
            self.screenBufPage[self.curScreen] = self.curPage
            self.screenBufBeats[self.curScreen] = self.curBeats
            self.invalidate_rect( 0, 0, self.width, self.height, self.curPage )

        if predraw >= 0 and self.screenBufPage[self.preScreen] != predraw:
            self.screenBufPage[self.preScreen] = predraw
            self.screenBufBeats[self.preScreen] = self.noteDB.getPage(predraw).beats
            self.invalidate_rect( 0, 0, self.width, self.height, predraw )
        elif self.screenBufPage[self.preScreen] == -1: # make sure predraw is assigned to a valid page at least
            self.screenBufPage[self.preScreen] = self.screenBufPage[self.curScreen]

        if clearNotes: # clear the notes now that we've sorted out the screen buffers
            self.clearSelectedNotes( oldPage )

        if self.curAction == "paste":
            self._updateClipboardArea()

    def setPlayhead( self, ticks ):
        self.invalidate_rect( self.playheadX-Config.PLAYHEAD_SIZE/2, 0, Config.PLAYHEAD_SIZE, self.height, self.curPage, False )
        self.playheadX = self.ticksToPixels( self.curBeats, ticks ) + Config.TRACK_SPACING_DIV2
        self.invalidate_rect( self.playheadX-Config.PLAYHEAD_SIZE/2, 0, Config.PLAYHEAD_SIZE, self.height, self.curPage, False )

    def setInterfaceMode( self, mode ):
        self.doneCurrentAction()

        if mode == "tool":
            mode = self.owner.getTool()

        if mode == "draw":
            self.interfaceMode = INTERFACEMODE.DRAW
        elif mode == "paste_notes":
            self.interfaceMode = INTERFACEMODE.PASTE_NOTES
            self.setCurrentAction("paste", self)
        elif mode == "paste_tracks":
            self.interfaceMode = INTERFACEMODE.PASTE_TRACKS
            self.setCurrentAction("paste", self )
        else:
            self.interfaceMode = INTERFACEMODE.DEFAULT

    def getSelectedNotes( self ):
        ids = []
        for t in range(Config.NUMBER_OF_TRACKS):
            ids.append( [ n.note.id for n in self.selectedNotes[t] ] )
        return ids

    #=======================================================
    #  Event Callbacks

    def size_allocate( self, widget, allocation ):
        self.alloc = allocation
    	width = allocation.width
    	height = allocation.height

    	self.drawingArea.set_size_request( width, height )

        if self.window != None:
            self.invalidate_rect( 0, 0, width, height, self.curPage, False )

    def handleButtonPress( self, widget, event ):

        TP.ProfileBegin( "TI::handleButtonPress" )

        if event.type == gtk.gdk._2BUTTON_PRESS:   self.buttonPressCount = 2
        elif event.type == gtk.gdk._3BUTTON_PRESS: self.buttonPressCount = 3
        else:                                      self.buttonPressCount = 1

        self.clickLoc = [ int(event.x), int(event.y) ]

        if self.curAction == "paste":
            self.doPaste()
            self.setCurrentAction("block-track-select")
            TP.ProfileEnd( "TI::handleButtonPress" )
            return


        # check if we clicked on the playhead
        if event.x >= self.playheadX and event.x <= self.playheadX + Config.PLAYHEAD_SIZE:
            self.setCurrentAction( "playhead-drag", self )
            TP.ProfileEnd( "TI::handleButtonPress" )
            return

        for i in range(Config.NUMBER_OF_TRACKS):
            if self.trackLimits[i][0] > event.y: break
            if self.trackLimits[i][1] < event.y: continue

            handled = 0
            notes = self.noteDB.getNotesByTrack( self.curPage, i, self )
            last = len(notes)-1
            for n in range(last+1):
                if i == self.drumIndex and n < last: # check to see if the next hit overlaps this one
                    if notes[n].getStartTick() == notes[n+1].getStartTick() and notes[n].getPitch() == notes[n+1].getPitch():
                        continue
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


    def handleButtonRelease( self, widget, event ):
        TP.ProfileBegin( "TI::handleButtonRelease" )

        if not self.curAction: #do track selection stuff here so that we can also handle marquee selection
            for i in range(Config.NUMBER_OF_TRACKS):
                if self.trackLimits[i][0] > event.y: break
                if self.trackLimits[i][1] < event.y: continue
                if event.button == 1:
                    if self.buttonPressCount == 1:   self.owner.toggleTrack( i, False )
                    elif self.buttonPressCount == 2: self.owner.toggleTrack( i, True )
                    else:                            self.owner.clearTracks()
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

    def handleMotion( self, widget, event ):
        TP.ProfileBegin( "TI::handleMotion::Common" )

        if event.is_hint:
            x, y, state = self.window.get_pointer()
            event.x = float(x)
            event.y = float(y)
            event.state = state

        TP.ProfileEnd( "TI::handleMotion::Common" )

        if self.curAction == "paste":
            TP.ProfileBegin( "TI::handleMotion::Paste" )
            top = Config.NUMBER_OF_TRACKS
            for i in range(Config.NUMBER_OF_TRACKS):
                if self.trackLimits[i][0] > event.y: break
                if self.trackLimits[i][1] < event.y: continue
                top = i
                break
            self.updatePaste( self.pixelsToTicks( self.curBeats, event.x ), top )
            TP.ProfileEnd( "TI::handleMotion::Paste" )
        elif event.state & gtk.gdk.BUTTON1_MASK:
            TP.ProfileBegin( "TI::handleMotion::Drag" )

            if not self.curAction: # no action is in progress yet we're dragging, start a marquee
                self.setCurrentAction( "marquee", self )

            if self.curAction == "note-drag-onset":
                self.noteDragOnset( event )

            elif self.curAction == "note-drag-duration":
                self.noteDragDuration( event )

            elif self.curAction == "note-drag-pitch":
                self.noteDragPitch( event )

            elif self.curAction == "note-drag-pitch-drum":
                self.noteDragPitch( event, True )

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

    def setCurrentAction( self, action, obj = None ):
        if self.curAction:
            self.doneCurrentAction()

        self.curAction = action
        self.curActionObject = obj

        if   action == "note-drag-onset":      self.updateDragLimits()
        elif action == "note-drag-duration":   self.updateDragLimits()
        elif action == "note-drag-pitch":      self.updateDragLimits()
        elif action == "note-drag-pitch-drum": self.updateDragLimits()
        elif action == "paste":
            self._updateClipboardArea()
            self.setCursor("paste")

    def doneCurrentAction( self ):
        if not self.curAction: return
        action = self.curAction
        self.curAction = False

        if   action == "note-drag-onset":      self.doneNoteDrag()
        elif action == "note-drag-duration":   self.doneNoteDrag()
        elif action == "note-drag-pitch":      self.doneNoteDrag()
        elif action == "note-drag-pitch-drum": self.doneNoteDrag()
        elif action == "paste":
            self.owner.cleanupClipboard()

    def trackToggled( self, trackN = -1 ):
        if trackN == -1: self.invalidate_rect( 0, 0, self.width, self.height )
        else: self.invalidate_rect( 0, self.trackLimits[trackN][0], self.width, self.trackLimits[trackN][1]-self.trackLimits[trackN][0] )

    def selectionChanged( self ):
        if   self.curAction == "note-drag-onset":      self.updateDragLimits()
        elif self.curAction == "note-drag-duration":   self.updateDragLimits()
        elif self.curAction == "note-drag-pitch":      self.updateDragLimits()
        elif self.curAction == "note-drag-pitch-drum": self.updateDragLimits()
        for i in range(Config.NUMBER_OF_TRACKS):
            if len(self.selectedNotes[i]):
                self.owner.setContextState( CONTEXT.NOTE, True )
                self.owner.setContext( CONTEXT.NOTE )
                return
        self.owner.setContextState( CONTEXT.NOTE, False )

    def applyNoteSelection( self, mode, trackN, which, page = -1 ):
        if page == -1: page = self.curPage
        if mode == SELECTNOTES.ALL:
            track = self.noteDB.getNotesByTrack( page, trackN, self )
            map( lambda note:note.setSelected( True ), track )
            self.selectedNotes[trackN] = []
            map( lambda note:self.selectedNotes[trackN].append(note), track )
        elif mode == SELECTNOTES.NONE:
            track = self.noteDB.getNotesByTrack( page, trackN, self )
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
            notes = self.noteDB.getNotesByTrack( page, trackN, self )
            for n in range(len(notes)):
                if notes[n] in which:
                    if notes[n].setSelected( True ):
                        self.selectedNotes[trackN].append( notes[n] )
                else:
                    if notes[n].setSelected( False ):
                        self.selectedNotes[trackN].remove( notes[n] )

    def selectNotesByBar( self, trackN, start, stop, page = -1 ):
        for i in range(Config.NUMBER_OF_TRACKS):
            if i == trackN:
                notes = []
                track = self.noteDB.getNotesByTrack( self.curPage, trackN, self )
                for n in range(len(track)):
                    if track[n].testOnset( start, stop ): notes.append(track[n])
                if not Config.ModKeys.ctrlDown: self.applyNoteSelection( SELECTNOTES.EXCLUSIVE, trackN, notes, page )
                else:                           self.applyNoteSelection( SELECTNOTES.ADD, trackN, notes, page )
            else:
                if not Config.ModKeys.ctrlDown: self.applyNoteSelection( SELECTNOTES.NONE, i, [], page )
        self.selectionChanged()

    def selectNotesByTrack( self, trackN, page = -1 ):
        if Config.ModKeys.ctrlDown:
            self.applyNoteSelection( SELECTNOTES.ALL, trackN, [], page )
        else:
            for i in range(Config.NUMBER_OF_TRACKS):
                if i == trackN: self.applyNoteSelection( SELECTNOTES.ALL, trackN, [], page )
                else:           self.applyNoteSelection( SELECTNOTES.NONE, i, [], page )
        self.selectionChanged()

    def selectNotes( self, noteDic, ignoreCtrl = False, page = -1 ):
        if Config.ModKeys.ctrlDown and not ignoreCtrl:
            for i in noteDic:
                self.applyNoteSelection( SELECTNOTES.FLIP, i, noteDic[i], page )
        else:
            for i in range(Config.NUMBER_OF_TRACKS):
                if i in noteDic: self.applyNoteSelection( SELECTNOTES.EXCLUSIVE, i, noteDic[i], page )
                else:            self.applyNoteSelection( SELECTNOTES.NONE, i, [], page )
        self.selectionChanged()

    def deselectNotes( self, noteDic, page = -1 ):
        for i in noteDic:
            self.applyNoteSelection( SELECTNOTES.REMOVE, i, noteDic[i], page )
        self.selectionChanged()

    def clearSelectedNotes( self, page = -1 ):
        for i in range(Config.NUMBER_OF_TRACKS):
            self.applyNoteSelection( SELECTNOTES.NONE, i, [], page )
        self.selectionChanged()

    def updateDragLimits( self ):
        self.dragLimits = [ [-9999,9999], [-9999,9999], [-9999,9999] ] # initialize to big numbers!
        maxRightBound = self.noteDB.getPage(self.curPage).ticks

        for i in range(Config.NUMBER_OF_TRACKS):
            if not len(self.selectedNotes[i]): continue  # no selected notes here

            track = self.noteDB.getNotesByTrack( self.curPage, i, self )
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
                    thisNote.updateDragLimits( self.dragLimits, leftBound, rightBound, widthBound, maxRightBound )
                thisNote = nextNote
            # do the last note
            if thisNote.getSelected():
                thisNote.updateDragLimits( self.dragLimits, leftBound, maxRightBound, maxRightBound, maxRightBound )

    def noteDragOnset( self, event ):
        do = self.pixelsToTicks( self.curBeats, event.x - self.clickLoc[0] )
        do = min( self.dragLimits[0][1], max( self.dragLimits[0][0], do ) )

        stream = []
        for i in range(Config.NUMBER_OF_TRACKS):
            tstream = []
            for note in self.selectedNotes[i]:
                note.noteDragOnset( do, tstream )
            if len(tstream):
                stream += [ self.curPage, i, PARAMETER.ONSET, len(tstream)//2 ] + tstream
        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )

    def noteDragDuration( self, event ):
        dd = self.pixelsToTicks( self.curBeats, event.x - self.clickLoc[0] )
        dd = min( self.dragLimits[2][1], max( self.dragLimits[2][0], dd ) )

        stream = []
        for i in range(Config.NUMBER_OF_TRACKS):
            tstream = []
            for note in self.selectedNotes[i]:
                note.noteDragDuration( dd, tstream )
            if len(tstream):
                stream += [ self.curPage, i, PARAMETER.DURATION, len(tstream)//2 ] + tstream
        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )

    def noteDragPitch( self, event, drum = False ):
        if not drum: dp = self.pixelsToPitch( event.y - self.clickLoc[1] )
        else: dp = self.pixelsToPitchDrum( event.y - self.clickLoc[1] )
        dp = min( self.dragLimits[1][1], max( self.dragLimits[1][0], dp ) )

        stream = []
        for i in range(Config.NUMBER_OF_TRACKS):
            tstream = []
            for note in self.selectedNotes[i]:
                note.noteDragPitch( dp, tstream )
            if len(tstream):
                stream += [ self.curPage, i, PARAMETER.PITCH, len(tstream)//2 ] + tstream
        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )

    def doneNoteDrag( self ):
        for i in range(Config.NUMBER_OF_TRACKS):
            for note in self.selectedNotes[i]:
                note.doneNoteDrag( self )

    def noteStepOnset( self, step ):
        stream = []
        for i in range(Config.NUMBER_OF_TRACKS):
            if not len(self.selectedNotes[i]): continue  # no selected notes here

            tstream = []
            track = self.noteDB.getNotesByTrack( self.curPage, i, self )
            if step < 0: # moving to the left, iterate forwards
                leftBound = 0
                for n in range(len(track)):
                    leftBound = track[n].noteDecOnset( step, leftBound, tstream )
            else:        # moving to the right, iterate backwards
                rightBound = self.noteDB.getPage(self.curPage).ticks
                for n in range(len(track)-1, -1, -1 ):
                    rightBound = track[n].noteIncOnset( step, rightBound, tstream )

            if len(tstream):
                stream += [ self.curPage, i, PARAMETER.ONSET, len(tstream)//2 ] + tstream

        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )

    def noteStepPitch( self, step ):
        stream = []
        for i in range(Config.NUMBER_OF_TRACKS):
            if not len(self.selectedNotes[i]): continue  # no selected notes here

            tstream = []
            if step < 0:
                for n in self.selectedNotes[i]:
                    n.noteDecPitch( step, tstream )
            else:
                for n in self.selectedNotes[i]:
                    n.noteIncPitch( step, tstream )

            if len(tstream):
                stream += [ self.curPage, i, PARAMETER.PITCH, len(tstream)//2 ] + tstream

        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )

    def noteStepDuration( self, step ):
        stream = []
        for i in range(Config.NUMBER_OF_TRACKS):
            if not len(self.selectedNotes[i]): continue  # no selected notes here

            tstream = []
            if step < 0:
                for n in self.selectedNotes[i]:
                    n.noteDecDuration( step, tstream )
            else:
                track = self.noteDB.getNotesByTrack( self.curPage, i, self )
                for j in range(len(track)-1):
                    track[j].noteIncDuration( step, track[j+1].getStartTick(), tstream )
                track[len(track)-1].noteIncDuration( step, self.noteDB.getPage(self.curPage).ticks, tstream )

            if len(tstream):
                stream += [ self.curPage, i, PARAMETER.DURATION, len(tstream)//2 ] + tstream

        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )

    def noteStepVolume( self, step ):
        stream = []
        for i in range(Config.NUMBER_OF_TRACKS):
            if not len(self.selectedNotes[i]): continue  # no selected notes here

            tstream = []
            if step < 0:
                for n in self.selectedNotes[i]:
                    n.noteDecVolume( step, tstream )
            else:
                for n in self.selectedNotes[i]:
                    n.noteIncVolume( step, tstream )

            if len(tstream):
                stream += [ self.curPage, i, PARAMETER.AMPLITUDE, len(tstream)//2 ] + tstream

        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )


    def updateMarquee( self, event ):
        if self.marqueeLoc:
            oldX = self.marqueeRect[0][0]
            oldEndX = self.marqueeRect[0][0] + self.marqueeRect[1][0]
            oldY = self.marqueeRect[0][1]
            oldEndY = self.marqueeRect[0][1] + self.marqueeRect[1][1]
        else:
            oldX = oldEndX = self.clickLoc[0]
            oldY = oldEndY = self.clickLoc[1]

        self.marqueeLoc = [ int(event.x), int(event.y) ]
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
        self.invalidate_rect( x-1, y-1, width+2, height+2, self.curPage, False )

    def doneMarquee( self, event ):
        if self.marqueeLoc:
            stop =  [ self.marqueeRect[0][0] + self.marqueeRect[1][0], self.marqueeRect[0][1] + self.marqueeRect[1][1] ]

            select = {}

            for i in range(Config.NUMBER_OF_TRACKS):
                intersectionY = [ max(self.marqueeRect[0][1],self.trackLimits[i][0]), min(stop[1],self.trackLimits[i][1]) ]
                if intersectionY[0] > intersectionY[1]:
                    continue

                notes = []
                track = self.noteDB.getNotesByTrack( self.curPage, i, self )
                for n in range(len(track)):
                    hit = track[n].handleMarqueeSelect( self,
                                      [ self.marqueeRect[0][0], intersectionY[0] ], \
                                      [ stop[0], intersectionY[1] ] )
                    if hit: notes.append(track[n])

                if len(notes): select[i] = notes

            self.selectNotes( select )

        self.marqueeLoc = False
        self.doneCurrentAction()

        self.invalidate_rect( self.marqueeRect[0][0]-1, self.marqueeRect[0][1]-1, self.marqueeRect[1][0]+2, self.marqueeRect[1][1]+2, self.curPage, False )

    def updatePlayhead( self, event ):
        x = min( self.trackWidth - self.pixelsPerTick[self.curBeats], max( Config.TRACK_SPACING_DIV2, event.x ) )
        self.setPlayhead( self.pixelsToTicks( self.curBeats, x ) )

    def donePlayhead( self, event ):
        x = min( self.trackWidth - self.pixelsPerTick[self.curBeats], max( Config.TRACK_SPACING_DIV2, event.x ) )
        ticks = self.pixelsToTicks( self.curBeats, x )
        print "set playhead to %d ticks" % (ticks)
        self.doneCurrentAction()

    def updatePaste( self, tick, track ):
        if self.interfaceMode == INTERFACEMODE.PASTE_TRACKS: tick = 0
        if self.pasteTick == tick and self.pasteTrack == track: return
        if self.noteDB.getPage(self.curPage).ticks < tick < 0 \
           or track > self.drumIndex \
           or ( track == self.drumIndex and not self.clipboardDrumTrack ):
            if self.pasteRect:
                self.invalidate_rect( self.pasteRect[0][0], self.pasteRect[0][1], self.pasteRect[1][0], self.pasteRect[1][1], self.curPage, False )
                self.pasteTick = self.pasteTrack = -1
                self.pasteRect = False
            return
        if self.pasteRect:
            self.invalidate_rect( self.pasteRect[0][0], self.pasteRect[0][1], self.pasteRect[1][0], self.pasteRect[1][1], self.curPage, False )
        if self.clipboardDrumTrack:
            bottom = self.drumIndex
        else:
            bottom = self.drumIndex - 1
            for t in range(self.drumIndex-1,self.clipboardTrackTop-1,-1):
                if self.clipboardArea["tracks"][t]: break
                bottom -= 1
        end = -tick + min( self.noteDB.getPage(self.curPage).ticks, tick + self.clipboardArea["limit"][1]-self.clipboardArea["limit"][0] )
        self.pasteTick = tick
        self.pasteTrack = track
        self.pasteRect = [ [ self.ticksToPixels( self.curBeats, tick ), \
                             self.trackLimits[track][0] ], \
                           [ self.ticksToPixels( self.curBeats, end), \
                             self.trackLimits[bottom][1] ] ]
        self.invalidate_rect( self.pasteRect[0][0], self.pasteRect[0][1], self.pasteRect[1][0], self.pasteRect[1][1], self.curPage, False )

    def doPaste( self ):
        if self.pasteTrack == -1:
            self.doneCurrentAction()
            return

        trackMap = {}
        for t in range(self.pasteTrack,self.drumIndex):
            ind = t+self.clipboardTrackTop-self.pasteTrack
            if ind >= self.drumIndex: break
            if not self.clipboardArea["tracks"][ind]:
                continue
            trackMap[t] = ind
        if self.clipboardDrumTrack:
            trackMap[self.drumIndex] = self.drumIndex
        new = self.owner.pasteClipboard( self.pasteTick - self.clipboardArea["limit"][0], trackMap )
        if self.interfaceMode == INTERFACEMODE.PASTE_NOTES and self.curPage in new:
            noteDic = {}
            for t in range(Config.NUMBER_OF_TRACKS):
                if len(new[self.curPage][t]):
                    noteDic[t] = [ self.noteDB.getNote( self.curPage, t, n, self ) for n in new[self.curPage][t] ]
            self.selectNotes(noteDic)
        elif self.interfaceMode == INTERFACEMODE.PASTE_TRACKS:
            for t in range(self.drumIndex):
                ind = t + self.clipboardTrackTop - self.pasteTrack
                if ind >= self.drumIndex or ind < 0: self.owner.setTrack( t, False )
                else: self.owner.setTrack( t, self.clipboardArea["tracks"][ind] )
            self.owner.setTrack( self.drumIndex, self.clipboardDrumTrack )

        self.doneCurrentAction()

    def donePaste( self ):
        if self.pasteRect:
            self.invalidate_rect( self.pasteRect[0][0], self.pasteRect[0][1], self.pasteRect[1][0], self.pasteRect[1][1], self.curPage, False )
            self.pasteTick = self.pasteTrack = -1
            self.pasteRect = False
        self.setInterfaceMode("tool")
        # make a fake event for updateTooltip
        event = gtk.gdk.Event(gtk.gdk.MOTION_NOTIFY)
        x, y, state = self.window.get_pointer()
        event.x = float(x)
        event.y = float(y)
        event.state = state
        self.updateTooltip( event )

    def updateTooltip( self, event ):

        # check clicked the playhead
        if event.x >= self.playheadX and event.x <= self.playheadX + Config.PLAYHEAD_SIZE:
            self.setCursor("drag-playhead")
            return

        for i in range(Config.NUMBER_OF_TRACKS):
            if self.trackLimits[i][0] > event.y: break
            if self.trackLimits[i][1] < event.y: continue

            notes = self.noteDB.getNotesByTrack( self.curPage, i, self )
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

    def draw( self, buf, noescape = True, timeout = 0 ):
        if not self.screenBufDirty[buf]: return True

        TP.ProfileBegin( "TrackInterface::draw" )

        startX = self.screenBufDirtyRect[buf].x
        startY = self.screenBufDirtyRect[buf].y
        stopX = self.screenBufDirtyRect[buf].x + self.screenBufDirtyRect[buf].width
        stopY = self.screenBufDirtyRect[buf].y + self.screenBufDirtyRect[buf].height

        beatStart = Config.TRACK_SPACING_DIV2
        beats = self.screenBufBeats[buf]
        beatSpacing = self.beatSpacing[beats]

        pixmap = self.screenBuf[buf]

        resume = self.screenBufResume[buf]

        self.gc.set_clip_rectangle( self.screenBufDirtyRect[buf] )

        self.gc.set_line_attributes( Config.BEAT_LINE_SIZE, gtk.gdk.LINE_ON_OFF_DASH, gtk.gdk.CAP_BUTT, gtk.gdk.JOIN_MITER )
        # regular tracks
        for i in range( resume[0], self.drumIndex ):
            if resume[1] == 0:
                if startY > self.trackLimits[i][1]: continue
                if stopY < self.trackLimits[i][0]: break

                # draw background
                if self.owner.getTrackSelected( i ):
                    pixmap.draw_drawable( self.gc, self.image["trackBGSelected"], 0, 0, 0, self.trackLimits[i][0], self.trackFullWidth, self.trackFullHeight )
                else:
                    pixmap.draw_drawable( self.gc, self.image["trackBG"], 0, 0, 0, self.trackLimits[i][0], self.trackFullWidth, self.trackFullHeight )

                # draw beat lines
                self.gc.foreground = self.beatColor
                for j in range(1,self.screenBufBeats[buf]):
                    x = beatStart + j*beatSpacing
                    pixmap.draw_line( self.gc, x, self.trackRect[i].y, x, self.trackRect[i].y+self.trackRect[i].height )

                resume[1] = 1 # background drawn

            # draw notes
            TP.ProfileBegin("TI::draw notes")
            notes = self.noteDB.getNotesByTrack( self.screenBufPage[buf], i, self )
            for n in range( resume[2], len(notes) ):
                # check escape
                if not noescape and time.time() > timeout:
                    resume[0] = i
                    resume[2] = n
                    TP.ProfilePause( "TrackInterface::draw" )
                    return False

                if not notes[n].draw( pixmap, self.gc, startX, stopX ): break
            TP.ProfileEnd("TI::draw notes")

            # finished a track, reset the resume values for the next one
            resume[1] = 0
            resume[2] = 0

        # drum track
        if stopY > self.trackLimits[self.drumIndex][0]:

            if resume[1] == 0:
                # draw background
                if self.owner.getTrackSelected( self.drumIndex ):
                    pixmap.draw_drawable( self.gc, self.image["trackBGDrumSelected"], 0, 0, 0, self.trackLimits[self.drumIndex][0], self.trackFullWidth, self.trackFullHeightDrum )
                else:
                    pixmap.draw_drawable( self.gc, self.image["trackBGDrum"], 0, 0, 0, self.trackLimits[self.drumIndex][0], self.trackFullWidth, self.trackFullHeightDrum )

                # draw beat lines
                self.gc.foreground = self.beatColor
                for j in range(1,self.screenBufBeats[buf]):
                    x = beatStart + j*beatSpacing
                    pixmap.draw_line( self.gc, x, self.trackRect[self.drumIndex].y, x, self.trackRect[self.drumIndex].y+self.trackRect[self.drumIndex].height )

                resume[1] = 1 # background drawn

            # draw notes
            notes = self.noteDB.getNotesByTrack( self.screenBufPage[buf], self.drumIndex, self )
            for n in range( resume[2], len(notes) ):
                # check escape
                if not noescape and time.time() > timeout:
                    resume[0] = i
                    resume[2] = n
                    TP.ProfilePause( "TrackInterface::draw" )
                    return False
                if not notes[n].draw( pixmap, self.gc, startX, stopX ): break

        self.screenBufDirty[buf] = False

        TP.ProfileEnd( "TrackInterface::draw" )

        return True

    def expose( self, DA, event ):

        if self.screenBufDirty[self.curScreen]:
            self.draw( self.curScreen )

        TP.ProfileBegin( "TrackInterface::expose" )

        startX = event.area.x
        startY = event.area.y
        stopX = event.area.x + event.area.width
        stopY = event.area.y + event.area.height

        #print "%d %d %d %d" % (startX,startY,stopX,stopY)

        self.gc.set_clip_rectangle( event.area )

        # draw base
        DA.window.draw_drawable( self.gc, self.screenBuf[self.curScreen], startX, startY, startX, startY, event.area.width, event.area.height )

        # draw playhead
        self.gc.set_line_attributes( Config.PLAYHEAD_SIZE, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_BUTT, gtk.gdk.JOIN_MITER )
        self.gc.foreground = self.playheadColor
        DA.window.draw_line( self.gc, self.playheadX, startY, self.playheadX, stopY )

        if self.marqueeLoc:                 # draw the selection rect
            self.gc.set_line_attributes( Config.MARQUEE_SIZE, gtk.gdk.LINE_ON_OFF_DASH, gtk.gdk.CAP_BUTT, gtk.gdk.JOIN_MITER )
            self.gc.foreground = self.marqueeColor
            DA.window.draw_rectangle( self.gc, False, self.marqueeRect[0][0], self.marqueeRect[0][1], self.marqueeRect[1][0], self.marqueeRect[1][1] )

        if self.pasteRect:                  # draw the paste highlight
            self.gc.set_function( gtk.gdk.INVERT )
            for t in range(self.pasteTrack,self.drumIndex):
                ind = t+self.clipboardTrackTop-self.pasteTrack
                if ind >= self.drumIndex: break
                if not self.clipboardArea["tracks"][ind]:
                    continue
                DA.window.draw_rectangle( self.gc, True, self.pasteRect[0][0], self.trackLimits[t][0] + Config.TRACK_SPACING_DIV2, self.pasteRect[1][0], self.trackHeight )
            if self.clipboardDrumTrack:
                DA.window.draw_rectangle( self.gc, True, self.pasteRect[0][0], self.trackLimits[self.drumIndex][0] + Config.TRACK_SPACING_DIV2, self.pasteRect[1][0], self.trackHeightDrum )
            self.gc.set_function( gtk.gdk.COPY )

        self.drawingAreaDirty = False

        TP.ProfileEnd( "TrackInterface::expose" )

    def invalidate_rect( self, x, y, width, height, page = -1, base = True ):
        #print "%d %d %d %d Page %d CurPage %d" % (x,y,width,height,page,self.curPage)
        self.dirtyRectToAdd.x = x
        self.dirtyRectToAdd.y = y
        self.dirtyRectToAdd.width = width
        self.dirtyRectToAdd.height = height

        #print "dirty %d %d %d %d %d %d" % (x, y, width, height, x+width, y+height)
        if page == self.curPage or page == -1:
            if base: # the base image has been dirtied
                if not self.screenBufDirty[self.curScreen]:
                    self.screenBufDirtyRect[self.curScreen].x = x
                    self.screenBufDirtyRect[self.curScreen].y = y
                    self.screenBufDirtyRect[self.curScreen].width = width
                    self.screenBufDirtyRect[self.curScreen].height = height
                else:
                    self.screenBufDirtyRect[self.curScreen] = self.screenBufDirtyRect[self.curScreen].union( self.dirtyRectToAdd )
                self.screenBufResume[self.curScreen] = [0,0,0]
                self.screenBufDirty[self.curScreen] = True
            if self.drawingArea.window != None:
                self.drawingArea.window.invalidate_rect( self.dirtyRectToAdd, True )
            self.drawingAreaDirty = True

        if page == self.screenBufPage[self.preScreen] or page == -1:
            if not self.screenBufDirty[self.preScreen]:
                self.screenBufDirtyRect[self.preScreen].x = x
                self.screenBufDirtyRect[self.preScreen].y = y
                self.screenBufDirtyRect[self.preScreen].width = width
                self.screenBufDirtyRect[self.preScreen].height = height
            else:
                self.screenBufDirtyRect[self.preScreen] = self.screenBufDirtyRect[self.preScreen].union( self.dirtyRectToAdd )
            self.screenBufResume[self.preScreen] = [0,0,0]
            self.screenBufDirty[self.preScreen] = True

        #self.queue_draw()

    def getTrackOrigin( self, track ):
        return ( self.trackRect[track].x, self.trackRect[track].y )

    def ticksToPixels( self, beats, ticks ):
        return int(round( ticks * self.pixelsPerTick[beats] ))
    def pixelsToTicks( self, beats, pixels ):
        return int(round( pixels * self.ticksPerPixel[beats] ))
    def pitchToPixels( self, pitch ):
        return int(round(  ( Config.MAXIMUM_PITCH - pitch ) * self.pixelsPerPitch ))
    def pixelsToPitch( self, pixels ):
        return int(round(-pixels*self.pitchPerPixel))
    def pitchToPixelsDrum( self, pitch ):
        return int(round(  ( Config.MAXIMUM_PITCH_DRUM - pitch ) * self.pixelsPerPitchDrum ))
    def pixelsToPitchDrum( self, pixels ):
        return int(round(-pixels*self.pitchPerPixelDrum))
