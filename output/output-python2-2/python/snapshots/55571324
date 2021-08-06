#!/usr/bin/env python

import gtk

import common.Config as Config
from sugar.graphics.toolbutton import ToolButton
from sugar.graphics.toggletoolbutton import ToggleToolButton
from sugar.graphics.radiotoolbutton import RadioToolButton
from sugar.graphics.palette import Palette
from sugar.graphics.icon import Icon
from common.Util.ThemeWidgets import *
from gettext import gettext as _

#Generation palette
import gobject
from common.Generation.Generator import GenerationParameters
#Generation palette and Properties palette
from common.Generation.GenerationConstants import GenerationConstants
from common.Generation.GenerationRythm import GenerationRythm
from common.Generation.GenerationPitch import GenerationPitch

#Properties palette
from common.Util.NoteDB import PARAMETER
from common.Generation.Drunk import *
import common.Generation.Utils as Utils
from types import *
from math import sqrt
from random import *

class mainToolbar(gtk.Toolbar):
    def __init__(self,toolbox, edit):
        gtk.Toolbar.__init__(self)

        def _insertSeparator(x = 1):
            for i in range(x):
                self.separator = gtk.SeparatorToolItem()
                self.separator.set_expand(True)
                self.separator.set_draw(True)
                self.insert(self.separator,-1)
                self.separator.show()

        self.toolbox = toolbox
        self.edit = edit

        self.tooltips = gtk.Tooltips()

        #Play button
        self.playButton = ToggleToolButton('media-playback-start')
        self.playButtonHandler = self.playButton.connect('toggled', self.handlePlayPause)
        self.insert(self.playButton, -1)
        self.playButton.show()
        self.playButton.set_tooltip(_('Play / Pause'))

        #Stop button
        self.stopButton = ToolButton('media-playback-stop')
        self.stopButton.connect('clicked', self.handleStop)
        self.insert(self.stopButton, -1)
        self.stopButton.show()
        self.stopButton.set_tooltip(_('Stop'))

        #Play button Image
        self.playButtonImg = gtk.Image()
        self.playButtonImg.set_from_icon_name('media-playback-start', gtk.ICON_SIZE_LARGE_TOOLBAR)
        self.playButtonImg.show()

        #Pause button Image
        self.pauseButtonImg = gtk.Image()
        self.pauseButtonImg.set_from_icon_name('media-playback-pause', gtk.ICON_SIZE_LARGE_TOOLBAR)
        self.pauseButtonImg.show()

        #Record button
        self.recordButton = ToggleToolButton('recordK')
        self.recordButton.connect('clicked', self.edit.handleKeyboardRecordButton)
        self.insert(self.recordButton, -1)
        self.recordButton.show()
        self.recordButton.set_tooltip(_('Record keyboard'))

        #RecordOgg button
        self.recordOggButton = ToggleToolButton('recordO')
        self.recordOggButton.connect('clicked', self.edit.handleAudioRecord)
        self.insert(self.recordOggButton, -1)
        self.recordOggButton.show()
        self.recordOggButton.set_tooltip(_('Record to ogg'))

        _insertSeparator(1)

        #Pointer button
        self._pointerPalette = pointerPalette(_('Select Tool'), self.edit)
        self.pointerButton = RadioToolButton('edit-pointer', group = None)
        self.pointerButton.set_palette(self._pointerPalette)
        self.pointerButton.connect('toggled', self.edit.handleToolClick, 'default')
        self.insert(self.pointerButton, -1)
        self.pointerButton.show()

        #Draw button
        self._drawPalette = drawPalette(_('Draw Tool'), self.edit)
        self.drawButton = RadioToolButton('edit-pencil', group = self.pointerButton)
        self.drawButton.set_palette(self._drawPalette)
        self.drawButton.connect('toggled', self.edit.handleToolClick, 'draw')
        self.insert(self.drawButton, -1)
        self.drawButton.show()

        #Paint button
        self._paintPalette = paintPalette(_('Paint Tool'), self.edit)
        self.paintButton = RadioToolButton('edit-brush', group = self.pointerButton)
        self.paintButton.set_palette(self._paintPalette)
        self.paintButton.connect('toggled', self.edit.handleToolClick, 'paint')
        self.insert(self.paintButton, -1)
        self.paintButton.show()

        _insertSeparator(1)

        #Duplicate button
        self.duplicateButton = ToggleToolButton('duplicate')
        self.duplicateButton.connect('toggled', self.handleDuplicate)
        self.insert(self.duplicateButton, -1)
        self.duplicateButton.show()
        self.duplicateButton.set_tooltip(_('Duplicate'))

        #Volume / Tempo button
        self._volumeTempoPalette = volumeTempoPalette(_('Volume / Tempo'), self.edit)
        self.volumeTempoButton = ToggleToolButton('voltemp')
        self.volumeTempoButton.set_palette(self._volumeTempoPalette)
        self.insert(self.volumeTempoButton, -1)
        self.volumeTempoButton.show()

    def handlePlayPause(self, widget, data = None):
        if widget.get_active():
            self.edit.handlePlay(widget)
            self.edit._generateToolbar.handler_block(self.edit._generateToolbar.playButtonHandler)
            self.edit._generateToolbar.playButton.set_active(True)
            self.edit._generateToolbar.handler_unblock(self.edit._generateToolbar.playButtonHandler)
            widget.set_icon_widget(self.pauseButtonImg)
            self.edit._generateToolbar.playButton.set_icon_widget(self.edit._generateToolbar.pauseButtonImg)
        else:
            self.edit.handleStop(widget, False)
            self.edit._generateToolbar.handler_block(self.edit._generateToolbar.playButtonHandler)
            self.edit._generateToolbar.playButton.set_active(False)
            self.edit._generateToolbar.handler_unblock(self.edit._generateToolbar.playButtonHandler)
            widget.set_icon_widget(self.playButtonImg)
            self.edit._generateToolbar.playButton.set_icon_widget(self.edit._generateToolbar.playButtonImg)

    def handleStop(self, widget, data = None):
        self.edit.handleStop(widget, True)
        self.playButton.set_active(False)
        if self.recordButton.get_active():
            self.recordButton.set_active(False)

    def handleDuplicate(self, widget):
        if widget.get_active():
            if self.edit.getContext() == 0: #Page
                self.edit.pageDuplicate()
            elif self.edit.getContext() == 1: #Track
                self.edit.trackDuplicateWidget(widget)
            elif self.edit.getContext() == 2: #Note
                self.edit.noteDuplicateWidget(widget)
            widget.set_active(False)

