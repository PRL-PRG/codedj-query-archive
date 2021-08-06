
import pygtk
pygtk.require( '2.0' )
import gtk
import gobject

import os, commands

from gettext import gettext as _

from sugar.graphics.palette import Palette, WidgetInvoker
from sugar.graphics.radiotoolbutton import RadioToolButton
from sugar.graphics.toolbutton import ToolButton
from sugar.graphics.toggletoolbutton import ToggleToolButton
from sugar.graphics.combobox import ComboBox
from sugar.graphics.toolcombobox import ToolComboBox

from common.Util.ThemeWidgets import *

import common.Config as Config


class JamToolbar( gtk.Toolbar ):

    def __init__( self, owner ):
        gtk.Toolbar.__init__( self )

        self.owner = owner

        self.toolItem = {}

        self.volumeImg = gtk.Image()

        self.stopButton = ToolButton('media-playback-stop')
        self.stopButton.connect('clicked',self.handleStopButton)
        self.insert(self.stopButton, -1)
        self.stopButton.show()
        self.stopButton.set_tooltip(_('Stop'))

        self.pauseButton = ToggleToolButton('media-playback-pause')
        self.pauseButton.connect('clicked',self.handlePauseButton)
        self.insert(self.pauseButton, -1)
        self.pauseButton.show()
        self.pauseButton.set_tooltip(_('pause'))

        self.volumeAdjustment = gtk.Adjustment( 0.0, 0, 1.0, 0.1, 0.1, 0 )
        self.volumeAdjustment.connect( 'value-changed', self.handleVolume )
        self.volumeSlider = gtk.HScale( adjustment = self.volumeAdjustment )
        self.volumeSlider.set_size_request( 270, -1 )
        self.volumeSlider.set_draw_value( False )
        self._add_tooltip( self.volumeSlider, _("Master Volume") )
        self._insert_widget( self.volumeSlider, -1 )
        self._insert_widget( self.volumeImg, -1 )

        self._insert_separator( True )

        self.tempoImg = gtk.Image()

        self.delayedTempo = 0 # used to store tempo updates while the slider is active
        self.tempoSliderActive = False

        self.tempoAdjustment = gtk.Adjustment( Config.PLAYER_TEMPO_LOWER, Config.PLAYER_TEMPO_LOWER, Config.PLAYER_TEMPO_UPPER+1, 10, 10, 0 )
        self.tempoAdjustmentHandler = self.tempoAdjustment.connect( 'value-changed', self.handleTempo )
        self.tempoSlider = gtk.HScale( adjustment = self.tempoAdjustment )
        self.tempoSlider.set_size_request( 270, -1 )
        self.tempoSlider.set_draw_value( False )
        self.tempoSlider.connect("button-press-event", self.handleTempoSliderPress)
        self.tempoSlider.connect("button-release-event", self.handleTempoSliderRelease)
        self._add_tooltip( self.tempoSlider, _("Tempo") )
        self._insert_widget( self.tempoSlider, -1 )
        self._insert_widget( self.tempoImg, -1 )

        self._insert_separator( True )

        label = gtk.Label( _("Sync to:") )
        self.syncLabel = gtk.ToolItem()
        self.syncLabel.add( label )
        self.insert( self.syncLabel, -1 )

        self.comboBox = ComboBox()
        self.comboBox.append_item( 1, _("1 Beat") )
        self.comboBox.append_item( 2, _("2 Beats") )
        self.comboBox.append_item( 3, _("3 Beats") )
        self.comboBox.append_item( 4, _("4 Beats") )
        self.comboBox.append_item( 5, _("5 Beats") )
        self.comboBox.append_item( 6, _("6 Beats") )
        self.comboBox.append_item( 7, _("7 Beats") )
        self.comboBox.append_item( 8, _("8 Beats") )
        self.comboBox.append_item( 9, _("9 Beats") )
        self.comboBox.append_item( 10, _("10 Beats") )
        self.comboBox.append_item( 11, _("11 Beats") )
        self.comboBox.append_item( 12, _("12 Beats") )
        self.comboBox.set_active( 4 - 1 ) # default 4 beats
        self.comboBox.connect( "changed", self.changeSync )
        self.syncBox = ToolComboBox( self.comboBox )
        self.insert( self.syncBox, -1 )

        self.show_all()

    #def _add_palette( self, widget, palette, position = Palette.DEFAULT ):
    def _add_palette( self, widget, palette ):
        widget._palette = palette
        widget._palette.props.invoker = WidgetInvoker( widget )
        #widget._palette.set_property( "position", position )

    def _add_tooltip( self, widget, tooltip ):
        #self._add_palette( widget, Palette( tooltip ), Palette.DEFAULT )
        self._add_palette( widget, Palette( tooltip ) )

    def _insert_widget( self, widget, pos ):
        self.toolItem[ widget ] = gtk.ToolItem()
        self.toolItem[ widget ].add( widget )
        self.insert( self.toolItem[ widget ], pos )

    def _insert_separator( self, expand = False ):
        separator = gtk.SeparatorToolItem()
        separator.set_draw( False )
        separator.set_expand( expand )
        self.insert( separator, -1 )

    def setSyncBeats( self, beats ):
        self.comboBox.set_active( beats - 1 )

    def changeSync( self, widget ):
        self.owner._setSyncBeats( widget.get_active() + 1 )

    def mapRange( self, value, ilower, iupper, olower, oupper ):
        if value == iupper:
            return oupper
        return olower + int( (oupper-olower+1)*(value-ilower)/float(iupper-ilower) )

    def handleStopButton( self, widget ):
        self.owner.setStopped()

    def handlePauseButton (self, widget ):
        if widget.get_active():
            self.owner.setPaused(True)
        else:
            self.owner.setPaused(False)

    def handleVolume( self, widget ):
        self.owner._setVolume( widget.get_value() )

        img = self.mapRange( widget.value, widget.lower, widget.upper, 0, 3 )
        self.volumeImg.set_from_file(Config.TAM_TAM_ROOT + '/icons/volume' + str(img) + '.svg')

    def handleTempo( self, widget ):
        if self.owner.network.isPeer():
            self.owner.requestTempoChange(int(widget.get_value()))
        else:
            self._updateTempo( widget.get_value() )

    def setTempo( self, tempo, quiet = False ):
        if self.tempoSliderActive:
            self.delayedTempo = tempo 
        elif quiet:
            self.tempoAdjustment.handler_block( self.tempoAdjustmentHandler )
            self.tempoAdjustment.set_value( self.delayedTempo )
            self._updateTempo( tempo )
            self.tempoAdjustment.handler_unblock( self.tempoAdjustmentHandler )
        else:
            self.tempoAdjustment.set_value( tempo )

    def _updateTempo( self, tempo ):
        self.owner._setTempo( tempo )

        img = self.mapRange( tempo, self.tempoAdjustment.lower, self.tempoAdjustment.upper, 1, 8 )
        self.tempoImg.set_from_file(Config.TAM_TAM_ROOT + '/icons/tempo' + str(img) + '.svg')

    def handleTempoSliderPress(self, widget, event):
        self.tempoSliderActive = True

    def handleTempoSliderRelease(self, widget, event):
        self.tempoSliderActive = False
        if self.owner.network.isPeer() and self.delayedTempo != 0:
            if self.owner.getTempo() != self.delayedTempo:
                self.setTempo( self.delayedTempo, True )
            self.delayedTempo = 0
            self.owner.sendSyncQuery()


