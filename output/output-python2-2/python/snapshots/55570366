import pygtk
pygtk.require('2.0')
import gtk
import gobject
import os
from common.Util.ThemeWidgets import *
import common.Config as Config
import commands
Tooltips = Config.Tooltips()

class LoopSettings( gtk.VBox ):
    def __init__( self, popup, playFunction, setChannelFunction, doneLoopSettingsPopup ):
        gtk.VBox.__init__( self )
        self.tooltips = gtk.Tooltips()
        self.popup = popup
        self.playFunction = playFunction
        self.setChannel = setChannelFunction
        self.doneLoopSettingsPopup = doneLoopSettingsPopup
        self.loopedSound = False
        self.soundLength = 1.00
        self.start = 0
        self.end = 1.00
        self.dur = 0.01
        self.register = 0
        self.ok = True

        self.settingsBox = gtk.HBox()
        self.pack_start(self.settingsBox)

        self.fixed = gtk.Fixed()
        self.settingsBox.pack_start(self.fixed)

        self.mainBox = gtk.VBox()

        self.controlsBox = gtk.HBox()

        self.GUI = {}

        nameBox = gtk.VBox()
        self.nameEntry = gtk.Entry()
        self.nameEntry.set_text("name_of_the_sound")
        nameBox.pack_start(self.nameEntry)
        self.mainBox.pack_start(nameBox, False, False, 5)

        loopedBox = gtk.HBox()
        loopedLabel = gtk.Label("Looped sound: ")
        loopedBox.pack_start(loopedLabel)
        loopedToggle = ImageToggleButton(Config.IMAGE_ROOT+"checkOff.svg",Config.IMAGE_ROOT+"checkOn.svg")
        loopedToggle.connect('button-press-event', self.handleLooped )
        loopedBox.pack_start(loopedToggle)
        self.mainBox.pack_start(loopedBox, False, False, 5)

        categoryBox = gtk.HBox()
        categoryMenu = gtk.MenuBar()
        cmenu = gtk.Menu()
        for cat in Config.CATEGORIES:
            if cat != 'all':
                entry = gtk.MenuItem(cat)
                cmenu.append(entry)
                entry.connect("activate", self.handleCategory, cat)
                entry.show()
        self.categoryButton = gtk.Button("Category")
        self.categoryButton.connect_object("event", self.categoryBtnPress, cmenu)
        categoryBox.pack_end(self.categoryButton)
        #self.mainBox.pack_start(categoryBox, False, False, 5)

        registerBox = gtk.HBox()
        registerMenu = gtk.MenuBar()
        rmenu = gtk.Menu()
        self.registerList = ['LOW', 'MID', 'HIGH', 'PUNCH']
        for reg in self.registerList:
            entry = gtk.MenuItem(reg)
            rmenu.append(entry)
            entry.connect("activate", self.handleRegister, self.registerList.index(reg))
            entry.show()
        self.registerButton = gtk.Button("Register")
        self.registerButton.connect_object("event", self.registerBtnPress, rmenu)
        registerBox.pack_end(self.registerButton)
        self.mainBox.pack_start(registerBox, False, False, 5)

        startBox = gtk.VBox()
        self.startAdjust = gtk.Adjustment( 0.01, 0, 1., .001, .001, 0)
        self.GUI['startSlider'] = ImageVScale( Config.IMAGE_ROOT + "sliderEditVolume.png", self.startAdjust, 7 )
        self.startAdjust.connect("value-changed", self.handleStart)
        self.GUI['startSlider'].set_inverted(True)
        self.GUI['startSlider'].set_size_request(50, 200)
        self.startEntry = gtk.Entry()
        self.startEntry.set_width_chars(5)
        self.handleStart( self.startAdjust )
        startBox.pack_start(self.GUI['startSlider'], True, True, 5)
        startBox.pack_start(self.startEntry, True, True, 5)
        self.controlsBox.pack_start(startBox)

        endBox = gtk.VBox()
        self.endAdjust = gtk.Adjustment( 0.9, 0, 1, .001, .001, 0)
        self.GUI['endSlider'] = ImageVScale( Config.IMAGE_ROOT + "sliderEditVolume.png", self.endAdjust, 7 )
        self.endAdjust.connect("value-changed", self.handleEnd)
        self.GUI['endSlider'].set_inverted(True)
        self.GUI['endSlider'].set_size_request(50, 200)
        self.endEntry = gtk.Entry()
        self.endEntry.set_width_chars(5)
        self.handleEnd( self.endAdjust )
        endBox.pack_start(self.GUI['endSlider'], True, True, 5)
        endBox.pack_start(self.endEntry, True, True, 5)
        self.controlsBox.pack_start(endBox)

        durBox = gtk.VBox()
        self.durAdjust = gtk.Adjustment( 0.01, 0, 0.2, .001, .001, 0)
        self.GUI['durSlider'] = ImageVScale( Config.IMAGE_ROOT + "sliderEditVolume.png", self.durAdjust, 7 )
        self.durAdjust.connect("value-changed", self.handleDur)
        self.GUI['durSlider'].set_inverted(True)
        self.GUI['durSlider'].set_size_request(50, 200)
        self.durEntry = gtk.Entry()
        self.durEntry.set_width_chars(5)
        self.handleDur( self.durAdjust )
        durBox.pack_start(self.GUI['durSlider'], True, True, 5)
        durBox.pack_start(self.durEntry, True, True, 5)
        self.controlsBox.pack_start(durBox)

        self.mainBox.pack_start(self.controlsBox, False, False, 5)

        previewBox = gtk.VBox()
        self.playStopButton = ImageToggleButton(Config.IMAGE_ROOT + 'miniplay.png', Config.IMAGE_ROOT + 'stop.png')
        self.playStopButton.connect('button-press-event' , self.handlePlayButton)
        previewBox.pack_start(self.playStopButton)
        self.mainBox.pack_start(previewBox, False, False, 5)

        checkBox = gtk.VBox()
        checkButton = ImageButton(Config.IMAGE_ROOT + 'check.png')
        checkButton.connect('clicked' , self.handleCheck)
        checkBox.pack_start(checkButton)
        self.mainBox.pack_start(checkBox, False, False, 5)

        self.fixed.put( self.mainBox, 0, 0 )

        self.show_all()

    def handleCheck(self, widget):
        ofile = open(Config.DATA_DIR + "/sounds_settings", 'a')
        name = self.nameEntry.get_text()
        if self.loopedSound:
            tied = str(Config.INST_TIED)
        else:
            tied = str(Config.INST_SIMP)
        register = str(self.register)
        melo = 'melo'
        category = 'mysounds'
        start = str(self.start)
        end = str(self.end)
        dur = str(self.dur)

        ofile.write(name + ' ' + tied + ' ' + register + ' ' + melo + ' ' + category + ' ' + start + ' ' + end + ' ' + dur + '\n')

        ofile.close()
        (s,o) = commands.getstatusoutput('cp ' + Config.SNDS_DIR + '/' + self.oldName + ' ' + Config.SNDS_DIR + '/' + name)
        self.doneLoopSettingsPopup()

    def set_name(self, name):
        self.oldName = name
        self.nameEntry.set_text('_' + name)

    def set_values(self, soundLength):
        self.soundLength = soundLength
        self.handleStart(self.GUI['startSlider'])
        self.handleEnd(self.GUI['endSlider'])

    def handleLooped(self, widget, data=None):
        if widget.get_active() == True:
            self.loopedSound = False
        else:
            self.loopedSound = True

    def categoryBtnPress(self, widget, event):
        if event.type == gtk.gdk.BUTTON_PRESS:
            widget.popup(None, None, None, event.button, event.time)
            return True
        return False

    def handleCategory(self, widget, category):
        self.category = category
        self.categoryButton.set_label(self.category)

    def registerBtnPress(self, widget, event):
        if event.type == gtk.gdk.BUTTON_PRESS:
            widget.popup(None, None, None, event.button, event.time)
            return True
        return False

    def handleRegister(self, widget, register):
        self.register = register
        self.registerButton.set_label(self.registerList[self.register])

    def handleStart(self, widget, data=None):
        self.startSlider = self.startAdjust.value
        self.start = self.startSlider * self.soundLength
        if self.start > self.end:
            self.start = self.end
        self.startEntry.set_text(str(self.start))
        self.setChannel('lstart', self.start)

    def handleEnd(self, widget, data=None):
        self.endSlider = self.endAdjust.value
        self.end = self.endSlider * self.soundLength
        if self.end < self.start:
            self.end = self.start
        self.endEntry.set_text(str(self.end))
        self.setChannel('lend', self.end)

    def handleDur(self, widget, data=None):
        self.dur = self.durAdjust.value
        self.durEntry.set_text(str(self.dur))
        self.setChannel('ldur', self.dur)

    def handlePlayButton(self, widget, data=None):
        if self.ok:
            self.playFunction(widget.get_active(), self.loopedSound)
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
