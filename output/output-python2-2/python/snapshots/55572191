import pygtk
pygtk.require('2.0')
import gtk
from types import *
from math import sqrt
from random import *
from Generation.Drunk import *
from Generation.GenerationConstants import GenerationConstants
from Util.ThemeWidgets import *
from Util.NoteDB import PARAMETER
import Config
Tooltips = Config.Tooltips()

class Properties( gtk.VBox ):
    def __init__( self, noteDB, doneHandler, popup ):
        gtk.VBox.__init__( self )
        self.tooltips = gtk.Tooltips()
        self.noteDB = noteDB
        #self.doneHandler = doneHandler
        self.popup = popup
        self.popup.resize( 545, 378 )

        self.context = "page"
        self.notes = {} # notes indexed by page and track
        self.setup = False # flag to block note updates durning setup
        
        self.line = Line(0, 100)
        self.drunk = Drunk(0, 100)
        self.droneAndJump = DroneAndJump(0, 100)
        self.repeter = Repeter(0, 100)
        self.loopseg = Loopseg(0, 100)
        self.algoTypes = [self.line, self.drunk, self.droneAndJump, self.repeter, self.loopseg]
        self.algorithm = self.algoTypes[0]

        #self.set_size_request( 300, 200 )

        self.filterType = 0
        self.minValue = 0.
        self.maxValue = 100.
        self.paraValue = 20.

        self.activeWidget = None

        self.pageIds = []
            
        self.GUI = {}
        self.parametersBox = RoundHBox(fillcolor=Config.INST_BCK_COLOR, bordercolor=Config.PANEL_BCK_COLOR)
        #self.parametersBox.set_border_width(1)
        self.parametersBox.set_radius(10)
        self.pack_start(self.parametersBox)
        self.fixed = gtk.Fixed()
        self.parametersBox.pack_start( self.fixed )

        self.controlsBox = gtk.HBox()
 
        #-- Page Properties ------------------------------------------------
        self.pageBox = RoundHBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        self.pageBox.set_size_request( 125, -1 )
        self.pageBox.set_border_width(3)
        self.pageBox.set_radius(10)
        beatBox = gtk.VBox()
        self.beatAdjust = gtk.Adjustment( 4, 2, 12, 1, 1, 0)
        self.GUI['beatSlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.beatAdjust, 7 )
        self.GUI['beatSlider'].connect("button-release-event", self.handleBeat)
        self.GUI['beatSlider'].set_snap( 1 )
        self.GUI['beatSlider'].set_inverted(True)
        self.GUI['beatSlider'].set_size_request(50, 200)
        beatBox.pack_start( self.GUI['beatSlider'] )
        self.beatLabel = gtk.Image()
        self.beatLabel.set_from_file(Config.IMAGE_ROOT + 'volume3.png')
        self.beatAdjust.connect("value-changed", self.updateBeatLabel)
        self.updateBeatLabel( self.beatAdjust )
        beatBox.pack_start( self.beatLabel )
        self.pageBox.pack_start( beatBox )
        colorBox = gtk.VBox()
        self.GUI["color0Button"] = ImageRadioButton( None, Config.IMAGE_ROOT+"pageThumbnailBut0.png", Config.IMAGE_ROOT+"pageThumbnailBut0Down.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI["color0Button"].set_size_request( 80, -1 )
        self.GUI["color0Button"].connect( "clicked", self.handleColor, 0 )
        colorBox.pack_start( self.GUI["color0Button"] )
        self.GUI["color1Button"] = ImageRadioButton( self.GUI["color0Button"], Config.IMAGE_ROOT+"pageThumbnailBut1.png", Config.IMAGE_ROOT+"pageThumbnailBut1Down.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI["color1Button"].connect( "clicked", self.handleColor, 1 )
        colorBox.pack_start( self.GUI["color1Button"] )
        self.GUI["color2Button"] = ImageRadioButton( self.GUI["color0Button"], Config.IMAGE_ROOT+"pageThumbnailBut2.png", Config.IMAGE_ROOT+"pageThumbnailBut2Down.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI["color2Button"].connect( "clicked", self.handleColor, 2 )
        colorBox.pack_start( self.GUI["color2Button"] )
        self.GUI["color3Button"] = ImageRadioButton( self.GUI["color0Button"], Config.IMAGE_ROOT+"pageThumbnailBut3.png", Config.IMAGE_ROOT+"pageThumbnailBut3Down.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI["color3Button"].connect( "clicked", self.handleColor, 3 )
        colorBox.pack_start( self.GUI["color3Button"] )
        self.pageBox.pack_start( colorBox )
        self.pageBox.show_all()
        #self.controlsBox.pack_start(self.pageBox)

        #-- Note Properties ------------------------------------------------
        pitchBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        pitchBox.set_border_width(3)
        pitchBox.set_radius(10)
        self.GUI['pitchUp'] = ImageButton( Config.IMAGE_ROOT+"arrowEditUp.png", Config.IMAGE_ROOT+"arrowEditUpDown.png", Config.IMAGE_ROOT+"arrowEditUpOver.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI['pitchUp'].connect( "clicked", lambda w:self.stepPitch( 1 ) )
        self.GUI['pitchGen'] = ImageToggleButton( Config.IMAGE_ROOT+"diceProp.png", Config.IMAGE_ROOT+"dicePropSel.png", Config.IMAGE_ROOT+"dicePropSel.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI['pitchGen'].connect( "clicked", self.openAlgoBox, 'pitch' )
        pitchBox.pack_start( self.GUI['pitchGen'], False, False, 5 )
        pitchBox.pack_start( self.GUI['pitchUp'] )
        self.pitchIcon = gtk.Image()
        self.pitchIcon.set_from_file(Config.IMAGE_ROOT + 'propPitch2.png')
        pitchBox.pack_start(self.pitchIcon)
        self.GUI['pitchDown'] = ImageButton( Config.IMAGE_ROOT+"arrowEditDown.png", Config.IMAGE_ROOT+"arrowEditDownDown.png", Config.IMAGE_ROOT+"arrowEditDownOver.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI['pitchDown'].connect( "clicked", lambda w:self.stepPitch( -1 ) )
        pitchBox.pack_start( self.GUI['pitchDown'] )
        self.controlsBox.pack_start(pitchBox)

        volumeBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        volumeBox.set_border_width(3)
        volumeBox.set_radius(10)
        self.GUI['volumeUp'] = ImageButton( Config.IMAGE_ROOT+"arrowEditUp.png", Config.IMAGE_ROOT+"arrowEditUpDown.png", Config.IMAGE_ROOT+"arrowEditUpOver.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI['volumeUp'].connect( "clicked", lambda w:self.stepVolume( 0.1 ) )
        self.GUI['volumeGen'] = ImageToggleButton( Config.IMAGE_ROOT+"diceProp.png", Config.IMAGE_ROOT+"dicePropSel.png", Config.IMAGE_ROOT+"dicePropSel.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI['volumeGen'].connect( "clicked", self.openAlgoBox, 'volume' )
        volumeBox.pack_start( self.GUI['volumeGen'], False, False, 5 )
        volumeBox.pack_start( self.GUI['volumeUp'] )
        self.volumeIcon = gtk.Image()
        self.volumeIcon.set_from_file(Config.IMAGE_ROOT + 'volume3.png')
        volumeBox.pack_start(self.volumeIcon)
        self.GUI['volumeDown'] = ImageButton( Config.IMAGE_ROOT+"arrowEditDown.png", Config.IMAGE_ROOT+"arrowEditDownDown.png", Config.IMAGE_ROOT+"arrowEditDownOver.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI['volumeDown'].connect( "clicked", lambda w:self.stepVolume( -0.1 ) )
        volumeBox.pack_start( self.GUI['volumeDown'] )
        self.controlsBox.pack_start(volumeBox)

        panBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        panBox.set_border_width(3)
        panBox.set_radius(10)
        self.panAdjust = gtk.Adjustment( 0.5, 0, 1, .1, .1, 0)
        self.GUI['panSlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.panAdjust, 7 )
        self.panAdjust.connect("value-changed", self.handlePan)
        self.GUI['panSlider'].set_snap( 0.1 )
        self.GUI['panSlider'].set_inverted(True)
        self.GUI['panSlider'].set_size_request(50, 200)
        self.panLabel = gtk.Image()
        self.handlePan( self.panAdjust )
        self.GUI['panGen'] = ImageToggleButton( Config.IMAGE_ROOT+"diceProp.png", Config.IMAGE_ROOT+"dicePropSel.png", Config.IMAGE_ROOT+"dicePropSel.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI['panGen'].connect( "clicked", self.openAlgoBox, 'pan' )
        panBox.pack_start(self.GUI['panGen'], True, True, 5)
        panBox.pack_start(self.GUI['panSlider'], True, True, 5)
        panBox.pack_start(self.panLabel, False, padding=10)
        self.controlsBox.pack_start(panBox)

        reverbBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        reverbBox.set_border_width(3)
        reverbBox.set_radius(10)
        self.reverbAdjust = gtk.Adjustment(0.1, 0, 1, 0.1, 0.1, 0)
        self.GUI['reverbSlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.reverbAdjust, 7 )
        self.reverbAdjust.connect("value-changed", self.handleReverb)
        self.GUI['reverbSlider'].set_snap( 0.1 )
        self.GUI['reverbSlider'].set_inverted(True)
        self.GUI['reverbSlider'].set_size_request(50, 200)
        self.reverbLabel = gtk.Image()
        self.handleReverb( self.reverbAdjust )
        self.GUI['reverbGen'] = ImageToggleButton( Config.IMAGE_ROOT+"diceProp.png", Config.IMAGE_ROOT+"dicePropSel.png", Config.IMAGE_ROOT+"dicePropSel.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI['reverbGen'].connect( "clicked", self.openAlgoBox, 'reverb' )
        reverbBox.pack_start(self.GUI['reverbGen'], True, True, 5)        
        reverbBox.pack_start(self.GUI['reverbSlider'], True, True, 5)
        reverbBox.pack_start(self.reverbLabel, False, padding=10)
        self.controlsBox.pack_start(reverbBox)

        attackBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        attackBox.set_border_width(3)
        attackBox.set_radius(10)
        self.attackAdjust = gtk.Adjustment(0.04, 0.03, 1, .01, .01, 0)
        self.GUI['attackSlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.attackAdjust, 7 )
        self.attackAdjust.connect("value-changed", self.handleAttack)
        self.GUI['attackSlider'].set_snap( 0.01 )
        self.GUI['attackSlider'].set_inverted(True)
        self.GUI['attackSlider'].set_size_request(50, 200)
        self.attackLabel = gtk.Image()
        self.handleAttack( self.attackAdjust )
        self.GUI['attackGen'] = ImageToggleButton( Config.IMAGE_ROOT+"diceProp.png", Config.IMAGE_ROOT+"dicePropSel.png", Config.IMAGE_ROOT+"dicePropSel.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI['attackGen'].connect( "clicked", self.openAlgoBox, 'attack' )
        attackBox.pack_start(self.GUI['attackGen'], True, True, 5)        
        attackBox.pack_start(self.GUI['attackSlider'], True, True, 5)
        attackBox.pack_start(self.attackLabel, False, padding=10)
        self.controlsBox.pack_start(attackBox)

        decayBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        decayBox.set_border_width(3)
        decayBox.set_radius(10)
        self.decayAdjust = gtk.Adjustment(0.31, 0.03, 1, .01, .01, 0)
        self.GUI['decaySlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.decayAdjust, 7 )
        self.decayAdjust.connect("value-changed", self.handleDecay)
        self.GUI['decaySlider'].set_snap( 0.01 )
        self.GUI['decaySlider'].set_inverted(True)
        self.GUI['decaySlider'].set_size_request(50, 200)
        self.decayLabel = gtk.Image()
        self.handleDecay( self.decayAdjust )
        self.GUI['decayGen'] = ImageToggleButton( Config.IMAGE_ROOT+"diceProp.png", Config.IMAGE_ROOT+"dicePropSel.png", Config.IMAGE_ROOT+"dicePropSel.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI['decayGen'].connect( "clicked", self.openAlgoBox, 'decay' )
        decayBox.pack_start(self.GUI['decayGen'], True, True, 5)        
        decayBox.pack_start(self.GUI['decaySlider'], True, True, 5)
        decayBox.pack_start(self.decayLabel, False, padding=10)
        self.controlsBox.pack_start(decayBox)

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
        self.GUI['cutoffSlider'].set_size_request(50, 200)
        self.GUI['cutoffGen'] = ImageToggleButton( Config.IMAGE_ROOT+"diceProp.png", Config.IMAGE_ROOT+"dicePropSel.png", Config.IMAGE_ROOT+"dicePropSel.png", backgroundFill = Config.PANEL_COLOR )
        self.GUI['cutoffGen'].connect( "clicked", self.openAlgoBox, 'cutoff' )
        self.filterSliderBox.pack_start(self.GUI['cutoffGen'], True, True, 5)        
        self.filterSliderBox.pack_start(self.GUI['cutoffSlider'], True, True, 5)
        self.filterLabel = gtk.Image()
        self.filterLabel.set_from_file(Config.IMAGE_ROOT + 'propFilter1.png')
        self.filterSliderBox.pack_start(self.filterLabel, False, padding=10)

        filterBox.pack_start(self.filterSliderBox)

        self.controlsBox.pack_start(filterBox)
        
        self.algoBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        self.algoBox.set_size_request( -1, 378 )
        self.algoBox.set_border_width(3)
        self.algoBox.set_radius(10)
        #self.algoBox = gtk.VBox()
        
        algoUpperBox = gtk.HBox()
        
        algoRadioButtonBox = gtk.VBox()
        algoRadioButtonBox.set_size_request(100, 150)
        #algoRadioButtonBox = RoundHBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        #algoRadioButtonBox.set_border_width(3)
        #algoRadioButtonBox.set_radius(10)

        self.GUI['line'] = ImageRadioButton( None, Config.IMAGE_ROOT + 'propLine.png', Config.IMAGE_ROOT + 'propLineDown.png', Config.IMAGE_ROOT + 'propLineOver.png' )
        self.GUI['line'].connect( "toggled", self.handleAlgo, 0 )
        algoRadioButtonBox.pack_start( self.GUI['line'], False, False, 1 )
        self.GUI['drunk'] = ImageRadioButton( self.GUI['line'], Config.IMAGE_ROOT + 'propDrunk.png', Config.IMAGE_ROOT + 'propDrunkDown.png', Config.IMAGE_ROOT + 'propDrunkOver.png' )
        self.GUI['drunk'].connect( "toggled", self.handleAlgo, 1 )
        algoRadioButtonBox.pack_start( self.GUI['drunk'], False, False, 1 )
        self.GUI['droneJump'] = ImageRadioButton( self.GUI['line'], Config.IMAGE_ROOT + 'propDroneJump.png', Config.IMAGE_ROOT + 'propDroneJumpDown.png', Config.IMAGE_ROOT + 'propDroneJumpOver.png' )
        self.GUI['droneJump'].connect( "toggled", self.handleAlgo, 2 )
        algoRadioButtonBox.pack_start( self.GUI['droneJump'], False, False, 1 )
        self.GUI['repeater'] = ImageRadioButton( self.GUI['line'], Config.IMAGE_ROOT + 'propRepeater.png', Config.IMAGE_ROOT + 'propRepeaterDown.png', Config.IMAGE_ROOT + 'propRepeaterOver.png' )
        self.GUI['repeater'].connect( "toggled", self.handleAlgo, 3 )
        algoRadioButtonBox.pack_start( self.GUI['repeater'], False, False, 1 )
        self.GUI['loopseg'] = ImageRadioButton( self.GUI['line'], Config.IMAGE_ROOT + 'propLoopseg.png', Config.IMAGE_ROOT + 'propLoopsegDown.png', Config.IMAGE_ROOT + 'propLoopsegOver.png' )
        self.GUI['loopseg'].connect( "toggled", self.handleAlgo, 4 )
        algoRadioButtonBox.pack_start( self.GUI['loopseg'], False, False, 1 )
        
        algoUpperBox.pack_start(algoRadioButtonBox)
        
        algoSlidersBox = gtk.HBox()
        algoSlidersBox.set_size_request(150, 320)
        #algoSlidersBox = RoundHBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        #algoSlidersBox.set_border_width(3)
        #algoSlidersBox.set_radius(10)        
        minBox = gtk.VBox()
        self.minAdjust = gtk.Adjustment(0, 0, 100, 1, 1, 0)
        self.GUI['minSlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.minAdjust, 7 )
        self.GUI['minSlider'].set_snap(1)
        self.minAdjust.connect("value-changed", self.handleMin)
        self.GUI['minSlider'].set_inverted(True)
        self.GUI['minSlider'].set_size_request(50, 200)
        minBox.pack_start(self.GUI['minSlider'], True, True, 5)     
        algoSlidersBox.pack_start(minBox)
        
        maxBox = gtk.VBox()
        self.maxAdjust = gtk.Adjustment(100, 0, 100, 1, 1, 0)
        self.GUI['maxSlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.maxAdjust, 7 )
        self.GUI['maxSlider'].set_snap(1)
        self.maxAdjust.connect("value-changed", self.handleMax)
        self.GUI['maxSlider'].set_inverted(True)
        self.GUI['maxSlider'].set_size_request(50, 200)
        maxBox.pack_start(self.GUI['maxSlider'], True, True, 5)     
        algoSlidersBox.pack_start(maxBox)
        
        paraBox = gtk.VBox()
        self.paraAdjust = gtk.Adjustment(20, 0, 100, 1, 1, 0)
        self.GUI['paraSlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.paraAdjust, 7 )
        self.GUI['paraSlider'].set_snap(1)
        self.paraAdjust.connect("value-changed", self.handlePara)
        self.GUI['paraSlider'].set_inverted(True)
        self.GUI['paraSlider'].set_size_request(50, 200)
        paraBox.pack_start(self.GUI['paraSlider'], True, True, 5)     
        algoSlidersBox.pack_start(paraBox)         
        
        algoUpperBox.pack_start(algoSlidersBox)
        
        self.algoBox.pack_start(algoUpperBox)
        
        #transButtonBox = RoundHBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        #transButtonBox.set_border_width(3)
        #transButtonBox.set_radius(10)
        transButtonBox = gtk.HBox()
        transButtonBox.set_size_request(150, 50)
        
        # create cancel/check button
        self.GUI["checkButton"] = ImageButton(Config.IMAGE_ROOT + 'check.png', backgroundFill=Config.PANEL_COLOR )
        self.GUI["checkButton"].connect("clicked", self.apply)
 
        self.GUI["cancelButton"] = ImageButton(Config.IMAGE_ROOT + 'closeA.png', backgroundFill=Config.PANEL_COLOR )
        self.GUI["cancelButton"].connect("clicked", self.cancel)
        
        transButtonBox.pack_end(self.GUI["checkButton"], False, False, 10)
        transButtonBox.pack_end(self.GUI["cancelButton"], False, False)
        self.algoBox.pack_start(transButtonBox)
        
        self.fixed.put( self.controlsBox, 0, 0 )        
        self.algoBox.show_all()

        # set tooltips
        for key in self.GUI:
            if Tooltips.PROP.has_key(key):
                self.tooltips.set_tip(self.GUI[key],Tooltips.PROP[key])
        self.tooltips.set_tip(self.GUI['paraSlider'], 'Random')
                

        self.show_all()

    def openAlgoBox( self, widget, data=None ):
        if widget.get_active() == True:
            self.property = data
            if self.activeWidget:
                self.activeWidget.set_active(False)
            self.activeWidget = widget
            if self.context == "page": 
                if self.algoBox.parent == None: self.fixed.put( self.algoBox, 671, 0 )
                else:                           self.fixed.move( self.algoBox, 671, 0 )
                self.popup.resize( 927, 378 )
            else:                      
                self.popup.resize( 801, 378 )
                if self.algoBox.parent == None: self.fixed.put( self.algoBox, 545, 0 )
                else:                           self.fixed.move( self.algoBox, 545, 0 )
        else:
            self.property = None
            self.activeWidget = None
            if self.algoBox.parent != None:
                if self.context == "page": self.popup.resize( 671, 378 )
                else:                      self.popup.resize( 545, 378 )
                self.fixed.remove( self.algoBox )
            
    def setContext( self, context, scale, pageIds = None, trackIds = None, notes = {} ):
        self.context = context
        self.scale = GenerationConstants.SCALES[scale]
        self.notes = {}
        self.pageIds = pageIds
        self.trackIds = trackIds
        
        try:
            self.activeWidget.set_active(False)
            self.activeWidget = None
        except:
            self.activeWidget = None
            
        if context == "page":
            if self.pageBox.parent == None:
                self.controlsBox.pack_start( self.pageBox )
                self.controlsBox.reorder_child( self.pageBox, 0 )
                self.controlsBox.set_size_request( 671, 378 )
                self.popup.resize( 671, 378 )
            self.trackIds = [0,1,2,3,4]
            for p in pageIds:
                self.notes[p] = {}
                for t in range(Config.NUMBER_OF_TRACKS):
                    self.notes[p][t] = self.noteDB.getNotesByTrack( p, t )
            page = self.noteDB.getPage(pageIds[0])
            self.beatAdjust.set_value(page.beats)
            btn = "color%dButton" % page.color 
            self.GUI[btn].set_active(True)
        elif context == "track":
            if self.pageBox.parent != None:
                self.controlsBox.remove( self.pageBox )
                self.controlsBox.set_size_request( 545, 378 )
                self.popup.resize( 545, 378 )
            for p in pageIds:
                self.notes[p] = {}
                for t in trackIds:
                    self.notes[p][t] = self.noteDB.getNotesByTrack( p, t )
        else:
            if self.pageBox.parent != None:
                self.controlsBox.remove( self.pageBox )
                self.controlsBox.set_size_request( 545, 378 )
                self.popup.resize( 545, 378 )
            self.notes = notes
            self.pageIds = self.notes.keys()
            self.trackIds = self.notes[self.pageIds[0]].keys()

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
                        self.GUI['cutoffGen'].hide()        
                    else:
                        if n.cs.filterType == 1:
                            self.GUI['filterTypeLowButton'].set_active(True)
                        if n.cs.filterType == 2:
                            self.GUI['filterTypeHighButton'].set_active(True)
                        if n.cs.filterType == 3:
                            self.GUI['filterTypeBandButton'].set_active(True)
                        self.filterLabel.show()
                        self.GUI['cutoffSlider'].show()
                        self.GUI['cutoffGen'].show()
                    self.filterType = n.cs.filterType
                    self.cutoffAdjust.set_value( n.cs.filterCutoff )
                    self.setup = False
                    return

    def handleColor( self, widget, index ):
        stream = []
        for page in self.pageIds:
            stream += [ page, index ]
        if len(stream):
            self.noteDB.updatePages( [ PARAMETER.PAGE_COLOR, len(stream)//2 ] + stream )

    def updateBeatLabel( self, adjust ):
        beats = int(adjust.value)
        self.beatLabel.set_from_file(Config.IMAGE_ROOT + 'propBeats' + str(beats) + '.png')

    def handleBeat( self, widget, signal_id ):
        beats = int(widget.get_adjustment().value)
        stream = []
        for page in self.pageIds:
            stream += [ page, beats ]
        if len(stream):
            self.noteDB.updatePages( [ PARAMETER.PAGE_BEATS, len(stream)//2 ] + stream )


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

    def algoPitch( self, list, algorithm ):
        maxValue = max(list[0], list[1])
        scaleLength = len(self.scale)-1
        stream = []
        for t in range(len(self.trackIds)):
            trackLength = 0
            for p in range(len(self.pageIds)):
                trackLength += len(self.notes[self.pageIds[p]][self.trackIds[t]])
            algorithm.__init__(list[0], list[1], trackLength)
            for p in range(len(self.pageIds)):                
                substream = []
                if self.trackIds[t] != Config.NUMBER_OF_TRACKS-1:
                    for n in self.notes[self.pageIds[p]][self.trackIds[t]]:
                        val = algorithm.getNextValue(list[2], maxValue)
                        substream += [ n.id, self.scale[int(val*0.01*scaleLength)]+36 ]
                    if len(substream):
                        stream += [ self.pageIds[p], self.trackIds[t], PARAMETER.PITCH, len(substream)//2 ] + substream    
                else:
                    for n in self.notes[self.pageIds[p]][self.trackIds[t]]:
                        val = algorithm.getNextValue(list[2], maxValue)
                        val = int((val*0.12)*2+24)
                        if val in GenerationConstants.DRUMPITCH.keys():
                            val = GenerationConstants.DRUMPITCH[val]
                        substream += [ n.id, val ]
                    if len(substream):
                        stream += [ self.pageIds[p], self.trackIds[t], PARAMETER.PITCH, len(substream)//2 ] + substream    
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
            
    def algoVolume( self, list, algorithm ):
        maxValue = max(list[0], list[1])
        stream = []
        for t in range(len(self.trackIds)):
            trackLength = 0
            for p in range(len(self.pageIds)):
                trackLength += len(self.notes[self.pageIds[p]][self.trackIds[t]])
            algorithm.__init__(list[0], list[1], trackLength)
            for p in range(len(self.pageIds)):                
                substream = []
                for n in self.notes[self.pageIds[p]][self.trackIds[t]]:
                    val = algorithm.getNextValue(list[2], maxValue)
                    substream += [ n.id, min( Config.MAXIMUM_AMPLITUDE, val*0.01 ) ]
                if len(substream):
                    stream += [ self.pageIds[p], self.trackIds[t], PARAMETER.AMPLITUDE, len(substream)//2 ] + substream    
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

    def algoPan( self, list, algorithm ):
        maxValue = max(list[0], list[1])
        stream = []
        for t in range(len(self.trackIds)):
            trackLength = 0
            for p in range(len(self.pageIds)):
                trackLength += len(self.notes[self.pageIds[p]][self.trackIds[t]])
            algorithm.__init__(list[0], list[1], trackLength)
            for p in range(len(self.pageIds)):                
                substream = []
                for n in self.notes[self.pageIds[p]][self.trackIds[t]]:
                    val = algorithm.getNextValue(list[2], maxValue)
                    substream += [ n.id, val*0.01 ]
                if len(substream):
                    stream += [ self.pageIds[p], self.trackIds[t], PARAMETER.PAN, len(substream)//2 ] + substream    
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
                
    def algoReverb( self, list, algorithm ):
        maxValue = max(list[0], list[1])
        stream = []
        for t in range(len(self.trackIds)):
            trackLength = 0
            for p in range(len(self.pageIds)):
                trackLength += len(self.notes[self.pageIds[p]][self.trackIds[t]])
            algorithm.__init__(list[0], list[1], trackLength)
            for p in range(len(self.pageIds)):                
                substream = []
                for n in self.notes[self.pageIds[p]][self.trackIds[t]]:
                    val = algorithm.getNextValue(list[2], maxValue)
                    substream += [ n.id, val*0.02 ]
                if len(substream):
                    stream += [ self.pageIds[p], self.trackIds[t], PARAMETER.REVERB, len(substream)//2 ] + substream    
        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )                

    def handleAttack( self, adjust ):
        val = adjust.value #*adjust.value
        img = min( 4, int(val * 4) )
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

    def algoAttack( self, list, algorithm ):
        maxValue = max(list[0], list[1])
        stream = []
        for t in range(len(self.trackIds)):
            trackLength = 0
            for p in range(len(self.pageIds)):
                trackLength += len(self.notes[self.pageIds[p]][self.trackIds[t]])
            algorithm.__init__(list[0], list[1], trackLength)
            for p in range(len(self.pageIds)):                
                substream = []
                for n in self.notes[self.pageIds[p]][self.trackIds[t]]:
                    val = algorithm.getNextValue(list[2], maxValue)
                    substream += [ n.id, val*0.01 ]
                if len(substream):
                    stream += [ self.pageIds[p], self.trackIds[t], PARAMETER.ATTACK, len(substream)//2 ] + substream    
        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )
            
    def handleDecay( self, adjust ):
        val = adjust.value #*adjust.value
        img = min( 4, int(val * 4) )
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

    def algoDecay( self, list, algorithm ):
        maxValue = max(list[0], list[1])
        stream = []
        for t in range(len(self.trackIds)):
            trackLength = 0
            for p in range(len(self.pageIds)):
                trackLength += len(self.notes[self.pageIds[p]][self.trackIds[t]])
            algorithm.__init__(list[0], list[1], trackLength)
            for p in range(len(self.pageIds)):                
                substream = []
                for n in self.notes[self.pageIds[p]][self.trackIds[t]]:
                    val = algorithm.getNextValue(list[2], maxValue)
                    substream += [ n.id, val*0.01 ]
                if len(substream):
                    stream += [ self.pageIds[p], self.trackIds[t], PARAMETER.DECAY, len(substream)//2 ] + substream    
        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )
            
    def handleFilterType( self, widget, type ):

        if widget.get_active():
            if self.filterType == 0:
                self.filterLabel.show()
                self.GUI['cutoffSlider'].show()
                self.GUI['cutoffGen'].show()

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
            self.GUI['cutoffGen'].hide()
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

    def algoCutoff( self, list, algorithm ):
        maxValue = max(list[0], list[1])
        stream = []
        for t in range(len(self.trackIds)):
            trackLength = 0
            for p in range(len(self.pageIds)):
                trackLength += len(self.notes[self.pageIds[p]][self.trackIds[t]])
            algorithm.__init__(list[0], list[1], trackLength)
            for p in range(len(self.pageIds)):                
                substream = []
                for n in self.notes[self.pageIds[p]][self.trackIds[t]]:
                    val = algorithm.getNextValue(list[2], maxValue)
                    substream += [ n.id, val*70+100 ]
                if len(substream):
                    stream += [ self.pageIds[p], self.trackIds[t], PARAMETER.FILTERCUTOFF, len(substream)//2 ] + substream    
        if len(stream):
            self.noteDB.updateNotes( stream + [-1] )

        self.updateFilterLabel()
        
    def handleAlgo( self, widget, data ):
        self.algorithm = self.algoTypes[data]
        paraTooltips = ['Random', 'Maximum step', 'Maximum step', 'Maximum step', 'Maximum step']
        self.tooltips.set_tip(self.GUI['paraSlider'], paraTooltips[data])
        
    def handleMin( self, adjust ):
        self.minValue = adjust.value

    def handleMax( self, adjust ):
        self.maxValue = adjust.value
    
    def handlePara( self, adjust ):
        self.paraValue = adjust.value
    
    def apply( self, widget, data=None ):
        valList = [self.minValue, self.maxValue, self.paraValue]
        if self.property == 'pitch':
            self.algoPitch(valList, self.algorithm)
        elif self.property == 'volume':
            self.algoVolume(valList, self.algorithm)
        elif self.property == 'pan':
            self.algoPan(valList, self.algorithm)
        elif self.property == 'reverb':
            self.algoReverb(valList, self.algorithm)
        elif self.property == 'attack':
            self.algoAttack(valList, self.algorithm)
        elif self.property == 'decay':
            self.algoDecay(valList, self.algorithm)
        elif self.property == 'cutoff':
            self.algoCutoff(valList, self.algorithm)            
        self.cancel(self.activeWidget)
    
    def cancel( self, widget, data=None ):
        self.activeWidget.set_active(False)
   
    def updateFilterLabel( self ):
        val = (self.cutoffAdjust.value-self.cutoffAdjust.lower)/(self.cutoffAdjust.upper-self.cutoffAdjust.lower)
        img = min( 5, int(val * 6) )
        self.filterLabel.set_from_file(Config.IMAGE_ROOT + 'propFilter%d.%d' % (self.filterType, img) + '.png')






