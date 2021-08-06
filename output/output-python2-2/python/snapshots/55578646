import pygtk
pygtk.require('2.0')
import gtk

from Framework.Generation.Generator import GenerationParameters
from Framework.Generation.GenerationConstants import GenerationConstants

class GenerationParametersWindow( gtk.Window ):
    def __init__( self, generateFunction, handleCloseWindowCallback ):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        self.rythmMethod = GenerationConstants.DEFAULT_RYTHM_METHOD
        self.pitchMethod = GenerationConstants.DEFAULT_PITCH_METHOD
        self.pattern = GenerationConstants.DEFAULT_PATTERN   
        self.scale = GenerationConstants.DEFAULT_SCALE     
        self.generateFunction = generateFunction        
        self.setupWindow( handleCloseWindowCallback )
        
    def setupWindow( self, handleCloseWindowCallback ):
        self.set_position( gtk.WIN_POS_CENTER_ON_PARENT )
        self.set_title("Algorithmic generator")
        self.set_border_width(5)
        self.connect( "delete_event", handleCloseWindowCallback )
        self.generationBox = gtk.VBox(False, 2)
        self.sliderBox = gtk.VBox(False, 5)
        self.labelRythmMethodBox = gtk.VBox(False, 5)
        self.rythmMethodBox = gtk.HBox(False, 5)
        self.labelPitchMethodBox = gtk.VBox(False, 5)
        self.pitchMethodBox = gtk.HBox(False, 5)
        self.labelPatternBox = gtk.VBox(False, 5)
        self.patternBox = gtk.HBox(False, 5)
        self.labelScaleBox = gtk.VBox(False, 5)
        self.scaleBox = gtk.HBox(False, 5)
        self.buttonBox = gtk.HBox(False, 5)
        self.add(self.generationBox)

        # Create parameters sliders box
        #TODO: remove magic numbers
        self.densityAdjust = self.initSlider("density", GenerationConstants.DEFAULT_DENSITY, 0, 1, 0.01, 0, 2)
        self.regularityAdjust = self.initSlider("regularity", GenerationConstants.DEFAULT_REPETE, 0, 1, 0.01, 0, 2)
        self.pitchStepAdjust = self.initSlider("pitch max step", GenerationConstants.DEFAULT_STEP, -10, 10, 1, 2, 0)
        self.articulationAdjust = self.initSlider("stacato / legato", GenerationConstants.DEFAULT_ARTICULE, 0, 1, 0.01, 0, 2)

        # Create melodic rythm methods box
        self.labelRythmMethodBox.pack_start(gtk.Label("melodic rythm generation method"), False, False, 0)

        celluleButton = gtk.RadioButton(None, "Cellule")
        celluleButton.set_active(True)
        celluleButton.connect("toggled", self.rythmMethodCallback, 0)
        self.rythmMethodBox.pack_start(celluleButton, True, True, 0)
        celluleButton.show()

        xnoiseButton = gtk.RadioButton(celluleButton, "Xnoise")
        xnoiseButton.connect("toggled", self.rythmMethodCallback, 1)
        self.rythmMethodBox.pack_start(xnoiseButton, True, True, 0)
        xnoiseButton.show()

        # Create pitch generation methods box
        self.labelPitchMethodBox.pack_start(gtk.Label("pitch generation method"), False, False, 0)

        melodicButton = gtk.RadioButton(None, "melodic")
        melodicButton.set_active(True)
        melodicButton.connect("toggled", self.pitchMethodCallback, 0)
        self.pitchMethodBox.pack_start(melodicButton, True, True, 0)
        melodicButton.show()

        harmonicButton = gtk.RadioButton(melodicButton, "harmonic")
        harmonicButton.connect("toggled", self.pitchMethodCallback, 1)
        self.pitchMethodBox.pack_start(harmonicButton, True, True, 0)
        harmonicButton.show()

        # Create pitch patterns box
        self.labelPatternBox.pack_start(gtk.Label("pitch pattern"), False, False, 0)

        drunkButton = gtk.RadioButton(None, "Drunk")
        drunkButton.connect("toggled", self.patternCallback, "Drunk")
        self.patternBox.pack_start(drunkButton, True, True, 0)
        drunkButton.show()

        droneButton = gtk.RadioButton(drunkButton, "DroneJump")
        droneButton.connect("toggled", self.patternCallback, "DroneAndJump")
        self.patternBox.pack_start(droneButton, True, True, 0)
        droneButton.show()

        repeterButton = gtk.RadioButton(drunkButton, "Repeter")
        repeterButton.connect("toggled", self.patternCallback, "Repeter")
        self.patternBox.pack_start(repeterButton, True, True, 0)
        repeterButton.show()

        loopsegButton = gtk.RadioButton(drunkButton, "Loopseg")
        loopsegButton.set_active(True)
        loopsegButton.connect("toggled", self.patternCallback, "Loopseg")
        self.patternBox.pack_start(loopsegButton, True, True, 0)
        loopsegButton.show()

        # Create scales box
        self.labelScaleBox.pack_start(gtk.Label("scales"), False, False, 0)

        majorButton = gtk.RadioButton(None, "Major")
        majorButton.connect("toggled", self.scaleCallback, "major")
        self.scaleBox.pack_start(majorButton, True, True, 0)
        majorButton.show()

        minorHButton = gtk.RadioButton(majorButton, "Minor H")
        minorHButton.connect("toggled", self.scaleCallback, "harmonic minor")
        self.scaleBox.pack_start(minorHButton, True, True, 0)
        minorHButton.show()

        minorNButton = gtk.RadioButton(majorButton, "Minor N")
        minorNButton.connect("toggled", self.scaleCallback, "natural minor")
        self.scaleBox.pack_start(minorNButton, True, True, 0)
        minorNButton.show()

        phrygienButton = gtk.RadioButton(majorButton, "Phrygien")
        phrygienButton.connect("toggled", self.scaleCallback, "phrygien")
        self.scaleBox.pack_start(phrygienButton, True, True, 0)
        phrygienButton.show()

        generateButton = gtk.Button('Generate')
        generateButton.connect("clicked", self.generate)
        self.buttonBox.pack_start(generateButton)

        self.generationBox.pack_start(self.sliderBox, 5)
        self.generationBox.pack_start(self.labelRythmMethodBox, 5)
        self.generationBox.pack_start(self.rythmMethodBox, 5)
        self.generationBox.pack_start(self.labelPitchMethodBox, 5)
        self.generationBox.pack_start(self.pitchMethodBox, 5)
        self.generationBox.pack_start(self.labelPatternBox, 5)    
        self.generationBox.pack_start(self.patternBox, 5)
        self.generationBox.pack_start(self.labelScaleBox, 5)
        self.generationBox.pack_start(self.scaleBox, 5)
        self.generationBox.pack_start(self.buttonBox, 5)

    def getGenerationParameters( self ):
        return GenerationParameters( self.densityAdjust.value,
                                     self.regularityAdjust.value,
                                     self.pitchStepAdjust.value,
                                     self.articulationAdjust.value,
                                     self.rythmMethod,
                                     self.pitchMethod,
                                     self.pattern,
                                     self.scale )

    def generate(self, data=None):
        self.generateFunction( self.getGenerationParameters() )

    def rythmMethodCallback( self, widget, rythmMethod ):
        if widget.get_active():
            self.rythmMethod = rythmMethod

    def pitchMethodCallback( self, widget, pitchMethod ):
        if widget.get_active():
            self.pitchMethod = pitchMethod
    
    def patternCallback( self, widget, pattern ):
        if widget.get_active():
            self.pattern = pattern

    def scaleCallback( self, widget, scale ):
        if widget.get_active():
            self.scale = scale

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
