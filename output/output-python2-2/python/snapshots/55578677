import pygtk
pygtk.require('2.0')
import gtk
from types import *

class NoteParametersWindow( gtk.Window ):
    def __init__(self, note, getNoteParameters ):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        if type( note ) is DictType:
            self.trackDictionary = note
            self.inputType = 0
        elif type( note ) is InstanceType:
            self.note = note
            self.inputType = 1
    
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

        applyButton = gtk.Button( " apply " )
        applyButton.connect( "clicked", self.applyParametersChange )
        self.parametersBox.pack_start( applyButton )

    def applyParametersChange( self, data=None ):
        self.getNoteParameters()
        self.window.destroy()

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
