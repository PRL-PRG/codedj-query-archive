import pygtk
pygtk.require('2.0')
import gtk

from Framework.Generation.Generator import GenerationParameters
from Framework.Generation.GenerationConstants import GenerationConstants

class GenerationParametersWindow( gtk.Window ):
    def __init__( self, generateFunction ):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        #gtk.Window.__init__( self, gtk.WINDOW_POPUP )
        
        self.generateFunction = generateFunction
        
        self.setupWindow()
        
    def setupWindow( self ):
        self.set_position( gtk.WIN_POS_CENTER_ON_PARENT )
        self.set_title("Algorithmic generator")
        self.set_border_width(5)
        self.connect("delete_event", self.delete_event)
        self.bbox = gtk.VBox(False, 2)
        self.add(self.bbox)

        #TODO: remove magic numbers
        self.densityAdjust = self.initSlider("density", .35, 0, 1, 0.01, 0, 2)
        self.durationChangeAdjust = self.initSlider("duration change", .5, 0, 1, 0.01, 0, 2)
        self.pitchStepAdjust = self.initSlider("pitch max step", 5, -10, 10, 1, 2, 0)
        self.panoramisationAdjust = self.initSlider("pan", 0, -0.99, 0.99, 0.01, 0, 2)
        self.articulationAdjust = self.initSlider("stacato / legato", .8, 0, 1, 0.01, 0, 2)

        self.generateButton = self.initButtons("Generate", self.generate)
   
    def getGenerationParameters( self ):
        return GenerationParameters( GenerationConstants.DEFAULT_BEAT,
                                     self.densityAdjust.value,
                                     self.durationChangeAdjust.value,
                                     self.pitchStepAdjust.value,
                                     self.articulationAdjust.value,
                                     self.panoramisationAdjust.value )

    def delete_event(self, widget, event, data=None):
        return False

    def generate(self, data=None):
        self.generateFunction( self.getGenerationParameters() )
        self.window.destroy()

    def initButtons(self, label, buttonFunction):
        button = gtk.Button(label)
        button.connect("clicked", buttonFunction)
        self.bbox.pack_start(button)

    def initToggles(self, toggleName, label, toggleFunction1):
        toggleToggleName = 'self.' + toggleName + 'Toggle'
        toggleToggleName = gtk.ToggleButton(label, False)
        toggleToggleName.connect("toggled", toggleFunction1)
        self.bbox.pack_start(toggleToggleName)

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