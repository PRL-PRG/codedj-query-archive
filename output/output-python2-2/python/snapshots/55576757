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

        self.volumeFunctions = {}
        for trackIndex in range(Constants.NUMBER_OF_TRACKS):
            self.volumeFunctions[ trackIndex ] = self.initSlider( "track %d " % ( trackIndex+1), .8, 0, 1, 0.01, 0, 2).get_value

#yes, yes I know: MAGIC NUMBERS!!!

        self.closeButton = self.initButton(" close ", self.hideWindow )

    def getVolumeFunctions( self ):
        return self.volumeFunctions

    def initButton(self, label, buttonFunction):
        button = gtk.Button(label)
        button.connect("clicked", buttonFunction)
        self.bbox.pack_start(button)

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
