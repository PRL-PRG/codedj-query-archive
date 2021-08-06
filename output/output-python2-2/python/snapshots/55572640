#!/usr/bin/env python

import gtk
import Config
from sugar.graphics.toolbutton import ToolButton
from sugar.graphics.toggletoolbutton import ToggleToolButton
from sugar.graphics.palette import Palette
from sugar.graphics.icon import Icon
from Util.ThemeWidgets import *
from gettext import gettext as _

#Generation palette
from Generation.Generator import GenerationParameters
#Generation palette and Properties palette
from Generation.GenerationConstants import GenerationConstants

#Properties palette
from Util.NoteDB import PARAMETER 
from Generation.Drunk import *
from types import *
from math import sqrt
from random import *

class mainToolbar(gtk.Toolbar):
    def __init__(self,toolbox, edit):
        gtk.Toolbar.__init__(self)

        def _insertSeparator(x = 1):
            for i in range(x):
                self.separator = gtk.SeparatorToolItem()
                self.separator.set_draw(True)
                self.insert(self.separator,-1)
                self.separator.show()   

        self.toolbox = toolbox
        self.edit = edit

        self.tooltips = gtk.Tooltips()

        #Play button
        self._playPalette = playPalette(_('Play / Stop'), self.edit)
        self.playButton = ToggleToolButton('play')
        self.playButton.set_palette(self._playPalette)
        self.playButton.connect('toggled', self.handlePlayStop)
        self.insert(self.playButton, -1)
        self.playButton.show()
        
        #Pause button
        self.pauseButton = ToolButton('pstop')
        self.pauseButton.connect('clicked', self.handlePause)
        self.pauseButton.set_sensitive(False)
        self.insert(self.pauseButton, -1)
        self.pauseButton.show()
        
        #Rewind button
        self.rewindButton = ToolButton('rewind')
        self.rewindButton.connect('clicked', self.edit.handleRewind)
        self.insert(self.rewindButton, -1)
        self.rewindButton.show()
        
        #Record button
        self._recordPalette = recordPalette(_('Record'), self.edit)
        self.recordButton = ToggleToolButton('record')
        self.recordButton.set_palette(self._recordPalette)
        self.recordButton.connect('clicked', self.edit.handleKeyboardRecordButton)
        self.insert(self.recordButton, -1)
        self.recordButton.show()
        
        _insertSeparator(2)
        
        #Pencil button
        self._pencilPalette = pencilPalette(_('Draw Tool'), self.edit, self)
        self.pencilButton = ToggleToolButton('pencil')
        self.pencilButton.set_palette(self._pencilPalette)
        self.pencilButton.connect('toggled', self.handlePencil)
        self.insert(self.pencilButton, -1)
        self.pencilButton.show()
        
        #Duplicate button
        self.duplicateButton = ToggleToolButton('duplicate')
        self.duplicateButton.connect('toggled', self.handleDuplicate)
        self.insert(self.duplicateButton, -1)
        self.duplicateButton.show()
        
        _insertSeparator(8)
        
        #Volume / Tempo button
        self._volumeTempoPalette = volumeTempoPalette(_('Volume / Tempo'), self.edit)
        self.volumeTempoButton = ToggleToolButton('voltemp')
        self.volumeTempoButton.set_palette(self._volumeTempoPalette)
        #self.volumeTempoButton.connect(None)
        self.insert(self.volumeTempoButton, -1)
        self.volumeTempoButton.show()
        
        #Generation button
        self._generationPalette = generationPalette(_('Generation'), self.edit)
        self.generationButton = ToggleToolButton('dice')
        #self.generationButton.connect(None)
        self.generationButton.set_palette(self._generationPalette)
        self.insert(self.generationButton, -1)
        self.generationButton.show()
        
        #Properties button
        self._propertiesPalette = propertiesPalette(_('Properties'), self.edit)
        self.propsButton = ToggleToolButton('props')
        self.propsButton.set_palette(self._propertiesPalette)
        self.insert(self.propsButton, -1)
        self.propsButton.show() 
        
    def handlePlayStop(self, widget, data = None):
        if widget.get_active():
            self.edit.handlePlay(widget)
            self.rewindButton.set_sensitive(False)
            self.pauseButton.set_sensitive(True)
        else:
            self.edit.handleStop(widget)
            self.rewindButton.set_sensitive(True)
            self.pauseButton.set_sensitive(False)
            
    def handlePause(self, widget, data = None):
        self.edit.handleStop(widget, False)
        self.playButton.set_active(False)
        
    def handlePencil(self, widget, data = None):
        if widget.get_active():
            if self._pencilPalette.checkbox.get_active():
                self.edit.handleToolClick2(widget, 'paint')
            else:
                self.edit.handleToolClick2(widget, 'draw')
        else:
            self.edit.handleToolClick2(widget, 'default')
            
    def handleDuplicate(self, widget):
        if widget.get_active():
            if self.edit.getContext() == 0: #Page
                self.edit.pageDuplicate()
            elif self.edit.getContext() == 1: #Track
                self.edit.trackDuplicateWidget(widget)
            elif self.edit.getContext() == 2: #Note
                self.edit.noteDuplicateWidget(widget)
            widget.set_active(False)
        
