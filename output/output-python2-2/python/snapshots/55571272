#!/usr/bin/env python

import gtk
import common.Config as Config

from sugar.graphics.toolbutton import ToolButton
from sugar.graphics.toggletoolbutton import ToggleToolButton
from sugar.graphics.palette import Palette
from gettext import gettext as _

class playToolbar(gtk.Toolbar):
    def __init__(self,toolbox, miniTamTam):
        gtk.Toolbar.__init__(self)

        def _insertSeparator(x = 1):
            for i in range(x):
                self.separator = gtk.SeparatorToolItem()
                self.separator.set_draw(True)
                self.insert(self.separator,-1)
                self.separator.show()

        self.toolbox = toolbox
        self.miniTamTam = miniTamTam

        self.tooltips = gtk.Tooltips()

        self.playButton = ToggleToolButton('media-playback-start')
        self.playButton.connect('clicked',self.miniTamTam.handlePlayButton)
        self.insert(self.playButton, -1)
        self.playButton.show()
        self.playButton.set_tooltip(_('Play / Stop'))

        _insertSeparator(1)

        self.balanceSliderImgLeft = gtk.Image()
        self.balanceSliderImgRight = gtk.Image()
        self.balanceSliderImgLeft.set_from_file(Config.IMAGE_ROOT + 'dru2.png')
        self.balanceSliderImgRight.set_from_file(Config.IMAGE_ROOT + 'instr2.png')
        self.balanceSliderImgLeftTool = gtk.ToolItem()
        self.balanceSliderImgLeftTool.add(self.balanceSliderImgLeft)
        self.balanceSliderImgRightTool = gtk.ToolItem()
        self.balanceSliderImgRightTool.add(self.balanceSliderImgRight)
        self.balanceSliderAdj = gtk.Adjustment(value=50, lower=0, upper=100, step_incr=1, page_incr=0, page_size=0)
        self.balanceSliderAdj.connect("value_changed" , self.miniTamTam.handleBalanceSlider)
        self.balanceSlider =  gtk.HScale(adjustment = self.balanceSliderAdj)
        self.balanceSlider.set_size_request(250,15)
        self.balanceSlider.set_inverted(False)
        self.balanceSlider.set_draw_value(False)
        self.balanceSliderTool = gtk.ToolItem()
        self.balanceSliderTool.add(self.balanceSlider)
        self.insert(self.balanceSliderImgLeftTool, -1)
        self.insert(self.balanceSliderTool, -1)
        self.insert(self.balanceSliderImgRightTool, -1)
        self.balanceSliderImgLeft.show()
        self.balanceSliderImgLeftTool.show()
        self.balanceSliderImgRight.show()
        self.balanceSliderImgRightTool.show()
        self.balanceSlider.show()
        self.balanceSliderTool.show()
        self.balanceSliderTool.set_tooltip(self.tooltips, _('Balance'))

        _insertSeparator(1)

        self.reverbSliderImgRight = gtk.Image()
        self.reverbSliderImgRight.set_from_file(Config.IMAGE_ROOT + 'reverb0.png')
        self.reverbSliderImgRightTool = gtk.ToolItem()
        self.reverbSliderImgRightTool.add(self.reverbSliderImgRight)

        self.reverbSliderAdj = gtk.Adjustment(value=0.1, lower=0, upper=1, step_incr=0.1, page_incr=0, page_size=0)
        self.reverbSliderAdj.connect("value_changed" , self.miniTamTam.handleReverbSlider)
        self.reverbSlider =  gtk.HScale(adjustment = self.reverbSliderAdj)
        self.reverbSlider.set_size_request(250,15)
        self.reverbSlider.set_inverted(False)
        self.reverbSlider.set_draw_value(False)
        self.reverbSliderTool = gtk.ToolItem()
        self.reverbSliderTool.add(self.reverbSlider)
        self.insert(self.reverbSliderTool, -1)
        self.insert(self.reverbSliderImgRightTool, -1)
        self.reverbSliderImgRight.show()
        self.reverbSliderImgRightTool.show()
        self.reverbSlider.show()
        self.reverbSliderTool.show()
        self.reverbSliderTool.set_tooltip(self.tooltips, _('Reverb'))


class recordToolbar(gtk.Toolbar):
    def __init__(self,toolbox, miniTamTam):
        gtk.Toolbar.__init__(self)

        def _insertSeparator(x = 1):
            for i in range(x):
                self.separator = gtk.SeparatorToolItem()
                self.separator.set_draw(True)
                self.insert(self.separator,-1)
                self.separator.show()

        self.toolbox = toolbox
        self.miniTamTam = miniTamTam

        self.micRec1Button = ToolButton('rec1')
        self.micRec1Button.connect('clicked',self.miniTamTam.micRec,'mic1')
        self.insert(self.micRec1Button, -1)
        self.micRec1Button.show()
        self.micRec1Button.set_tooltip(_('Record microphone into slot 1'))

        self.micRec2Button = ToolButton('rec2')
        self.micRec2Button.connect('clicked',self.miniTamTam.micRec,'mic2')
        self.insert(self.micRec2Button, -1)
        self.micRec2Button.show()
        self.micRec2Button.set_tooltip(_('Record microphone into slot 2'))

        self.micRec3Button = ToolButton('rec3')
        self.micRec3Button.connect('clicked',self.miniTamTam.micRec,'mic3')
        self.insert(self.micRec3Button, -1)
        self.micRec3Button.show()
        self.micRec3Button.set_tooltip(_('Record microphone into slot 3'))

        self.micRec4Button = ToolButton('rec4')
        self.micRec4Button.connect('clicked',self.miniTamTam.micRec,'mic4')
        self.insert(self.micRec4Button, -1)
        self.micRec4Button.show()
        self.micRec4Button.set_tooltip(('Record microphone into slot 4'))

        _insertSeparator()

        self.keyboardRecButton = ToggleToolButton('keyrec')
        self.keyboardRecButton.connect('clicked', self.miniTamTam.sequencer.handleRecordButton )
        self.insert(self.keyboardRecButton, -1)
        self.keyboardRecButton.show()
        self.keyboardRecButton.set_tooltip(_('Click to record a loop'))

        self.keyboardRecOverButton = ToggleToolButton('overrec')
        self.keyboardRecOverButton.connect('clicked', self.miniTamTam.sequencer.handleOverButton )
        self.insert(self.keyboardRecOverButton, -1)
        self.keyboardRecOverButton.show()
        self.keyboardRecOverButton.set_tooltip(_('Click to add a loop'))
        #self.keyboardRecOverButton.set_sensitive(False)

        self.keyboardRecMinusButton = ToolButton('minusrec')
        self.keyboardRecMinusButton.connect('clicked', self.miniTamTam.sequencer.clearSequencer )
        self.insert(self.keyboardRecMinusButton, -1)
        self.keyboardRecMinusButton.show()
        self.keyboardRecMinusButton.set_tooltip(_('Click to clear all loops'))

        _insertSeparator()

        self.loopSetButton = ToggleToolButton('loop')
        self.loopSetButton.connect('clicked', self.miniTamTam.handleLoopSettingsBtn)
        self.insert(self.loopSetButton, -1)
        self.loopSetButton.show()
        self.loopSetButton.set_tooltip(_('Add new sound'))
