#!/usr/bin/env python

import gtk
import Config

from sugar.graphics.toolbutton import ToolButton
from sugar.graphics.toggletoolbutton import ToggleToolButton
from sugar.graphics.palette import Palette
from sugar.graphics.icon import Icon
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