class playPalette(Palette):
    def __init__(self, label, edit):
        Palette.__init__(self, label)
        
        self.edit = edit
        
class recordPalette(Palette):
    def __init__(self, label, edit):
        Palette.__init__(self, label)
        
        self.edit = edit
        
        self.recordOggButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/record.svg')
        self.recordOggButton.connect('clicked', self.edit.handleAudioRecord)
        self.recordOggButton.show()
        self.set_content(self.recordOggButton)
        
class pencilPalette(Palette):
    def __init__(self, label, edit, _mainToolbar):
        Palette.__init__(self, label)
        
        self.edit = edit
        self._mainToolbar = _mainToolbar
        
        self.pencilBox = gtk.VBox()
        
        self.checkbox = gtk.CheckButton(label = _('Continuous'))
        self.checkbox.connect('toggled',self.handleCheckBox)
        
        self.timeSigHBox = gtk.HBox()
        self.timeSigImage = gtk.Image()
        self.timeSigImage.set_from_file(Config.TAM_TAM_ROOT + '/icons/notedur.svg')
        self.timeSigBox = BigComboBox()
        durs = [_('1/2'), _('1/4'), _('1/8'), _('1/16'), _('1/32')]
        for dur in durs:
            self.timeSigBox.append_item(durs.index(dur),dur)
        self.timeSigBox.set_active(0)
        self.timeSigHBox.pack_start(self.timeSigImage, False, False, padding = 5)
        self.timeSigHBox.pack_start(self.timeSigBox, False, False, padding = 5)
        
        self.pencilBox.pack_start(self.checkbox, False, False, padding = 5)
        self.pencilBox.pack_start(self.timeSigHBox, False, False, padding = 5)
        self.pencilBox.show_all()
        
        self.set_content(self.pencilBox)
    
    def handleCheckBox(self, widget, data = None):
        if widget.get_active():
            if self._mainToolbar.pencilButton.get_active():
                self.edit.handleToolClick2(widget, 'paint')
        else:
            if self._mainToolbar.pencilButton.get_active():
                self.edit.handleToolClick2(widget, 'draw')
            else:
                self.edit.handleToolClick2(widget, 'default')
            
    
class volumeTempoPalette(Palette):
    def __init__(self, label, edit):
        Palette.__init__(self, label)
        
        self.edit = edit
        
        self.volumeTempoBox = gtk.VBox()
        
        self.volumeSliderBox = gtk.HBox()
        self.volumeSliderLabel = gtk.Label(_('Volume'))
        self.volumeSliderAdj = gtk.Adjustment(Config.DEFAULT_VOLUME, 0, 100, 1, 1, 0)
        self.volumeSliderAdj.connect('value-changed', self.edit.handleVolume)
        self.volumeSlider =  gtk.HScale(adjustment = self.volumeSliderAdj)
        self.volumeSlider.set_size_request(250,-1)
        self.volumeSlider.set_inverted(False)
        self.volumeSlider.set_draw_value(False)
        self.volumeSliderBox.pack_start(self.volumeSliderLabel, False, False, padding = 5)
        self.volumeSliderBox.pack_end(self.volumeSlider, False, False, padding = 5)
        
        self.tempoSliderBox = gtk.HBox()
        self.tempoSliderLabel = gtk.Label(_('Tempo'))
        self.tempoSliderAdj = gtk.Adjustment(Config.PLAYER_TEMPO, 40, 240, 1, 1, 0)
        self.tempoSliderAdj.connect('value-changed', self.edit.handleTempo)
        self.tempoSlider =  gtk.HScale(adjustment = self.tempoSliderAdj)
        self.tempoSlider.set_size_request(250,-1)
        self.tempoSlider.set_inverted(False)
        self.tempoSlider.set_draw_value(False)
        self.tempoSliderBox.pack_start(self.tempoSliderLabel, False, False, padding = 5)
        self.tempoSliderBox.pack_end(self.tempoSlider, False, False, padding = 5)
        
        self.volumeTempoBox.pack_start(self.volumeSliderBox, padding = 5)
        self.volumeTempoBox.pack_start(self.tempoSliderBox, padding = 5)
        self.volumeTempoBox.show_all()
        
        self.set_content(self.volumeTempoBox)  

