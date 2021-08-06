import pygtk
pygtk.require('2.0')
import gtk

from Framework.Generation.Generator import GenerationParameters
from Framework.Generation.Generator import VariationParameters
from Framework.Generation.GenerationConstants import GenerationConstants

class GenerationParametersWindow( gtk.Window ):
    def __init__( self, generateFunction, variateFunction, handleCloseWindowCallback ):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        self.rythmMethod = GenerationConstants.DEFAULT_RYTHM_METHOD
        self.pitchMethod = GenerationConstants.DEFAULT_PITCH_METHOD
        self.pattern = GenerationConstants.DEFAULT_PATTERN   
        self.scale = GenerationConstants.DEFAULT_SCALE   
        self.pitchVariation = GenerationConstants.DEFAULT_PITCH_VARIATION  
        self.rythmVariation = GenerationConstants.DEFAULT_RYTHM_VARIATION
        self.generateFunction = generateFunction     
        self.variateFunction = variateFunction  
        self.sourceVariation = 0 
        self.setupWindow( handleCloseWindowCallback )
        
    def setupWindow( self, handleCloseWindowCallback ):
        self.set_position( gtk.WIN_POS_CENTER_ON_PARENT )
        self.set_title("Algorithmic generator")
        self.set_border_width(5)
        self.connect( "delete_event", handleCloseWindowCallback )
        self.mainBox = gtk.VBox(False, 5)
        self.generationBox = gtk.VBox(False, 2)
        self.variationBox = gtk.VBox(False, 2)
        self.sliderBox = gtk.VBox(False, 2)
        self.labelRythmMethodBox = gtk.VBox(False, 2)
        self.rythmMethodBox = gtk.HBox(False, 2)
        self.labelSourceVariationBox = gtk.VBox(False, 2)
        self.sourceVariationBox = gtk.HBox(False, 2)
        self.labelPitchMethodBox = gtk.VBox(False, 2)
        self.pitchMethodBox = gtk.HBox(False, 2)
        self.labelPatternBox = gtk.VBox(False, 2)
        self.patternBox = gtk.HBox(False, 2)
        self.labelScaleBox = gtk.VBox(False, 2)
        self.scaleBox = gtk.HBox(False, 2)
        self.buttonBox = gtk.HBox(False, 2)

        self.labelPitchVariationBox = gtk.VBox(False, 2)
        self.pitchVariationBox = gtk.HBox(False, 2)
        self.labelRythmVariationBox = gtk.VBox(False, 2)
        self.rythmVariationBox = gtk.HBox(False, 2)

        self.generationBox.pack_start( gtk.Label( "GENERATION" ), False, False, 0 )
        # Create parameters sliders box
        #TODO: remove magic numbers
        self.densityAdjust = self.initSlider("density", GenerationConstants.DEFAULT_DENSITY, 0, 1, 0.01, 0, 2)
        self.rythmRegularityAdjust = self.initSlider("rythm regularity", GenerationConstants.DEFAULT_RYTHM_REGULARITY, 0, 1, 0.01, 0, 2)
        self.pitchStepAdjust = self.initSlider("pitch max step", GenerationConstants.DEFAULT_STEP, -10, 10, 1, 2, 0)
        self.pitchRegularityAdjust = self.initSlider("pitch regularity", GenerationConstants.DEFAULT_PITCH_REGULARITY, 0, 1, 0.01, 0, 2)
        self.articulationAdjust = self.initSlider("stacato / legato", GenerationConstants.DEFAULT_ARTICULE, 0, 1, 0.01, 0, 2)
        self.generationBox.pack_start(self.sliderBox, 5)

        # Create melodic rythm methods box
        self.labelRythmMethodBox.pack_start(gtk.Label("melodic rythm generation method"), False, False, 0)
        self.generationBox.pack_start(self.labelRythmMethodBox, 3)
        rythmMethodType = ['Cellule', 'Xnoise' ]
        self.initRadioButton( rythmMethodType, self.rythmMethodCallback, self.rythmMethodBox )
        self.generationBox.pack_start(self.rythmMethodBox, 3)

        # Create pitch generation methods box
        self.labelPitchMethodBox.pack_start(gtk.Label("pitch generation method"), False, False, 0)
        self.generationBox.pack_start(self.labelPitchMethodBox, 3)
        pitchMethodType = [ 'melodic', 'harmonic' ]
        self.initRadioButton( pitchMethodType, self.pitchMethodCallback, self.pitchMethodBox )
        self.generationBox.pack_start(self.pitchMethodBox, 3)

        # Create pitch patterns box
        self.labelPatternBox.pack_start(gtk.Label("pitch pattern"), False, False, 0)
        self.generationBox.pack_start(self.labelPatternBox, 3)    
        patternType = [ 'Drunk', 'DroneJump', 'Repeter', 'Loopseg' ]
        self.initRadioButton( patternType, self.patternCallback, self.patternBox )
        self.generationBox.pack_start(self.patternBox, 3)

        # Create scales box
        self.labelScaleBox.pack_start(gtk.Label("scales"), False, False, 0)
        self.generationBox.pack_start(self.labelScaleBox, 3)
        scalesType = [ 'Major', 'Minor H', 'Minor N', 'Phrygien' ]
        self.initRadioButton( scalesType, self.scaleCallback, self.scaleBox )
        self.generationBox.pack_start(self.scaleBox, 3)

        # create generate button
        generateButton = gtk.Button('Generate')
        generateButton.connect("clicked", self.generate)
        self.buttonBox.pack_start(generateButton)
        self.generationBox.pack_start(self.buttonBox, 3)

        # create variation box
        self.variationBox.pack_start( gtk.Label( "VARIATION" ), False, False, 0 )

        # create source variation box
        self.labelSourceVariationBox.pack_start(gtk.Label("pages sources"), False, False, 0)
        self.variationBox.pack_start(self.labelSourceVariationBox, 3)
        sourceVariationType = [ 1, 2, 3, 4, 5 ]
        self.initSourceRadioButton( sourceVariationType, self.sourceVariationCallback, self.sourceVariationBox )
        self.variationBox.pack_start(self.sourceVariationBox, 3)

        # create pitch variation box
        self.labelPitchVariationBox.pack_start(gtk.Label("pitch variation"), False, False, 0)
        self.variationBox.pack_start(self.labelPitchVariationBox, 3)
        pitchVariationType = [ 'Copy', 'Markov', 'Reverse', 'Sort', 'Shuffle' ]
        self.initRadioButton( pitchVariationType, self.pitchVariationCallback, self.pitchVariationBox )
        self.variationBox.pack_start(self.pitchVariationBox, 3)

        # create rythm variation box
        self.labelRythmVariationBox.pack_start(gtk.Label("rythm variation"), False, False, 0)
        self.variationBox.pack_start(self.labelRythmVariationBox, 3)
        rythmVariationType = [ 'Copy', 'Reverse',  'Shuffle' ]
        self.initRadioButton( rythmVariationType, self.rythmVariationCallback, self.rythmVariationBox )
        self.variationBox.pack_start(self.rythmVariationBox, 3)

        # create variate button
        variateButton = gtk.Button('Variate')
        variateButton.connect("clicked", self.variate)
        self.variationBox.pack_start(variateButton, 3)

        self.mainBox.pack_start(self.generationBox)
        self.mainBox.pack_start(self.variationBox)

        self.add(self.mainBox)

    def getGenerationParameters( self ):
        return GenerationParameters( self.densityAdjust.value,
                                     self.rythmRegularityAdjust.value,
                                     self.pitchStepAdjust.value,
                                     self.pitchRegularityAdjust.value,
                                     self.articulationAdjust.value,
                                     self.rythmMethod,
                                     self.pitchMethod,
                                     self.pattern,
                                     self.scale )

    def generate(self, data=None):
        self.generateFunction( self.getGenerationParameters() )

    def getVariationParameters( self ):
        return VariationParameters( self.sourceVariation, 
                                                            self.pitchVariation,
                                                            self.rythmVariation )

    def variate( self, data=None ):
        self.variateFunction( self.getVariationParameters() )

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

    def sourceVariationCallback( self, widget, data ):
        if widget.get_active():
            self.sourceVariation = int( data )

    def pitchVariationCallback( self, widget, data ):
        if widget.get_active():
            self.pitchVariation = data

    def rythmVariationCallback( self, widget, data ):
        if widget.get_active():
            self.rythmVariation = data

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

    def initRadioButton( self, labelList, methodCallback, box ):
        for i in range( len( labelList ) ):
            if i == 0:
                button = gtk.RadioButton( None, labelList[ i ] )
            else:
                button = gtk.RadioButton( button, labelList[ i ] )
            button.connect( "toggled", methodCallback, i )
            box.pack_start( button, True, True, 0 )

    def initSourceRadioButton( self, labelList, methodCallback, box ):
        for i in range( len( labelList ) ):
            if i == 0:
                button = gtk.RadioButton(None, str( labelList[ i ] ) )
            else:
                button = gtk.RadioButton( button, str( labelList[ i ] ) )
            button.connect( "toggled", methodCallback, i )
            box.pack_start( button, True, True, 0 )
            if i == 0:
                button.set_active(True)
