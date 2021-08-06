#!/usr/bin/env python

import gtk
import Config

from sugar.graphics.toolbutton import ToolButton
from sugar.graphics.toggletoolbutton import ToggleToolButton
from sugar.graphics.palette import Palette
from sugar.graphics.icon import Icon
from Util.ThemeWidgets import *
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
        #self.playButton.connect(None)
        self.insert(self.playButton, -1)
        self.playButton.show()
        
        #Rewind button
        self.rewindButton = ToggleToolButton('rewind')
        #self.rewindButton.connect(None)
        self.insert(self.rewindButton, -1)
        self.rewindButton.show()
        
        #Record button
        self.recordButton = ToggleToolButton('record')
        #self.recordButton.connect(None)
        self.insert(self.recordButton, -1)
        self.recordButton.show()
        
        _insertSeparator(2)
        
        #Pencil button
        self._pencilPalette = pencilPalette(_('Draw Tool'), self.edit)
        self.pencilButton = ToggleToolButton('pencil')
        self.pencilButton.set_palette(self._pencilPalette)
        #self.pencilButton.connect(None)
        self.insert(self.pencilButton, -1)
        self.pencilButton.show()
        
        _insertSeparator(4)
        
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
        self._propsPalette = propsPalette(_('Properties'), self.edit)
        self.propsButton = ToggleToolButton('props')
        self.propsButton.set_palette(self._propsPalette)
        #self.propsButton.connect(None)
        self.insert(self.propsButton, -1)
        self.propsButton.show()   
        
class playPalette(Palette):
    def __init__(self, label, edit):
        Palette.__init__(self, label)
        
        self.edit = edit
        
class pencilPalette(Palette):
    def __init__(self, label, edit):
        Palette.__init__(self, label)
        
        self.edit = edit
        
        self.pencilBox = gtk.VBox()
        
        self.checkbox = gtk.CheckButton(label = _('Non-continuous'))
        
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
            
    
class volumeTempoPalette(Palette):
    def __init__(self, label, edit):
        Palette.__init__(self, label)
        
        self.edit = edit
        
        self.volumeTempoBox = gtk.VBox()
        
        self.volumeSliderBox = gtk.HBox()
        self.volumeSliderLabel = gtk.Label(_('Volume'))
        self.volumeSliderAdj = gtk  .Adjustment(value=0, lower=0, upper=1, step_incr=0.1, page_incr=0, page_size=0)
        self.volumeSlider =  gtk.HScale(adjustment = self.volumeSliderAdj)
        self.volumeSlider.set_size_request(250,15)
        self.volumeSlider.set_inverted(False)
        self.volumeSlider.set_draw_value(False)
        self.volumeSliderBox.pack_start(self.volumeSliderLabel, False, False, padding = 5)
        self.volumeSliderBox.pack_end(self.volumeSlider, False, False, padding = 5)
        
        self.tempoSliderBox = gtk.HBox()
        self.tempoSliderLabel = gtk.Label(_('Tempo'))
        self.tempoSliderAdj = gtk.Adjustment(value=0, lower=0, upper=1, step_incr=0.1, page_incr=0, page_size=0)
        self.tempoSlider =  gtk.HScale(adjustment = self.tempoSliderAdj)
        self.tempoSlider.set_size_request(250,15)
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
        