class generationPalette(Palette):
    def __init__(self, label, edit):
        Palette.__init__(self, label)
        
        self.edit = edit
        
        self.rythmDensity = GenerationConstants.DEFAULT_DENSITY
        self.rythmRegularity = GenerationConstants.DEFAULT_RYTHM_REGULARITY
        self.pitchRegularity = GenerationConstants.DEFAULT_PITCH_REGULARITY 
        self.pitchStep = GenerationConstants.DEFAULT_STEP
        self.duration = GenerationConstants.DEFAULT_DURATION
        self.silence = GenerationConstants.DEFAULT_SILENCE
        self.rythmMethod = GenerationConstants.DEFAULT_RYTHM_METHOD
        self.pitchMethod = GenerationConstants.DEFAULT_PITCH_METHOD
        self.pattern = GenerationConstants.DEFAULT_PATTERN   
        self.scale = GenerationConstants.DEFAULT_SCALE
        
        self.mainBox = gtk.VBox()
        self.slidersBox = gtk.HBox()
        self.scaleModeBox = gtk.VBox()
        self.decisionBox = gtk.HBox()
        
        self.XYSliderBox1 = RoundFixed(fillcolor = '#CCCCCC', bordercolor = '#000000')
        self.XYSliderBox1.set_size_request(200,200)
        self.XYButton1 = ImageToggleButton( Config.TAM_TAM_ROOT + '/icons/XYBut.svg', Config.TAM_TAM_ROOT + '/icons/XYButDown.svg')
        self.XAdjustment1 = gtk.Adjustment(self.rythmDensity * 100, 0, 100, 1, 1, 1)
        self.XAdjustment1.connect("value-changed", self.handleXAdjustment1)
        self.YAdjustment1 = gtk.Adjustment(self.rythmRegularity * 100, 0, 100, 1, 1, 1)
        self.YAdjustment1.connect("value-changed", self.handleYAdjustment1)
        self.XYSlider1 = XYSlider( self.XYSliderBox1, self.XYButton1, self.XAdjustment1, self.YAdjustment1, False, True )
        
        self.XYSliderBox2 = RoundFixed(fillcolor = '#CCCCCC', bordercolor = '#000000')
        self.XYSliderBox2.set_size_request(200,200)
        self.XYButton2 = ImageToggleButton( Config.TAM_TAM_ROOT + '/icons/XYBut.svg', Config.TAM_TAM_ROOT + '/icons/XYButDown.svg')
        self.XAdjustment2 = gtk.Adjustment(self.pitchRegularity * 100, 0, 100, 1, 1, 1)
        self.XAdjustment2.connect("value-changed", self.handleXAdjustment2)
        self.YAdjustment2 = gtk.Adjustment(self.pitchStep * 100, 0, 100, 1, 1, 1)
        self.YAdjustment2.connect("value-changed", self.handleYAdjustment2)
        self.XYSlider2 = XYSlider( self.XYSliderBox2, self.XYButton2, self.XAdjustment2, self.YAdjustment2, False, True )
        
        self.XYSliderBox3 = RoundFixed(fillcolor = '#CCCCCC', bordercolor = '#000000')
        self.XYSliderBox3.set_size_request(200,200)
        self.XYButton3 = ImageToggleButton( Config.TAM_TAM_ROOT + '/icons/XYBut.svg', Config.TAM_TAM_ROOT + '/icons/XYButDown.svg')
        self.XAdjustment3 = gtk.Adjustment(self.duration * 100, 0, 100, 1, 1, 1)
        self.XAdjustment3.connect("value-changed", self.handleXAdjustment3)
        self.YAdjustment3 = gtk.Adjustment(self.silence * 100, 0, 100, 1, 1, 1)
        self.YAdjustment3.connect("value-changed", self.handleYAdjustment3)
        self.XYSlider3 = XYSlider( self.XYSliderBox3, self.XYButton3, self.XAdjustment3, self.YAdjustment3, False, True )
        
        self.slidersBox.pack_start(self.XYSlider1, False, False, padding = 5)
        self.slidersBox.pack_start(self.XYSlider2, False, False, padding = 5)
        self.slidersBox.pack_start(self.XYSlider3, False, False, padding = 5)
        
        self.scaleBoxHBox = gtk.HBox()
        self.scaleBoxLabel = gtk.Label(_('Scale: '))
        self.scaleBox = BigComboBox()
        scales = [_('Major scale'), _('Harmonic minor scale'), _('Natural minor scale'), _('Phrygian scale'), _('Dorian scale'), _('Lydian scale'), _('Myxolidian scale')]
        for scale in scales:
            self.scaleBox.append_item(scales.index(scale), scale)
        self.scaleBox.connect('changed', self.handleScale)
        self.scaleBox.set_active(0)
        
        self.modeBoxHBox = gtk.HBox()
        self.modeBoxLabel = gtk.Label(_('Mode: '))
        self.modeBox = BigComboBox()
        modes = [_('Drunk'), _('Drone and Jump'), _('Repeater'), _('Loop segments')]
        for mode in modes:
            self.modeBox.append_item(modes.index(mode), mode)
        self.modeBox.connect('changed', self.handleMode)
        self.modeBox.set_active(0)
        
        self.scaleBoxHBox.pack_start(self.scaleBoxLabel, False, False, padding = 10)
        self.scaleBoxHBox.pack_start(self.scaleBox, False, False, padding = 10)
        self.modeBoxHBox.pack_start(self.modeBoxLabel, False, False, padding = 10)
        self.modeBoxHBox.pack_start(self.modeBox, False, False, padding = 10)
        self.scaleModeBox.pack_start(self.scaleBoxHBox, False, False, padding = 5)
        self.scaleModeBox.pack_start(self.modeBoxHBox, False, False, padding = 5)
        
        self.acceptButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/accept.svg')
