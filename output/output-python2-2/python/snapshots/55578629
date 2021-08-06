import pygtk
pygtk.require( '2.0' )
import gtk

from math import floor

from Framework.Constants import Constants
from GUI.GUIConstants import GUIConstants
from GUI.Core.NoteParametersWindow import NoteParametersWindow

from Framework.Core.Profiler import TP

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
        
        self.curAction = False
        self.curActionObject = False

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
        self.trackWidth = width - 2
   
        trackCount = 0
        for trackID in self.trackIDs:
            self.trackViews[trackID].set_size_request( self.trackWidth, self.trackHeight )
            self.trackViews[trackID].setPositionOffset( (0, trackCount*(self.trackHeight+trackSpacing)) )
            trackCount += 1

    def setCurrentTracks( self, trackViews ):

        oldLen = len(self.trackViews)
        
        if oldLen and trackViews != self.trackViews: self.clearSelectedNotes( False ) # clear all the currently selected notes

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

        self.redraw()
        

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

    def doneCurrentAction( self ):
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

    def clearSelectedNotes( self, ignoreNote ):
        for trackID in self.trackIDs:
            self.trackViews[trackID].clearSelectedNotes( ignoreNote )

    def handleButtonPress( self, drawingArea, event ):
        TP.ProfileBegin( "BV::handleButtonPress" )
        trackSpacing = self.getTrackSpacing()

        trackTop = 0
        for trackID in self.trackIDs:
            handled = self.trackViews[trackID].handleButtonPress( self, event )
            trackTop += self.trackHeight + trackSpacing
            if handled or trackTop > event.y: break

        if handled: self.redraw()

        TP.ProfileEnd( "BV::handleButtonPress" )

        if event.button == 3:
            self.noteParameters = NoteParametersWindow( self.trackDictionary, self.getNoteParameters )

    def handleButtonRelease( self, drawingArea, event ):
        
        if not self.curAction: 
            trackSpacing = self.getTrackSpacing()

            trackTop = 0
            for trackID in self.trackIDs:
                handled = self.trackViews[trackID].handleButtonRelease( self, event )
                trackTop += self.trackHeight + trackSpacing
                if handled or trackTop > event.y: break
        
            if handled: self.redraw()

            return

        if self.curActionObject != self:
            self.curActionObject.handleButtonRelease( self, event )

        return

    def handleMotion( self, drawingArea, event ):
        
        if not self.curAction: return

        if self.curActionObject != self:
            self.curActionObject.handleMotion( self, event )

        self.redraw()

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
        parentRect = self.get_allocation()
        
        beatCount = int(round( self.beatsPerPageAdjustment.value, 0 ))

        for trackID in self.trackIDs:
            self.trackViews[trackID].draw( context, 
                                           beatCount,
                                           trackID in self.selectedTrackIDs )
        
        TP.ProfileEnd( "BackgroundView::draw" )        
          
    def redraw( self ):
        self.queue_draw()

    
