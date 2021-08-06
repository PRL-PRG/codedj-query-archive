#!/usr/bin/env python

import gtk
import Config

from sugar.graphics.toolbutton import ToolButton
from sugar.graphics.toggletoolbutton import ToggleToolButton
from sugar.graphics.palette import Palette
from sugar.graphics.icon import Icon
from Util.ThemeWidgets import *
from Generation.Generator import GenerationParameters
from Generation.GenerationConstants import GenerationConstants
from gettext import gettext as _

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
        self.pauseButton.connect('clicked', self.handlePause, False)
        self.pauseButton.set_sensitive(False)
        self.insert(self.pauseButton, -1)
        self.pauseButton.show()
        
        #Rewind button
        self.rewindButton = ToolButton('rewind')
        self.rewindButton.connect('clicked', self.edit.handleRewind)
        self.insert(self.rewindButton, -1)
        self.rewindButton.show()
        
        #Record button
        self.recordButton = ToggleToolButton('record')
        #self.recordButton.connect(None)
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
        
        _insertSeparator(4)
        
        #Generation button
        self._generationPalette = generationPalette(_('Generation'), self.edit)
        self.generationButton = ToggleToolButton('dice')
        #self.generationButton.connect(None)
        self.generationButton.set_palette(self._generationPalette)
        self.insert(self.generationButton, -1)
        self.generationButton.show()
        
        #Properties button
        self._propsPalette = propsPalette(_('Properties'), self.edit)
        self.propsButton = ToggleToolButton('props')
        self.propsButton.set_palette(self._propsPalette)
        #self.propsButton.connect(None)
        self.insert(self.propsButton, -1)
        self.propsButton.show() 
        
        #Duplicate button
        self.duplicateButton = ToggleToolButton('duplicate')
        self.duplicateButton.connect('toggled', self.handleDuplicate)
        self.insert(self.duplicateButton, -1)
        self.duplicateButton.show()
        
        #Volume / Tempo button
        self._volumeTempoPalette = volumeTempoPalette(_('Volume / Tempo'), self.edit)
        self.volumeTempoButton = ToggleToolButton('voltemp')
        self.volumeTempoButton.set_palette(self._volumeTempoPalette)
        #self.volumeTempoButton.connect(None)
        self.insert(self.volumeTempoButton, -1)
        self.volumeTempoButton.show()
        
    def handlePlayStop(self, widget, data = None):
        if widget.get_active():
            self.edit.handlePlay(widget)
            self.rewindButton.set_sensitive(False)
            self.pauseButton.set_sensitive(True)
        else:
            self.edit.handleStop(widget)
            self.rewindButton.set_sensitive(True)
            self.pauseButton.set_sensitive(False)
            
    def handlePause(self, widget, data):
        self.edit.handleStop(widget, data)
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
        self.timeSigBox = gtk.combo_box_new_text()
        self.timeSigBox.append_text(_('1/2'))
        self.timeSigBox.append_text(_('1/4'))
        self.timeSigBox.append_text(_('1/8'))
        self.timeSigBox.append_text(_('1/16'))
        self.timeSigBox.append_text(_('1/32'))
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
        self.volumeSliderAdj = gtk.Adjustment(value=0, lower=0, upper=1, step_incr=0.1, page_incr=0, page_size=0)
        self.volumeSlider =  gtk.HScale(adjustment = self.volumeSliderAdj)
        self.volumeSlider.set_size_request(250,-1)
        self.volumeSlider.set_inverted(False)
        self.volumeSlider.set_draw_value(False)
        self.volumeSliderBox.pack_start(self.volumeSliderLabel, False, False, padding = 5)
        self.volumeSliderBox.pack_end(self.volumeSlider, False, False, padding = 5)
        
        self.tempoSliderBox = gtk.HBox()
        self.tempoSliderLabel = gtk.Label(_('Tempo'))
        self.tempoSliderAdj = gtk.Adjustment(value=0, lower=0, upper=1, step_incr=0.1, page_incr=0, page_size=0)
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
        
