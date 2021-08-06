import pygtk
pygtk.require('2.0')
import gtk

from Framework.Constants import Constants

class MixerWindow( gtk.Window ):
    def __init__(self):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        self.setupWindow()

    def hideWindow( self, data=None ):
        self.hide_all()

    def setupWindow( self ):
        self.set_position(gtk.WIN_POS_CENTER_ON_PARENT)
        self.set_title("Mixing board")
        self.set_border_width(5)
        self.bbox = gtk.VBox(False, 2)
        self.add(self.bbox)

        #TODO: really cheap temporary system, yes, yes I know: MAGIC NUMBERS!!!
        if Constants.NUMBER_OF_TRACKS >= 1:
            self.track1Adjust = self.initSlider( "track 1", .8, 0, 1, 0.01, 0, 2)
        if Constants.NUMBER_OF_TRACKS >= 2:
            self.track2Adjust = self.initSlider("track 2", .8, 0, 1, 0.01, 0, 2)
        if Constants.NUMBER_OF_TRACKS >= 3:
            self.track3Adjust = self.initSlider( "track 3", .8, 0, 1, 0.01, 0, 2)
        if Constants.NUMBER_OF_TRACKS >= 4:
            self.track4Adjust = self.initSlider( "track 4", .8, 0, 1, 0.01, 0, 2)
        if Constants.NUMBER_OF_TRACKS >= 5:
            self.track5Adjust = self.initSlider( "track 5", .8, 0, 1, 0.01, 0, 2)
        if Constants.NUMBER_OF_TRACKS >= 6:
            self.track6Adjust = self.initSlider( "track 6", .8, 0, 1, 0.01, 0, 2)
        if Constants.NUMBER_OF_TRACKS >= 7:
            self.track7Adjust = self.initSlider( "track 7", .8, 0, 1, 0.01, 0, 2)
        if Constants.NUMBER_OF_TRACKS >= 8:
            self.track8Adjust = self.initSlider( "track 8", .8, 0, 1, 0.01, 0, 2)

        self.okButton = self.initButton(" close ", self.hideWindow )

    def getMixerValues( self ):
        return (  self.slider1Adjust.value,
                        self.slider2Adjust.value )

    def initButton(self, label, buttonFunction):
        button = gtk.Button(label)
        button.connect("clicked", buttonFunction)
        self.bbox.pack_start(button)

# connect to a function getMixerValues pour accessibilit/ constante
    def initSlider(self, label, initValue, minValue, maxValue, incStep, policy, digits):
        sliderAdjust = gtk.Adjustment(initValue, minValue, maxValue, incStep, incStep, 0)
        slider = gtk.HScale(sliderAdjust)
        slider.set_update_policy(policy)
        slider.set_digits(digits)
        slider.set_value_pos(1)
        slider.set_size_request(250, 25)
        self.bbox.pack_start(gtk.Label(label), False, False, 0)
        self.bbox.pack_start(slider)
        return sliderAdjust

class MixerValues( MixerWindow ):
    def getMixerValues( self ):
        return (MixerWindow.slider1Adjust.value, MixerWindow.slider2Adjust.value)