class generateToolbar(gtk.Toolbar):
    def __init__(self,toolbox, edit):
        gtk.Toolbar.__init__(self)

        def _insertSeparator(x = 1):
            for i in range(x):
                self.separator = gtk.SeparatorToolItem()
                self.separator.set_expand(True)
                self.separator.set_draw(False)
                self.insert(self.separator,-1)
                self.separator.show()

        self.toolbox = toolbox
        self.edit = edit

        self.tooltips = gtk.Tooltips()

        #Play button
        self.playButton = ToggleToolButton('media-playback-start')
        self.playButtonHandler = self.playButton.connect('toggled', self.handlePlayPause)
        self.insert(self.playButton, -1)
        self.playButton.show()
        self.playButton.set_tooltip(_('Play / Pause'))

        #Stop button
        self.stopButton = ToolButton('media-playback-stop')
        self.stopButton.connect('clicked', self.handleStop)
        self.insert(self.stopButton, -1)
        self.stopButton.show()
        self.stopButton.set_tooltip(_('Stop'))

        #Play button Image
        self.playButtonImg = gtk.Image()
        self.playButtonImg.set_from_icon_name('media-playback-start', gtk.ICON_SIZE_LARGE_TOOLBAR)
        self.playButtonImg.show()

        #Pause button Image
        self.pauseButtonImg = gtk.Image()
        self.pauseButtonImg.set_from_icon_name('media-playback-pause', gtk.ICON_SIZE_LARGE_TOOLBAR)
        self.pauseButtonImg.show()

        _insertSeparator(1)

        #BigGeneration button
        self.bigGenerationButton = ToolButton('diceB')
        self.bigGenerationButton.connect('clicked', self.edit.createNewTune)
        self.insert(self.bigGenerationButton, -1)
        self.bigGenerationButton.show()
        self.bigGenerationButton.set_tooltip(_('Generate Tune'))

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

    def handlePlayPause(self, widget, data = None):
        if widget.get_active():
            self.edit.handlePlay(widget)
            self.edit._mainToolbar.handler_block(self.edit._mainToolbar.playButtonHandler)
            self.edit._mainToolbar.playButton.set_active(True)
            self.edit._mainToolbar.handler_unblock(self.edit._mainToolbar.playButtonHandler)
            widget.set_icon_widget(self.pauseButtonImg)
            self.edit._mainToolbar.playButton.set_icon_widget(self.edit._mainToolbar.pauseButtonImg)
        else:
            self.edit.handleStop(widget, False)
            self.edit._mainToolbar.handler_block(self.edit._mainToolbar.playButtonHandler)
            self.edit._mainToolbar.playButton.set_active(False)
            self.edit._mainToolbar.handler_unblock(self.edit._mainToolbar.playButtonHandler)
            widget.set_icon_widget(self.playButtonImg)
            self.edit._mainToolbar.playButton.set_icon_widget(self.edit._mainToolbar.playButtonImg)

    def handleStop(self, widget, data = None):
        self.edit.handleStop(widget, True)
        self.playButton.set_active(False)
        if self.edit._mainToolbar.recordButton.get_active():
            self.edit._mainToolbar.recordButton.set_active(False)

class pointerPalette(Palette):
    def __init__(self, label, edit):
        Palette.__init__(self, label)

        self.edit = edit

        self.pointerBox = gtk.VBox()

        self.snapGridHBox = gtk.HBox()
        self.snapGridImage = gtk.Image()
        self.snapGridImage.set_from_file(Config.TAM_TAM_ROOT + '/icons/grid.svg')
        self.snapGridBox = BigComboBox()
        self.snapGridBox.connect('changed', self.handleSnapGrid)
        self.gridDurs = [1, 2, 3, 4, 6, 12, 24]
        durs = [_('1/12'), _('1/6'), _('1/4'), _('1/3'), _('1/2'), _('1'), _('2') ]
        for dur in durs:
            self.snapGridBox.append_item(durs.index(dur),dur)
        self.snapGridBox.set_active(0)
        self.snapGridHBox.pack_start(self.snapGridImage, False, False, padding = 5)
        self.snapGridHBox.pack_start(self.snapGridBox, False, False, padding = 5)

        self.pointerBox.pack_start(self.snapGridHBox, False, False, padding = 5)
        self.pointerBox.show_all()

        self.set_content(self.pointerBox)

        pass
        #self.noteDur = widget.props.value

    def handleSnapGrid(self, widget):
        data = widget.props.value
        grid = int(self.gridDurs[data])
        self.edit.trackInterface.setPointerGrid(grid)

class drawPalette(Palette):
    def __init__(self, label, edit):
        Palette.__init__(self, label)

        self.edit = edit

        self.drawBox = gtk.VBox()

        self.snapGridHBox = gtk.HBox()
        self.snapGridImage = gtk.Image()
        self.snapGridImage.set_from_file(Config.TAM_TAM_ROOT + '/icons/grid.svg')
        self.snapGridBox = BigComboBox()
        self.snapGridBox.connect('changed', self.handleSnapGrid)
        self.gridDurs = [1, 2, 3, 4, 6, 12, 24]
        durs = [_('1/12'), _('1/6'), _('1/4'), _('1/3'), _('1/2'), _('1'), _('2') ]
        for dur in durs:
            self.snapGridBox.append_item(durs.index(dur),dur)
        self.snapGridBox.set_active(0)
        self.snapGridHBox.pack_start(self.snapGridImage, False, False, padding = 5)
        self.snapGridHBox.pack_start(self.snapGridBox, False, False, padding = 5)

        self.drawBox.pack_start(self.snapGridHBox, False, False, padding = 5)
        self.drawBox.show_all()

        self.set_content(self.drawBox)

        pass
        #self.noteDur = widget.props.value

    def handleSnapGrid(self, widget):
        data = widget.props.value
        grid = int(self.gridDurs[data])
        self.edit.trackInterface.setDrawGrid(grid)