class generationPalette(Palette):
    def __init__(self, label, edit):
        Palette.__init__(self, label)
        
        self.edit = edit
        
        self.mainBox = gtk.VBox()
        self.slidersBox = gtk.HBox()
        self.scaleModeBox = gtk.HBox()
        self.decisionBox = gtk.HBox()
        
        self.XYSliderBox1 = RoundFixed(fillcolor = '#CCCCCC', bordercolor = '#000000')
        self.XYSliderBox1.set_size_request(200,200)
        self.XYButton1 = ImageToggleButton( Config.TAM_TAM_ROOT + '/icons/XYBut.svg', Config.TAM_TAM_ROOT + '/icons/XYButDown.svg')
        self.XAdjustment1 = gtk.Adjustment( 1, 0, 100, 1, 1, 1 )
        self.YAdjustment1 = gtk.Adjustment( 1, 0, 100, 1, 1, 1 )
        self.XYSlider1 = XYSlider( self.XYSliderBox1, self.XYButton1, self.XAdjustment1, self.YAdjustment1, False, True )
        
        self.XYSliderBox2 = RoundFixed(fillcolor = '#CCCCCC', bordercolor = '#000000')
        self.XYSliderBox2.set_size_request(200,200)
        self.XYButton2 = ImageToggleButton( Config.TAM_TAM_ROOT + '/icons/XYBut.svg', Config.TAM_TAM_ROOT + '/icons/XYButDown.svg')
        self.XAdjustment2 = gtk.Adjustment( 1, 0, 100, 1, 1, 1 )
        self.YAdjustment2 = gtk.Adjustment( 1, 0, 100, 1, 1, 1 )
        self.XYSlider2 = XYSlider( self.XYSliderBox2, self.XYButton2, self.XAdjustment2, self.YAdjustment2, False, True )
        
        self.XYSliderBox3 = RoundFixed(fillcolor = '#CCCCCC', bordercolor = '#000000')
        self.XYSliderBox3.set_size_request(200,200)
        self.XYButton3 = ImageToggleButton( Config.TAM_TAM_ROOT + '/icons/XYBut.svg', Config.TAM_TAM_ROOT + '/icons/XYButDown.svg')
        self.XAdjustment3 = gtk.Adjustment( 1, 0, 100, 1, 1, 1 )
        self.YAdjustment3 = gtk.Adjustment( 1, 0, 100, 1, 1, 1 )
        self.XYSlider3 = XYSlider( self.XYSliderBox3, self.XYButton3, self.XAdjustment3, self.YAdjustment3, False, True )
        
        self.slidersBox.pack_start(self.XYSlider1, False, False, padding = 5)
        self.slidersBox.pack_start(self.XYSlider2, False, False, padding = 5)
        self.slidersBox.pack_start(self.XYSlider3, False, False, padding = 5)
        
        self.scaleBoxLabel = gtk.Label(_('Scale: '))
        self.scaleBox = gtk.combo_box_new_text()
        for scale in [_('Major scale'), _('Harmonic minor scale'), _('Natural minor scale'), _('Phrygian scale'), _('Dorian scale'), _('Lydian scale'), _('Myxolidian scale')]:
            self.scaleBox.append_text(scale)
        self.scaleBox.set_active(0)
        
        self.modeBoxLabel = gtk.Label(_('Mode: '))
        self.modeBox = gtk.combo_box_new_text()
        for mode in [_('Drunk'), _('Drone and Jump'), _('Repeater'), _('Loop segments')]:
            self.modeBox.append_text(mode)
        self.modeBox.set_active(0)
        
        self.scaleModeBox.pack_start(self.scaleBoxLabel, False, False, padding = 10)
        self.scaleModeBox.pack_start(self.scaleBox, False, False, padding = 10)
        self.scaleModeBox.pack_start(self.modeBoxLabel, False, False, padding = 10)
        self.scaleModeBox.pack_start(self.modeBox, False, False, padding = 10)
        
        self.acceptButton = Icon('stock-accept')
        self.cancelButton = Icon('activity-stop')
        self.decisionBox.pack_start(self.cancelButton, False, False, padding = 5)
        self.decisionBox.pack_start(self.acceptButton, False, False, padding = 5)
        
        self.mainBox.pack_start(self.slidersBox, False, False, padding = 5)
        self.mainBox.pack_start(self.scaleModeBox, False, False, padding = 5)
        self.mainBox.pack_start(self.decisionBox, False, False, padding = 5)
        self.mainBox.show_all()
 
        
        self.set_content(self.mainBox)