class recordToolbar(gtk.Toolbar):
    def __init__(self, jam):
        gtk.Toolbar.__init__(self)

        def _insertSeparator(x = 1):
            for i in range(x):
                self.separator = gtk.SeparatorToolItem()
                self.separator.set_draw(True)
                self.insert(self.separator,-1)
                self.separator.show()

        #self.toolbox = toolbox
        self.jam = jam

        self.micRec1Button = ToolButton('rec1')
        self.micRec1Button.connect('clicked',self.jam.micRec,'mic1')
        self.insert(self.micRec1Button, -1)
        self.micRec1Button.show()
        self.micRec1Button.set_tooltip(_('Record microphone into slot 1'))

        self.micRec2Button = ToolButton('rec2')
        self.micRec2Button.connect('clicked',self.jam.micRec,'mic2')
        self.insert(self.micRec2Button, -1)
        self.micRec2Button.show()
        self.micRec2Button.set_tooltip(_('Record microphone into slot 2'))

        self.micRec3Button = ToolButton('rec3')
        self.micRec3Button.connect('clicked',self.jam.micRec,'mic3')
        self.insert(self.micRec3Button, -1)
        self.micRec3Button.show()
        self.micRec3Button.set_tooltip(_('Record microphone into slot 3'))

        self.micRec4Button = ToolButton('rec4')
        self.micRec4Button.connect('clicked',self.jam.micRec,'mic4')
        self.insert(self.micRec4Button, -1)
        self.micRec4Button.show()
        self.micRec4Button.set_tooltip(('Record microphone into slot 4'))

        _insertSeparator()

        self._loopSettingsPalette = LoopSettingsPalette(_('Add new Sound'), self.jam)
        self.loopSetButton = ToggleToolButton('loop')
        self.loopSetButton.set_palette(self._loopSettingsPalette)
        self.insert(self.loopSetButton, -1)
        self.loopSetButton.show()

        self.show_all()

