import pygtk
pygtk.require( '2.0' )
import gtk

from math import floor

import Config
from Edit.NoteInterface import NoteInterface
from Edit.HitInterface import HitInterface
#from GUI.Core.NoteParametersWindow import NoteParametersWindow

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
    PASTE = 2

class TrackInterface( gtk.EventBox ):
    
    def __init__( self, onNoteDrag ):
        gtk.EventBox.__init__( self )

        self.drawingArea = gtk.DrawingArea()
        self.drawingAreaDirty = False # are we waiting to draw?
        self.add( self.drawingArea )
        self.dirtyRectToAdd = gtk.gdk.Rectangle() # used by the invalidate_rect function
        
        self.fullWidth = 1 # store the maximum allowed width
        self.width = 1
        self.height = 1

        self.interfaceMode = INTERFACEMODE.DEFAULT
        
        self.note = {}          # list of pages, tracks, and notes: self.note[pageId][trackId][noteId]
        self.pageBeatCount = {} # keep track of the beat count for each page
        self.pageNoteCount = {} # keep track of how many notes are on a page (so we can get rid of them when they're empty)
        self.noteMap = {}       # maps note ids to self.note[p][t][i]s

        self.curPage = -1   # this isn't a real page at all!
        self.beatCount = 4

        self.trackSelected = []
        self.selectedNotes = []
        for i in range(0,Config.NUMBER_OF_TRACKS):
            self.trackSelected.insert( 0, False )
            self.selectedNotes.insert( 0, [] )

        self.curAction = False          # stores the current mouse action
        self.curActionObject = False    # stores the object that in handling the action

        self.buttonPressCount = 1   # used on release events to indicate double/triple releases
        self.clickLoc = [0,0]       # location of the last click
        self.marqueeLoc = False     # current drag location of the marquee
        self.marqueeRect = [[0,0],[0,0]]
        
        self.playheadX = Config.TRACK_SPACING_DIV2

        self.cursor = { \
            "default":          None, \
            "drag-onset":       gtk.gdk.Cursor(gtk.gdk.SB_RIGHT_ARROW), \
            "drag-pitch":       gtk.gdk.Cursor(gtk.gdk.SB_V_DOUBLE_ARROW), \
            "drag-duration":    gtk.gdk.Cursor(gtk.gdk.SB_H_DOUBLE_ARROW), \
            "drag-playhead":    gtk.gdk.Cursor(gtk.gdk.LEFT_SIDE), \
            "pencil":           gtk.gdk.Cursor(gtk.gdk.PENCIL), \
            "error":            None }

        self.add_events(gtk.gdk.POINTER_MOTION_MASK|gtk.gdk.POINTER_MOTION_HINT_MASK)

        self.connect( "size-allocate", self.size_allocate )
        
        self.drawingArea.connect( "expose-event", self.expose )
        self.connect( "button-press-event", self.handleButtonPress )
        self.connect( "button-release-event", self.handleButtonRelease )
        self.connect( "motion-notify-event", self.handleMotion )

        self.onNoteDrag = onNoteDrag
        
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

        # screen buffers
        self.screenBuf = [ gtk.gdk.Pixmap( win, self.width, self.height ), \
                           gtk.gdk.Pixmap( win, self.width, self.height ) ]
        self.screenBufPage = [ -1, -1 ]
        self.screenBufDirtyRect =  [ gtk.gdk.Rectangle(), gtk.gdk.Rectangle() ]
        self.screenBufDirty = [ False, False ]
        self.screenBufResume = [ [0,0], [0,0] ] # allows for stopping and restarting in the middle of a draw
        self.curScreen = 0
        self.preScreen = 1


    #=======================================================
    #  Module Interface

    # noteParams: { "page":pagelist, "track":tracklist, "note":noteIDlist }
    def addNotes( self, noteParams, noteCount ):
        at = {}
        
        for i in range(noteCount):
            p = noteParams["page"][i]
            t = noteParams["track"][i]
            if p not in at:
                at[p] = [0] * Config.NUMBER_OF_TRACKS
                #at[p] = []
                #for j in range(Config.NUMBER_OF_TRACKS): at[p].append(0)
            if p not in self.note: 
                self.note[p] = map(lambda x:[], range(Config.NUMBER_OF_TRACKS))
                #self.note[p] = []
                #for j in range(Config.NUMBER_OF_TRACKS):
                    #self.note[p].append( [] )
                self.pageBeatCount[p] = noteParams["beatCount"][i]
                self.pageNoteCount[p] = 0
            csnote = noteParams["csnote"][i]
            if noteParams["track"][i] == self.drumIndex:
                note = HitInterface( self, p, noteParams["track"][i], noteParams["note"][i], \
                                     csnote["pitch"], csnote["onset"], csnote["duration"], csnote["amplitude"], \
                                     self.image["hit"], self.image["hitSelected"], self.trackColors[noteParams["track"][i]] )
            else:
                note = NoteInterface( self, p, noteParams["track"][i], noteParams["note"][i], \
                                      csnote["pitch"], csnote["onset"], csnote["duration"], csnote["amplitude"], \
                                      self.image["note"], self.image["noteSelected"], self.trackColors[noteParams["track"][i]] )
            while at[p][t] > 0:
                startT = self.note[p][t][at[p][t]-1].getStartTick()
                if startT <= csnote["onset"]: 
                    if startT < csnote["onset"]: break
                    elif self.note[p][t][at[p][t]-1].getPitch <= csnote["pitch"]: break
                at[p][t] -= 1
            last = len(self.note[p][t])
            while at[p][t] < last:
                startT = self.note[p][t][at[p][t]].getStartTick()
                if startT >= csnote["onset"]: 
                    if startT > csnote["onset"]: break
                    elif self.note[p][t][at[p][t]].getPitch >= csnote["pitch"]: break
                at[p][t] += 1
            self.note[p][t].insert( at[p][t], note )
            self.pageNoteCount[p] += 1
            at[p][t] += 1 # assume the next note will fall after this one
            
        for page in at:
            self.updateNoteMap( page )

    # noteParams: { "page":pagelist, "track":tracklist, "note":noteIDlist }
    def updateNotes( self, noteParams, noteCount ):
        for i in range(noteCount):
            p = noteParams["page"][i]
            t = noteParams["track"][i]
            id = noteParams["note"]
            csnote = noteParams["csnote"]
            self.resortNote( p, t, id, csnote["pitch"], csnote["onset"] )
            self.note[p][t][self.noteMap[p][id]].updateParams( csnote["pitch"], csnote["onset"], csnote["duration"], csnote["amplitude"] )

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
        
        for page in modified:
            for i in range(Config.NUMBER_OF_TRACKS):
                j = len(self.note[page][i])-1
                while j >= 0:
                    if self.note[page][i][j] == None: del self.note[page][i][j]
                    j -= 1
            self.updateNoteMap( page )

    def predrawPage( self, page ):
        if self.screenBufPage[self.preScreen] != page:
            self.screenBufPage[self.preScreen] = page
            self.invalidate_rect( 0, 0, self.width, self.height, page )
        
    def displayPage( self, page, beatCount, predraw = -1 ):
        if page == self.curPage and self.beatCount == beatCount: return
        
        if self.curPage >= 0 and self.curPage != page: clearNotes = True
        else: clearNotes = False
        
        self.curPage = page
        
        if self.screenBufPage[self.preScreen] == self.curPage: # we predrew this page, so smart!
            t = self.preScreen
            self.preScreen = self.curScreen
            self.curScreen = t
            self.invalidate_rect( 0, 0, self.width, self.height, self.curPage, False )
        else: # we need to draw this page from scratch
            self.screenBufPage[self.curScreen] = self.curPage
            self.invalidate_rect( 0, 0, self.width, self.height, self.curPage )
        
        if predraw >= 0 and self.screenBufPage[self.preScreen] != predraw:
            self.screenBufPage[self.preScreen] = predraw
            self.invalidate_rect( 0, 0, self.width, self.height, predraw )
        
        if clearNotes: # clear the notes now that we've sorted out the screen buffers
            self.clearSelectedNotes()            
        
        if page not in self.note: # create a blank page if the page doesn't already exist
            self.note[page] = []
            for i in range(Config.NUMBER_OF_TRACKS):
                self.note[page].append( [] )
            self.pageBeatCount[page] = beatCount
            self.pageNoteCount[page] = 0
        
        self.updateBeatCount( beatCount )

    def updateBeatCount( self, beatCount ):
        self.beatCount = beatCount
        
        self.pixelsPerTick = self.trackWidth//(self.beatCount*Config.TICKS_PER_BEAT)
        self.ticksPerPixel = 1.0/self.pixelsPerTick
        self.beatSpacing = self.pixelsPerTick*Config.TICKS_PER_BEAT

        if self.pageBeatCount[self.curPage] != beatCount:
            self.pageBeatCount[self.curPage] = beatCount
            for i in range(Config.NUMBER_OF_TRACKS):
                track = self.note[self.curPage][i]
                map( lambda note:note.updateTransform( True ), track )
        
        if self.window != None:
            self.invalidate_rect( 0, 0, self.fullWidth, self.height, self.curPage )
            
    def setPlayhead( self, ticks ):
        self.invalidate_rect( self.playheadX-Config.PLAYHEAD_SIZE/2, 0, Config.PLAYHEAD_SIZE, self.height, self.curPage, False )
        self.playheadX = self.ticksToPixels( ticks ) + Config.TRACK_SPACING_DIV2
        self.invalidate_rect( self.playheadX-Config.PLAYHEAD_SIZE/2, 0, Config.PLAYHEAD_SIZE, self.height, self.curPage, False )

    def getSelectedTracks( self ):
        r = []
        for i in range( len(self.trackSelected) ):
            if self.trackSelected[i]: r.append( i )
        return r
        
    def setInterfaceMode( self, mode ):
        if mode == "Draw":
            self.interfaceMode = INTERFACEMODE.DRAW
        elif mode == "Paste":
            self.interfaceMode = INTERFACEMODE.PASTE
        else:
            self.interfaceMode = INTERFACEMODE.DEFAULT
        
    # private
    def updateNoteMap( self, page ):
        self.noteMap[page] = {}
        for i in range(Config.NUMBER_OF_TRACKS):
            for j in range(len(self.note[page][i])):
                self.noteMap[page][self.note[page][i][j].getId()] = j
                
    def resortNote( self, p, t, id, pitch, onset ):
        ins = out = self.noteMap[p][id]            
        while ins > 0: # check backward
            startT = self.note[p][t][ins-1].getStartTick()
            if startT >= onset: 
                if startT > onset or self.note[p][t][ins-1].getPitch() < pitch: ins -= 1
                else: break
            else: break
        if ins == out: # check forward
            while ins < len(self.note[p][t])-1:
                startT = self.note[p][t][ins+1].getStartTick()
                if startT <= onset: 
                    if startT < onset or self.note[p][t][ins+1].getPitch() >= pitch: ins += 1
                    else: break
                else: break
        if ins != out: # resort
            if ins > out: 
                for j in range( out+1, ins+1 ):
                    self.noteMap[p][self.note[p][t][j].getId()] -= 1
            else: 
                for j in range( ins, out ):
                    self.noteMap[p][self.note[p][t][j].getId()] += 1
            self.noteMap[p][id] = ins
            n = self.note[p][t].pop( out )
            self.note[p][t].insert( ins, n )

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
        
        # check if we clicked on the playhead
        if event.x >= self.playheadX and event.x <= self.playheadX + Config.PLAYHEAD_SIZE:
            self.setCurrentAction( "playhead-drag", self )
            TP.ProfileEnd( "TI::handleButtonPress" )
            return 

        for i in range(Config.NUMBER_OF_TRACKS):
            if self.trackLimits[i][0] > event.y: break
            if self.trackLimits[i][1] < event.y: continue
            
            handled = 0
            notes = self.note[self.curPage][i]
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

    def handleMotion( self, widget, event ):
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

    def setCurrentAction( self, action, obj ):
        if self.curAction:
            print "BackgroundView - Action already in progress!"

        self.curAction = action
        self.curActionObject = obj

        if   action == "note-drag-onset":      self.updateDragLimits()
        elif action == "note-drag-duration":   self.updateDragLimits()
        elif action == "note-drag-pitch":      self.updateDragLimits()
        elif action == "note-drag-pitch-drum": self.updateDragLimits()

    def doneCurrentAction( self ):
        if   self.curAction == "note-drag-onset":      self.doneNoteDrag()
        elif self.curAction == "note-drag-duration":   self.doneNoteDrag()
        elif self.curAction == "note-drag-pitch":      self.doneNoteDrag()
        elif self.curAction == "note-drag-pitch-drum": self.doneNoteDrag()

        self.curAction = False
        self.curActionObject = False

    def toggleTrack( self, trackN, exclusive ):
        if exclusive:
            for i in range(Config.NUMBER_OF_TRACKS):
                self.trackSelected[i] = False
            self.trackSelected[trackN] = True
            self.invalidate_rect( 0, 0, self.width, self.height, self.curPage )
        else:
            self.trackSelected[trackN] = not self.trackSelected[trackN]
            self.invalidate_rect( 0, self.trackLimits[trackN][0], self.width, self.trackLimits[trackN][1]-self.trackLimits[trackN][0], self.curPage )

    def selectionChanged( self ):
        if   self.curAction == "note-drag-onset":      self.updateDragLimits()
        elif self.curAction == "note-drag-duration":   self.updateDragLimits()
        elif self.curAction == "note-drag-pitch":      self.updateDragLimits()
        elif self.curAction == "note-drag-pitch-drum": self.updateDragLimits()

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
        for i in range(Config.NUMBER_OF_TRACKS):
            if i == trackN: 
                notes = []
                track = self.note[self.curPage][trackN]
                for n in range(len(track)):
                    if track[n].testOnset( start, stop ): notes.append(track[n])
                if not Config.ModKeys.ctrlDown: self.applyNoteSelection( SELECTNOTES.EXCLUSIVE, trackN, notes )
                else:                    self.applyNoteSelection( SELECTNOTES.ADD, trackN, notes )
            else:
                if not Config.ModKeys.ctrlDown: self.applyNoteSelection( SELECTNOTES.NONE, i, [] )
        self.selectionChanged()
        
    def selectNotesByTrack( self, trackN ):
        if Config.ModKeys.ctrlDown:
            self.applyNoteSelection( SELECTNOTES.ALL, trackN, [] )
        else:
            for i in range(Config.NUMBER_OF_TRACKS):
                if i == trackN: self.applyNoteSelection( SELECTNOTES.ALL, trackN, [] )
                else:           self.applyNoteSelection( SELECTNOTES.NONE, i, [] )
        self.selectionChanged()

    def selectNotes( self, noteDic ):
        if Config.ModKeys.ctrlDown:
            for i in noteDic:
                self.applyNoteSelection( SELECTNOTES.FLIP, i, noteDic[i] )
        else:
            for i in range(Config.NUMBER_OF_TRACKS):
                if i in noteDic: self.applyNoteSelection( SELECTNOTES.EXCLUSIVE, i, noteDic[i] )
                else:            self.applyNoteSelection( SELECTNOTES.NONE, i, [] )
        self.selectionChanged()

    def deselectNotes( self, noteDic ):
        for i in noteDic: 
            self.applyNoteSelection( SELECTNOTES.REMOVE, i, noteDic[i] )
        self.selectionChanged()

    def clearSelectedNotes( self ):
        for i in range(Config.NUMBER_OF_TRACKS):
            self.applyNoteSelection( SELECTNOTES.NONE, i, [] )
        self.selectionChanged()

    def updateDragLimits( self ):
        self.dragLimits = [ [-9999,9999], [-9999,9999], [-9999,9999] ] # initialize to big numbers!
        maxRightBound = self.beatCount * Config.TICKS_PER_BEAT
            
        for i in range(Config.NUMBER_OF_TRACKS):
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
                    thisNote.updateDragLimits( self.dragLimits, leftBound, rightBound, widthBound, maxRightBound )
                thisNote = nextNote
            # do the last note
            if thisNote.getSelected(): 
                thisNote.updateDragLimits( self.dragLimits, leftBound, maxRightBound, maxRightBound, maxRightBound )

    def noteDragOnset( self, event ):
        do = self.pixelsToTicks( event.x - self.clickLoc[0] )
        do = min( self.dragLimits[0][1], max( self.dragLimits[0][0], do ) )
        dp = 0
        dd = 0
        
        for i in range(Config.NUMBER_OF_TRACKS):
            changed = []
            for note in self.selectedNotes[i]:
                ret = note.noteDrag(self, do, dp, dd)
                if ret:
                    if i == self.drumIndex: self.resortNote( self.curPage, i, ret[0], ret[1], ret[2] )
                    changed += [ret]
            if len(changed): self.onNoteDrag( changed )

    def noteDragDuration( self, event ):
        do = 0
        dp = 0
        dd = self.pixelsToTicks( event.x - self.clickLoc[0] )
        dd = min( self.dragLimits[2][1], max( self.dragLimits[2][0], dd ) )

        for i in range(Config.NUMBER_OF_TRACKS):
            changed = []
            for note in self.selectedNotes[i]:
                ret = note.noteDrag(self, do, dp, dd)
                if ret: changed += [ret]
            self.onNoteDrag( changed )
            
    def noteDragPitch( self, event, drum = False ):
        do = 0
        if not drum: dp = self.pixelsToPitch( event.y - self.clickLoc[1] )
        else: dp = self.pixelsToPitchDrum( event.y - self.clickLoc[1] )
        dp = min( self.dragLimits[1][1], max( self.dragLimits[1][0], dp ) )
        dd = 0

        for i in range(Config.NUMBER_OF_TRACKS):
            changed = []
            for note in self.selectedNotes[i]:
                ret = note.noteDrag(self, do, dp, dd)
                if ret: 
                    if i == self.drumIndex: self.resortNote( self.curPage, i, ret[0], ret[1], ret[2] )
                    changed += [ret]
            self.onNoteDrag( changed )

    def doneNoteDrag( self ):
        for i in range(Config.NUMBER_OF_TRACKS):
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
        
        self.invalidate_rect( self.marqueeRect[0][0]-1, self.marqueeRect[0][1]-1, self.marqueeRect[1][0]+2, self.marqueeRect[1][1]+2, self.curPage, False )
    
    def updatePlayhead( self, event ):
        x = min( self.trackWidth - self.pixelsPerTick, max( Config.TRACK_SPACING_DIV2, event.x ) )
        self.setPlayhead( self.pixelsToTicks( x ) )
        
        
    def donePlayhead( self, event ):
        x = min( self.trackWidth - self.pixelsPerTick, max( Config.TRACK_SPACING_DIV2, event.x ) )
        ticks = self.pixelsToTicks( x )
        print "set playhead to %d ticks" % (ticks)     
        self.doneCurrentAction()
        
    def updateTooltip( self, event ):
        
        # check clicked the playhead
        if event.x >= self.playheadX and event.x <= self.playheadX + Config.PLAYHEAD_SIZE:
            self.setCursor("drag-playhead")
            return 
        
        for i in range(Config.NUMBER_OF_TRACKS):
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
    
    def predraw( self, buf, noescape = True ):
        TP.ProfileBegin( "TrackInterface::predraw" )

        startX = self.screenBufDirtyRect[buf].x
        startY = self.screenBufDirtyRect[buf].y
        stopX = self.screenBufDirtyRect[buf].x + self.screenBufDirtyRect[buf].width
        stopY = self.screenBufDirtyRect[buf].y + self.screenBufDirtyRect[buf].height

        pixmap = self.screenBuf[buf]

        resume = self.screenBufResume[buf]

        self.gc.set_clip_rectangle( self.screenBufDirtyRect[buf] )
        
        self.gc.set_line_attributes( Config.BEAT_LINE_SIZE, gtk.gdk.LINE_ON_OFF_DASH, gtk.gdk.CAP_BUTT, gtk.gdk.JOIN_MITER )
        # regular tracks
        for i in range( resume[0], self.drumIndex ):
            if resume[0] == 0:
                if startY > self.trackLimits[i][1]: continue
                if stopY < self.trackLimits[i][0]: break

                # draw background
                if self.trackSelected[i]:
                    pixmap.draw_drawable( self.gc, self.image["trackBGSelected"], 0, 0, 0, self.trackLimits[i][0], self.trackFullWidth, self.trackFullHeight )
                else:
                    pixmap.draw_drawable( self.gc, self.image["trackBG"], 0, 0, 0, self.trackLimits[i][0], self.trackFullWidth, self.trackFullHeight )
            
                # draw beat lines
                self.gc.foreground = self.beatColor
                beatStart = Config.TRACK_SPACING_DIV2
                for j in range(1,self.beatCount):
                    x = beatStart + j*self.beatSpacing
                    pixmap.draw_line( self.gc, x, self.trackRect[i].y, x, self.trackRect[i].y+self.trackRect[i].height )
            
            # draw notes
            notes = self.note[self.curPage][i]
            for n in range( resume[1], len(notes) ):
                # check escape
                if 0:
                    resume[0] = i
                    resume[1] = n
                    TP.ProfilePause( "TrackInterface::predraw" )
                    return False
                    
                if not notes[n].draw( pixmap, self.gc, startX, stopX ): break
            
            # finished a track, reset the resume values for the next one
            resume[0] = 0
            resume[1] = 0                
 
        # drum track
        if stopY > self.trackLimits[self.drumIndex][0]:
        
            if resume[0] == 0:        
                # draw background
                if self.trackSelected[self.drumIndex]:
                    pixmap.draw_drawable( self.gc, self.image["trackBGDrumSelected"], 0, 0, 0, self.trackLimits[self.drumIndex][0], self.trackFullWidth, self.trackFullHeightDrum )
                else:
                    pixmap.draw_drawable( self.gc, self.image["trackBGDrum"], 0, 0, 0, self.trackLimits[self.drumIndex][0], self.trackFullWidth, self.trackFullHeightDrum )
            
                # draw beat lines
                self.gc.foreground = self.beatColor
                beatStart = Config.TRACK_SPACING_DIV2
                for j in range(1,self.beatCount):
                    x = beatStart + j*self.beatSpacing
                    pixmap.draw_line( self.gc, x, self.trackRect[self.drumIndex].y, x, self.trackRect[self.drumIndex].y+self.trackRect[self.drumIndex].height )
                
            # draw notes
            notes = self.note[self.curPage][self.drumIndex]
            for n in range( resume[1], len(notes) ):
                # check escape
                if 0:
                    resume[0] = i
                    resume[1] = n
                    TP.ProfilePause( "TrackInterface::predraw" )
                    return False
                if not notes[n].draw( pixmap, self.gc, startX, stopX ): break
                
        self.screenBufDirty[buf] = False
                        
        TP.ProfileEnd( "TrackInterface::predraw" )
        
        return True

    def expose( self, DA, event ):    
        
        if self.screenBufDirty[self.curScreen]:
            self.predraw( self.curScreen )
        
        TP.ProfileBegin( "TrackInterface::expose" )
    
        startX = event.area.x
        startY = event.area.y
        stopX = event.area.x + event.area.width
        stopY = event.area.y + event.area.height
        
        self.gc.set_clip_rectangle( event.area )
        
        #print "%d %d %d %d" % (startX,startY,stopX,stopY)
        
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
            
        self.drawingAreaDirty = False

        TP.ProfileEnd( "TrackInterface::expose" )        
          
    def invalidate_rect( self, x, y, width, height, page, base = True ):
        self.dirtyRectToAdd.x = x
        self.dirtyRectToAdd.y = y
        self.dirtyRectToAdd.width = width
        self.dirtyRectToAdd.height = height
        
        if page == self.curPage:
            if base: # the base image has been dirtied
                if not self.screenBufDirty[self.curScreen]:
                    self.screenBufDirtyRect[self.curScreen].x = x
                    self.screenBufDirtyRect[self.curScreen].y = y
                    self.screenBufDirtyRect[self.curScreen].width = width
                    self.screenBufDirtyRect[self.curScreen].height = height
                else:
                    self.screenBufDirtyRect[self.curScreen] = self.screenBufDirtyRect[self.curScreen].union( self.dirtyRectToAdd )
                self.screenBufResume[self.curScreen] = [0,0]
                self.screenBufDirty[self.curScreen] = True
            if self.drawingArea.window != None:
                self.drawingArea.window.invalidate_rect( self.dirtyRectToAdd, True )
            self.drawingAreaDirty = True
        
        elif page == self.screenBufPage[self.preScreen]:
            if not self.screenBufDirty[self.preScreen]:
                self.screenBufDirtyRect[self.preScreen].x = x
                self.screenBufDirtyRect[self.preScreen].y = y
                self.screenBufDirtyRect[self.preScreen].width = width
                self.screenBufDirtyRect[self.preScreen].height = height
            else:
                self.screenBufDirtyRect[self.preScreen] = self.screenBufDirtyRect[self.preScreen].union( self.dirtyRectToAdd )
            self.screenBufResume[self.preScreen] = [0,0]
            self.screenBufDirty[self.preScreen] = True
            
        #self.queue_draw()

    def getTrackOrigin( self, track ):
        return ( self.trackRect[track].x, self.trackRect[track].y )

    def ticksToPixels( self, ticks ):
        return int(round( ticks * self.pixelsPerTick ))
    def pixelsToTicks( self, pixels ):
        return int(round( pixels * self.ticksPerPixel ))
    def pitchToPixels( self, pitch ):
        return int(round(  ( Config.MAXIMUM_PITCH - pitch ) * self.pixelsPerPitch ))
    def pixelsToPitch( self, pixels ):
        return int(round(-pixels*self.pitchPerPixel))
    def pitchToPixelsDrum( self, pitch ):
        return int(round(  ( Config.MAXIMUM_PITCH_DRUM - pitch ) * self.pixelsPerPitchDrum ))
    def pixelsToPitchDrum( self, pixels ):
        return int(round(-pixels*self.pitchPerPixelDrum))
