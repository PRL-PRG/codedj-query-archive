import pygtk
pygtk.require('2.0')
import gtk
from types import *
from math import sqrt
from Util.ThemeWidgets import *
import Config

class TrackProperties( gtk.Window ):
    def __init__(self):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        self.set_type_hint(gtk.gdk.WINDOW_TYPE_HINT_DIALOG)
        self.set_position( gtk.WIN_POS_CENTER )
        self.set_default_size(30, 300)
        self.move(270, 305)
        self.set_border_width(0)
        self.set_decorated(False)
        self.mainBox = RoundVBox(fillcolor="#FFF", bordercolor="#FFF")
        self.mainBox.set_radius(10)

        self.pitch = .4
        self.volume = .4
        self.pan = .4
        self.reverb = .4
        self.attack = .4
        self.decay = .4
        self.filterCutoff = .4
        self.filterType = 0

        self.setupWindow()
        self.add(self.mainBox)
        self.show_all()

    def delete_event(self, widget, event, data=None):
        return False

    def setupWindow( self ):
        self.connect("delete_event", self.delete_event)
        self.parametersBox = RoundVBox(fillcolor=Config.INST_BCK_COLOR, bordercolor=Config.PANEL_BCK_COLOR)
        self.parametersBox.set_border_width(1)
        self.parametersBox.set_radius(10)
        self.mainBox.pack_start(self.parametersBox)

        controlsBox = gtk.HBox()
 
        pitchBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        pitchBox.set_border_width(3)
        pitchBox.set_radius(10)
        self.pitchAdjust = gtk.Adjustment(.4, 0, 1, .01, .01, 0)
        self.pitchAdjust.connect("value-changed", self.handlePitch)
        pitchSlider = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.pitchAdjust, 7 )
        pitchSlider.set_inverted(True)
        pitchSlider.set_size_request(50, 250)
        self.pitchLabel = gtk.Image()
        self.pitchLabel.set_from_file(Config.IMAGE_ROOT + 'propPitch2.png')
        pitchBox.pack_start(pitchSlider, True, True, 5)
        pitchBox.pack_start(self.pitchLabel, False, padding=10)
        controlsBox.pack_start(pitchBox)

        volumeBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        volumeBox.set_border_width(3)
        volumeBox.set_radius(10)
        self.volumeAdjust = gtk.Adjustment(.4, 0, 1, .01, .01, 0)
        self.volumeAdjust.connect("value-changed", self.handleVolume)
        volumeSlider = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.volumeAdjust, 7 )
        volumeSlider.set_inverted(True)
        volumeSlider.set_size_request(50, 250)
        self.volumeLabel = gtk.Image()
        self.volumeLabel.set_from_file(Config.IMAGE_ROOT + 'propVolume1.png')
        volumeBox.pack_start(volumeSlider, True, True, 5)
        volumeBox.pack_start(self.volumeLabel, False, padding=10)
        controlsBox.pack_start(volumeBox)

        panBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        panBox.set_border_width(3)
        panBox.set_radius(10)
        self.panAdjust = gtk.Adjustment(.4, 0, 1, .01, .01, 0)
        self.panAdjust.connect("value-changed", self.handlePan)
        panSlider = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.panAdjust, 7 )
        panSlider.set_inverted(True)
        panSlider.set_size_request(50, 250)
        self.panLabel = gtk.Image()
        self.panLabel.set_from_file(Config.IMAGE_ROOT + 'propPan1.png')
        panBox.pack_start(panSlider, True, True, 5)
        panBox.pack_start(self.panLabel, False, padding=10)
        controlsBox.pack_start(panBox)

        reverbBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        reverbBox.set_border_width(3)
        reverbBox.set_radius(10)
        self.reverbAdjust = gtk.Adjustment(.4, 0, 1, .01, .01, 0)
        self.reverbAdjust.connect("value-changed", self.handleReverb)
        reverbSlider = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.reverbAdjust, 7 )
        reverbSlider.set_inverted(True)
        reverbSlider.set_size_request(50, 250)
        self.reverbLabel = gtk.Image()
        self.reverbLabel.set_from_file(Config.IMAGE_ROOT + 'propReverb2.png')
        reverbBox.pack_start(reverbSlider, True, True, 5)
        reverbBox.pack_start(self.reverbLabel, False, padding=10)
        controlsBox.pack_start(reverbBox)

        attackBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        attackBox.set_border_width(3)
        attackBox.set_radius(10)
        self.attackAdjust = gtk.Adjustment(.4, 0, 1, .01, .01, 0)
        self.attackAdjust.connect("value-changed", self.handleAttack)
        attackSlider = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.attackAdjust, 7 )
        attackSlider.set_inverted(True)
        attackSlider.set_size_request(50, 250)
        self.attackLabel = gtk.Image()
        self.attackLabel.set_from_file(Config.IMAGE_ROOT + 'propAtt1.png')
        attackBox.pack_start(attackSlider, True, True, 5)
        attackBox.pack_start(self.attackLabel, False, padding=10)
        controlsBox.pack_start(attackBox)

        decayBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        decayBox.set_border_width(3)
        decayBox.set_radius(10)
        self.decayAdjust = gtk.Adjustment(.4, 0, 1, .01, .01, 0)
        self.decayAdjust.connect("value-changed", self.handleDecay)
        decaySlider = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.decayAdjust, 7 )
        decaySlider.set_inverted(True)
        decaySlider.set_size_request(50, 250)
        self.decayLabel = gtk.Image()
        self.decayLabel.set_from_file(Config.IMAGE_ROOT + 'propDec1.png')
        decayBox.pack_start(decaySlider, True, True, 5)
        decayBox.pack_start(self.decayLabel, False, padding=10)
        controlsBox.pack_start(decayBox)

        filterBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        filterBox.set_border_width(3)
        filterBox.set_radius(10)

        filterSliderBox = gtk.HBox()
        self.cutoffAdjust = gtk.Adjustment(.4, 0, 1, .01, .01, 0)
        self.cutoffAdjust.connect("value-changed", self.handleFilter, 0)
        cutoffSlider = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.cutoffAdjust, 7 )
        cutoffSlider.set_inverted(True)
        cutoffSlider.set_size_request(50, 250)
        filterSliderBox.pack_start(cutoffSlider, True, True, 5)

        self.filterTypeAdjust = gtk.Adjustment( 0, 0, 2, 1, 1, 0)
        self.filterTypeAdjust.connect("value-changed", self.handleFilter, 1)
        filterTypeSlider = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.filterTypeAdjust, slider_border = 7, snap = 1 )
        filterTypeSlider.set_inverted(True)
        filterTypeSlider.set_size_request(50, 250)
        filterSliderBox.pack_start(filterTypeSlider, True, True, 5)

        filterBox.pack_start(filterSliderBox)

        self.filterLabel = gtk.Image()
        self.filterLabel.set_from_file(Config.IMAGE_ROOT + 'propFilter1.png')
        filterBox.pack_start(self.filterLabel, False, padding=10)

        controlsBox.pack_start(filterBox)
        self.parametersBox.pack_start(controlsBox)

        # Transport Panel Setup
        transportBox = RoundVBox(fillcolor=Config.INST_BCK_COLOR, bordercolor=Config.PANEL_BCK_COLOR)
        transportBox.set_border_width(1)
        transportBox.set_radius(10)

        # create cancel/check button
        transButtonBox = gtk.HBox()
        checkButton = ImageButton(Config.IMAGE_ROOT + 'check.png')
        checkButton.connect("clicked", self.applyParametersChange)
 
        cancelButton = ImageButton(Config.IMAGE_ROOT + 'closeA.png')
        cancelButton.connect("clicked", self.cancel)

        transButtonBox.pack_end(checkButton, False, False, 10)
        transButtonBox.pack_end(cancelButton, False, False)
        transportBox.pack_start(transButtonBox, False, False, 5) 
        self.parametersBox.pack_start(transportBox)



    def handlePitch( self, adjust ):
        self.pitch = adjust.value
        img = int(self.pitch * 5.)
        self.pitchLabel.set_from_file(Config.IMAGE_ROOT + 'propPitch' + str(img) + '.png')

    def handleVolume( self, adjust ):
        self.volume = adjust.value
        img = int(self.volume * 3.4)
        self.volumeLabel.set_from_file(Config.IMAGE_ROOT + 'propVolume' + str(img) + '.png')

    def handlePan( self, adjust ):
        self.pan = adjust.value
        img = int(self.pan * 4.)
        self.panLabel.set_from_file(Config.IMAGE_ROOT + 'propPan' + str(img) + '.png')

    def handleReverb( self, adjust ):
        self.reverb = adjust.value
        img = int(self.reverb * 5.)
        self.reverbLabel.set_from_file(Config.IMAGE_ROOT + 'propReverb' + str(img) + '.png')

    def handleAttack( self, adjust ):
        self.attack = adjust.value
        img = int(self.attack * 4.)
        self.attackLabel.set_from_file(Config.IMAGE_ROOT + 'propAtt' + str(img) + '.png')

    def handleDecay( self, adjust ):
        self.decay = adjust.value
        img = int(self.decay * 4.)
        self.decayLabel.set_from_file(Config.IMAGE_ROOT + 'propDec' + str(img) + '.png')

    def handleFilter( self, adjust, slider ):
        if slider == 0:
            self.filterCutoff = adjust.value
        else:
            self.filterType = adjust.value

        img = int(self.filterCutoff * 5.) + (int(self.filterType)*6)
        self.filterLabel.set_from_file(Config.IMAGE_ROOT + 'propFilter' + str(img) + '.png')

    def cancel( self, widget, data=None ):
        self.window.destroy()

    def applyParametersChange( self, data=None ):
        #self.getNoteParameters()
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