class propsPalette(Palette):
    def __init__(self, label, edit):
        Palette.__init__(self, label)
        
        self.edit = edit
        
        self.mainBox = gtk.VBox()

        self.gridDivisionBox = gtk.HBox()
        self.gridDivisionLabel = gtk.Label(_('Grid division: '))
        self.gridDivisionSliderAdj = gtk.Adjustment(value=0, lower=0, upper=16, step_incr=1, page_incr=0, page_size=0)
        self.gridDivisionSlider =  gtk.HScale(adjustment = self.gridDivisionSliderAdj)
        self.gridDivisionSlider.set_size_request(200,-1)
        self.gridDivisionSlider.set_value_pos(gtk.POS_RIGHT)
        self.gridDivisionBox.pack_start(self.gridDivisionLabel, False, False, padding = 5)
        self.gridDivisionBox.pack_end(self.gridDivisionSlider, False, False, padding = 52)
        
        self.pageColorBox = gtk.HBox()
        self.pageColorLabel = gtk.Label(_('Page color: '))
        self.pageColorBox.pack_start(self.pageColorLabel, False, False, padding = 5)
        
        self.transposeBox = gtk.HBox()
        self.transposeLabel = gtk.Label(_('Transposition: '))
        self.transposeDownButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/arrow-down.svg')
        self.transposeUpButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/arrow-up.svg')
        self.transposeCheckButton = gtk.CheckButton()
        self.transposeBox.pack_start(self.transposeLabel, False, False, padding = 5)
        self.transposeBox.pack_end(self.transposeCheckButton, False, False, padding = 5)
        self.transposeBox.pack_end(self.transposeUpButton, False, False, padding = 50)
        self.transposeBox.pack_end(self.transposeDownButton, False, False, padding = 5)
        
        self.volumeBox = gtk.HBox()
        self.volumeLabel = gtk.Label(_('Volume: '))
        self.volumeDownButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/arrow-down.svg')
        self.volumeUpButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/arrow-up.svg')
        self.volumeCheckButton = gtk.CheckButton()
        self.volumeBox.pack_start(self.volumeLabel, False, False, padding = 5)
        self.volumeBox.pack_end(self.volumeCheckButton, False, False, padding = 5)
        self.volumeBox.pack_end(self.volumeUpButton, False, False, padding = 50)
        self.volumeBox.pack_end(self.volumeDownButton, False, False, padding = 5)
        
        self.panBox = gtk.HBox()
        self.panLabel = gtk.Label(_('Pan: '))
        self.panSliderAdj = gtk.Adjustment(value=50, lower=0, upper=100, step_incr=1, page_incr=0, page_size=0)
        self.panSlider =  gtk.HScale(adjustment = self.panSliderAdj)
        self.panSlider.set_size_request(200,-1)
        self.panSlider.set_value_pos(gtk.POS_RIGHT)
        self.panCheckButton = gtk.CheckButton()
        self.panBox.pack_start(self.panLabel, False, False, padding = 5)
        self.panBox.pack_end(self.panCheckButton, False, False, padding = 5)
        self.panBox.pack_end(self.panSlider, False, False, padding = 5)
        
        self.reverbBox = gtk.HBox()
        self.reverbLabel = gtk.Label(_('Reverb: '))
        self.reverbSliderAdj = gtk.Adjustment(value=0, lower=0, upper=16, step_incr=1, page_incr=0, page_size=0)
        self.reverbSlider =  gtk.HScale(adjustment = self.reverbSliderAdj)
        self.reverbSlider.set_size_request(200,-1)
        self.reverbSlider.set_value_pos(gtk.POS_RIGHT)
        self.reverbCheckButton = gtk.CheckButton()
        self.reverbBox.pack_start(self.reverbLabel, False, False, padding = 5)
        self.reverbBox.pack_end(self.reverbCheckButton, False, False, padding = 5)
        self.reverbBox.pack_end(self.reverbSlider, False, False, padding = 5)
        
        self.attackDurBox = gtk.HBox()
        self.attackDurLabel = gtk.Label(_('Attack duration: '))
        self.attackDurSliderAdj = gtk.Adjustment(value=0, lower=0, upper=16, step_incr=1, page_incr=0, page_size=0)
        self.attackDurSlider =  gtk.HScale(adjustment = self.attackDurSliderAdj)
        self.attackDurSlider.set_size_request(200,-1)
        self.attackDurSlider.set_value_pos(gtk.POS_RIGHT)
        self.attackDurCheckButton = gtk.CheckButton()
        self.attackDurBox.pack_start(self.attackDurLabel, False, False, padding = 5)
        self.attackDurBox.pack_end(self.attackDurCheckButton, False, False, padding = 5)
        self.attackDurBox.pack_end(self.attackDurSlider, False, False, padding = 5)
        
        self.decayDurBox = gtk.HBox()
        self.decayDurLabel = gtk.Label(_('Decay duration: '))
        self.decayDurSliderAdj = gtk.Adjustment(value=0, lower=0, upper=16, step_incr=1, page_incr=0, page_size=0)
        self.decayDurSlider =  gtk.HScale(adjustment = self.decayDurSliderAdj)
        self.decayDurSlider.set_size_request(200,-1)
        self.decayDurSlider.set_value_pos(gtk.POS_RIGHT)
        self.decayDurCheckButton = gtk.CheckButton()
        self.decayDurBox.pack_start(self.decayDurLabel, False, False, padding = 5)
        self.decayDurBox.pack_end(self.decayDurCheckButton, False, False, padding = 5)
        self.decayDurBox.pack_end(self.decayDurSlider, False, False, padding = 5)
        
        self.filterCutoffBox = gtk.HBox()
        self.filterCutoffLabel = gtk.Label(_('Filter cutoff: '))
        self.filterCutoffSliderAdj = gtk.Adjustment(value=0, lower=0, upper=16, step_incr=1, page_incr=0, page_size=0)
        self.filterCutoffSlider =  gtk.HScale(adjustment = self.filterCutoffSliderAdj)
        self.filterCutoffSlider.set_size_request(200,-1)
        self.filterCutoffSlider.set_value_pos(gtk.POS_RIGHT)
        self.filterCutoffCheckButton = gtk.CheckButton()
        self.filterCutoffBox.pack_start(self.filterCutoffLabel, False, False, padding = 5)
        self.filterCutoffBox.pack_end(self.filterCutoffCheckButton, False, False, padding = 5)
        self.filterCutoffBox.pack_end(self.filterCutoffSlider, False, False, padding = 5)
        
        self.filterTypeBox = gtk.HBox()
        self.filterTypeLabel = gtk.Label(_('Filter Type: '))
        self.filterTypeComboBox = BigComboBox()
        for type in [_('Lowpass'),_('Bandpass'),_('Highpass')]:
            self.filterTypeComboBox.append_item(0, type)
        self.filterTypeComboBox.set_active(0)
        self.filterTypeBox.pack_start(self.filterTypeLabel, False, False, padding = 5)
        self.filterTypeBox.pack_end(self.filterTypeComboBox, False, False, padding = 55)
        
        self.generationLabel = gtk.Label(_('Generation'))
        
        self.generationTypeBox = gtk.HBox()
        self.generationTypeLabel = gtk.Label(_('Type: '))
        self.generationTypeComboBox = BigComboBox()
        for type in [_('Line'),_('Drunk'),_('Drone and Jump'),_('Repeater'),_('Loop Segments')]:
            self.generationTypeComboBox.append_item(0, type)
        self.generationTypeComboBox.set_active(0)
        self.generationTypeBox.pack_start(self.generationTypeLabel, False, False, padding = 5)
        self.generationTypeBox.pack_end(self.generationTypeComboBox, False, False, padding = 55)
        
        self.minimumBox = gtk.HBox()
        self.minimumLabel = gtk.Label(_('Minimum: '))
        self.minimumSliderAdj = gtk.Adjustment(value=0, lower=0, upper=16, step_incr=1, page_incr=0, page_size=0)
        self.minimumSlider =  gtk.HScale(adjustment = self.minimumSliderAdj)
        self.minimumSlider.set_size_request(200,-1)
        self.minimumSlider.set_value_pos(gtk.POS_RIGHT)
        self.minimumBox.pack_start(self.minimumLabel, False, False, padding = 5)
        self.minimumBox.pack_end(self.minimumSlider, False, False, padding = 52)
        
        self.maximumBox = gtk.HBox()
        self.maximumLabel = gtk.Label(_('Maximum: '))
        self.maximumSliderAdj = gtk.Adjustment(value=0, lower=0, upper=16, step_incr=1, page_incr=0, page_size=0)
        self.maximumSlider =  gtk.HScale(adjustment = self.maximumSliderAdj)
        self.maximumSlider.set_size_request(200,-1)
        self.maximumSlider.set_value_pos(gtk.POS_RIGHT)
        self.maximumBox.pack_start(self.maximumLabel, False, False, padding = 5)
        self.maximumBox.pack_end(self.maximumSlider, False, False, padding = 52)
        
        self.randomBox = gtk.HBox()
        self.randomLabel = gtk.Label(_('Random: '))
        self.randomSliderAdj = gtk.Adjustment(value=0, lower=0, upper=16, step_incr=1, page_incr=0, page_size=0)
        self.randomSlider =  gtk.HScale(adjustment = self.randomSliderAdj)
        self.randomSlider.set_size_request(200,-1)
        self.randomSlider.set_value_pos(gtk.POS_RIGHT)
        self.randomBox.pack_start(self.randomLabel, False, False, padding = 5)
        self.randomBox.pack_end(self.randomSlider, False, False, padding = 52)
        
        self.decisionBox = gtk.HBox()
        self.acceptButton = Icon('stock-accept')
        self.cancelButton = Icon('activity-stop')
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
        self.mainBox.pack_start(self.filterCutoffBox, padding = 5)
        self.mainBox.pack_start(self.filterTypeBox, padding = 5)
        self.mainBox.pack_start(self.generationLabel, padding = 15)
        self.mainBox.pack_start(self.generationTypeBox, padding = 5)
        self.mainBox.pack_start(self.minimumBox, padding = 5)
        self.mainBox.pack_start(self.maximumBox, padding = 5)
        self.mainBox.pack_start(self.randomBox, padding = 5)
        self.mainBox.pack_start(self.decisionBox, padding = 5)
        self.mainBox.show_all()
        
        self.set_content(self.mainBox)
        
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
        self.scaleModeBox = gtk.HBox()
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
        
        self.scaleBoxLabel = gtk.Label(_('Scale: '))
        self.scaleBox = BigComboBox()
        scales = [_('Major scale'), _('Harmonic minor scale'), _('Natural minor scale'), _('Phrygian scale'), _('Dorian scale'), _('Lydian scale'), _('Myxolidian scale')]
        for scale in scales:
            self.scaleBox.append_item(scales.index(scale), scale)
        self.scaleBox.connect('changed', self.handleScale)
        self.scaleBox.set_active(0)
        
        self.modeBoxLabel = gtk.Label(_('Mode: '))
        self.modeBox = BigComboBox()
        modes = [_('Drunk'), _('Drone and Jump'), _('Repeater'), _('Loop segments')]
        for mode in modes:
            self.modeBox.append_item(modes.index(mode), mode)
        self.modeBox.connect('changed', self.handleMode)
        self.modeBox.set_active(0)
        
        self.scaleModeBox.pack_start(self.scaleBoxLabel, False, False, padding = 10)
        self.scaleModeBox.pack_start(self.scaleBox, False, False, padding = 10)
        self.scaleModeBox.pack_start(self.modeBoxLabel, False, False, padding = 10)
        self.scaleModeBox.pack_start(self.modeBox, False, False, padding = 10)
        
        self.acceptButton = gtk.Button(label = _('Accept'))
#        self.acceptButton = IconButton('stock-accept')
        self.acceptButton.connect('clicked',self.generate)
        self.cancelButton = gtk.Button(label = _('Cancel'))
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

