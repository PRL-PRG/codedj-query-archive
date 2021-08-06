import pygtk
pygtk.require('2.0')
import gtk
from types import *
from math import sqrt
from Util.ThemeWidgets import *
from Util.NoteDB import PARAMETER
import Config
Tooltips = Config.Tooltips()

class Properties( gtk.VBox ):
    def __init__( self, noteDB, doneHandler ):
        gtk.VBox.__init__( self )
        self.tooltips = gtk.Tooltips()
        self.noteDB = noteDB
        self.doneHandler = doneHandler

        self.context = "page"
        self.notes = {} # notes indexed by page and track
        self.setup = False # flag to block note updates durning setup

        self.filterType = 0

        self.GUI = {}
        self.parametersBox = RoundVBox(fillcolor=Config.INST_BCK_COLOR, bordercolor=Config.PANEL_BCK_COLOR)
        self.parametersBox.set_border_width(1)
        self.parametersBox.set_radius(10)
        self.pack_start(self.parametersBox)

        controlsBox = gtk.HBox()
 
        pitchBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        pitchBox.set_border_width(3)
        pitchBox.set_radius(10)
        self.GUI['pitchUp'] = ImageButton( Config.IMAGE_ROOT+"arrowEditUp.png", Config.IMAGE_ROOT+"arrowEditUpDown.png", Config.IMAGE_ROOT+"arrowEditUpOver.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI['pitchUp'].connect( "clicked", lambda w:self.stepPitch( 1 ) )
        pitchBox.pack_start( self.GUI['pitchUp'] )
        self.pitchIcon = gtk.Image()
        self.pitchIcon.set_from_file(Config.IMAGE_ROOT + 'propPitch2.png')
        pitchBox.pack_start(self.pitchIcon)
        self.GUI['pitchDown'] = ImageButton( Config.IMAGE_ROOT+"arrowEditDown.png", Config.IMAGE_ROOT+"arrowEditDownDown.png", Config.IMAGE_ROOT+"arrowEditDownOver.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI['pitchDown'].connect( "clicked", lambda w:self.stepPitch( -1 ) )
        pitchBox.pack_start( self.GUI['pitchDown'] )
        controlsBox.pack_start(pitchBox)

        volumeBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        volumeBox.set_border_width(3)
        volumeBox.set_radius(10)
        self.GUI['volumeUp'] = ImageButton( Config.IMAGE_ROOT+"arrowEditUp.png", Config.IMAGE_ROOT+"arrowEditUpDown.png", Config.IMAGE_ROOT+"arrowEditUpOver.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI['volumeUp'].connect( "clicked", lambda w:self.stepVolume( 0.1 ) )
        volumeBox.pack_start( self.GUI['volumeUp'] )
        self.volumeIcon = gtk.Image()
        self.volumeIcon.set_from_file(Config.IMAGE_ROOT + 'volume3.png')
        volumeBox.pack_start(self.volumeIcon)
        self.GUI['volumeDown'] = ImageButton( Config.IMAGE_ROOT+"arrowEditDown.png", Config.IMAGE_ROOT+"arrowEditDownDown.png", Config.IMAGE_ROOT+"arrowEditDownOver.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI['volumeDown'].connect( "clicked", lambda w:self.stepVolume( -0.1 ) )
        volumeBox.pack_start( self.GUI['volumeDown'] )
        controlsBox.pack_start(volumeBox)

        panBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        panBox.set_border_width(3)
        panBox.set_radius(10)
        self.panAdjust = gtk.Adjustment( 0.5, 0, 1, .1, .1, 0)
        self.GUI['panSlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.panAdjust, 7 )
        self.panAdjust.connect("value-changed", self.handlePan)
        self.GUI['panSlider'].set_snap( 0.1 )
        self.GUI['panSlider'].set_inverted(True)
        self.GUI['panSlider'].set_size_request(50, 175)
        self.panLabel = gtk.Image()
        self.handlePan( self.panAdjust )
        panBox.pack_start(self.GUI['panSlider'], True, True, 5)
        panBox.pack_start(self.panLabel, False, padding=10)
        controlsBox.pack_start(panBox)

        reverbBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        reverbBox.set_border_width(3)
        reverbBox.set_radius(10)
        self.reverbAdjust = gtk.Adjustment(0.1, 0, 1, 0.1, 0.1, 0)
        self.GUI['reverbSlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.reverbAdjust, 7 )
        self.reverbAdjust.connect("value-changed", self.handleReverb)
        self.GUI['reverbSlider'].set_snap( 0.1 )
        self.GUI['reverbSlider'].set_inverted(True)
        self.GUI['reverbSlider'].set_size_request(50, 175)
        self.reverbLabel = gtk.Image()
        self.handleReverb( self.reverbAdjust )
        reverbBox.pack_start(self.GUI['reverbSlider'], True, True, 5)
        reverbBox.pack_start(self.reverbLabel, False, padding=10)
        controlsBox.pack_start(reverbBox)

        attackBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        attackBox.set_border_width(3)
        attackBox.set_radius(10)
        self.attackAdjust = gtk.Adjustment(0.04, 0.03, 1, .01, .01, 0)
        self.GUI['attackSlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.attackAdjust, 7 )
        self.attackAdjust.connect("value-changed", self.handleAttack)
        self.GUI['attackSlider'].set_snap( 0.01 )
        self.GUI['attackSlider'].set_inverted(True)
        self.GUI['attackSlider'].set_size_request(50, 175)
        self.attackLabel = gtk.Image()
        self.handleAttack( self.attackAdjust )
        attackBox.pack_start(self.GUI['attackSlider'], True, True, 5)
        attackBox.pack_start(self.attackLabel, False, padding=10)
        controlsBox.pack_start(attackBox)

        decayBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        decayBox.set_border_width(3)
        decayBox.set_radius(10)
        self.decayAdjust = gtk.Adjustment(0.31, 0.03, 1, .01, .01, 0)
        self.GUI['decaySlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.decayAdjust, 7 )
        self.decayAdjust.connect("value-changed", self.handleDecay)
        self.GUI['decaySlider'].set_snap( 0.01 )
        self.GUI['decaySlider'].set_inverted(True)
        self.GUI['decaySlider'].set_size_request(50, 175)
        self.decayLabel = gtk.Image()
        self.handleDecay( self.decayAdjust )
        decayBox.pack_start(self.GUI['decaySlider'], True, True, 5)
        decayBox.pack_start(self.decayLabel, False, padding=10)
        controlsBox.pack_start(decayBox)

        filterBox = RoundHBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        filterBox.set_border_width(3)
        filterBox.set_radius(10)

        filterTypeBox = gtk.VBox()
        self.GUI['filterTypeLowButton'] = ImageToggleButton(Config.IMAGE_ROOT + 'propLow3.png', Config.IMAGE_ROOT + 'propLow3Sel.png', Config.IMAGE_ROOT + 'propLow3Over.png')
        self.GUI['filterTypeLowButton'].connect( "toggled", self.handleFilterType, 1 )
        filterTypeBox.pack_start( self.GUI['filterTypeLowButton'] )
        self.GUI['filterTypeHighButton'] = ImageToggleButton(Config.IMAGE_ROOT + 'propHi3.png', Config.IMAGE_ROOT + 'propHi3Sel.png', Config.IMAGE_ROOT + 'propHi3Over.png')
        self.GUI['filterTypeHighButton'].connect( "toggled", self.handleFilterType, 2 )
        filterTypeBox.pack_start( self.GUI['filterTypeHighButton'] )
        self.GUI['filterTypeBandButton'] = gtk.ToggleButton( "B" )
        self.GUI['filterTypeBandButton'] = ImageToggleButton(Config.IMAGE_ROOT + 'propBand3.png', Config.IMAGE_ROOT + 'propBand3Sel.png', Config.IMAGE_ROOT + 'propBand3Over.png')
        self.GUI['filterTypeBandButton'].connect( "toggled", self.handleFilterType, 3 )
        filterTypeBox.pack_start( self.GUI['filterTypeBandButton'] )
        filterBox.pack_start( filterTypeBox )

        self.filterSliderBox = gtk.VBox()
        self.filterSliderBox.set_size_request(50, -1)
        self.cutoffAdjust = gtk.Adjustment(1000, 100, 7000, 100, 100, 0)
        self.GUI['cutoffSlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.cutoffAdjust, 7 )
        self.GUI['cutoffSlider'].set_snap(100)
        self.cutoffAdjust.connect("value-changed", self.handleFilter)
        self.GUI['cutoffSlider'].set_inverted(True)
        self.GUI['cutoffSlider'].set_size_request(50, 175)
        self.filterSliderBox.pack_start(self.GUI['cutoffSlider'], True, True, 5)
        self.filterLabel = gtk.Image()
        self.filterLabel.set_from_file(Config.IMAGE_ROOT + 'propFilter1.png')
        self.filterSliderBox.pack_start(self.filterLabel, False, padding=10)

        filterBox.pack_start(self.filterSliderBox)

        controlsBox.pack_start(filterBox)
        self.parametersBox.pack_start(controlsBox)

        # set tooltips
        for key in self.GUI:
            if Tooltips.PROP.has_key(key):
                self.tooltips.set_tip(self.GUI[key],Tooltips.PROP[key])

        self.show_all()
    
    def setContext( self, context, pageIds = None, trackIds = None, notes = {} ):
        self.context = context
        self.notes = {} 
        if context == "page":
            for p in pageIds:
                self.notes[p] = {}
                for t in range(Config.NUMBER_OF_TRACKS):
                    self.notes[p][t] = self.noteDB.getNotesByTrack( p, t )
        elif context == "track":
            for p in pageIds:
                self.notes[p] = {}
                for t in trackIds:
                    self.notes[p][t] = self.noteDB.getNotesByTrack( p, t )
        else:
            self.notes = notes

        for p in self.notes: 
            for t in self.notes[p]:
                if len(self.notes[p][t]):
                    # initialize values from first note
                    self.setup = True
                    n = self.notes[p][t][0]
                    self.panAdjust.set_value( n.cs.pan )
                    self.reverbAdjust.set_value( n.cs.reverbSend )
                    self.attackAdjust.set_value( n.cs.attack )
                    self.decayAdjust.set_value( n.cs.decay )
                    if n.cs.filterType == 0:
                        self.GUI['filterTypeLowButton'].set_active(False)
                        self.GUI['filterTypeHighButton'].set_active(False)
                        self.GUI['filterTypeBandButton'].set_active(False)
                        self.filterLabel.hide()
                        self.GUI['cutoffSlider'].hide()
                    else:
                        if n.cs.filterType == 1:
                            self.GUI['filterTypeLowButton'].set_active(True)
                        if n.cs.filterType == 2:
                            self.GUI['filterTypeHighButton'].set_active(True)
                        if n.cs.filterType == 3:
                            self.GUI['filterTypeBandButton'].set_active(True)
                        self.filterLabel.show()
                        self.GUI['cutoffSlider'].show()
                    self.filterType = n.cs.filterType
                    self.cutoffAdjust.set_value( n.cs.filterCutoff )
                    self.setup = False
                    return

    def stepPitch( self, step ):
        stream = []
        for p in self.notes:
            for t in self.notes[p]:
                substream = []
                if step > 0:
                    if t != Config.NUMBER_OF_TRACKS-1:  # regular note
                        for n in self.notes[p][t]:
                            if n.cs.pitch != Config.MAXIMUM_PITCH:
                                substream += [ n.id, min( Config.MAXIMUM_PITCH, n.cs.pitch + step ) ]
                    else:                               # drum note
                        for n in self.notes[p][t]:
                            if n.cs.pitch != Config.MAXIMUM_PITCH_DRUM:
                                substream += [ n.id, min( Config.MAXIMUM_PITCH_DRUM, n.cs.pitch + step*Config.PITCH_STEP_DRUM ) ]
                else:
                    if t != Config.NUMBER_OF_TRACKS-1:  # regular note
                        for n in self.notes[p][t]:
                            if n.cs.pitch != Config.MINIMUM_PITCH:
                                substream += [ n.id, max( Config.MINIMUM_PITCH, n.cs.pitch + step ) ]
                    else:                               # drum note
                        for n in self.notes[p][t]:
                            if n.cs.pitch != Config.MINIMUM_PITCH_DRUM:
                                substream += [ n.id, max( Config.MINIMUM_PITCH_DRUM, n.cs.pitch + step*Config.PITCH_STEP_DRUM ) ]
                if len(substream):
                    stream += [ p, t, PARAMETER.PITCH, len(substream)//2 ] + substream
        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )
                    
    def stepVolume( self, step ):
        stream = []
        for p in self.notes:
            for t in self.notes[p]:
                substream = []
                if step > 0:
                    for n in self.notes[p][t]:
                        if n.cs.amplitude != Config.MAXIMUM_AMPLITUDE:
                            substream += [ n.id, min( Config.MAXIMUM_AMPLITUDE, n.cs.amplitude + step ) ]
                else:
                    for n in self.notes[p][t]:
                        if n.cs.amplitude != Config.MINIMUM_AMPLITUDE:
                            substream += [ n.id, max( Config.MINIMUM_AMPLITUDE, n.cs.amplitude + step ) ]
                if len(substream):
                    stream += [ p, t, PARAMETER.AMPLITUDE, len(substream)//2 ] + substream
        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )
         
    def handlePan( self, adjust ):
        img = min( 4, int(adjust.value * 5) )
        self.panLabel.set_from_file(Config.IMAGE_ROOT + 'propPan' + str(img) + '.png')
        if not self.setup:
            stream = []
            for p in self.notes:
                for t in self.notes[p]:
                    if len(self.notes[p][t]):
                        stream += [ p, t, PARAMETER.PAN, len(self.notes[p][t]) ]
                        for n in self.notes[p][t]:
                            stream += [ n.id, adjust.value ]
            if len(stream):
                self.noteDB.updateNotes( stream + [-1] )

    def handleReverb( self, adjust ):
        img = min( 5, int(adjust.value * 6) )
        self.reverbLabel.set_from_file(Config.IMAGE_ROOT + 'propReverb' + str(img) + '.png')
        if not self.setup:
            stream = []
            for p in self.notes:
                for t in self.notes[p]:
                    if len(self.notes[p][t]):
                        stream += [ p, t, PARAMETER.REVERB, len(self.notes[p][t]) ]
                        for n in self.notes[p][t]:
                            stream += [ n.id, adjust.value ]
            if len(stream):
                self.noteDB.updateNotes( stream + [-1] )

    def handleAttack( self, adjust ):
        val = adjust.value*adjust.value
        img = min( 4, int(val * 5) )
        self.attackLabel.set_from_file(Config.IMAGE_ROOT + 'propAtt' + str(img) + '.png')
        if not self.setup:
            stream = []
            for p in self.notes:
                for t in self.notes[p]:
                    if len(self.notes[p][t]):
                        stream += [ p, t, PARAMETER.ATTACK, len(self.notes[p][t]) ]
                        for n in self.notes[p][t]:
                            stream += [ n.id, adjust.value ]
            if len(stream):
                self.noteDB.updateNotes( stream + [-1] )

    def handleDecay( self, adjust ):
        val = adjust.value*adjust.value
        img = min( 4, int(val * 5) )
        self.decayLabel.set_from_file(Config.IMAGE_ROOT + 'propDec' + str(img) + '.png')
        if not self.setup:
            stream = []
            for p in self.notes:
                for t in self.notes[p]:
                    if len(self.notes[p][t]):
                        stream += [ p, t, PARAMETER.DECAY, len(self.notes[p][t]) ]
                        for n in self.notes[p][t]:
                            stream += [ n.id, adjust.value ]
            if len(stream):
                self.noteDB.updateNotes( stream + [-1] )

    def handleFilterType( self, widget, type ):

        if widget.get_active():
            if self.filterType == 0:
                self.filterLabel.show()
                self.GUI['cutoffSlider'].show()

            self.filterType = type
            self.updateFilterLabel()

            if widget != self.GUI['filterTypeLowButton'] and self.GUI['filterTypeLowButton'].get_active():
                self.GUI['filterTypeLowButton'].set_active( False )
            if widget != self.GUI['filterTypeBandButton'] and self.GUI['filterTypeBandButton'].get_active():
                self.GUI['filterTypeBandButton'].set_active( False )
            if widget != self.GUI['filterTypeHighButton'] and self.GUI['filterTypeHighButton'].get_active():
                self.GUI['filterTypeHighButton'].set_active( False )
            if not self.setup:
                typestream = []
                cutoffstream = []
                cutoff = self.cutoffAdjust.value
                for p in self.notes:
                    for t in self.notes[p]:
                        if len(self.notes[p][t]):
                            substream = []
                            typestream += [ p, t, PARAMETER.FILTERTYPE, len(self.notes[p][t]) ]
                            for n in self.notes[p][t]:
                                typestream += [ n.id, type ]
                                if n.cs.filterCutoff != cutoff:
                                    substream += [ n.id, cutoff ]
                            if len(substream):
                                cutoffstream += [ p, t, PARAMETER.FILTERCUTOFF, len(substream)//2 ] + substream
                if len(typestream):
                    self.noteDB.updateNotes( typestream + [-1] )
                if len(cutoffstream):
                    self.noteDB.updateNotes( cutoffstream + [-1] )

        elif type == self.filterType:
            self.filterType = 0
            self.filterLabel.hide()
            self.GUI['cutoffSlider'].hide()
            if not self.setup:
                typestream = []
                for p in self.notes:
                    for t in self.notes[p]:
                        if len(self.notes[p][t]):
                            typestream += [ p, t, PARAMETER.FILTERTYPE, len(self.notes[p][t]) ]
                            for n in self.notes[p][t]:
                                typestream += [ n.id, 0 ]
                if len(typestream):
                    self.noteDB.updateNotes( typestream + [-1] )

    def handleFilter( self, adjust ):
        stream = []
        for p in self.notes:
            for t in self.notes[p]:
                if len(self.notes[p][t]):
                    stream += [ p, t, PARAMETER.FILTERCUTOFF, len(self.notes[p][t]) ]
                    for n in self.notes[p][t]:
                        stream += [ n.id, adjust.value ]
        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )


        self.updateFilterLabel()

   
    def updateFilterLabel( self ):
        val = (self.cutoffAdjust.value-self.cutoffAdjust.lower)/(self.cutoffAdjust.upper-self.cutoffAdjust.lower)
        img = min( 5, int(val * 6) )
        self.filterLabel.set_from_file(Config.IMAGE_ROOT + 'propFilter%d.%d' % (self.filterType, img) + '.png')

    def cancel( self, widget, data=None ):
        self.window.destroy()

    def applyParametersChange( self, data=None ):
        #self.getNoteParameters()
        self.window.destroy()

    def filterCallback( self, widget, data=None):
        if widget.get_active():
            self.filterType = data

    def tiedCallback( self, widget, data=None ):
        self.tied = widget.get_active()

    def overlapCallback( self, widget, data=None ):
        self.overlap = widget.get_active()

    def handleCutoffScale( self, widget, data=None ):
        self.filterCutoff = int( pow( self.filterCutoffAdjust.value, 2) * 19980 + 20 )
        self.filterCutoffLabel.set_text( str( self.filterCutoff  ) )


