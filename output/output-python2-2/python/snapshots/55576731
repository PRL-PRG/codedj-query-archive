import pygtk
pygtk.require('2.0')
import gtk
from types import *
from math import sqrt

class NoteParametersWindow( gtk.Window ):
    def __init__(self, note, getNoteParameters ):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        if type( note ) is DictType:
            self.trackDictionary = note
            self.inputType = 0
        elif type( note ) is InstanceType:
            self.note = note
            self.inputType = 1

        self.tied = False
        self.overlap = False
        self.filterType = 0    
        self.getNoteParameters = getNoteParameters

        self.setupWindow()
        self.show_all()

    def delete_event(self, widget, event, data=None):
        return False

    def setupWindow( self ):
        self.set_position(gtk.WIN_POS_CENTER_ON_PARENT)
        self.set_title("Note parameters")
        self.set_border_width(5)
        self.connect("delete_event", self.delete_event)
        self.parametersBox = gtk.VBox(False, 2)
        self.add(self.parametersBox)

        if self.inputType == 0:
            self.pitchAdjust = self.initSlider( " pitch ",0 ,-12 , 12, 1, 0, 0 )
            self.amplitudeAdjust = self.initSlider( " amplitude ", 1., 0, 2, .01, 0, 2 )
            self.panAdjust = self.initSlider( " pan ", .5, 0, 1, .01, 0, 2)
            self.reverbSendAdjust = self.initSlider( " reverb gain ", 1, 0, 4, .01, 0, 2 )
            self.attackAdjust = self.initSlider( "attack", .002, 0, 1, .001, 0, 3 )
            self.decayAdjust = self.initSlider( "decay", .098, 0, 1, .001, 0, 3 )
        elif self.inputType == 1:
            self.pitchAdjust = self.initSlider( " pitch ",self.note.pitch, 24, 48, 1, 0, 0 )
            self.amplitudeAdjust = self.initSlider( " amplitude ", self.note.amplitude, 0, 1, .01, 0, 2 )
            self.panAdjust = self.initSlider( " pan ", self.note.pan, 0, 1, .01, 0, 2)
            self.reverbSendAdjust = self.initSlider( " reverb gain ", self.note.reverbSend, 0, 1, .01, 0, 2 )
            self.attackAdjust = self.initSlider( "attack", self.note.attack, 0, 1, .001, 0, 3 )
            self.decayAdjust = self.initSlider( "decay", self.note.decay, 0, 1, .001, 0, 3 )

        self.toggleBox = gtk.HBox(False, 2)
        self.parametersBox.pack_start(self.toggleBox)

        tiedButton = gtk.ToggleButton("tied note")
        tiedButton.connect("toggled", self.tiedCallback, 0)
        self.toggleBox.pack_start(tiedButton, True, True, 0)
        tiedButton.show()

        overlapButton = gtk.ToggleButton("overlaped note")
        overlapButton.connect("toggled", self.overlapCallback, 0)
        self.toggleBox.pack_start(overlapButton, True, True, 0)
        overlapButton.show()

        if self.inputType == 0:
            tiedButton.set_active(False)
        elif self.inputType == 1:
            tiedButton.set_active(self.note.tied)

        self.parametersBox.pack_start(gtk.Label("filter"), False, False, 0)
        self.filterBox = gtk.HBox( False, 2 )
        self.parametersBox.pack_start( self.filterBox )

        offButton = gtk.RadioButton(None, "off")
        offButton.connect("toggled", self.filterCallback, 0)
        self.filterBox.pack_start(offButton, True, True, 0)
        offButton.show()

        lpButton = gtk.RadioButton(offButton, "lp")
        lpButton.connect("toggled", self.filterCallback, 1)
        self.filterBox.pack_start(lpButton, True, True, 0)
        lpButton.show()

        hpButton = gtk.RadioButton(offButton, "hp")
        hpButton.connect("toggled", self.filterCallback, 2)
        self.filterBox.pack_start(hpButton, True, True, 0)
        hpButton.show()

        bpButton = gtk.RadioButton(offButton, "bp")
        bpButton.connect("toggled", self.filterCallback, 3)
        self.filterBox.pack_start(bpButton, True, True, 0)
        bpButton.show()

        brButton = gtk.RadioButton(offButton, "br")
        brButton.connect("toggled", self.filterCallback, 4)
        self.filterBox.pack_start(brButton, True, True, 0)
        brButton.show()

        if self.inputType == 0:
            offButton.set_active(True)
        elif self.inputType == 1:
            if self.note.filterType == 0: offButton.set_active(True)
            if self.note.filterType == 1: lpButton.set_active(True)
            if self.note.filterType == 2: hpButton.set_active(True)
            if self.note.filterType == 3: bpButton.set_active(True)
            if self.note.filterType == 4: brButton.set_active(True)

        self.scaleFilterBox = gtk.HBox( False, 2 )
        self.parametersBox.pack_start( self.scaleFilterBox )

        if self.inputType == 0:
            self.filterCutoffAdjust = gtk.Adjustment(.25, 0, 1, 0.001, 0.001, 0)
            currentValue = self.filterCutoff = 1300
            self.filterCutoff = currentValue
        elif self.inputType == 1:
            currentValue = self.filterCutoff = self.note.filterCutoff
            scaleCurrentValue = sqrt( ( self.note.filterCutoff - 20 ) / 19980. )
            self.filterCutoffAdjust = gtk.Adjustment(scaleCurrentValue, 0, 1, 0.001, 0.001, 0)
        slider = gtk.HScale(self.filterCutoffAdjust)
        slider.set_update_policy(0)
        slider.set_digits(3)
        slider.set_draw_value(False)
        slider.set_size_request(250, 25)
        slider.connect("value-changed", self.handleCutoffScale) 
        self.scaleFilterBox.pack_start(slider)
        self.filterCutoffLabel = gtk.Label( str( currentValue ) )
        self.scaleFilterBox.pack_start(self.filterCutoffLabel, False, False, 0)

        applyButton = gtk.Button( " apply " )
        applyButton.connect( "clicked", self.applyParametersChange )
        self.parametersBox.pack_start( applyButton )

    def applyParametersChange( self, data=None ):
        self.getNoteParameters()
        self.window.destroy()

    def filterCallback( self, widget, data=None):
        if widget.get_active():
            self.filterType = data

    def tiedCallback( self, widget, data=None ):
        self.tied = widget.get_active()

    def overlapCallback( self, widget, data=None ):
        self.overlap = widget.get_active()

    def handleCutoffScale( self, widget, data=None ):
        self.filterCutoff = int( pow( self.filterCutoffAdjust.value, 2) * 19980 + 20 )
        self.filterCutoffLabel.set_text( str( self.filterCutoff  ) )

    def initSlider(self, label, initValue, minValue, maxValue, incStep, policy, digits):
        sliderAdjust = gtk.Adjustment(initValue, minValue, maxValue, incStep, incStep, 0)
        slider = gtk.HScale(sliderAdjust)
        slider.set_update_policy(policy)
        slider.set_digits(digits)
        slider.set_value_pos(1)
        slider.set_size_request(250, 25)
        self.parametersBox.pack_start(gtk.Label(label), False, False, 0)
        self.parametersBox.pack_start(slider)
        return sliderAdjust