class paintPalette(Palette):
    def __init__(self, label, edit):
        Palette.__init__(self, label)

        self.edit = edit

        self.paintBox = gtk.VBox()

        self.noteDurHBox = gtk.HBox()
        self.noteDurImage = gtk.Image()
        self.noteDurImage.set_from_file(Config.TAM_TAM_ROOT + '/icons/notedur.svg')
        self.noteDurBox = BigComboBox()
        self.noteDurBox.connect('changed', self.handleNoteDur)
        self.noteDurs = [1, 2, 3, 4, 6, 12, 24]
        self.durs = [_('1/12'), _('1/6'), _('1/4'), _('1/3'), _('1/2'), _('1'), _('2') ]
        for dur in self.durs:
            self.noteDurBox.append_item(self.durs.index(dur),dur)
        self.noteDurBox.set_active(2)
        self.noteDurHBox.pack_start(self.noteDurImage, False, False, padding = 5)
        self.noteDurHBox.pack_start(self.noteDurBox, False, False, padding = 5)

        self.snapGridHBox = gtk.HBox()
        self.snapGridImage = gtk.Image()
        self.snapGridImage.set_from_file(Config.TAM_TAM_ROOT + '/icons/grid.svg')
        self.snapGridBox = BigComboBox()
        self.snapGridBox.connect('changed', self.handleSnapGrid)
        self.gridDurs = [1, 2, 3, 4, 6, 12, 24]
        durs = [_('1/12'), _('1/6'), _('1/4'), _('1/3'), _('1/2'), _('1'), _('2')]
        for dur in durs:
            self.snapGridBox.append_item(durs.index(dur),dur)
        self.snapGridBox.set_active(2)
        self.snapGridHBox.pack_start(self.snapGridImage, False, False, padding = 5)
        self.snapGridHBox.pack_start(self.snapGridBox, False, False, padding = 5)

        self.paintBox.pack_start(self.noteDurHBox, False, False, padding = 5)
        self.paintBox.pack_start(self.snapGridHBox, False, False, padding = 5)
        self.paintBox.show_all()

        self.set_content(self.paintBox)

    def resizeNoteDur(self):
        oldActive = self.noteDurBox.get_active()
        len = self.snapGridBox.get_active()
        self.noteDurBox.remove_all()
        for dur in self.durs[0:len+1]:
            self.noteDurBox.append_item(self.durs.index(dur), dur)
        if oldActive <= len:
            self.noteDurBox.set_active(oldActive)
        else:
            self.noteDurBox.set_active(len)

    def handleNoteDur(self, widget):
        data = widget.props.value
        noteDur = int(self.noteDurs[data])
        self.edit.trackInterface.setPaintNoteDur(noteDur)

    def handleSnapGrid(self, widget):
        data = widget.props.value
        grid = int(self.gridDurs[data])
        self.edit.trackInterface.setPaintGrid(grid)
        self.resizeNoteDur()

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

        self.XYSlider1MainBox = gtk.VBox()
        self.XYSlider1TopLabel = gtk.Label(_('Rythm'))
        self.XSlider1BottomLabelBox = gtk.HBox()
        self.XSlider1Img = gtk.Image()
        self.XSlider1Img.set_from_file(Config.TAM_TAM_ROOT + '/icons/sideR.svg')
        self.XSlider1BottomLabel = gtk.Label(_('Density'))
        self.YSlider1BottomLabelBox = gtk.HBox()
        self.YSlider1Img = gtk.Image()
        self.YSlider1Img.set_from_file(Config.TAM_TAM_ROOT + '/icons/updownR.svg')
        self.YSlider1BottomLabel = gtk.Label(_('Regularity'))
        self.XYSliderBox1 = RoundFixed(fillcolor = '#CCCCCC', bordercolor = '#000000')
        self.XYSliderBox1.set_size_request(200,200)
        self.XYButton1 = ImageToggleButton( Config.TAM_TAM_ROOT + '/icons/XYBut.svg', Config.TAM_TAM_ROOT + '/icons/XYButDown.svg')
        self.XAdjustment1 = gtk.Adjustment(self.rythmDensity[0] * 100, 0, 100, 1, 1, 1)
        self.XAdjustment1.connect("value-changed", self.handleXAdjustment1)
        self.YAdjustment1 = gtk.Adjustment(self.rythmRegularity[0] * 100, 0, 100, 1, 1, 1)
        self.YAdjustment1.connect("value-changed", self.handleYAdjustment1)
        self.XYSlider1 = XYSlider( self.XYSliderBox1, self.XYButton1, self.XAdjustment1, self.YAdjustment1, False, True )
        self.XSlider1BottomLabelBox.pack_start(self.XSlider1Img, False, False, padding = 5)
        self.XSlider1BottomLabelBox.pack_start(self.XSlider1BottomLabel, False, False, padding = 5)
        self.YSlider1BottomLabelBox.pack_start(self.YSlider1Img, False, False, padding = 5)
        self.YSlider1BottomLabelBox.pack_start(self.YSlider1BottomLabel, False, False, padding = 5)
        self.XYSlider1MainBox.pack_start(self.XYSlider1TopLabel, False, False, padding = 5)
        self.XYSlider1MainBox.pack_start(self.XYSlider1, False, False, padding = 2)
        self.XYSlider1MainBox.pack_start(self.XSlider1BottomLabelBox, False, False, padding = 2)
        self.XYSlider1MainBox.pack_start(self.YSlider1BottomLabelBox, False, False, padding = 2)

        self.XYSlider2MainBox = gtk.VBox()
        self.XYSlider2TopLabel = gtk.Label(_('Pitch'))
        self.XSlider2BottomLabelBox = gtk.HBox()
        self.XSlider2Img = gtk.Image()
        self.XSlider2Img.set_from_file(Config.TAM_TAM_ROOT + '/icons/sideR.svg')
        self.XSlider2BottomLabel = gtk.Label(_('Regularity'))
        self.YSlider2BottomLabelBox = gtk.HBox()
        self.YSlider2Img = gtk.Image()
        self.YSlider2Img.set_from_file(Config.TAM_TAM_ROOT + '/icons/updownR.svg')
        self.YSlider2BottomLabel = gtk.Label(_('Maximum step'))
        self.XYSliderBox2 = RoundFixed(fillcolor = '#CCCCCC', bordercolor = '#000000')
        self.XYSliderBox2.set_size_request(200,200)
        self.XYButton2 = ImageToggleButton( Config.TAM_TAM_ROOT + '/icons/XYBut.svg', Config.TAM_TAM_ROOT + '/icons/XYButDown.svg')
        self.XAdjustment2 = gtk.Adjustment(self.pitchRegularity[0] * 100, 0, 100, 1, 1, 1)
        self.XAdjustment2.connect("value-changed", self.handleXAdjustment2)
        self.YAdjustment2 = gtk.Adjustment(self.pitchStep[0] * 100, 0, 100, 1, 1, 1)
        self.YAdjustment2.connect("value-changed", self.handleYAdjustment2)
        self.XYSlider2 = XYSlider( self.XYSliderBox2, self.XYButton2, self.XAdjustment2, self.YAdjustment2, False, True )
        self.XSlider2BottomLabelBox.pack_start(self.XSlider2Img, False, False, padding = 5)
        self.XSlider2BottomLabelBox.pack_start(self.XSlider2BottomLabel, False, False, padding = 5)
        self.YSlider2BottomLabelBox.pack_start(self.YSlider2Img, False, False, padding = 5)
        self.YSlider2BottomLabelBox.pack_start(self.YSlider2BottomLabel, False, False, padding = 5)
        self.XYSlider2MainBox.pack_start(self.XYSlider2TopLabel, False, False, padding = 5)
        self.XYSlider2MainBox.pack_start(self.XYSlider2, False, False, padding = 2)
        self.XYSlider2MainBox.pack_start(self.XSlider2BottomLabelBox, False, False, padding = 2)
        self.XYSlider2MainBox.pack_start(self.YSlider2BottomLabelBox, False, False, padding = 2)

        self.XYSlider3MainBox = gtk.VBox()
        self.XYSlider3TopLabel = gtk.Label(_('Duration'))
        self.XSlider3BottomLabelBox = gtk.HBox()
        self.XSlider3Img = gtk.Image()
        self.XSlider3Img.set_from_file(Config.TAM_TAM_ROOT + '/icons/sideR.svg')
        self.XSlider3BottomLabel = gtk.Label(_('Note duration'))
        self.YSlider3BottomLabelBox = gtk.HBox()
        self.YSlider3Img = gtk.Image()
        self.YSlider3Img.set_from_file(Config.TAM_TAM_ROOT + '/icons/updownR.svg')
        self.YSlider3BottomLabel = gtk.Label(_('Silence density'))
        self.XYSliderBox3 = RoundFixed(fillcolor = '#CCCCCC', bordercolor = '#000000')
        self.XYSliderBox3.set_size_request(200,200)
        self.XYButton3 = ImageToggleButton( Config.TAM_TAM_ROOT + '/icons/XYBut.svg', Config.TAM_TAM_ROOT + '/icons/XYButDown.svg')
        self.XAdjustment3 = gtk.Adjustment(self.duration[0] * 100, 0, 100, 1, 1, 1)
        self.XAdjustment3.connect("value-changed", self.handleXAdjustment3)
        self.YAdjustment3 = gtk.Adjustment(self.silence[0] * 100, 0, 100, 1, 1, 1)
        self.YAdjustment3.connect("value-changed", self.handleYAdjustment3)
        self.XYSlider3 = XYSlider( self.XYSliderBox3, self.XYButton3, self.XAdjustment3, self.YAdjustment3, False, True )
        self.XSlider3BottomLabelBox.pack_start(self.XSlider3Img, False, False, padding = 5)
        self.XSlider3BottomLabelBox.pack_start(self.XSlider3BottomLabel, False, False, padding = 5)
        self.YSlider3BottomLabelBox.pack_start(self.YSlider3Img, False, False, padding = 5)
        self.YSlider3BottomLabelBox.pack_start(self.YSlider3BottomLabel, False, False, padding = 5)
        self.XYSlider3MainBox.pack_start(self.XYSlider3TopLabel, False, False, padding = 5)
        self.XYSlider3MainBox.pack_start(self.XYSlider3, False, False, padding = 2)
        self.XYSlider3MainBox.pack_start(self.XSlider3BottomLabelBox, False, False, padding = 2)
        self.XYSlider3MainBox.pack_start(self.YSlider3BottomLabelBox, False, False, padding = 2)

        self.slidersBox.pack_start(self.XYSlider1MainBox, False, False, padding = 5)
        self.slidersBox.pack_start(self.XYSlider2MainBox, False, False, padding = 5)
        self.slidersBox.pack_start(self.XYSlider3MainBox, False, False, padding = 5)

        self.previewBox = gtk.HBox()
        self.previewDA = gtk.DrawingArea()
        self.previewDA.set_size_request( -1, 100 )
        self.previewDA.connect( "size-allocate", self.handlePreviewAlloc )
        self.previewDA.connect( "expose-event", self.handlePreviewExpose )
        self.previewBox.pack_start( self.previewDA, True, True, padding = 5 )

        self.scaleBoxHBox = gtk.HBox()
        self.scaleBoxLabel = gtk.Label(_('Scale: '))
        self.scaleBox = BigComboBox()
        scales = [_('Major scale'), _('Harmonic minor scale'), _('Natural minor scale'), _('Phrygian scale'), _('Dorian scale'), _('Lydian scale'), _('Myxolidian scale')]
        for scale in scales:
            self.scaleBox.append_item(scales.index(scale), scale)
        self.scaleBox.connect('changed', self.handleScale)

        self.modeBoxHBox = gtk.HBox()
        self.modeBoxLabel = gtk.Label(_('Mode: '))
        self.modeBox = BigComboBox()
        modes = [_('Drunk'), _('Drone and Jump'), _('Repeater'), _('Loop segments')]
        for mode in modes:
            self.modeBox.append_item(modes.index(mode), mode)
        self.modeBox.connect('changed', self.handleMode)

        self.scaleBoxHBox.pack_start(self.scaleBoxLabel, False, False, padding = 10)
        self.scaleBoxHBox.pack_start(self.scaleBox, False, False, padding = 10)
        self.modeBoxHBox.pack_start(self.modeBoxLabel, False, False, padding = 10)
        self.modeBoxHBox.pack_start(self.modeBox, False, False, padding = 10)
        self.scaleModeBox.pack_start(self.scaleBoxHBox, False, False, padding = 5)
        self.scaleModeBox.pack_start(self.modeBoxHBox, False, False, padding = 5)

        self.acceptButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/accept.svg')
        self.acceptButton.connect('clicked',self.generate)
        self.cancelButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/cancel.svg')
        self.cancelButton.connect('clicked',self.cancel)
        self.decisionBox.pack_start(self.cancelButton, False, False, padding = 5)
        self.decisionBox.pack_start(self.acceptButton, False, False, padding = 5)

        self.mainBox.pack_start(self.slidersBox, False, False, padding = 5)
        self.mainBox.pack_start( self.previewBox, False, False, padding = 5 )
        self.mainBox.pack_start(self.scaleModeBox, False, False, padding = 5)
        self.mainBox.pack_start(self.decisionBox, False, False, padding = 5)
        self.mainBox.show_all()

        self.set_content(self.mainBox)

        #-- preview drawing -----------------------------------
        win = gtk.gdk.get_default_root_window()
        self.gc = gtk.gdk.GC( win )
        self.parametersDirty = False
        self.drawingPreview = False
        self.predrawTarget = 0
        self.predrawIdleAbort = False
        self.predrawBuffer = False
        # self.predrawBuffer is initialized in handlePreviewAlloc
        pix = gtk.gdk.pixbuf_new_from_file( Config.IMAGE_ROOT+"sampleBG.png" )
        self.sampleBg = gtk.gdk.Pixmap( win, pix.get_width(), pix.get_height() )
        self.sampleBg.draw_pixbuf( self.gc, pix, 0, 0, 0, 0, pix.get_width(), pix.get_height(), gtk.gdk.RGB_DITHER_NONE )
        self.sampleBg.endOffset = pix.get_width()-5
        self.sampleNoteHeight = 7
        if True: # load clipmask
            pix = gtk.gdk.pixbuf_new_from_file(Config.IMAGE_ROOT+'sampleNoteMask.png')
            pixels = pix.get_pixels()
            stride = pix.get_rowstride()
            channels = pix.get_n_channels()
            bitmap = ""
            byte = 0
            shift = 0
            for j in range(pix.get_height()):
                offset = stride*j
                for i in range(pix.get_width()):
                    r = pixels[i*channels+offset]
                    if r != "\0": byte += 1 << shift
                    shift += 1
                    if shift > 7:
                        bitmap += "%c" % byte
                        byte = 0
                        shift = 0
                if shift > 0:
                    bitmap += "%c" % byte
                    byte = 0
                    shift = 0
            self.sampleNoteMask = gtk.gdk.bitmap_create_from_data( None, bitmap, pix.get_width(), pix.get_height() )
            self.sampleNoteMask.endOffset = pix.get_width()-3

        colormap = self.previewDA.get_colormap()
        self.colors = { "Beat_Line":   colormap.alloc_color( "#959595", True, True ),
                        "Note_Border": colormap.alloc_color( Config.BG_COLOR, True, True ),
                        "Note_Fill":   colormap.alloc_color( Config.FG_COLOR, True, True ) }

        self.scaleBox.set_active(0)
        self.modeBox.set_active(0)


    def handleXAdjustment1( self, data ):
        self.rythmDensity = [self.XAdjustment1.value * .01 for x in range(4)]

        self.parametersChanged()

    def handleYAdjustment1( self, data ):
        self.rythmRegularity = [self.YAdjustment1.value * .01 for x in range(4)]
        self.parametersChanged()

    def handleXAdjustment2( self, data ):
        self.pitchRegularity = [self.XAdjustment2.value * .01 for x in range(4)]
        self.parametersChanged()

    def handleYAdjustment2( self, data ):
        self.pitchStep = [self.YAdjustment2.value * .01 for x in range(4)]
        self.parametersChanged()

    def handleXAdjustment3( self, data ):
        self.duration = [self.XAdjustment3.value * .01 for x in range(4)]
        self.parametersChanged()

    def handleYAdjustment3( self, data ):
        self.silence = [self.YAdjustment3.value * .01 for x in range(4)]
        self.parametersChanged()

    def handleScale(self, widget, data = None):
        self.scale = widget.props.value
        self.edit.scale = self.scale
        self.parametersChanged()

    def handleMode( self, widget, data = None ):
        self.pattern = [widget.props.value for x in range(4)]
        self.parametersChanged()

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
       # print self.rythmDensity, self.rythmRegularity,self.pitchRegularity,self.pitchStep, self.duration, self.silence,self.pattern
        self.popdown(True)

    ############ generate a preview melody ##############s
    def previewGenerator(self, parameters):
        makeRythm = GenerationRythm()
        makePitch = GenerationPitch()
        table_duration = Utils.scale(parameters.articule[0], GenerationConstants.ARTICULATION_SCALE_MIN_MAPPING, GenerationConstants.ARTICULATION_SCALE_MAX_MAPPING, GenerationConstants.ARTICULATION_SCALE_STEPS)
        table_pitch = GenerationConstants.SCALES[parameters.scale]
        beat = self.edit.noteDB.pages[self.edit.tuneInterface.getSelectedIds()[0]].beats
        barLength = Config.TICKS_PER_BEAT * beat
        trackNotes = []

        rythmSequence = makeRythm.celluleRythmSequence(parameters, barLength, 0)
        pitchSequence = makePitch.drunkPitchSequence(len(rythmSequence),parameters, table_pitch, 0)
        gainSequence = self.makeGainSequence(rythmSequence)
        durationSequence = self.makeDurationSequence(rythmSequence, parameters, table_duration, barLength)

        for i in range(len(rythmSequence)):
            if random() > parameters.silence[0]:
                trackNotes.append([rythmSequence[i], pitchSequence[i], gainSequence[i], durationSequence[i]])
        #print "-------------------------------------------------------",trackNotes
        return ( trackNotes, beat )

    def makeGainSequence( self, onsetList ):
        gainSequence = []
        max = GenerationConstants.GAIN_MAX_BOUNDARY
        midMax = GenerationConstants.GAIN_MID_MAX_BOUNDARY
        midMin = GenerationConstants.GAIN_MID_MIN_BOUNDARY
        min = GenerationConstants.GAIN_MIN_BOUNDARY
        for onset in onsetList:
            if onset == 0:
                gainSequence.append(uniform(midMax, max))
            elif ( onset % Config.TICKS_PER_BEAT) == 0:
                gainSequence.append(uniform(midMin, midMax))
            else:
                gainSequence.append(uniform(min, midMin))
        return gainSequence

    def makeDurationSequence( self, onsetList, parameters, table_duration, barLength ):
        durationSequence = []
        if len( onsetList ) > 1:
            for i in range(len(onsetList) - 1):
                durationSequence.append((onsetList[i+1] - onsetList[i]) * Utils.prob2( table_duration ))
            durationSequence.append(( barLength - onsetList[-1]) * Utils.prob2( table_duration ))
        elif len( onsetList ) == 1:
            durationSequence.append( ( barLength - onsetList[0] ) * Utils.prob2( table_duration ))
        return durationSequence

    def parametersChanged( self ):
        if not self.drawingPreview:
            self.drawPreview()
        else:
            self.parametersDirty = True

    def drawPreview( self, force = False ):
        if not self.predrawBuffer:
            return # not alloc'ed yet

        if self.drawingPreview and not force:
            return # should never happen

        notes, beats = self.previewGenerator( self.getGenerationParameters() )
        self.parametersDirty = False

        if force:
            if self.drawingPreview:
                self.predrawIdleAbort = True
            self._idleDraw( notes, beats, True, True )
        else:
            self.drawingPreview = True
            gobject.idle_add( self._idleDraw, notes, beats, True, False )

    def _idleDraw( self, notes, beats, fresh, force ):
        if self.predrawIdleAbort and not force:
            self.predrawIdleAbort = False
            return False

        pixmap = self.predrawBuffer[self.predrawTarget]

        if fresh:
            # draw bg
            pixmap.draw_drawable( self.gc, self.sampleBg, 0, 0, 0, 0, self.previewDA.width-5, self.previewDA.height )
            pixmap.draw_drawable( self.gc, self.sampleBg, self.sampleBg.endOffset, 0, self.previewDA.width-5, 0, 5, self.previewDA.height )
            # draw beat lines
            self.gc.set_line_attributes( Config.BEAT_LINE_SIZE, gtk.gdk.LINE_ON_OFF_DASH, gtk.gdk.CAP_BUTT, gtk.gdk.JOIN_MITER )
            self.gc.foreground = self.colors["Beat_Line"]
            for i in range(1,beats):
                x = self.beatSpacing[beats][i]
                pixmap.draw_line( self.gc, x, 1, x, self.previewDA.height-1 )

            if not force:
                gobject.idle_add( self._idleDraw, notes, beats, False, False )
                return False

        if force: N = len(notes)
        else:     N = min( 3, len( notes ) ) # adjust this value to get a reasonable response

        self.gc.set_clip_mask( self.sampleNoteMask )
        for i in range( N ): # draw N notes
            note = notes.pop()
            x = self.ticksToPixels( beats, note[0] )
            endX = self.ticksToPixels( beats, note[0] + note[3] ) - 3 # include end cap offset
            width = endX - x
            y = self.pitchToPixels( note[1] )
            # draw fill
            self.gc.foreground = self.colors["Note_Fill"]
            self.gc.set_clip_origin( x, y-self.sampleNoteHeight )
            pixmap.draw_rectangle( self.gc, True, x+1, y+1, width+1, self.sampleNoteHeight-2 )
            # draw border
            self.gc.foreground = self.colors["Note_Border"]
            self.gc.set_clip_origin( x, y )
            pixmap.draw_rectangle( self.gc, True, x, y, width, self.sampleNoteHeight )
            self.gc.set_clip_origin( endX-self.sampleNoteMask.endOffset, y )
            pixmap.draw_rectangle( self.gc, True, endX, y, 3, self.sampleNoteHeight )
        self.gc.set_clip_rectangle( self.clearClipMask )

        if not len(notes):
            self.predrawTarget = not self.predrawTarget
            self.previewDA.queue_draw()

            self.drawingPreview = False

            if self.parametersDirty:
                self.drawPreview()

            return False

        return True

    def handlePreviewAlloc( self, widget, allocation ):
        win = gtk.gdk.get_default_root_window()
        self.previewDA.width = allocation.width
        self.previewDA.height = allocation.height
        self.predrawBuffer = [ gtk.gdk.Pixmap( win, allocation.width, allocation.height ),
                               gtk.gdk.Pixmap( win, allocation.width, allocation.height ) ]
        self.clearClipMask = gtk.gdk.Rectangle( 0, 0, allocation.width, allocation.height )

        self.pitchPerPixel = float(Config.NUMBER_OF_POSSIBLE_PITCHES-1) / (self.previewDA.height - self.sampleNoteHeight)
        self.pixelsPerPitch = float(self.previewDA.height - self.sampleNoteHeight)/(Config.MAXIMUM_PITCH - Config.MINIMUM_PITCH)
        self.pixelsPerTick = [0] + [ self.previewDA.width/float(i*Config.TICKS_PER_BEAT) for i in range(1,Config.MAXIMUM_BEATS+1) ]
        self.ticksPerPixel = [0] + [ 1.0/self.pixelsPerTick[i] for i in range(1,Config.MAXIMUM_BEATS+1) ]

        self.beatSpacing = [[0]]
        for i in range(1,Config.MAXIMUM_BEATS+1):
            self.beatSpacing.append( [ self.ticksToPixels( i, Config.TICKS_PER_BEAT*j ) for j in range(i) ] )

        self.drawPreview( True )

    def handlePreviewExpose( self, widget, event ):
        widget.window.draw_drawable( self.gc, self.predrawBuffer[not self.predrawTarget], event.area.x, event.area.y, event.area.x, event.area.y, event.area.width, event.area.height )

    def ticksToPixels( self, beats, ticks ):
        return int(round( ticks * self.pixelsPerTick[beats] ))
    def pitchToPixels( self, pitch ):
        return int(round( ( Config.MAXIMUM_PITCH - pitch ) * self.pixelsPerPitch ))


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

        self.line = Line(0, 100)
        self.drunk = Drunk(0, 100)
        self.droneAndJump = DroneAndJump(0, 100)
        self.repeter = Repeter(0, 100)
        self.loopseg = Loopseg(0, 100)
        self.algoTypes = [self.line, self.drunk, self.droneAndJump, self.repeter, self.loopseg]
        self.algorithm = self.algoTypes[0]
        self.geneMinimum = 0
        self.geneMaximum = 100
        self.geneRandom = 20

        self.setup = False
        self.hidden = False
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
        for color in (0,1,2,3):
            self.pageColorComboBox.append_item(color, text = None, icon_name = Config.IMAGE_ROOT + 'pageThumbnailBG' + str(color) + '.png', size = (30,40))
        self.pageColorComboBox.set_active(0)
        self.pageColorComboBox.connect('changed', self.handleColor)
        self.pageColorBox.pack_start(self.pageColorLabel, False, False, padding = 5)
        self.pageColorBox.pack_end(self.pageColorComboBox, False, False, padding = 55)

        self.pageSeparator = gtk.HSeparator()
        self.pageSeparator.set_size_request(20, -1)

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
        self.panSlider.set_update_policy(gtk.UPDATE_DISCONTINUOUS)
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
        self.reverbSlider.set_update_policy(gtk.UPDATE_DISCONTINUOUS)
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
        self.attackDurSlider.set_update_policy(gtk.UPDATE_DISCONTINUOUS)
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
        self.decayDurSlider.set_update_policy(gtk.UPDATE_DISCONTINUOUS)
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
            self.filterTypeComboBox.append_item(self.filterTypes.index(filtertype), filtertype)
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
        self.filterCutoffSlider.set_update_policy(gtk.UPDATE_DISCONTINUOUS)
        self.filterCutoffCheckButton = gtk.CheckButton()
        self.filterCutoffCheckButton.connect('toggled', self.handleGeneCheckButton)
        self.geneCheckButtonDic['filter'] = self.filterCutoffCheckButton
        self.filterCutoffBox.pack_start(self.filterCutoffLabel, False, False, padding = 5)
        self.filterCutoffBox.pack_end(self.filterCutoffCheckButton, False, False, padding = 5)
        self.filterCutoffBox.pack_end(self.filterCutoffSlider, False, False, padding = 5)

        self.generationMainBox = gtk.VBox()
        self.generationSeparator = gtk.HSeparator()
        self.generationLabel = gtk.Label(_('Generation'))

        self.generationTypeBox = gtk.HBox()
        self.generationTypeLabel = gtk.Label(_('Type: '))
        self.generationTypeComboBox = BigComboBox()
        for genetype in self.geneTypes:
            self.generationTypeComboBox.append_item(self.geneTypes.index(genetype), genetype)
        self.generationTypeComboBox.connect('changed', self.handleGeneTypes)
        self.generationTypeComboBox.set_active(0)
        self.generationTypeBox.pack_start(self.generationTypeLabel, False, False, padding = 5)
        self.generationTypeBox.pack_end(self.generationTypeComboBox, False, False, padding = 55)

        self.minimumBox = gtk.HBox()
        self.minimumLabel = gtk.Label(_('Minimum: '))
        self.minimumSliderAdj = gtk.Adjustment(0, 0, 100, 1, 1, 0)
        self.minimumSliderAdj.connect('value-changed', self.handleMinimum)
        self.minimumSlider =  gtk.HScale(adjustment = self.minimumSliderAdj)
        self.minimumSlider.set_size_request(200,-1)
        self.minimumSlider.set_update_policy(gtk.UPDATE_DISCONTINUOUS)
        self.minimumSlider.set_value_pos(gtk.POS_RIGHT)
        self.minimumBox.pack_start(self.minimumLabel, False, False, padding = 5)
        self.minimumBox.pack_end(self.minimumSlider, False, False, padding = 52)

        self.maximumBox = gtk.HBox()
        self.maximumLabel = gtk.Label(_('Maximum: '))
        self.maximumSliderAdj = gtk.Adjustment(100, 0, 100, 1, 1, 0)
        self.maximumSliderAdj.connect('value-changed', self.handleMaximum)
        self.maximumSlider =  gtk.HScale(adjustment = self.maximumSliderAdj)
        self.maximumSlider.set_size_request(200,-1)
        self.maximumSlider.set_update_policy(gtk.UPDATE_DISCONTINUOUS)
        self.maximumSlider.set_value_pos(gtk.POS_RIGHT)
        self.maximumBox.pack_start(self.maximumLabel, False, False, padding = 5)
        self.maximumBox.pack_end(self.maximumSlider, False, False, padding = 52)

        self.randomBox = gtk.HBox()
        self.randomLabel = gtk.Label(_('Random: '))
        self.randomSliderAdj = gtk.Adjustment(20, 0, 100, 1, 1, 0)
        self.randomSliderAdj.connect('value-changed', self.handleRandom)
        self.randomSlider =  gtk.HScale(adjustment = self.randomSliderAdj)
        self.randomSlider.set_size_request(200,-1)
        self.randomSlider.set_update_policy(gtk.UPDATE_DISCONTINUOUS)
        self.randomSlider.set_value_pos(gtk.POS_RIGHT)
        self.randomBox.pack_start(self.randomLabel, False, False, padding = 5)
        self.randomBox.pack_end(self.randomSlider, False, False, padding = 52)

        self.decisionBox = gtk.HBox()
        self.acceptButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/accept.svg')
        self.acceptButton.connect('clicked', self.acceptGeneration)
        self.cancelButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/cancel.svg')
        self.cancelButton.connect('clicked', self.resetGeneCheckButton)
        self.decisionBox.pack_start(self.cancelButton, False, False, padding = 5)
        self.decisionBox.pack_start(self.acceptButton, False, False, padding = 5)

        self.mainBox.pack_start(self.gridDivisionBox, padding = 3)
        self.mainBox.pack_start(self.pageColorBox, padding = 3)
        self.mainBox.pack_start(self.pageSeparator, padding = 10)
        self.mainBox.pack_start(self.transposeBox, padding = 3)
        self.mainBox.pack_start(self.volumeBox, padding = 3)
        self.mainBox.pack_start(self.panBox, padding = 3)
        self.mainBox.pack_start(self.reverbBox, padding = 3)
        self.mainBox.pack_start(self.attackDurBox, padding = 3)
        self.mainBox.pack_start(self.decayDurBox, padding = 3)
        self.mainBox.pack_start(self.filterTypeBox, padding = 3)
        self.mainBox.pack_start(self.filterCutoffBox, padding = 3)
        self.generationMainBox.pack_start(self.generationSeparator, padding = 5)
        self.generationMainBox.pack_start(self.generationLabel, padding = 10)
        self.generationMainBox.pack_start(self.generationTypeBox, padding = 3)
        self.generationMainBox.pack_start(self.minimumBox, padding = 3)
        self.generationMainBox.pack_start(self.maximumBox, padding = 3)
        self.generationMainBox.pack_start(self.randomBox, padding = 3)
        self.generationMainBox.pack_start(self.decisionBox, padding = 3)
        self.mainBox.pack_start(self.generationMainBox, padding = 3)
        self.mainBox.show_all()

        self.generationMainBox.hide()

        self.set_content(self.mainBox)

    def handlePopup(self, widget, data = None):
        if self.edit.getContext() == 0: #Page
            self.setContext('page', self.edit._generateToolbar._generationPalette.scale, self.edit.tuneInterface.getSelectedIds())
        elif self.edit.getContext() == 1: #Track
            self.setContext('track', self.edit._generateToolbar._generationPalette.scale, self.edit.tuneInterface.getSelectedIds(), [ i for i in range(Config.NUMBER_OF_TRACKS) if self.edit.trackSelected[i] ])
        elif self.edit.getContext() == 2: #Note
            ids = self.edit.trackInterface.getSelectedNotes()
            notes = { self.edit.displayedPage: {} }
            for t in range(Config.NUMBER_OF_TRACKS):
                if len(ids[t]):
                    notes[self.edit.displayedPage][t] = [ self.edit.noteDB.getNote( self.edit.displayedPage, t, id ) for id in ids[t] ]
            self.setContext('note', self.edit._generateToolbar._generationPalette.scale, notes = notes)

    def handlePopdown(self, widget, data = None):
        self.resetGeneCheckButton(self.cancelButton)

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

    def acceptGeneration( self, widget ):
        valList = [self.geneMinimum, self.geneMaximum, self.geneRandom]
        if self.geneCheckButtonDic['transpose'].get_active(): self.algoPitch(valList, self.algorithm)
        if self.geneCheckButtonDic['volume'].get_active(): self.algoVolume(valList, self.algorithm)
        if self.geneCheckButtonDic['pan'].get_active(): self.algoPan(valList, self.algorithm)
        if self.geneCheckButtonDic['reverb'].get_active(): self.algoReverb(valList, self.algorithm)
        if self.geneCheckButtonDic['attack'].get_active(): self.algoAttack(valList, self.algorithm)
        if self.geneCheckButtonDic['decay'].get_active(): self.algoDecay(valList, self.algorithm)
        if self.geneCheckButtonDic['filter'].get_active(): self.algoCutoff(valList, self.algorithm)

    def resetGeneCheckButton(self, widget):
        if self.hidden:
            self.generationMainBox.hide()

        for key in self.geneCheckButtonDic:
            self.geneCheckButtonDic[key].set_active(False)

    def handleGeneCheckButton(self, widget, data = None):
        self.hidden = True
        if widget.get_active():
            self.generationMainBox.show()
        else:
            for key in self.geneCheckButtonDic:
                if self.geneCheckButtonDic[key].get_active():
                    self.hidden = False
            if self.hidden:
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
            self.edit.noteDB.updateNotes( stream + [-1] )

    def handleGeneTypes(self, widget):
        self.algorithm = self.algoTypes[widget.props.value]

    def handleMinimum(self, adjust):
        self.geneMinimum = int(adjust.value)

    def handleMaximum(self, adjust):
        self.geneMaximum = int(adjust.value)

    def handleRandom(self, adjust):
        self.geneRandom = int(adjust.value)
