#!/usr/bin/env python

import gtk
import gobject
import os, commands
import common.Config as Config
from common.Util.ThemeWidgets import *

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

        self._loopSettingsPalette = LoopSettingsPalette(_('Add new Sound'), self.miniTamTam)
        self.loopSetButton = ToggleToolButton('loop')
        self.loopSetButton.set_palette(self._loopSettingsPalette)
        self.insert(self.loopSetButton, -1)
        self.loopSetButton.show()

class LoopSettingsPalette( Palette ):
    def __init__( self, label, mini ):
        Palette.__init__( self, label )
        self.connect('popup', self.handlePopup)
        self.connect('popdown', self.handlePopdown)

        self.mini = mini

        self.tooltips = gtk.Tooltips()
        self.loopedSound = False
        self.soundLength = 1.00
        self.start = 0
        self.end = 1.00
        self.dur = 0.01
        self.volume = 1
        self.register = 0
        self.ok = True

        self.mainBox = gtk.VBox()

        self.controlsBox = gtk.HBox()

        self.GUI = {}

        self.soundBox = gtk.HBox()
        self.soundLabel = gtk.Label(_('Sound: '))
        self.soundMenuBox = BigComboBox()
        self.sounds = os.listdir(Config.SNDS_DIR)
        for sound in self.sounds:
            self.soundMenuBox.append_item(self.sounds.index(sound), sound)
        self.soundMenuBox.connect('changed', self.handleSound)
        self.soundBox.pack_start(self.soundLabel, False, False, padding=10)
        self.soundBox.pack_start(self.soundMenuBox, False, False, padding=10)

        self.mainBox.pack_start(self.soundBox, False, False, 10)

        nameBox = gtk.VBox()
        self.nameEntry = gtk.Entry()
        entrycolor = gtk.gdk.Color()
        self.nameEntry.modify_text(gtk.STATE_NORMAL, entrycolor)
        self.nameEntry.set_text("name_of_the_sound")
        nameBox.pack_start(self.nameEntry)
        self.mainBox.pack_start(nameBox, False, False, 10)

        registerBox = gtk.HBox()
        self.registerBoxLabel = gtk.Label(_('Register: '))
        self.registerMenuBox = BigComboBox()
        self.registers = ['LOW', 'MID', 'HIGH', 'PUNCH']
        for reg in self.registers:
            self.registerMenuBox.append_item(self.registers.index(reg), reg)
        self.registerMenuBox.connect('changed', self.handleRegister)
        registerBox.pack_start(self.registerBoxLabel, False, False, padding=10)
        registerBox.pack_end(self.registerMenuBox, False, False, padding=10)
        self.mainBox.pack_start(registerBox, False, False, 10)

        loopedBox = gtk.HBox()
        loopedLabel = gtk.Label("Looped sound: ")
        loopedToggle = ImageToggleButton(Config.IMAGE_ROOT+"checkOff.svg",Config.IMAGE_ROOT+"checkOn.svg")
        loopedToggle.connect('button-press-event', self.handleLooped )
        loopedBox.pack_start(loopedLabel, False, False, padding=10)
        loopedBox.pack_end(loopedToggle, False, False, padding=10)
        self.mainBox.pack_start(loopedBox, False, False, 10)

        startBox = gtk.VBox()
        self.startAdjust = gtk.Adjustment( 0.01, 0, 1., .001, .001, 0)
        self.GUI['startSlider'] = gtk.VScale( adjustment = self.startAdjust )
        self.startAdjust.connect("value-changed", self.handleStart)
        self.GUI['startSlider'].set_inverted(True)
        self.GUI['startSlider'].set_size_request(50, 200)
        self.GUI['startSlider'].set_digits(3)
        self.handleStart( self.startAdjust )
        startBox.pack_start(self.GUI['startSlider'], True, True, 5)
        self.controlsBox.pack_start(startBox)

        endBox = gtk.VBox()
        self.endAdjust = gtk.Adjustment( 0.9, 0, 1, .001, .001, 0)
        self.GUI['endSlider'] = gtk.VScale( adjustment = self.endAdjust )
        self.endAdjust.connect("value-changed", self.handleEnd)
        self.GUI['endSlider'].set_inverted(True)
        self.GUI['endSlider'].set_size_request(50, 200)
        self.GUI['endSlider'].set_digits(3)
        self.handleEnd( self.endAdjust )
        endBox.pack_start(self.GUI['endSlider'], True, True, 5)
        self.controlsBox.pack_start(endBox)

        durBox = gtk.VBox()
        self.durAdjust = gtk.Adjustment( 0.01, 0, 0.2, .001, .001, 0)
        self.GUI['durSlider'] = gtk.VScale( adjustment = self.durAdjust )
        self.durAdjust.connect("value-changed", self.handleDur)
        self.GUI['durSlider'].set_inverted(True)
        self.GUI['durSlider'].set_size_request(50, 200)
        self.GUI['durSlider'].set_digits(3)
        self.handleDur( self.durAdjust )
        durBox.pack_start(self.GUI['durSlider'], True, True, 5)
        self.controlsBox.pack_start(durBox)

        volBox = gtk.VBox()
        self.volAdjust = gtk.Adjustment( 1, 0, 2, .01, .01, 0)
        self.GUI['volSlider'] = gtk.VScale( adjustment = self.volAdjust )
        self.volAdjust.connect("value-changed", self.handleVol)
        self.GUI['volSlider'].set_inverted(True)
        self.GUI['volSlider'].set_size_request(50, 200)
        self.GUI['volSlider'].set_digits(3)
        self.handleVol( self.volAdjust )
        volBox.pack_start(self.GUI['volSlider'], True, True, 5)
        self.controlsBox.pack_start(volBox)

        self.mainBox.pack_start(self.controlsBox, False, False, 10)

        previewBox = gtk.VBox()
        self.playStopButton = ImageToggleButton(Config.IMAGE_ROOT + 'miniplay.png', Config.IMAGE_ROOT + 'stop.png')
        self.playStopButton.connect('button-press-event' , self.handlePlayButton)
        previewBox.pack_start(self.playStopButton)
        self.mainBox.pack_start(previewBox, False, False, 10)

        checkBox = gtk.VBox()
        checkButton = ImageButton(Config.TAM_TAM_ROOT + '/icons/accept.svg')
        checkButton.connect('clicked' , self.handleCheck)
        checkBox.pack_start(checkButton)
        self.mainBox.pack_start(checkBox, False, False, 10)

        self.mainBox.show_all()
        self.set_content(self.mainBox)

    def handlePopup(self, widget, data=None):
        self.setButtonState()
        self.soundMenuBox.remove_all()
        self.sounds = os.listdir(Config.SNDS_DIR)
        for sound in self.sounds:
            self.soundMenuBox.append_item(self.sounds.index(sound), sound)
        self.nameEntry.set_text("name_of_the_sound")

    def handlePopdown(self, widget, data=None):
        if self.playStopButton.get_active() == True:
            self.mini.loopSettingsPlayStop(True, self.loopedSound)

    def handleSound(self, widget, data=None):
        self.sndname = self.sounds[widget.props.value]
        fullname = Config.SNDS_DIR + '/' + self.sndname
        results = commands.getstatusoutput("du -b %s" % fullname)
        if results[0] == 0:
            list = results[1].split()
            soundLength = float(list[0]) / 2 / 16000.
        self.nameEntry.set_text(self.sndname)
        self.set_values(soundLength)
        self.startAdjust.set_all( 0.01, 0, soundLength, .001, .001, 0)
        self.endAdjust.set_all( soundLength-0.01, 0, soundLength, .001, .001, 0)
        self.timeoutLoad = gobject.timeout_add(2000, self.loopSettingsDelay)

    def loopSettingsDelay(self):
        self.mini.load_ls_instrument(self.sndname)
        gobject.source_remove( self.timeoutLoad )

    def handleCheck(self, widget):
        if self.nameEntry.get_text() != self.sndname:
            oldName = self.sndname
            self.sndname = self.nameEntry.get_text()
            copy = True
        else:
            copy = False

        ofile = open(Config.SNDS_INFO_DIR + '/' + self.sndname, 'w')
        if self.loopedSound:
            tied = str(Config.INST_TIED)
        else:
            tied = str(Config.INST_SIMP)
        register = str(self.register)
        category = 'mysounds'
        start = str(self.start)
        end = str(self.end)
        dur = str(self.dur)
        vol = str(self.volume)

        ofile.write('TamTam idf v1\n')
        ofile.write(self.sndname + '\n')
        ofile.write(tied + '\n')
        ofile.write(register + '\n')
        ofile.write(start + '\n')
        ofile.write(end + '\n')
        ofile.write(dur + '\n')
        ofile.write(vol + '\n')
        ofile.write(self.sndname + '\n')
        ofile.write(Config.LIB_DIR+"/Images/"+self.sndname+".png\n")
        ofile.write(category)
        ofile.close()
        if copy:
            (s,o) = commands.getstatusoutput('cp ' + Config.SNDS_DIR + '/' + oldName + ' ' + Config.SNDS_DIR + '/' + self.sndname)

    def set_values(self, soundLength):
        self.soundLength = soundLength
        self.handleStart(self.GUI['startSlider'])
        self.handleEnd(self.GUI['endSlider'])

    def handleLooped(self, widget, data=None):
        if widget.get_active() == True:
            self.loopedSound = False
        else:
            self.loopedSound = True

    def handleRegister(self, widget, data=None):
        self.register = self.registers[widget.props.value]

    def handleStart(self, widget, data=None):
        self.start = self.startAdjust.value
        if self.start > self.end:
            self.start = self.end
        self.mini.loopSettingsChannel('lstart', self.start)

    def handleEnd(self, widget, data=None):
        self.end = self.endAdjust.value
        if self.end < self.start:
            self.end = self.start
        self.mini.loopSettingsChannel('lend', self.end)

    def handleDur(self, widget, data=None):
        self.dur = self.durAdjust.value
        self.mini.loopSettingsChannel('ldur', self.dur)

    def handleVol(self, widget, data=None):
        self.volume = self.volAdjust.value
        self.mini.loopSettingsChannel('lvol', self.volume)

    def handlePlayButton(self, widget, data=None):
        if self.ok:
            self.mini.loopSettingsPlayStop(widget.get_active(), self.loopedSound)
            if self.loopedSound == False and widget.get_active() == False:
                self.timeoutStop = gobject.timeout_add(int(self.soundLength * 1000)+500, self.playButtonState)

    def setButtonState(self):
        self.ok = False
        self.playStopButton.set_active(False)
        self.ok = True

    def playButtonState(self):
        self.ok = False
        self.playStopButton.set_active(False)
        gobject.source_remove(self.timeoutStop)
        self.ok = True
