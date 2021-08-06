import pygtk
pygtk.require('2.0')
import gtk

from Framework.Generation.Generator import GenerationParameters
from Framework.Generation.GenerationConstants import GenerationConstants

class GenerationParametersWindow( gtk.Window ):
    def __init__( self, generateFunction ):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        self.scale = GenerationConstants.DEFAULT_SCALE
        self.pattern = GenerationConstants.DEFAULT_PATTERN
        
        self.generateFunction = generateFunction
        
        self.setupWindow()
        
    def setupWindow( self ):
        self.set_position( gtk.WIN_POS_CENTER_ON_PARENT )
        self.set_title("Algorithmic generator")
        self.set_border_width(5)
        self.connect("delete_event", self.delete_event)
        self.generationBox = gtk.VBox(False, 2)
        self.sliderBox = gtk.VBox(False, 2)
        self.buttonBox = gtk.HBox(False, 2)
        self.add(self.generationBox)

        #TODO: remove magic numbers
        self.densityAdjust = self.initSlider("density", GenerationConstants.DEFAULT_DENSITY, 0, 1, 0.01, 0, 2)
        self.regularityAdjust = self.initSlider("regularity", GenerationConstants.DEFAULT_REPETE, 0, 1, 0.01, 0, 2)
        self.pitchStepAdjust = self.initSlider("pitch max step", GenerationConstants.DEFAULT_STEP, -10, 10, 1, 2, 0)
        self.articulationAdjust = self.initSlider("stacato / legato", GenerationConstants.DEFAULT_ARTICULE, 0, 1, 0.01, 0, 2)
        self.panoramisationAdjust = self.initSlider("pan", GenerationConstants.DEFAULT_PANNER, -0.99, 0.99, 0.01, 0, 2)

        scaleMenu = gtk.Menu()
        scaleMenuItem = gtk.MenuItem( "Scale" )
        scaleMenuItem.set_submenu( scaleMenu )
            
        scaleNames = GenerationConstants.SCALES.keys()
        for scaleName in scaleNames:
            menuItem = gtk.MenuItem( scaleName )
            menuItem.connect_object( "activate", self.setScale, scaleName )
            scaleMenu.append( menuItem )
        
        scaleMenuBar = gtk.MenuBar()
        scaleMenuBar.append( scaleMenuItem )
        self.buttonBox.pack_start( scaleMenuBar )

        patternMenu = gtk.Menu()
        patternMenuItem = gtk.MenuItem( "pitch pattern" )
        patternMenuItem.set_submenu( patternMenu )
            
        patternNames = GenerationConstants.PITCH_PATTERNS
        for patternName in patternNames:
            menuItem = gtk.MenuItem( patternName )
            menuItem.connect_object( "activate", self.setPitchPattern, patternName )
            patternMenu.append( menuItem )
        
        patternMenuBar = gtk.MenuBar()
        patternMenuBar.append( patternMenuItem )
        self.buttonBox.pack_start( patternMenuBar )

        generateButton = gtk.Button('Generate')
        generateButton.connect("clicked", self.generate)
        self.buttonBox.pack_start(generateButton)

        self.generationBox.pack_start(self.sliderBox, 2)
        self.generationBox.pack_start(self.buttonBox, 2)

    def getGenerationParameters( self ):
        return GenerationParameters( GenerationConstants.DEFAULT_BAR,
                                     self.densityAdjust.value,
                                     self.regularityAdjust.value,
                                     self.pitchStepAdjust.value,
                                     self.articulationAdjust.value,
                                     self.panoramisationAdjust.value,
                                     self.scale,
                                     self.pattern )

    def delete_event(self, widget, event, data=None):
        return False

    def setScale( self, scale ):
        self.scale = scale
    
    def setPitchPattern( self, pattern ):
        self.pattern = pattern

    def generate(self, data=None):
        self.generateFunction( self.getGenerationParameters() )
        self.window.destroy()

    def initSlider(self, label, initValue, minValue, maxValue, incStep, policy, digits):
        sliderAdjust = gtk.Adjustment(initValue, minValue, maxValue, incStep, incStep, 0)
        slider = gtk.HScale(sliderAdjust)
        slider.set_update_policy(policy)
        slider.set_digits(digits)
        slider.set_value_pos(1)
        slider.set_size_request(250, 25)
        self.sliderBox.pack_start(gtk.Label(label), False, False, 0)
        self.sliderBox.pack_start(slider)
        return sliderAdjust
