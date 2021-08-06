import pygtk
pygtk.require('2.0')
import gtk
import os
from Util.ThemeWidgets import *
import Config
Tooltips = Config.Tooltips()

class LoopSettings( gtk.VBox ):
    def __init__( self, popup, playFunction ):
        gtk.VBox.__init__( self )
        self.tooltips = gtk.Tooltips()
        self.popup = popup
        self.playFunction = playFunction

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
        self.mainBox.pack_start(categoryBox, False, False, 5)
        
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
        self.startAdjust = gtk.Adjustment( 0.01, 0, 0.5, .01, .01, 0)
        self.GUI['startSlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.startAdjust, 7 )
        self.startAdjust.connect("value-changed", self.handleStart)
        self.GUI['startSlider'].set_inverted(True)
        self.GUI['startSlider'].set_size_request(50, 200)
        self.startEntry = gtk.Entry()
        self.startEntry.set_width_chars(4)
        self.handleStart( self.startAdjust )        
        startBox.pack_start(self.GUI['startSlider'], True, True, 5)
        startBox.pack_start(self.startEntry, True, True, 5)
        self.controlsBox.pack_start(startBox)
        
        endBox = gtk.VBox()
        self.endAdjust = gtk.Adjustment( 0.9, 0, 1, .01, .01, 0)
        self.GUI['endSlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.endAdjust, 7 )
        self.endAdjust.connect("value-changed", self.handleEnd)
        self.GUI['endSlider'].set_inverted(True)
        self.GUI['endSlider'].set_size_request(50, 200)
        self.endEntry = gtk.Entry()
        self.endEntry.set_width_chars(4)
        self.handleEnd( self.endAdjust )
        endBox.pack_start(self.GUI['endSlider'], True, True, 5)
        endBox.pack_start(self.endEntry, True, True, 5)
        self.controlsBox.pack_start(endBox)        

        durBox = gtk.VBox()
        self.durAdjust = gtk.Adjustment( 0.01, 0, 0.2, .01, .01, 0)
        self.GUI['durSlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.durAdjust, 7 )
        self.durAdjust.connect("value-changed", self.handleDur)
        self.GUI['durSlider'].set_inverted(True)
        self.GUI['durSlider'].set_size_request(50, 200)
        self.durEntry = gtk.Entry()
        self.durEntry.set_width_chars(4)
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
        
        self.fixed.put( self.mainBox, 0, 0 )
        
        self.show_all()
        
    def set_name(self, name):
        self.nameEntry.set_text(name)
        
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
        self.startEntry.set_text(str(self.startAdjust.value))
        
    def handleEnd(self, widget, data=None):
        self.endEntry.set_text(str(self.endAdjust.value))
        
    def handleDur(self, widget, data=None):
        self.durEntry.set_text(str(self.durAdjust.value))
        
    def handlePlayButton(self, widget, data=None):
        self.playFunction(widget.get_active())