#        self.acceptButton = IconButton('stock-accept')
        self.acceptButton.connect('clicked',self.generate)
        self.cancelButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/cancel.svg')
        self.cancelButton.connect('clicked',self.cancel)
#        self.cancelButton = IconButton('activity-stop')
        self.decisionBox.pack_start(self.cancelButton, False, False, padding = 5)
        self.decisionBox.pack_start(self.acceptButton, False, False, padding = 5)
        
        self.mainBox.pack_start(self.slidersBox, False, False, padding = 5)
        self.mainBox.pack_start(self.scaleModeBox, False, False, padding = 5)
        self.mainBox.pack_start(self.decisionBox, False, False, padding = 5)
        self.mainBox.show_all()
 
        
        self.set_content(self.mainBox)
    
    def handleXAdjustment1( self, data ):
        self.rythmDensity = self.XAdjustment1.value * .01

    def handleYAdjustment1( self, data ):
        self.rythmRegularity = self.YAdjustment1.value * .01

    def handleXAdjustment2( self, data ):
        self.pitchRegularity = self.XAdjustment2.value * .01

    def handleYAdjustment2( self, data ):
        self.pitchStep = self.YAdjustment2.value * .01

    def handleXAdjustment3( self, data ):
        self.duration = self.XAdjustment3.value * .01

    def handleYAdjustment3( self, data ):
        self.silence = self.YAdjustment3.value * .01
        
    def getGenerationParameters( self ):
        return GenerationParameters( self.rythmDensity,
                                     self.rythmRegularity,
                                     self.pitchStep,
                                     self.pitchRegularity,
                                     self.duration,
                                     self.silence,
                                     self.rythmMethod,
                                     self.pitchMethod,
                                     self.pattern,
                                     self.scale )
    
    def handleScale(self, widget, data = None):
        self.scale = widget.props.value
        
    def handleMode( self, widget, data = None ):
        self.pattern = widget.props.value
    
    def cancel(self, widget, data = None):
        self.popdown(True)
    
    def generate(self, widget, data=None):
        context = self.edit.getContext()
        if context == 0: # Page
            mode = 'page' 
        elif context == 1: # Track
            mode = 'track'
        elif context == 2: # Note
            self.popdown(True)
            return
        self.edit.setPageGenerateMode(mode)
        self.edit.generate(self.getGenerationParameters())
        self.popdown(True)
        