class DesktopToolbar( gtk.Toolbar ):

    def __init__( self, owner ):
        gtk.Toolbar.__init__( self )

        self.owner = owner

        self._insert_separator( True )

        self.desktop = []

        btn = RadioToolButton( 'preset1', group = None )
        btn.connect( 'toggled', self.setDesktop, 0 )
        btn.set_tooltip( _('Desktop 1') )
        self.insert( btn, -1 )
        self.desktop.append( btn )

        for i in range(2,11):
            btn = RadioToolButton( 'preset%d'%i, group = self.desktop[0] )
            btn.connect( 'toggled', self.setDesktop, i-1 )
            btn.set_tooltip( _('Desktop %d'%i) )
            self.insert( btn, -1 )
            self.desktop.append( btn )

        self._insert_separator( True )

        self.show_all()

    def _insert_separator( self, expand = False ):
        separator = gtk.SeparatorToolItem()
        separator.set_draw( False )
        separator.set_expand( expand )
        self.insert( separator, -1 )

    def getDesktopButton( self, which ):
        return self.desktop[which]

    def setDesktop( self, widget, which ):
        if widget.get_active():
            self.owner._setDesktop( which )
class LoopSettingsPalette( Palette ):
    def __init__( self, label, jam ):
        Palette.__init__( self, label )
        self.connect('popup', self.handlePopup)
        self.connect('popdown', self.handlePopdown)

        self.jam = jam

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
            self.jam.loopSettingsPlayStop(True, self.loopedSound)

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
        self.jam.load_ls_instrument(self.sndname)
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
        self.jam.loopSettingsChannel('lstart', self.start)

    def handleEnd(self, widget, data=None):
        self.end = self.endAdjust.value
        if self.end < self.start:
            self.end = self.start
        self.jam.loopSettingsChannel('lend', self.end)

    def handleDur(self, widget, data=None):
        self.dur = self.durAdjust.value
        self.jam.loopSettingsChannel('ldur', self.dur)

    def handleVol(self, widget, data=None):
        self.volume = self.volAdjust.value
        self.jam.loopSettingsChannel('lvol', self.volume)

    def handlePlayButton(self, widget, data=None):
        if self.ok:
            self.jam.loopSettingsPlayStop(widget.get_active(), self.loopedSound)
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
