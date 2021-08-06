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
        self.move(300, 20)
        self.set_border_width(0)
        self.set_decorated(False)
        self.mainBox = RoundVBox(fillcolor="#FFF", bordercolor="#FFF")
        self.mainBox.set_radius(10)
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
        pitchSlider = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderbutviolet.png", self.pitchAdjust, 7 )
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
        volumeSlider = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderbutviolet.png", self.volumeAdjust, 7 )
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
        panSlider = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderbutviolet.png", self.panAdjust, 7 )
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
        reverbSlider = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderbutviolet.png", self.reverbAdjust, 7 )
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
        attackSlider = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderbutviolet.png", self.attackAdjust, 7 )
        attackSlider.set_inverted(True)
        attackSlider.set_size_request(50, 250)
        self.attackLabel = gtk.Image()
        self.attackLabel.set_from_file(Config.IMAGE_ROOT + 'propAtt1.png')
        attackBox.pack_start(attackSlider, True, True, 5)
        attackBox.pack_start(self.attackLabel, False, padding=10)
        controlsBox.pack_start(attackBox)





        self.parametersBox.pack_start(controlsBox)

    def handlePitch( self, adjust ):
        self.pitch = adjust.value
        img = int(self.pitch * 5.)
        self.pitchLabel.set_from_file(Config.IMAGE_ROOT + 'propPitch' + str(img) + '.png')

    def handleVolume( self, adjust ):
        self.volume = adjust.value
        img = int(self.volume * 3.)
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


