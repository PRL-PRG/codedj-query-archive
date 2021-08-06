import pygtk
pygtk.require('2.0')
import gtk
import shelve
from Generation.Generator import GenerationParameters
from Generation.Generator import VariationParameters
from Generation.GenerationConstants import GenerationConstants
from Util.ThemeWidgets import *
import Config

class GenerationParametersWindow( gtk.Window ):
    def __init__( self, generateFunction, variateFunction, handleCloseWindowCallback ):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        self.set_type_hint(gtk.gdk.WINDOW_TYPE_HINT_DIALOG)
        self.set_position( gtk.WIN_POS_CENTER )
        self.set_default_size(30, 300)
        self.set_border_width(0)
        self.set_decorated(False)
        self.mainBox = RoundVBox(fillcolor="#FFF", bordercolor="#FFF")
        self.mainBox.set_radius(10)

        self.handleCloseWindowCallback = handleCloseWindowCallback
        self.connect( "delete_event", handleCloseWindowCallback )

        self.rythmMethod = GenerationConstants.DEFAULT_RYTHM_METHOD
        self.pitchMethod = GenerationConstants.DEFAULT_PITCH_METHOD
        self.pattern = GenerationConstants.DEFAULT_PATTERN   
        self.scale = GenerationConstants.DEFAULT_SCALE   
        self.pitchVariation = GenerationConstants.DEFAULT_PITCH_VARIATION  
        self.rythmVariation = GenerationConstants.DEFAULT_RYTHM_VARIATION
        self.generateFunction = generateFunction     
        self.variateFunction = variateFunction  
        self.setupWindow()
        
    def setupWindow( self ):
        self.labelRythmMethodBox = gtk.VBox(False, 2)
        self.rythmMethodBox = gtk.HBox(False, 2)
        self.labelPitchMethodBox = gtk.VBox(False, 2)
        self.pitchMethodBox = gtk.HBox(False, 2)
        self.labelPatternBox = gtk.VBox(False, 2)
        self.patternBox = gtk.HBox(False, 2)
        self.labelScaleBox = gtk.VBox(False, 2)
        self.scaleBox = gtk.HBox(False, 2)

        self.rythmDensity = GenerationConstants.DEFAULT_DENSITY
        self.rythmRegularity = GenerationConstants.DEFAULT_RYTHM_REGULARITY
        self.pitchRegularity = GenerationConstants.DEFAULT_PITCH_REGULARITY 
        self.pitchStep = GenerationConstants.DEFAULT_STEP
        self.duration = GenerationConstants.DEFAULT_ARTICULE
        self.silence = .2 #GenerationConstants.DEFAULT_SILENCE

        # Generation Panel Setup
        generationBox = RoundVBox(fillcolor=Config.INST_BCK_COLOR, bordercolor=Config.PANEL_BCK_COLOR)
        generationBox.set_border_width(1)
        generationBox.set_radius(10)
        XYSlidersBox = gtk.HBox()

        self.col = gtk.gdk.color_parse(Config.PANEL_COLOR)

        XYSlider1Box = gtk.VBox()
        XYSlider1UpBox = RoundHBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        XYSlider1UpBox.set_border_width(3)
        XYSlider1UpBox.set_radius(10)
        self.XYSlider1DownBox = RoundHBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        self.XYSlider1DownBox.set_border_width(3)
        self.XYSlider1DownBox.set_radius(10)

        self.slider1Label = gtk.DrawingArea()
        self.slider1Label.modify_bg(gtk.STATE_NORMAL, self.col)
        colormap = self.slider1Label.get_colormap()
        self.bgColor = colormap.alloc_color( Config.PANEL_COLOR, True, True )
        self.slider1Label.set_size_request(228, 60)
        self.slider1Label.connect("expose-event", self.draw )
        XYSliderBox1 = self.formatRoundBox( RoundFixed(), Config.PANEL_COLOR )
        XYSliderBox1.set_size_request( 250, 250 )
        XYButton1 =  ImageToggleButton( Config.IMAGE_ROOT+"XYbut.png", Config.IMAGE_ROOT+"XYbutDown.png", backgroundFill=Config.PANEL_COLOR )
        self.XAdjustment1 = gtk.Adjustment( 100, 0, 200, 1, 1, 1 )
        self.XAdjustment1.connect("value-changed", self.handleXAdjustment1)
        self.YAdjustment1 = gtk.Adjustment( 100, 0, 200, 1, 1, 1 )
        self.YAdjustment1.connect("value-changed", self.handleYAdjustment1)
        xySlider1 = XYSlider( XYSliderBox1, XYButton1, self.XAdjustment1, self.YAdjustment1, False, True )
        XYSlider1UpBox.pack_start( xySlider1, False, False )

        self.XYSlider1DownBox.pack_start(self.slider1Label, False, False, 5)
        XYSlider1Box.pack_start(XYSlider1UpBox)
        XYSlider1Box.pack_start(self.XYSlider1DownBox)
        XYSlidersBox.pack_start(XYSlider1Box, False, False, 5)


        XYSlider2Box = gtk.VBox()
        XYSlider2UpBox = RoundHBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        XYSlider2UpBox.set_border_width(3)
        XYSlider2UpBox.set_radius(10)
        self.XYSlider2DownBox = RoundHBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        self.XYSlider2DownBox.set_border_width(3)
        self.XYSlider2DownBox.set_radius(10)

        self.slider2Label = gtk.DrawingArea()
        self.slider2Label.modify_bg(gtk.STATE_NORMAL, self.col)
        #colormap = self.slider1Label.get_colormap()
        #self.bgColor = colormap.alloc_color( Config.PANEL_COLOR, True, True )
        self.slider2Label.set_size_request(228, 60)
        self.slider2Label.connect("expose-event", self.draw2 )
        XYSliderBox2 = self.formatRoundBox( RoundFixed(), Config.PANEL_COLOR )
        XYSliderBox2.set_size_request( 250, 250 )
        XYButton2 =  ImageToggleButton( Config.IMAGE_ROOT+"XYbut.png", Config.IMAGE_ROOT+"XYbutDown.png", backgroundFill=Config.PANEL_COLOR )
        self.XAdjustment2 = gtk.Adjustment( 100, 0, 200, 1, 1, 1 )
        self.XAdjustment2.connect("value-changed", self.handleXAdjustment2)
        self.YAdjustment2 = gtk.Adjustment( 100, 0, 200, 1, 1, 1 )
        self.YAdjustment2.connect("value-changed", self.handleYAdjustment2)
        xySlider2 = XYSlider( XYSliderBox2, XYButton2, self.XAdjustment2, self.YAdjustment2, False, True )
        XYSlider2UpBox.pack_start( xySlider2, False, False )

        self.XYSlider2DownBox.pack_start(self.slider2Label, False, False, 5)
        XYSlider2Box.pack_start(XYSlider2UpBox)
        XYSlider2Box.pack_start(self.XYSlider2DownBox)
        XYSlidersBox.pack_start(XYSlider2Box, False, False, 5)


        XYSlider3Box = gtk.VBox()
        XYSlider3UpBox = RoundHBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        XYSlider3UpBox.set_border_width(3)
        XYSlider3UpBox.set_radius(10)
        self.XYSlider3DownBox = RoundHBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        self.XYSlider3DownBox.set_border_width(3)
        self.XYSlider3DownBox.set_radius(10)

        self.slider3Label = gtk.DrawingArea()
        self.slider3Label.modify_bg(gtk.STATE_NORMAL, self.col)
        #colormap = self.slider1Label.get_colormap()
        #self.bgColor = colormap.alloc_color( Config.PANEL_COLOR, True, True )
        self.slider3Label.set_size_request(228, 60)
        self.slider3Label.connect("expose-event", self.draw3 )
        XYSliderBox3 = self.formatRoundBox( RoundFixed(), Config.PANEL_COLOR )
        XYSliderBox3.set_size_request( 250, 250 )
        XYButton3 =  ImageToggleButton( Config.IMAGE_ROOT+"XYbut.png", Config.IMAGE_ROOT+"XYbutDown.png", backgroundFill=Config.PANEL_COLOR )
        self.XAdjustment3 = gtk.Adjustment( 100, 0, 200, 1, 1, 1 )
        self.XAdjustment3.connect("value-changed", self.handleXAdjustment3)
        self.YAdjustment3 = gtk.Adjustment( 100, 0, 200, 1, 1, 1 )
        self.YAdjustment3.connect("value-changed", self.handleYAdjustment3)
        xySlider3 = XYSlider( XYSliderBox3, XYButton3, self.XAdjustment3, self.YAdjustment3, False, True )
        XYSlider3UpBox.pack_start( xySlider3, False, False )

        self.XYSlider3DownBox.pack_start(self.slider3Label, False, False, 5)
        XYSlider3Box.pack_start(XYSlider3UpBox)
        XYSlider3Box.pack_start(self.XYSlider3DownBox)
        XYSlidersBox.pack_start(XYSlider3Box, False, False, 5)

        generationBox.pack_start(XYSlidersBox, False, False, 5) 

        self.mainBox.pack_start(generationBox)





        # Variation Panel setup
        variationBox = RoundVBox(fillcolor=Config.INST_BCK_COLOR, bordercolor=Config.PANEL_BCK_COLOR)
        variationBox.set_border_width(1)
        variationBox.set_radius(10)

        variationSpacingBox = gtk.VBox()

        COL_LEN = 2
                           
        box = RoundHBox(fillcolor=Config.INST_BCK_COLOR, bordercolor=Config.PANEL_COLOR)
        imagesName = ['pitchCopy', 'pitchReverse', 'pitchSort', 'pitchShuffle', 'pitchInvert', 'rytCopy', 'rytReverse', 'rytShuffle', 'rytShuffle', 'rytShuffle'] 
        imagesNum = len(imagesName)
        cols = ( imagesNum // COL_LEN )
        if imagesNum % COL_LEN is not 0:    #S'il y a un reste
            cols = cols + 1
                
        self.firstButton = None
        for col in range(cols):
            vBox = gtk.VBox()
            for var in range(COL_LEN):
                hBox = gtk.HBox()
                index = COL_LEN * col + var
                img = imagesName[index]
                iButton = ImageRadioButton(self.firstButton, Config.IMAGE_ROOT + img + '.png', Config.IMAGE_ROOT + img + '.png', Config.IMAGE_ROOT + img + '.png')
                if self.firstButton == None:
                    self.firstButton = iButton
                iButton.connect('clicked' , self.handleVariationButton , var)
                hBox.pack_start(iButton, False, False)
                vBox.pack_start(hBox, False, False)
            box.pack_start(vBox, False, False)
        variationSpacingBox.pack_start(box)
        variationBox.pack_start(variationSpacingBox, False, False, 5)

        self.mainBox.pack_start(variationBox)

        # Meta Algo panel setup
        metaAlgoBox = RoundVBox(fillcolor=Config.INST_BCK_COLOR, bordercolor=Config.PANEL_BCK_COLOR)
        metaAlgoBox.set_border_width(1)
        metaAlgoBox.set_radius(10)

        methodBox = gtk.HBox()        
        self.firstButton = None
        methodNames = ['drunk', 'droneJump', 'loopSeg', 'repeat']
        for meth in methodNames:
            iButton = ImageRadioButton(self.firstButton, Config.IMAGE_ROOT + meth + '.png', Config.IMAGE_ROOT + meth + 'Down.png', Config.IMAGE_ROOT + meth + 'Over.png')
            if self.firstButton == None:
                self.firstButton = iButton
            iButton.connect('clicked' , self.handleMethod , meth)
            methodBox.pack_start(iButton, False, False)
        metaAlgoBox.pack_start(methodBox, False, False, 5)

        self.mainBox.pack_start(metaAlgoBox)

      # Create melodic rythm methods box
        self.labelRythmMethodBox.pack_start(gtk.Label("melodic rythm generation method"), False, False, 0)
        #metaAlgoBox.pack_start(self.labelRythmMethodBox, 3)
        rythmMethodType = ['Cellule', 'Xnoise' ]
        self.initRadioButton( rythmMethodType, self.rythmMethodCallback, self.rythmMethodBox )
        #metaAlgoBox.pack_start(self.rythmMethodBox, 3)

        # Create pitch generation methods box
        self.labelPitchMethodBox.pack_start(gtk.Label("pitch generation method"), False, False, 0)
        #metaAlgoBox.pack_start(self.labelPitchMethodBox, 3)
        pitchMethodType = [ 'melodic', 'harmonic' ]
        self.initRadioButton( pitchMethodType, self.pitchMethodCallback, self.pitchMethodBox )
        #metaAlgoBox.pack_start(self.pitchMethodBox, 3)

        # Create pitch patterns box
        self.labelPatternBox.pack_start(gtk.Label("pitch pattern"), False, False, 0)
        #metaAlgoBox.pack_start(self.labelPatternBox, 3)    
        patternType = [ 'Drunk', 'DroneJump', 'Repeter', 'Loopseg' ]
        self.initRadioButton( patternType, self.patternCallback, self.patternBox )
        #metaAlgoBox.pack_start(self.patternBox, 3)

        # Create scales box
        self.labelScaleBox.pack_start(gtk.Label("scales"), False, False, 0)
        #metaAlgoBox.pack_start(self.labelScaleBox, 3)
        scalesType = [ 'Major', 'Minor H', 'Minor N', 'Phrygien' ]
        self.initRadioButton( scalesType, self.scaleCallback, self.scaleBox )
        #metaAlgoBox.pack_start(self.scaleBox, 3)


        # Transport Panel Setup
        transportBox = RoundVBox(fillcolor=Config.INST_BCK_COLOR, bordercolor=Config.PANEL_BCK_COLOR)
        transportBox.set_border_width(1)
        transportBox.set_radius(10)

        # Create save/load presets 
        transButtonBox = RoundHBox(fillcolor=Config.INST_BCK_COLOR, bordercolor=Config.PANEL_BCK_COLOR)
        transButtonBox.set_radius(10)

        saveButton = ImageButton(Config.TAM_TAM_ROOT + '/Resources/Images/save.png')
        saveButton.connect("clicked", self.handleSave, None)
        transButtonBox.pack_start(saveButton, False, False, 2)

        loadButton = ImageButton(Config.TAM_TAM_ROOT + '/Resources/Images/load.png')
        loadButton.connect("clicked", self.handleLoad, None)
        transButtonBox.pack_start(loadButton, False, False, 2)

        transportBox.pack_start(transButtonBox)

        # create cancel/check button
        checkButton = ImageButton(Config.IMAGE_ROOT + 'check.png')
        checkButton.connect("clicked", self.generate)
 
        cancelButton = ImageButton(Config.IMAGE_ROOT + 'closeA.png')
        cancelButton.connect("clicked", self.cancel)

        # create play/stop buttons
        playButton = ImageToggleButton(Config.IMAGE_ROOT + 'playTogOff.png', Config.IMAGE_ROOT + 'playTogOn.png')
        selButton = ImageToggleButton(Config.IMAGE_ROOT + 'playAll.png', Config.IMAGE_ROOT + 'playSel.png')
        transButtonBox.pack_end(checkButton, False, False, 10)
        transButtonBox.pack_end(cancelButton, False, False)
        transButtonBox.pack_end(selButton, False, False)
        transButtonBox.pack_end(playButton, False, False)
        transportBox.pack_start(transButtonBox, False, False, 10) 


        self.mainBox.pack_start(transportBox)
        self.add(self.mainBox)     
        self.loadPixmaps()          

    def loadPixmaps( self ):
        win = gtk.gdk.get_default_root_window()
        self.gc = gtk.gdk.GC( win )
        self.gc.foreground = self.bgColor

        self.arrowPixmap = []
        for i in range(2):	    
            pix = gtk.gdk.pixbuf_new_from_file(Config.IMAGE_ROOT + ['arrowSide.png', 'arrowUp.png'][i])
            map = gtk.gdk.Pixmap( win, pix.get_width(), pix.get_height() )
            map.draw_rectangle( self.gc, True, 0, 0, pix.get_width(), pix.get_height() )
            map.draw_pixbuf( self.gc, pix, 0, 0, 0, 0, pix.get_width(), pix.get_height(), gtk.gdk.RGB_DITHER_NONE )
            self.arrowPixmap.append(map)

        self.rythDensPixmap = []
        self.rythRegPixmap = []
        self.pitchRegPixmap = []
        self.pitchStepPixmap = []
        self.durPixmap = []
        self.silencePixmap = []
        pixmaps = [self.rythDensPixmap, self.rythRegPixmap, self.pitchRegPixmap, self.pitchStepPixmap, self.durPixmap, self.silencePixmap]
        pixmapNames = ['rythDens', 'rythReg', 'pitReg', 'pitStep', 'durLen', 'durDens'] 

        for inc in range(6):
            imgName = pixmapNames[inc]
            pixmap = pixmaps[inc]
            for i in range(6):	    
                pix = gtk.gdk.pixbuf_new_from_file(Config.IMAGE_ROOT + imgName + str(i+1) + '.png')
                map = gtk.gdk.Pixmap( win, pix.get_width(), pix.get_height() )
                map.draw_rectangle( self.gc, True, 0, 0, pix.get_width(), pix.get_height() )
                map.draw_pixbuf( self.gc, pix, 0, 0, 0, 0, pix.get_width(), pix.get_height(), gtk.gdk.RGB_DITHER_NONE )
                pixmap.append(map)


    def draw( self, widget, event ):
        imgX = 5 - int(self.rythmDensity * 5)
        imgY = 5 - int(self.rythmRegularity * 5)
        widget.window.draw_drawable( self.gc, self.arrowPixmap[0], 0, 0, 0, 18, 24, 24 )
        widget.window.draw_drawable( self.gc, self.rythDensPixmap[imgX], 0, 0, 24, 0, 90, 60 )
        widget.window.draw_drawable( self.gc, self.arrowPixmap[1], 0, 0, 114, 18, 24, 24 )
        widget.window.draw_drawable( self.gc, self.rythRegPixmap[imgY], 0, 0, 138, 0, 90, 60 )
        return True

    def draw2( self, widget, event ):
        imgX = 5 - int(self.pitchRegularity * 5)
        imgY = 5 - int(self.pitchStep * 5)
        widget.window.draw_drawable( self.gc, self.arrowPixmap[0], 0, 0, 0, 18, 24, 24 )
        widget.window.draw_drawable( self.gc, self.pitchRegPixmap[imgX], 0, 0, 24, 0, 90, 60 )
        widget.window.draw_drawable( self.gc, self.arrowPixmap[1], 0, 0, 114, 18, 24, 24 )
        widget.window.draw_drawable( self.gc, self.pitchStepPixmap[imgY], 0, 0, 138, 0, 90, 60 )
        return True

    def draw3( self, widget, event ):
        imgX = int(self.duration * 5)
        imgY = int(self.silence * 5)
        widget.window.draw_drawable( self.gc, self.arrowPixmap[0], 0, 0, 0, 18, 24, 24 )
        widget.window.draw_drawable( self.gc, self.durPixmap[imgX], 0, 0, 24, 0, 90, 60 )
        widget.window.draw_drawable( self.gc, self.arrowPixmap[1], 0, 0, 114, 18, 24, 24 )
        widget.window.draw_drawable( self.gc, self.silencePixmap[imgY], 0, 0, 138, 0, 90, 60 )
        return True




    def handleXAdjustment1( self, data ):
        self.rythmDensity = self.XAdjustment1.value / 200
        self.slider1Label.queue_draw()

    def handleYAdjustment1( self, data ):
        self.rythmRegularity = self.YAdjustment1.value / 200
        self.slider1Label.queue_draw()

    def handleXAdjustment2( self, data ):
        self.pitchRegularity = self.XAdjustment2.value / 200
        self.slider2Label.queue_draw()

    def handleYAdjustment2( self, data ):
        self.pitchStep = self.YAdjustment2.value / 200
        self.slider2Label.queue_draw()

    def handleXAdjustment3( self, data ):
        self.duration = self.XAdjustment3.value / 200
        self.slider3Label.queue_draw()

    def handleYAdjustment3( self, data ):
        self.silence = self.YAdjustment3.value / 200
        self.slider3Label.queue_draw()


    def getGenerationParameters( self ):
        return GenerationParameters( self.rythmDensity,
                                     self.rythmRegularity,
                                     self.pitchStep,
                                     self.pitchRegularity,
                                     self.duration,
                                     self.rythmMethod,
                                     self.pitchMethod,
                                     self.pattern,
                                     self.scale )

    def cancel( self, widget, data=None ):
        self.handleCloseWindowCallback()

    def generate(self, widget, data=None):
        self.generateFunction( self.getGenerationParameters() )


    def handleVariationButton( self, widget, var ):
        pass

    def variate( self, widget, data ):
        pass

    def handleMethod( self, widget, methode ):
        pass

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

    def formatRoundBox( self, box, fillcolor ):
        box.set_radius( 10 )
        box.set_border_width( 1 )
        box.set_fill_color( fillcolor )
        box.set_border_color( Config.INST_BCK_COLOR )
        return box


#=========================== PRESETS ================================

    def handleSave(self, widget, data):
        chooser = gtk.FileChooserDialog(title=None,action=gtk.FILE_CHOOSER_ACTION_SAVE, buttons=(gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_SAVE,gtk.RESPONSE_OK))

        if chooser.run() == gtk.RESPONSE_OK:
            try: 
                print 'INFO: save preset file %s' % chooser.get_filename()
                f = shelve.open( chooser.get_filename(), 'n')
                self.saveState(f)
                f.close()
            except IOError: 
                print 'ERROR: failed to save preset to file %s' % chooser.get_filename()

        chooser.destroy()
    
    def handleLoad(self, widget, data):
        
        chooser = gtk.FileChooserDialog(title=None,action=gtk.FILE_CHOOSER_ACTION_OPEN, buttons=(gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_OPEN,gtk.RESPONSE_OK))

        if chooser.run() == gtk.RESPONSE_OK:
            try: 
                print 'INFO: load preset state from file %s' % chooser.get_filename()
                f = shelve.open( chooser.get_filename(), 'r')
                self.loadState(f)
                f.close()
            except IOError: 
                print 'ERROR: failed to load preset state from file %s' % chooser.get_filename()

    def loadState( self, state ):
        pass
        self.rythmDensity = state['rythmDensity']
        self.rythmRegularity = state['rythmRegularity']
        self.pitchRegularity = state['pitchRegularity']
        self.pitchStep = state['pitchStep']
        self.duration = state['duration']
        self.silence = state['silence']

        self.XAdjustment1.set_value(self.rythmDensity*200)
        self.YAdjustment1.set_value(self.rythmRegularity*200)
        self.XAdjustment2.set_value(self.pitchRegularity*200)
        self.YAdjustment2.set_value(self.pitchStep*200)
        self.XAdjustment3.set_value(self.duration*200)
        self.YAdjustment3.set_value(self.silence*200)

    def saveState( self, state ):
        pass
        state['rythmDensity'] = self.rythmDensity
        state['rythmRegularity'] = self.rythmRegularity
        state['pitchRegularity'] = self.pitchRegularity
        state['pitchStep'] = self.pitchStep
        state['duration'] = self.duration
        state['silence'] = self.silence


#================================================================================= 

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
