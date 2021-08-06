import pygtk
pygtk.require( '2.0' )
import gtk

from math import floor

from Framework.Constants import Constants
from GUI.GUIConstants import GUIConstants
from GUI.Core.NoteParametersWindow import NoteParametersWindow

from Framework.Core.Profiler import TP

class SELECTNOTES:
    ALL = -1
    NONE = 0
    ADD = 1
    REMOVE = 2
    EXCLUSIVE = 3

#-------------------------------------------------------------
# This is a TEMPORARY implementaion of the BackgroundView,
# it was written quickly to get track selections working
#-------------------------------------------------------------

# TODO: Do I really have to subclass gtk.EventBox to get the button-press-event?
# (I wasn't getting it subclassing directly from DrawingArea)
class BackgroundView( gtk.EventBox ):
    #-----------------------------------
    # initialization
    #-----------------------------------
    def __init__( self, trackIDs, selectedTrackIDs, selectionChangedCallback, mutedTrackIDs, beatsPerPageAdjustment, trackDictionary, selectedPageIDs, updatePageCallback ):
        gtk.EventBox.__init__( self )
        
        self.drawingArea = gtk.DrawingArea()
        self.add( self.drawingArea )

        self.sizeInitialized = False
        
        self.trackViews = {}
        self.trackIDs = trackIDs
        self.selectedTrackIDs = selectedTrackIDs
        self.selectionChangedCallback = selectionChangedCallback
        self.mutedTrackIDs = mutedTrackIDs
        self.beatsPerPageAdjustment = beatsPerPageAdjustment
        self.trackDictionary = trackDictionary
        self.selectedPageIDs = selectedPageIDs
        self.updatePageCallback = updatePageCallback
        
        self.curAction = False          # stores the current mouse action
        self.curActionObject = False    # stores the object that in handling the action

        self.buttonPressCount = 1   # used on release events to indicate double/triple releases
        self.clickLoc = [0,0]       # location of the last click
        self.marqueeLoc = False     # current drag location of the marquee

        self.drawingArea.connect( "expose-event", self.draw )
        self.connect( "button-press-event", self.handleButtonPress )
        self.connect( "button-release-event", self.handleButtonRelease )
        self.connect( "motion-notify-event", self.handleMotion )
        
    #-----------------------------------
    # access methods
    #-----------------------------------
    def getTrackRect( self, trackID ):
        return gtk.gdk.Rectangle( GUIConstants.BORDER_SIZE,
                                  self.getTrackYLocation( trackID ), 
                                  self.getTrackWidth(), 
                                  self.getTrackHeight() )
        
    def getTrackWidth( self ):
        return self.get_allocation().width - 2 * ( GUIConstants.BORDER_SIZE + 2 )

    def getFullHeight( self ):
        return int( floor( self.get_allocation().height / len( self.trackIDs ) ) )

    def getTrackHeight( self ):
        return int( self.getFullHeight() - 2 * self.getTrackSpacing() )
    
    #TODO-> trackIDs should probably be ordered!
    # we're just using trackID as an index here (this will only work until you can remove tracks)
    def getTrackYLocation( self, trackID ):
        if self.getTrackHeight() < 0:
            return 0
        else:
            trackIndex = trackID
            
            trackHeight = int( floor( self.get_allocation().height / len( self.trackIDs ) ) )
            trackBackgroundYValue = trackHeight * trackIndex
            return trackBackgroundYValue + GUIConstants.BORDER_SIZE

    def getTrackSpacing( self ):
        return GUIConstants.TRACK_SPACING
    
    #-----------------------------------
    # callback methods
    #-----------------------------------
    def set_size_request( self, width, height ):
        self.sizeInitialized = True
        self.drawingArea.set_size_request( width, height )
        self.height = height
        self.width = width

        numTracks = len(self.trackIDs)
        trackSpacing = self.getTrackSpacing()
        if numTracks: self.trackHeight = int( floor( (height - trackSpacing*(numTracks-1)) / numTracks ) )
        else:         self.trackHeight = 1
        self.trackWidth = width
   
        trackCount = 0
        for trackID in self.trackIDs:
            self.trackViews[trackID].set_size_request( self.trackWidth, self.trackHeight )
            self.trackViews[trackID].setPositionOffset( (0, trackCount*(self.trackHeight+trackSpacing)) )
            trackCount += 1

    def setCurrentTracks( self, trackViews ):

        oldLen = len(self.trackViews)
        
        if oldLen and trackViews != self.trackViews: self.clearSelectedNotes() # clear all the currently selected notes

        self.trackViews = trackViews
        
        numTracks = len(self.trackViews)
        if oldLen != numTracks and self.sizeInitialized:        
            trackSpacing = self.getTrackSpacing()
            if numTracks: self.trackHeight = int( floor( (self.height - trackSpacing*(numTracks-1)) / numTracks ) )
            else:         self.trackHeight = 1
            trackCount = 0
            for trackID in self.trackIDs:
                self.trackViews[trackID].set_size_request( self.trackWidth, self.trackHeight )
                self.trackViews[trackID].setPositionOffset( (0, trackCount*(self.trackHeight+trackSpacing)) )
                trackCount += 1

        self.dirty()
        

    def getNoteParameters( self ):
        for trackID in self.selectedTrackIDs:
            for pageID in self.selectedPageIDs:
                for note in self.trackDictionary[ trackID ][ pageID ]:
                    newPitch = note.pitch + self.noteParameters.pitchAdjust.value
                    newAmplitude = note.amplitude *  self.noteParameters.amplitudeAdjust.value
                    newPan = self.noteParameters.panAdjust.value
                    newReverbSend = note.reverbSend * self.noteParameters.reverbSendAdjust.value
                    newAttack = self.noteParameters.attackAdjust.value
                    newDecay = self.noteParameters.decayAdjust.value
                    newFilterType = self.noteParameters.filterType
                    newFilterCutoff = self.noteParameters.filterCutoff
                    newTied = self.noteParameters.tied
                    newOverlap = self.noteParameters.overlap

                    note.pitch = self.noteParametersBoundaries( newPitch, note.pitch, Constants.MINIMUM_PITCH, Constants.MAXIMUM_PITCH )
                    note.amplitude = self.noteParametersBoundaries( newAmplitude, note.amplitude, Constants.MINIMUM_AMPLITUDE, Constants.MAXIMUM_AMPLITUDE )
                    note.reverbSend = self.noteParametersBoundaries( newReverbSend, note.reverbSend, Constants.MINIMUM_AMPLITUDE,               
                                                                                                                Constants.MAXIMUM_AMPLITUDE )                    
                    if newPan != note.pan:
                        note.pan = newPan

                    if newAttack != note.attack:
                        note.attack = newAttack

                    if newDecay != note.decay:
                        note.decay = newDecay

                    if newFilterType != note.filterType:
                        note.filterType = newFilterType

                    if newFilterCutoff != note.filterCutoff:
                        note.filterCutoff = newFilterCutoff

                    if newTied != note.tied:
                        note.tied = newTied

                    if newOverlap != note.overlap:
                        note.overlap = newOverlap

        self.updatePageCallback()

    def noteParametersBoundaries( self, newValue, noteValue, minBoundary, maxBoundary ):
                if newValue != noteValue:
                    if newValue >= minBoundary and newValue <= maxBoundary:
                        return  newValue
                    elif newValue < minBoundary:
                        return minBoundary
                    elif newValue > maxBoundary:
                        return maxBoundary
                else:
                    return noteValue

    #-----------------------------------
    # action and event methods
    #-----------------------------------
    def setCurrentAction( self, action, obj ):
        if self.curAction:
            print "BackgroundView - Action already in progress!"

        self.curAction = action
        self.curActionObject = obj

        if action == "note-drag-onset":      self.updateDragLimits()
        elif action == "note-drag-duration": self.updateDragLimits()
        elif action == "note-drag-pitch":    self.updateDragLimits()

    def doneCurrentAction( self ):
        if self.curAction == "note-drag-onset":      self.doneNoteDrag()
        elif self.curAction == "note-drag-duration": self.doneNoteDrag()
        elif self.curAction == "note-drag-pitch":    self.doneNoteDrag()

        self.curAction = False
        self.curActionObject = False

    def toggleTrack( self, trackID, exclusive ):
        if exclusive:
            self.selectedTrackIDs.clear()
            self.selectedTrackIDs.add( trackID )
        else:
            if trackID in self.selectedTrackIDs:
                self.selectedTrackIDs.discard( trackID )
            else:
                self.selectedTrackIDs.add( trackID )

    def selectionChanged( self ):
        if self.curAction == "note-drag-onset":      self.updateDragLimits()
        elif self.curAction == "note-drag-duration": self.updateDragLimits()
        elif self.curAction == "note-drag-pitch":    self.updateDragLimits()
        self.dirty()

    def selectNotesByBar( self, selID, startX, stopX ):
        beatCount = int(round( self.beatsPerPageAdjustment.value, 0 ))
        for trackID in self.trackIDs:
            if trackID == selID: 
                notes = self.trackViews[trackID].getNotesByBar( beatCount, startX, stopX )
                self.trackViews[trackID].selectNotes( SELECTNOTES.EXCLUSIVE, notes )
            else:
                self.trackViews[trackID].selectNotes( SELECTNOTES.NONE, [] )
        self.selectionChanged()
        
    def selectNotesByTrack( self, selID ):
        for trackID in self.trackIDs:
            if trackID == selID: self.trackViews[trackID].selectNotes( SELECTNOTES.ALL, [] )
            else:                self.trackViews[trackID].selectNotes( SELECTNOTES.NONE, [] )
        self.selectionChanged()

    def selectNotes( self, noteDic ):
        for trackID in self.trackIDs:
            if trackID in noteDic: self.trackViews[trackID].selectNotes( SELECTNOTES.EXCLUSIVE, noteDic[trackID] )
            else:                  self.trackViews[trackID].selectNotes( SELECTNOTES.NONE, [] )
        self.selectionChanged()
    
    def addNotesToSelection( self, noteDic ):
        for trackID in self.trackIDs:
            if trackID in noteDic: self.trackViews[trackID].selectNotes( SELECTNOTES.ADD, noteDic[trackID] )
        self.selectionChanged()

    def deselectNotes( self, noteDic ):
        for trackID in self.trackIDs:
            if trackID in noteDic: self.trackViews[trackID].selectNotes( SELECTNOTES.REMOVE, noteDic[trackID] )
        self.selectionChanged()

    def clearSelectedNotes( self ):
        for trackID in self.trackIDs:
            self.trackViews[trackID].selectNotes( SELECTNOTES.NONE, [] )
        self.selectionChanged()

    def updateDragLimits( self ):
        self.dragLimits = [ [-9999,9999], [-9999,9999], [-9999,9999] ] # initialize to big numbers!
        for trackID in self.trackIDs:
            self.trackViews[trackID].updateDragLimits( self.dragLimits )

    def noteDragOnset( self, event ):
        dx = event.x - self.clickLoc[0]
        dx = min( self.dragLimits[0][1], max( self.dragLimits[0][0], dx ) )
        dy = 0
        dw = 0
        
        for trackID in self.trackIDs:
            self.trackViews[trackID].noteDrag( self, dx, dy, dw )
        self.dirty()

    def noteDragDuration( self, event ):
        dx = 0
        dy = 0
        dw = event.x - self.clickLoc[0]
        dw = min( self.dragLimits[2][1], max( self.dragLimits[2][0], dw ) )

        for trackID in self.trackIDs:
            self.trackViews[trackID].noteDrag( self, dx, dy, dw )
        self.dirty()

    def noteDragPitch( self, event ):
        dx = 0
        dy = event.y - self.clickLoc[1]
        dy = min( self.dragLimits[1][1], max( self.dragLimits[1][0], dy ) )
        dw = 0
        
        for trackID in self.trackIDs:
            self.trackViews[trackID].noteDrag( self, dx, dy, dw )
        self.dirty()

    def doneNoteDrag( self ):
        for trackID in self.trackIDs:
            self.trackViews[trackID].doneNoteDrag( self )

    def updateMarquee( self, event ):
        self.marqueeLoc = [ event.x, event.y ]    
        parentRect = self.get_allocation()    
        if self.marqueeLoc[0] < 0: self.marqueeLoc[0] = 0
        elif self.marqueeLoc[0] > parentRect.width: self.marqueeLoc[0] = parentRect.width
        if self.marqueeLoc[1] < 0: self.marqueeLoc[1] = 0
        elif self.marqueeLoc[1] > parentRect.height: self.marqueeLoc[1] = parentRect.height

        self.dirty()

    def doneMarquee( self, event ):                
        if self.marqueeLoc:
            start = [ min(self.clickLoc[0],self.marqueeLoc[0]), \
                      min(self.clickLoc[1],self.marqueeLoc[1]) ]
            stop =  [ max(self.clickLoc[0],self.marqueeLoc[0]), \
                      max(self.clickLoc[1],self.marqueeLoc[1]) ]

            select = {}
            
            trackSpacing = self.getTrackSpacing()
            trackTop = 0
            for trackID in self.trackIDs:
                notes = self.trackViews[trackID].handleMarqueeSelect( self, start, stop )
                if notes: select[trackID] = notes
                trackTop += self.trackHeight + trackSpacing
                if trackTop > stop[1]: break
            
            self.selectNotes( select )

        self.marqueeLoc = False        
        self.doneCurrentAction()
        
        self.dirty()

    def handleButtonPress( self, drawingArea, event ):

        TP.ProfileBegin( "BV::handleButtonPress" )

        if event.type == gtk.gdk._2BUTTON_PRESS:   self.buttonPressCount = 2
        elif event.type == gtk.gdk._3BUTTON_PRESS: self.buttonPressCount = 3
        else:                                      self.buttonPressCount = 1

        self.clickLoc = [ event.x, event.y ]

        trackSpacing = self.getTrackSpacing()
        trackTop = 0
        for trackID in self.trackIDs:
            handled = self.trackViews[trackID].handleButtonPress( self, event )
            trackTop += self.trackHeight + trackSpacing
            if handled or trackTop > event.y: break

        if handled: 
            if not self.curAction: self.curAction = True # it was handled maybe no action was declared, set curAction to True anyway
            TP.ProfileEnd( "BV::handleButtonPress" )
            return 

        if event.button == 3:
            self.noteParameters = NoteParametersWindow( self.trackDictionary, self.getNoteParameters )
            self.setCurrentAction( "noteParameters", False )

        TP.ProfileEnd( "BV::handleButtonPress" )


    def handleButtonRelease( self, drawingArea, event ):
        TP.ProfileBegin( "BV::handleButtonRelease" )

        if not self.curAction: #do track selection stuff here so that we can also handle marquee selection
            trackSpacing = self.getTrackSpacing()
            trackTop = 0
            for trackID in self.trackIDs:
                handled = self.trackViews[trackID].handleButtonRelease( self, event, self.buttonPressCount )
                trackTop += self.trackHeight + trackSpacing
                if handled or trackTop > event.y: break
        
            if handled: self.dirty()
            
            TP.ProfileEnd( "BV::handleButtonRelease" )
            return

        if not self.curActionObject: # there was no real action to carry out
            self.curAction = False
            TP.ProfileEnd( "BV::handleButtonRelease" )
            return

        if self.curActionObject != self:
            if self.curActionObject.handleButtonRelease( self, event, self.buttonPressCount ):
                self.dirty()
            TP.ProfileEnd( "BV::handleButtonRelease" )
            return
            

        # we're doing the action ourselves

        if self.curAction == "marquee": self.doneMarquee( event )

        TP.ProfileEnd( "BV::handleButtonRelease" )
        return

    def handleMotion( self, drawingArea, event ):
        TP.ProfileBegin( "BV::handleMotion" )

        if not self.curAction: # no action is in progress yet we're dragging, start a marquee
            self.setCurrentAction( "marquee", self )

        if self.curAction == "note-drag-onset": 
            self.noteDragOnset( event )
            TP.ProfileEnd( "BV::handleMotion" )
            return

        if self.curAction == "note-drag-duration": 
            self.noteDragDuration( event )
            TP.ProfileEnd( "BV::handleMotion" )
            return

        if self.curAction == "note-drag-pitch": 
            self.noteDragPitch( event )
            TP.ProfileEnd( "BV::handleMotion" )
            return
        
        # we're doing the action ourselves
        
        if self.curAction == "marquee": self.updateMarquee( event )

        TP.ProfileEnd( "BV::handleMotion" )
        return
    
    def TEMPOLDSTUFF(self):

        #TODO change this to accomodate the space between tracks 
        trackHeight = ( drawingArea.get_allocation().height - 1 ) / len( self.trackIDs )
        trackID = int( floor( event.y / trackHeight ) )
        
        if event.type == gtk.gdk.BUTTON_PRESS:
            #single click toggles track selection
            if trackID in self.selectedTrackIDs:
                self.selectedTrackIDs.discard( trackID )
            else:
                self.selectedTrackIDs.add( trackID )
        elif event.type == gtk.gdk._2BUTTON_PRESS:
            #double click selects a single track
            self.selectedTrackIDs.clear()
            self.selectedTrackIDs.add( trackID )
            
        self.drawingArea.queue_draw()
        self.selectionChangedCallback()
        if event.button == 3:
            self.noteParameters = NoteParametersWindow( self.trackDictionary, self.getNoteParameters )
            
    #-----------------------------------
    # drawing methods
    #-----------------------------------
    def draw( self, drawingArea, event ):
        TP.ProfileBegin( "BackgroundView::draw" )

        context = drawingArea.window.cairo_create()
        context.set_antialias(0) # I don't know what to set this to to turn it off, and it doesn't seem to work anyway!?

        #parentRect = self.get_allocation()
        
        beatCount = int(round( self.beatsPerPageAdjustment.value, 0 ))

        for trackID in self.trackIDs:
            self.trackViews[trackID].draw( context, 
                                           beatCount,
                                           trackID in self.selectedTrackIDs )

       # if self.curAction == "note-drag":   # draw a cross at clickLoc
       #     lineW = 1
       #     context.set_line_width( lineW )    
       #     lineWDIV2 = lineW/2.0    
       #     context.set_source_rgb( 1, 1, 1 )

       #     context.move_to( self.clickLoc[0] + lineWDIV2 - 3, self.clickLoc[1] + lineWDIV2 )
       #     context.rel_line_to( 6, 0 )
       #     context.stroke()    
       #     context.move_to( self.clickLoc[0] + lineWDIV2, self.clickLoc[1] + lineWDIV2 - 3)
       #     context.rel_line_to( 0, 6 )
       #     context.stroke()    
        
        if self.marqueeLoc:                 # draw the selection rect
            lineW = 1
            context.set_line_width( lineW )    
            lineWDIV2 = lineW/2.0    

            context.move_to( self.clickLoc[0] + lineWDIV2, self.clickLoc[1] + lineWDIV2 )
            context.line_to( self.marqueeLoc[0] + lineWDIV2, self.clickLoc[1] + lineWDIV2 )
            context.line_to( self.marqueeLoc[0] + lineWDIV2, self.marqueeLoc[1] + lineWDIV2 )
            context.line_to( self.clickLoc[0] + lineWDIV2, self.marqueeLoc[1] + lineWDIV2 )
            context.close_path()
            context.set_source_rgb( 1, 1, 1 )
            context.stroke()    

        TP.ProfileEnd( "BackgroundView::draw" )        
          
    def dirty( self ):
        self.queue_draw()

    