class propertiesPalette(Palette):
    def __init__(self, label, edit):
        Palette.__init__(self, label)
        self.connect('popup', self.handlePopup)
        self.connect('popdown', self.handlePopdown)
        
        self.edit = edit
        
        self.filterTypes = [_('None'), _('Lowpass'), _('Bandpass'), _('Highpass')]
        self.geneTypes = [_('Line'),_('Drunk'),_('Drone and Jump'),_('Repeater'),_('Loop Segments')]
        self.colors = [_('Purple'), _('Green'), _('Blue'), _('Yellow')]
        self.currentFilterType = self.filterTypes[0]
        self.currentGeneType = self.geneTypes[0]
        
        self.setup = False
        self.geneCheckButtonDic = {}
        
        self.pageIds = []
        self.context = "page"
        
        self.mainBox = gtk.VBox()

        self.gridDivisionBox = gtk.HBox()
        self.gridDivisionLabel = gtk.Label(_('Grid division: '))
        self.gridDivisionSliderAdj = gtk.Adjustment(4, 2, 12, 1, 1, 0)
        self.gridDivisionSlider =  gtk.HScale(adjustment = self.gridDivisionSliderAdj)
        self.gridDivisionSlider.set_digits(0)
        self.gridDivisionSlider.connect('button-release-event', self.handleBeat)
        self.gridDivisionSlider.set_size_request(200,-1)
        self.gridDivisionSlider.set_value_pos(gtk.POS_RIGHT)
        self.gridDivisionBox.pack_start(self.gridDivisionLabel, False, False, padding = 5)
        self.gridDivisionBox.pack_end(self.gridDivisionSlider, False, False, padding = 52)
        
        self.pageColorBox = gtk.HBox()
        self.pageColorLabel = gtk.Label(_('Page color: '))
        self.pageColorComboBox = BigComboBox()
        for color in self.colors:
            self.pageColorComboBox.append_item(self.colors.index(color), color)
        self.pageColorComboBox.set_active(0)
        self.pageColorComboBox.connect('changed', self.handleColor)
        self.pageColorBox.pack_start(self.pageColorLabel, False, False, padding = 5)
        self.pageColorBox.pack_end(self.pageColorComboBox, False, False, padding = 55)
        
        self.transposeBox = gtk.HBox()
        self.transposeLabel = gtk.Label(_('Transposition: '))
        self.transposeDownButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/arrow-down.svg')
        self.transposeDownButton.connect('clicked', self.stepPitch, -1)
        self.transposeUpButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/arrow-up.svg')
        self.transposeUpButton.connect('clicked', self.stepPitch, 1)
        self.transposeCheckButton = gtk.CheckButton()
        self.transposeCheckButton.connect('toggled', self.handleGeneCheckButton)
        self.geneCheckButtonDic['transpose'] = self.transposeCheckButton
        self.transposeBox.pack_start(self.transposeLabel, False, False, padding = 5)
        self.transposeBox.pack_end(self.transposeCheckButton, False, False, padding = 5)
        self.transposeBox.pack_end(self.transposeUpButton, False, False, padding = 50)
        self.transposeBox.pack_end(self.transposeDownButton, False, False, padding = 5)
        
        self.volumeBox = gtk.HBox()
        self.volumeLabel = gtk.Label(_('Volume: '))
        self.volumeDownButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/arrow-down.svg')
        self.volumeDownButton.connect('clicked', self.stepVolume, -0.1)
        self.volumeUpButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/arrow-up.svg')
        self.volumeUpButton.connect('clicked', self.stepVolume, 0.1)
        self.volumeCheckButton = gtk.CheckButton()
        self.volumeCheckButton.connect('toggled', self.handleGeneCheckButton)
        self.geneCheckButtonDic['volume'] = self.volumeCheckButton
        self.volumeBox.pack_start(self.volumeLabel, False, False, padding = 5)
        self.volumeBox.pack_end(self.volumeCheckButton, False, False, padding = 5)
        self.volumeBox.pack_end(self.volumeUpButton, False, False, padding = 50)
        self.volumeBox.pack_end(self.volumeDownButton, False, False, padding = 5)
        
        self.panBox = gtk.HBox()
        self.panLabel = gtk.Label(_('Pan: '))
        self.panSliderAdj = gtk.Adjustment(0.5, 0, 1, .1, .1, 0)
        self.panSliderAdj.connect('value-changed', self.handlePan)
        self.panSlider =  gtk.HScale(adjustment = self.panSliderAdj)
        self.panSlider.set_size_request(200,-1)
        self.panSlider.set_value_pos(gtk.POS_RIGHT)
        self.panCheckButton = gtk.CheckButton()
        self.panCheckButton.connect('toggled', self.handleGeneCheckButton)
        self.geneCheckButtonDic['pan'] = self.panCheckButton
        self.panBox.pack_start(self.panLabel, False, False, padding = 5)
        self.panBox.pack_end(self.panCheckButton, False, False, padding = 5)
        self.panBox.pack_end(self.panSlider, False, False, padding = 5)
        
        self.reverbBox = gtk.HBox()
        self.reverbLabel = gtk.Label(_('Reverb: '))
        self.reverbSliderAdj = gtk.Adjustment(0.1, 0, 1, 0.1, 0.1, 0)
        self.reverbSliderAdj.connect("value-changed", self.handleReverb)
        self.reverbSlider =  gtk.HScale(adjustment = self.reverbSliderAdj)
        self.reverbSlider.set_size_request(200,-1)
        self.reverbSlider.set_value_pos(gtk.POS_RIGHT)
        self.reverbCheckButton = gtk.CheckButton()
        self.reverbCheckButton.connect('toggled', self.handleGeneCheckButton)
        self.geneCheckButtonDic['reverb'] = self.reverbCheckButton
        self.reverbBox.pack_start(self.reverbLabel, False, False, padding = 5)
        self.reverbBox.pack_end(self.reverbCheckButton, False, False, padding = 5)
        self.reverbBox.pack_end(self.reverbSlider, False, False, padding = 5)
        
        self.attackDurBox = gtk.HBox()
        self.attackDurLabel = gtk.Label(_('Attack duration: '))
        self.attackDurSliderAdj = gtk.Adjustment(0.04, 0.03, 1, .01, .01, 0)
        self.attackDurSliderAdj.connect('value-changed', self.handleAttack)
        self.attackDurSlider =  gtk.HScale(adjustment = self.attackDurSliderAdj)
        self.attackDurSlider.set_size_request(200,-1)
        self.attackDurSlider.set_value_pos(gtk.POS_RIGHT)
        self.attackDurCheckButton = gtk.CheckButton()
        self.attackDurCheckButton.connect('toggled', self.handleGeneCheckButton)
        self.geneCheckButtonDic['attack'] = self.attackDurCheckButton
        self.attackDurBox.pack_start(self.attackDurLabel, False, False, padding = 5)
        self.attackDurBox.pack_end(self.attackDurCheckButton, False, False, padding = 5)
        self.attackDurBox.pack_end(self.attackDurSlider, False, False, padding = 5)
        
        self.decayDurBox = gtk.HBox()
        self.decayDurLabel = gtk.Label(_('Decay duration: '))
        self.decayDurSliderAdj = gtk.Adjustment(0.31, 0.03, 1, .01, .01, 0)
        self.decayDurSliderAdj.connect('value-changed', self.handleDecay)
        self.decayDurSlider =  gtk.HScale(adjustment = self.decayDurSliderAdj)
        self.decayDurSlider.set_size_request(200,-1)
        self.decayDurSlider.set_value_pos(gtk.POS_RIGHT)
        self.decayDurCheckButton = gtk.CheckButton()
        self.decayDurCheckButton.connect('toggled', self.handleGeneCheckButton)
        self.geneCheckButtonDic['decay'] = self.decayDurCheckButton
        self.decayDurBox.pack_start(self.decayDurLabel, False, False, padding = 5)
        self.decayDurBox.pack_end(self.decayDurCheckButton, False, False, padding = 5)
        self.decayDurBox.pack_end(self.decayDurSlider, False, False, padding = 5)
        
        self.filterTypeBox = gtk.HBox()
        self.filterTypeLabel = gtk.Label(_('Filter Type: '))
        self.filterTypeComboBox = BigComboBox()
        for filtertype in self.filterTypes:
            self.filterTypeComboBox.append_item(self.filterTypes.index(filtertype), filtertype, Config.TAM_TAM_ROOT + '/icons/test.svg', (30,30))
        self.filterTypeComboBox.connect('changed', self.handleFilterTypes)
        self.filterTypeBox.pack_start(self.filterTypeLabel, False, False, padding = 5)
        self.filterTypeBox.pack_end(self.filterTypeComboBox, False, False, padding = 55)
        
        self.filterCutoffBox = gtk.HBox()
        self.filterCutoffLabel = gtk.Label(_('Filter cutoff: '))
        self.filterCutoffSliderAdj = gtk.Adjustment(1000, 100, 7000, 100, 100, 0)
        self.filterCutoffSliderAdj.connect('value-changed', self.handleFilter)
        self.filterCutoffSlider =  gtk.HScale(adjustment = self.filterCutoffSliderAdj)
        self.filterCutoffSlider.set_size_request(200,-1)
        self.filterCutoffSlider.set_value_pos(gtk.POS_RIGHT)
        self.filterCutoffCheckButton = gtk.CheckButton()
        self.filterCutoffCheckButton.connect('toggled', self.handleGeneCheckButton)
        self.geneCheckButtonDic['filter'] = self.filterCutoffCheckButton
        self.filterCutoffBox.pack_start(self.filterCutoffLabel, False, False, padding = 5)
        self.filterCutoffBox.pack_end(self.filterCutoffCheckButton, False, False, padding = 5)
        self.filterCutoffBox.pack_end(self.filterCutoffSlider, False, False, padding = 5)
        
        self.generationMainBox = gtk.VBox()
        self.generationLabel = gtk.Label(_('Generation'))
        
        self.generationTypeBox = gtk.HBox()
        self.generationTypeLabel = gtk.Label(_('Type: '))
        self.generationTypeComboBox = BigComboBox()
        for genetype in self.geneTypes:
            self.generationTypeComboBox.append_item(self.geneTypes.index(genetype), genetype, Config.TAM_TAM_ROOT + '/icons/test.svg', (30,30))
        self.generationTypeComboBox.connect('changed', self.handleGeneTypes)
        self.generationTypeComboBox.set_active(0)
        self.generationTypeBox.pack_start(self.generationTypeLabel, False, False, padding = 5)
        self.generationTypeBox.pack_end(self.generationTypeComboBox, False, False, padding = 55)
        
        self.minimumBox = gtk.HBox()
        self.minimumLabel = gtk.Label(_('Minimum: '))
        self.minimumSliderAdj = gtk.Adjustment(0, 0, 100, 1, 1, 0)
        self.minimumSlider =  gtk.HScale(adjustment = self.minimumSliderAdj)
        self.minimumSlider.set_size_request(200,-1)
        self.minimumSlider.set_value_pos(gtk.POS_RIGHT)
        self.minimumBox.pack_start(self.minimumLabel, False, False, padding = 5)
        self.minimumBox.pack_end(self.minimumSlider, False, False, padding = 52)
        
        self.maximumBox = gtk.HBox()
        self.maximumLabel = gtk.Label(_('Maximum: '))
        self.maximumSliderAdj = gtk.Adjustment(100, 0, 100, 1, 1, 0)
        self.maximumSlider =  gtk.HScale(adjustment = self.maximumSliderAdj)
        self.maximumSlider.set_size_request(200,-1)
        self.maximumSlider.set_value_pos(gtk.POS_RIGHT)
        self.maximumBox.pack_start(self.maximumLabel, False, False, padding = 5)
        self.maximumBox.pack_end(self.maximumSlider, False, False, padding = 52)
        
        self.randomBox = gtk.HBox()
        self.randomLabel = gtk.Label(_('Random: '))
        self.randomSliderAdj = gtk.Adjustment(20, 0, 100, 1, 1, 0)
        self.randomSlider =  gtk.HScale(adjustment = self.randomSliderAdj)
        self.randomSlider.set_size_request(200,-1)
        self.randomSlider.set_value_pos(gtk.POS_RIGHT)
        self.randomBox.pack_start(self.randomLabel, False, False, padding = 5)
        self.randomBox.pack_end(self.randomSlider, False, False, padding = 52)
        
        self.decisionBox = gtk.HBox()
        self.acceptButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/accept.svg')
        self.cancelButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/cancel.svg')
        self.decisionBox.pack_start(self.cancelButton, False, False, padding = 5)
        self.decisionBox.pack_start(self.acceptButton, False, False, padding = 5)
        
        self.mainBox.pack_start(self.gridDivisionBox, padding = 5)
        self.mainBox.pack_start(self.pageColorBox, padding = 5)
        self.mainBox.pack_start(self.transposeBox, padding = 5)
        self.mainBox.pack_start(self.volumeBox, padding = 5)
        self.mainBox.pack_start(self.panBox, padding = 5)
        self.mainBox.pack_start(self.reverbBox, padding = 5)
        self.mainBox.pack_start(self.attackDurBox, padding = 5)
        self.mainBox.pack_start(self.decayDurBox, padding = 5)
        self.mainBox.pack_start(self.filterTypeBox, padding = 5)
        self.mainBox.pack_start(self.filterCutoffBox, padding = 5)
        self.generationMainBox.pack_start(self.generationLabel, padding = 15)
        self.generationMainBox.pack_start(self.generationTypeBox, padding = 5)
        self.generationMainBox.pack_start(self.minimumBox, padding = 5)
        self.generationMainBox.pack_start(self.maximumBox, padding = 5)
        self.generationMainBox.pack_start(self.randomBox, padding = 5)
        self.generationMainBox.pack_start(self.decisionBox, padding = 5)
        self.mainBox.pack_start(self.generationMainBox, padding = 5)
        self.mainBox.show_all()
        
        self.generationMainBox.hide()
        
        self.set_content(self.mainBox)
    
    def handlePopup(self, widget, data = None):
        if self.edit.getContext() == 0: #Page
            self.setContext('page', self.edit._mainToolbar._generationPalette.scale, self.edit.tuneInterface.getSelectedIds())
        elif self.edit.getContext() == 1: #Track
            self.setContext('track', self.edit._mainToolbar._generationPalette.scale, self.edit.tuneInterface.getSelectedIds(), [ i for i in range(Config.NUMBER_OF_TRACKS) if self.edit.trackSelected[i] ])
        elif self.edit.getContext() == 2: #Note
            ids = self.edit.trackInterface.getSelectedNotes()
            notes = { self.edit.displayedPage: {} }
            for t in range(Config.NUMBER_OF_TRACKS):
                if len(ids[t]):
                    notes[self.edit.displayedPage][t] = [ self.edit.noteDB.getNote( self.edit.displayedPage, t, id ) for id in ids[t] ]
            self.setContext('note', self.edit._mainToolbar._generationPalette.scale, notes = notes)

    def handlePopdown(self, widget, data = None):
        self.resetGeneCheckButton()      
    
    def setContext( self, context, scale, pageIds = None, trackIds = None, notes = {} ):
        self.context = context
        self.scale = GenerationConstants.SCALES[scale]
        self.notes = {}
        self.pageIds = pageIds
        self.trackIds = trackIds
            
        if context == "page":
            self.trackIds = [0,1,2,3,4]
            for p in pageIds:
                self.notes[p] = {}
                for t in range(Config.NUMBER_OF_TRACKS):
                    self.notes[p][t] = self.edit.noteDB.getNotesByTrack( p, t )
            page = self.edit.noteDB.getPage(pageIds[0])
            self.gridDivisionSliderAdj.set_value(page.beats)
        elif context == "track":
            for p in pageIds:
                self.notes[p] = {}
                for t in trackIds:
                    self.notes[p][t] = self.edit.noteDB.getNotesByTrack( p, t )
        else:
            self.notes = notes
            self.pageIds = self.notes.keys()
            self.trackIds = self.notes[self.pageIds[0]].keys()

        for p in self.notes: 
            for t in self.notes[p]:
                if len(self.notes[p][t]):
                    # initialize values from first note
                    self.setup = True
                    n = self.notes[p][t][0]
                    self.panSliderAdj.set_value( n.cs.pan )
                    self.reverbSliderAdj.set_value( n.cs.reverbSend )
                    self.attackDurSliderAdj.set_value( n.cs.attack )
                    self.decayDurSliderAdj.set_value( n.cs.decay )
                    self.filterTypeComboBox.set_active(n.cs.filterType) 
                    self.currentFilterType = n.cs.filterType
                    self.filterCutoffSliderAdj.set_value( n.cs.filterCutoff )
                    self.setup = False
                    
    def resetGeneCheckButton(self):
        for key in self.geneCheckButtonDic:
            self.geneCheckButtonDic[key].set_active(False)
                    
    def handleGeneCheckButton(self, widget, data = None):
        hidden = True
        if widget.get_active():
            self.generationMainBox.show()
        else:
            for key in self.geneCheckButtonDic:
                if self.geneCheckButtonDic[key].get_active():
                    hidden = False
            if hidden:
                self.generationMainBox.hide()
                
        
    def handleBeat(self, widget, signal_id):
        beats = int(widget.get_adjustment().value)
        stream = []
        for page in self.pageIds:
            stream += [ page, beats ]
        if len(stream):
            self.edit.noteDB.updatePages( [ PARAMETER.PAGE_BEATS, len(stream)//2 ] + stream )
            
    def handleColor(self, widget):
        index = widget.props.value
        stream = []
        for page in self.pageIds:
            stream += [ page, index ]
        if len(stream):
            self.edit.noteDB.updatePages( [ PARAMETER.PAGE_COLOR, len(stream)//2 ] + stream )
            
    def stepPitch(self, widget, step):
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
            self.edit.noteDB.updateNotes( stream + [-1] )
            
    def stepVolume(self, widget, step):
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
            self.edit.noteDB.updateNotes( stream + [-1] )
            
    def handlePan(self, adjust):
        if not self.setup:
            stream = []
            for p in self.notes:
                for t in self.notes[p]:
                    if len(self.notes[p][t]):
                        stream += [ p, t, PARAMETER.PAN, len(self.notes[p][t]) ]
                        for n in self.notes[p][t]:
                            stream += [ n.id, adjust.value ]
            if len(stream):
                self.edit.noteDB.updateNotes( stream + [-1] )
            
    def handleReverb(self, adjust):
        if not self.setup:
            stream = []
            for p in self.notes:
                for t in self.notes[p]:
                    if len(self.notes[p][t]):
                        stream += [ p, t, PARAMETER.REVERB, len(self.notes[p][t]) ]
                        for n in self.notes[p][t]:
                            stream += [ n.id, adjust.value ]
            if len(stream):
                self.edit.noteDB.updateNotes( stream + [-1] )
                
    def handleAttack(self, adjust):
        if not self.setup:
            stream = []
            for p in self.notes:
                for t in self.notes[p]:
                    if len(self.notes[p][t]):
                        stream += [ p, t, PARAMETER.ATTACK, len(self.notes[p][t]) ]
                        for n in self.notes[p][t]:
                            stream += [ n.id, adjust.value ]
            if len(stream):
                self.edit.noteDB.updateNotes( stream + [-1] )
                
    def handleDecay(self, adjust):
        if not self.setup:
            stream = []
            for p in self.notes:
                for t in self.notes[p]:
                    if len(self.notes[p][t]):
                        stream += [ p, t, PARAMETER.DECAY, len(self.notes[p][t]) ]
                        for n in self.notes[p][t]:
                            stream += [ n.id, adjust.value ]
            if len(stream):
                self.edit.noteDB.updateNotes( stream + [-1] )

                
    def handleFilterTypes(self, widget):
        self.currentFilterType = widget.props.value
        
        if not self.currentFilterType:
            self.filterCutoffSlider.set_sensitive(False)
        else:
            self.filterCutoffSlider.set_sensitive(True)
            
        if not self.setup:
            if self.currentFilterType:
                typestream = []
                cutoffstream = []
                cutoff = self.filterCutoffSliderAdj.value
                for p in self.notes:
                    for t in self.notes[p]:
                        if len(self.notes[p][t]):
                            substream = []
                            typestream += [ p, t, PARAMETER.FILTERTYPE, len(self.notes[p][t]) ]
                            for n in self.notes[p][t]:
                                typestream += [ n.id, self.currentFilterType ]
                                if n.cs.filterCutoff != cutoff:
                                    substream += [ n.id, cutoff ]
                            if len(substream):
                                cutoffstream += [ p, t, PARAMETER.FILTERCUTOFF, len(substream)//2 ] + substream
                if len(typestream):
                    self.edit.noteDB.updateNotes( typestream + [-1] )
                if len(cutoffstream):
                    self.edit.noteDB.updateNotes( cutoffstream + [-1] )
            else:
                self.currentFilterType = 0
                typestream = []
                for p in self.notes:
                    for t in self.notes[p]:
                        if len(self.notes[p][t]):
                            typestream += [ p, t, PARAMETER.FILTERTYPE, len(self.notes[p][t]) ]
                            for n in self.notes[p][t]:
                                typestream += [ n.id, 0 ]
                if len(typestream):
                    self.edit.noteDB.updateNotes( typestream + [-1] )
    
    def handleGeneTypes(self, widget):
        self.currentGeneType = widget.props.value
        
    def handleFilter(self, adjust):
        stream = []
        for p in self.notes:
            for t in self.notes[p]:
                if len(self.notes[p][t]):
                    stream += [ p, t, PARAMETER.FILTERCUTOFF, len(self.notes[p][t]) ]
                    for n in self.notes[p][t]:
                        stream += [ n.id, adjust.value ]
        if len(stream):
            self.edit.noteDB.updateNotes( stream + [-1] )

