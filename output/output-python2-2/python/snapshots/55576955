import pygtk
pygtk.require('2.0')
import gtk
import time
import shelve

from GUI.Core.KeyMapping import *
from GUI.Core.ThemeWidgets import *
from GUI.GUIConstants import GUIConstants
from Framework.CSound.CSoundClient import CSoundClient
from Framework.Constants import Constants
from GUI.SynthLab.SynthLabParametersWindow import SynthLabParametersWindow
from SynthLab.SynthObjectsParameters import SynthObjectsParameters
from SynthLab.SynthLabConstants import SynthLabConstants

class SynthLabWindow( gtk.Window ):
    def __init__( self, client, table ):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        color = gtk.gdk.color_parse('#FFFFFF')
        self.modify_bg(gtk.STATE_NORMAL, color)
	self.set_border_width(GUIConstants.MAIN_WINDOW_PADDING)
	self.csnd = client
	self.table = table
        self.set_decorated(False)
        self.synthObjectsParameters = SynthObjectsParameters()
        self.locations = SynthLabConstants.INIT_LOCATIONS[:]    
        self.buttonState = 0
	self.instanceOpen = 0
	self.recordWait = 0 
        self.duration = 1.5 
        self.playingPitch = []
        self.connections = []
        self.straightConnections = []
        self.cablesPoints = [] 
        self.pix = 8
        self.add_events(gtk.gdk.KEY_PRESS_MASK)
        self.add_events(gtk.gdk.KEY_RELEASE_MASK)
        self.connect("key-press-event", self.onKeyPress)
        self.connect("key-release-event", self.onKeyRelease)
        self.setupWindow()

    def setupWindow( self ):
        self.set_position( gtk.WIN_POS_CENTER_ON_PARENT )
        self.set_title("Synth Lab")
        self.mainBox = gtk.VBox()
	self.subBox = gtk.HBox()
	self.drawingBox = RoundVBox(fillcolor=GUIConstants.INST_BCK_COLOR)
	self.drawingBox.set_border_width(GUIConstants.BORDER_SIZE)
	self.drawingBox.set_radius(10)
	self.presetBox = RoundVBox(fillcolor=GUIConstants.PANEL_COLOR)
	self.presetBox.set_border_width(GUIConstants.BORDER_SIZE)
	self.presetBox.set_radius(10)
	self.presetBox.set_size_request(100, 790)
	self.subBox.pack_start(self.drawingBox, True, True)
	self.subBox.pack_start(self.presetBox, True, True)
	self.mainBox.pack_start(self.subBox)
	self.commandBox = gtk.HBox()

        self.sliderBox = RoundHBox(fillcolor=GUIConstants.PANEL_COLOR)
	self.sliderBox.set_border_width(GUIConstants.BORDER_SIZE)
	self.sliderBox.set_radius(10)
	self.commandBox.pack_start(self.sliderBox)
        self.buttonBox = RoundHBox(fillcolor=GUIConstants.PANEL_COLOR)
	self.buttonBox.set_border_width(GUIConstants.BORDER_SIZE)
	self.buttonBox.set_radius(10)
	self.commandBox.pack_start(self.buttonBox)
	self.mainBox.pack_start(self.commandBox)

        self.drawingArea = gtk.DrawingArea()
        self.drawingArea.set_size_request(1080, 790)
        self.col = gtk.gdk.color_parse(GUIConstants.INST_BCK_COLOR)
        self.drawingArea.modify_bg(gtk.STATE_NORMAL, self.col)
        self.drawingArea.add_events(gtk.gdk.BUTTON_PRESS_MASK)
        self.drawingArea.add_events(gtk.gdk.BUTTON_RELEASE_MASK)
        self.drawingArea.add_events(gtk.gdk.POINTER_MOTION_MASK)
        self.drawingArea.connect( "button-press-event", self.handleButtonPress )
        self.drawingArea.connect( "button-release-event", self.handleButtonRelease )
        self.drawingArea.connect( "motion-notify-event", self.handleMotion )
        self.drawingArea.connect("expose-event", self.draw)
        self.drawingBox.pack_start(self.drawingArea, False, False, 5)  
	self.presets = self.initRadioButton(SynthLabConstants.PRESET, self.presetCallback, self.presetBox)
	self.durLabel = gtk.Image()
        self.durLabel.set_from_file(GUIConstants.IMAGE_ROOT + 'dur2.png')
        self.durAdjust = gtk.Adjustment(1.5, .5, 4, .01, .01, 0)
        self.durAdjust.connect("value-changed", self.handleDuration)
        self.durationSlider = ImageHScale( Constants.TAM_TAM_ROOT + "/Resources/Images/sliderbutviolet.png", self.durAdjust, 7 )
        self.durationSlider.set_digits(2)
        self.durationSlider.set_value_pos(1)
        self.durationSlider.set_inverted(False)
        self.durationSlider.set_size_request(750, 30)
        self.sliderBox.pack_start(self.durationSlider, True, True, 5)
	self.sliderBox.pack_start(self.durLabel, False, padding=10)
        saveButton = ImageButton(Constants.TAM_TAM_ROOT + '/Resources/Images/save.png')
        saveButton.connect("clicked", self.handleSave, None)
        self.buttonBox.pack_start(saveButton, False, False, 2)

        loadButton = ImageButton(Constants.TAM_TAM_ROOT + '/Resources/Images/load.png')
        loadButton.connect("clicked", self.handleLoad, None)
        self.buttonBox.pack_start(loadButton, False, False, 2)

	self.recordButton = ImageToggleButton(GUIConstants.IMAGE_ROOT + 'record2.png', GUIConstants.IMAGE_ROOT + 'record2sel.png')
        self.recordButton.connect("clicked", self.recordSound)
        self.buttonBox.pack_start(self.recordButton, False, False, 2)

        resetButton = ImageButton(Constants.TAM_TAM_ROOT + '/Resources/Images/reset.png')
        resetButton.connect("clicked", self.handleReset, None)
        self.buttonBox.pack_start(resetButton, False, False, 2)

        closeButton = ImageButton(Constants.TAM_TAM_ROOT + '/Resources/Images/close.png')
        closeButton.connect("clicked", self.handleClose, None)
        self.buttonBox.pack_start(closeButton, False, False, 2)

        self.add(self.mainBox)

        self.writeTables( self.synthObjectsParameters.types, self.synthObjectsParameters.controlsParameters, 
                        self.synthObjectsParameters.sourcesParameters, self.synthObjectsParameters.fxsParameters )
	self.presetCallback(self.presets,0)

    def onKeyPress(self,widget,event):
        midiPitch = KEY_MAP[event.hardware_keycode]
        if midiPitch not in self.playingPitch:
	    if self.recordWait == 0:
                self.playingPitch.append( midiPitch )
                self.playNote( midiPitch )
	    else:
        	self.csnd.sendText("perf.InputMessage('i5204 0.005 " + str(self.duration) + " " + str(self.table) + "')")
		self.recordWait = 0
        	time.sleep(0.005)
                self.playingPitch.append( midiPitch )
                self.playNote( midiPitch )
		self.waitRecording()	

    def waitRecording(self):
	time.sleep(self.duration)	
	self.recordButton.set_active(False)
        
    def onKeyRelease( self, widget, event ):
        midiPitch = KEY_MAP[event.hardware_keycode]
        if midiPitch in self.playingPitch:
            self.playingPitch.remove( midiPitch )

    def handleDuration( self, data ):
        self.duration = self.durAdjust.value
	img = int((self.duration - .5) * 1.425 + 1)
	self.durLabel.set_from_file(GUIConstants.IMAGE_ROOT + 'dur' + str(img) + '.png')

    def playNote( self, midiPitch ):
        cpsPitch = 261.626*pow(1.0594633, midiPitch-36)
        mess = "perf.InputMessage('i5203 0 " + str(self.duration) + " " + str(cpsPitch) + "')"
        self.csnd.sendText( mess )

    def handleClose( self, widget, data ):
        self.hide()

    def handleReset( self, widget, data ):
        self.locations = SynthLabConstants.INIT_LOCATIONS[:]    
        self.duration = 1 
        self.connections = []
        self.synthObjectsParameters.__init__()
        self.writeTables( self.synthObjectsParameters.types, self.synthObjectsParameters.controlsParameters, 
                        self.synthObjectsParameters.sourcesParameters, self.synthObjectsParameters.fxsParameters )
        self.synthObjectsParameters.update()
        time.sleep(.01)
        self.allConnections()
        self.controlToSrcConnections()
        time.sleep(.01)
        self.controlToFxConnections()
        time.sleep(.01)
        self.audioConnections()
        time.sleep(.01)
        self.queue_draw()

    def handleButtonRelease( self, widget, event ):
        if self.buttonState:
	    self.buttonState = 0
            self.queue_draw()

    def handleButtonPress( self, widget, event):
        if event.button == 1:
            for i in self.locations:
                if (i[0]-self.pix) < event.x < (i[0]+self.pix) and (i[1]+(SynthLabConstants.HALF_SIZE-self.pix)) < event.y < (i[1]+(SynthLabConstants.HALF_SIZE+self.pix)) and self.locations.index(i) < 12:
                    self.setConnection( 1, event, self.locations.index(i) )
                    gate = 0
                    break
                elif (i[0]-SynthLabConstants.HALF_SIZE) < event.x < (i[0]+SynthLabConstants.HALF_SIZE) and (i[1]-(SynthLabConstants.HALF_SIZE+self.pix)) < event.y < (i[1]-(SynthLabConstants.HALF_SIZE-self.pix)) and 3 < self.locations.index(i) < 8:
                    self.setConnection( 2, event, self.locations.index(i) )
                    gate = 0
                    break
                elif (i[0]+(SynthLabConstants.HALF_SIZE-self.pix)) < event.x < (i[0]+(SynthLabConstants.HALF_SIZE+self.pix)) and (i[1]-SynthLabConstants.HALF_SIZE) < event.y < (i[1]+SynthLabConstants.HALF_SIZE) and self.locations.index(i) > 7:
                    self.setConnection( 2, event, self.locations.index(i) )
                    gate = 0
                    break
                elif (i[0]-self.pix) < event.x < (i[0]+self.pix) and (i[1]-(SynthLabConstants.HALF_SIZE+self.pix)) < event.y < (i[1]-(SynthLabConstants.HALF_SIZE-self.pix)) and self.locations.index(i) > 7:
                    self.setConnection( 2, event, self.locations.index(i) )
                    gate = 0
                    break
                elif (i[0]-SynthLabConstants.HALF_SIZE) < event.x < (i[0]+SynthLabConstants.HALF_SIZE) and (i[1]-SynthLabConstants.HALF_SIZE) < event.y < (i[1]+SynthLabConstants.HALF_SIZE):
                    self.buttonState = 1
                    self.choosen = self.locations.index(i)
                    gate = 0
                    break
                else:
                    gate = 1
            if gate:
                self.deleteCable(  event )
        elif event.button == 3:
            for i in self.locations:
                if (i[0]-SynthLabConstants.HALF_SIZE) < event.x < (i[0]+SynthLabConstants.HALF_SIZE) and (i[1]-SynthLabConstants.HALF_SIZE) < event.y < (i[1]+SynthLabConstants.HALF_SIZE):
		    if self.instanceOpen:
		        self.synthLabParametersWindow.destroy()
                    instanceID = self.locations.index(i)
                    self.synthLabParametersWindow = SynthLabParametersWindow( instanceID, self.synthObjectsParameters, self.writeTables, self.playNote )
		    self.instanceOpen = 1

    def handleMotion( self, widget, event ):
        if self.buttonState == 1 and self.choosen != 12:
            if 0+SynthLabConstants.HALF_SIZE < event.x < 1200-SynthLabConstants.HALF_SIZE:
                X = event.x
            if 0+SynthLabConstants.HALF_SIZE < event.y < 820-SynthLabConstants.HALF_SIZE:
                Y = event.y
            self.mouse = [ X, Y ]
            self.locations[self.choosen] = [ X, Y ]
 	    if Y > 700:     
                self.queue_draw_area(0,695, 1200, 120)
	    else:
                self.queue_draw_area(X-40, Y-40, 80, 80)

    def setConnection( self, gate, event, sourceLocation ):
        if gate == 1: # output connection
            self.temp = []
            self.temp.append( (sourceLocation, 0, SynthLabConstants.HALF_SIZE ) )
        if gate == 2: 
            # source control parameter input connection 
            if self.temp[0][0] < 4 and sourceLocation < 8:
                first = self.nearest(event.x - self.locations[sourceLocation][0], [-18, -6, 7, 20]) 
                second = -SynthLabConstants.HALF_SIZE 
                self.temp.append( (sourceLocation, first, second, 0) )
                self.connections.append( self.temp )
            # fx control parameter input connection 
            if self.temp[0][0] < 4 and 7 < sourceLocation < 12:
                first = SynthLabConstants.HALF_SIZE
                second = self.nearest(event.y - self.locations[sourceLocation][1], [-15, -5, 5, 15]) 
                self.temp.append( (sourceLocation, first, second, 0) )
                self.connections.append( self.temp )
            # source and fx to fx and out connection
            if self.temp[0][0] > 3 and self.temp[0][0] < 12 and sourceLocation > 7:
                refused = self.connectionGating()
                if sourceLocation not in refused:
                    first = 0
                    second = -SynthLabConstants.HALF_SIZE
                else:
                    print 'refused'
                self.temp.append( (sourceLocation, first, second, 0) )
                self.connections.append( self.temp )
            self.allConnections()
            self.controlToSrcConnections()
            time.sleep(.01)
            self.controlToFxConnections()
            time.sleep(.01)
            self.audioConnections()
            time.sleep(.01)
            lastTable = [0]*12
            for i in range(12):
                if i in self.outputs:            
                    lastTable[i] = (self.synthObjectsParameters.types[i]+1)           
            mess = "perf.InputMessage('f5203 0 16 -2 " + " "  .join([str(n) for n in lastTable]) + " 0 0 0 0')"
            self.csnd.sendText( mess )
            time.sleep(.01)
            self.queue_draw()

    def nearest( self, val, mainList ):
        diffList = [abs(i-val) for i in mainList]
        return mainList[diffList.index(min(diffList))]
        
    def draw( self, widget, event ):
        context = self.drawingArea.window.cairo_create()
        context.set_line_width( 2 ) 
	context.move_to(0, 710)
	context.line_to(1080, 710)
	if self.buttonState == 1:
	    for i in self.locations:
                X, Y = i[0], i[1]
	        context.move_to(X-20, Y-20) 
                context.line_to(X+20, Y-20)
                context.line_to(X+20, Y+20)
                context.line_to(X-20, Y+20)
                context.line_to(X-20, Y-20)
	elif self.buttonState == 0:
	    for i in self.locations:
	        if i[1] > 710:
		    ind = self.locations.index(i)
		    self.locations[ind][0] = SynthLabConstants.INIT_LOCATIONS[ind][0]
		    self.locations[ind][1] = SynthLabConstants.INIT_LOCATIONS[ind][1]
            for i in self.locations:
                index = self.locations.index(i)
                context.set_source_pixbuf(self.pixbufs[index], i[0]-SynthLabConstants.HALF_SIZE, i[1]-SynthLabConstants.HALF_SIZE) 
                context.paint()
        if self.connections and not self.buttonState:
            for i in self.connections:                  
                context.move_to( self.locations[i[0][0]][0]+i[0][1], self.locations[i[0][0]][1]+i[0][2])
                context.line_to( self.locations[i[1][0]][0]+i[1][1], self.locations[i[1][0]][1]+i[1][2])
        context.set_source_rgb( .5, .5, .5 )  
        context.stroke() 

    def connectionGating( self ):
        self.straightConnections = [[i[0][0], i[1][0]] for i in self.connections]
        self.fxConnections = [i for i in self.straightConnections if 7 < i[0] < 12 and 7 < i[1] < 12]

        fxConnectionRefused = [i[0] for i in self.fxConnections if i[1] == self.temp[0][0]]
        fxConnectionRefused2 = [k[0] for j in fxConnectionRefused for k in self.fxConnections if k[1] == j]
        fxConnectionRefused.extend(fxConnectionRefused2)

        return fxConnectionRefused

    def writeTables( self, typesTable, controlParametersTable, sourceParametersTable, fxParametersTable ):
        mess = "perf.InputMessage('f5200 0 16 -2 " + " "  .join([str(n) for n in controlParametersTable]) + "')"
        self.csnd.sendText( mess )
        time.sleep(.01)
        mess = "perf.InputMessage('f5201 0 16 -2 " + " "  .join([str(n) for n in sourceParametersTable]) + "')"
        self.csnd.sendText( mess )
        time.sleep(.01)
        mess = "perf.InputMessage('f5202 0 16 -2 " + " "  .join([str(n) for n in fxParametersTable]) + "')"
        self.csnd.sendText( mess )
        time.sleep(.01)
        lastTable = [0]*12
        self.allConnections()
        for i in range(12):
            if i in self.outputs:            
                lastTable[i] = (typesTable[i]+1)
        mess = "perf.InputMessage('f5203 0 16 -2 " + " "  .join([str(n) for n in lastTable]) + " 0 0 0 0')"
        self.csnd.sendText( mess )
        time.sleep(.01)
        self.loadPixbufs(typesTable)
	self.queue_draw()

    def recordSound( self, widget, data=None ):
	if widget.get_active() == True:
	    self.recordWait = 1
	else: 
	    self.recordWait = 0

    def allConnections( self ): 
        self.straightConnections = []
        self.outputs = []
        self.inputs = []
        self.checkConnections = []
        self.cablesPoints = []
        for i in self.connections:
            first = i[0][0]
            second = i[1][0]
            self.straightConnections.append([first, second])
            self.outputs.append(first)
            self.inputs.append(second)
            self.checkConnections.extend([first, second])
            firstX = self.locations[i[0][0]][0] + i[0][1]
            firstY = self.locations[i[0][0]][1] + i[0][2]
            secondX = self.locations[i[1][0]][0] + i[1][1]
            secondY = self.locations[i[1][0]][1] + i[1][2]
            XPoint = [int(firstX), int(secondX)]
            YPoint = [int(firstY), int(secondY)]
            self.cablesPoints.append([XPoint, YPoint])

    def deleteCable( self, event ):
        if self.cablesPoints:
	    print self.cablesPoints
            gate = 1
            for point in self.cablesPoints:
                Xmin = min(point[0])
                Xmax = max(point[0])
                Ymin = min(point[1])
                Ymax = max(point[1])
                if event.x in range(Xmin, Xmax) and event.y in range(Ymin, Ymax):
                    XDiff = (event.x - Xmin) / (Xmax - Xmin)
                    YDiff = (event.y - Ymin) / (Ymax - Ymin)
                    if Xmin == point[0][0] and Ymin == point[1][0] or Xmax == point[0][0] and Ymax == point[1][0]:
                            if -.08 < (XDiff - YDiff) < .08:
                                if gate:
                                    del self.connections[self.cablesPoints.index(point)]
                                    self.connectAndDraw()
                                    gate = 0
                    else:
                            if .92 < (XDiff + YDiff) < 1.08:
                                if gate:
                                    del self.connections[self.cablesPoints.index(point)]
                                    self.connectAndDraw()
                                    gate = 0
	else:
	    print '********** no connections **********'

    def connectAndDraw( self ):
        self.allConnections()
        self.controlToSrcConnections()
        time.sleep(.01)
        self.controlToFxConnections()
        time.sleep(.01)
        self.audioConnections()
        time.sleep(.01)
        self.queue_draw_area(0, 0, 1200, 790)

    def controlToSrcConnections( self ):
        self.contSrcConnections = []
        for i in self.connections:
            if i[0][0] < 4 and 3 < i[1][0] < 8:
                offset = (SynthLabConstants.HALF_SIZE+i[1][1]) / (SynthLabConstants.PIC_SIZE/4)
                self.contSrcConnections.append([i[0][0], i[1][0], offset])           
        table = [0 for i in range(16)]
        sources = [source for source in range(4,8) if source in self.outputs]
        for source in sources:
            for entre in range(4):
                value = sum([2**(li[0]+1) for li in self.contSrcConnections if li[1] == source and li[2] == entre], 1)
                table[(source % 4) * 4 + entre] = value
        mess = "perf.InputMessage('f5204 0 16 -2 " + " "  .join([str(n) for n in table]) + "')"
        self.csnd.sendText( mess )

    def controlToFxConnections( self ):
        self.contFxConnections = []
        for i in self.connections:
            if i[0][0] < 4 and 7 < i[1][0] < 12:
                offset = ((SynthLabConstants.HALF_SIZE/2)+i[1][2]) / (SynthLabConstants.PIC_SIZE/6)
                self.contFxConnections.append([i[0][0], i[1][0], offset])
        table = [0 for i in range(16)]
        fxs = [fx for fx in range(8,12) if fx in self.outputs]
        for fx in fxs:
            for entre in range(4):
                value = sum([2**(li[0]+1) for li in self.contFxConnections if li[1] == fx and li[2] == entre], 1)
                table[(fx % 4) * 4 + entre] = value
        mess = "perf.InputMessage('f5205 0 16 -2 " + " "  .join([str(n) for n in table]) + "')"
        self.csnd.sendText( mess )

    def audioConnections( self ):
        self.srcFxConnections = [i for i in self.straightConnections if 3 < i[0] < 8 and 7 < i[1] < 12]
        self.fxConnections = [i for i in self.straightConnections if 7 < i[0] < 12 and 7 < i[1] < 12]
        self.outConnections = [i[0] for i in self.straightConnections if i[1] == 12]

        table = []
        for fx in range(8, 12):
            value = 0
            for li in self.srcFxConnections:
                if li[1] == fx:
                    value += pow(2, li[0]-4)
            table.append(value)

        for fx in range(8, 12):
            value = 0
            for li in self.fxConnections:
                if li[1] == fx:
                    value += pow(2, li[0]-8)
            table.append(value)

        for sig in range(4, 12):
            value = 0
            if sig in self.outConnections:
                value = 1
            table.append(value)
        mess = "perf.InputMessage('f5206 0 16 -2 " + " "  .join([str(n) for n in table]) + "')"
        self.csnd.sendText( mess )

    def loadPixbufs( self, typesList ):
        self.pixbufs = []
        for i in range(13):	    
            if i < 4:
		img = SynthLabConstants.CHOOSE_TYPE_PLUS[0][typesList[i]]
                self.pixbufs.append(gtk.gdk.pixbuf_new_from_file(GUIConstants.IMAGE_ROOT + img + '.png'))
            elif i < 8:
		img = SynthLabConstants.CHOOSE_TYPE_PLUS[1][typesList[i]]
                self.pixbufs.append(gtk.gdk.pixbuf_new_from_file(GUIConstants.IMAGE_ROOT + img + '.png'))
            elif i < 12:
		img = SynthLabConstants.CHOOSE_TYPE_PLUS[2][typesList[i]]
                self.pixbufs.append(gtk.gdk.pixbuf_new_from_file(GUIConstants.IMAGE_ROOT + img + '.png'))
            else:
                self.pixbufs.append(gtk.gdk.pixbuf_new_from_file(Constants.TAM_TAM_ROOT + '/Resources/Images/speaker.png'))

    def handleSave(self, widget, data):
        chooser = gtk.FileChooserDialog(title=None,action=gtk.FILE_CHOOSER_ACTION_SAVE, buttons=(gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_SAVE,gtk.RESPONSE_OK))

        if chooser.run() == gtk.RESPONSE_OK:
            try: 
                print 'INFO: save SynthLab file %s' % chooser.get_filename()
                f = shelve.open( chooser.get_filename(), 'n')
                self.saveState(f)
                f.close()
            except IOError: 
                print 'ERROR: failed to save SynthLab to file %s' % chooser.get_filename()

        chooser.destroy()
    
    def handleLoad(self, widget, data):
        chooser = gtk.FileChooserDialog(title=None,action=gtk.FILE_CHOOSER_ACTION_OPEN, buttons=(gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_OPEN,gtk.RESPONSE_OK))

        if chooser.run() == gtk.RESPONSE_OK:
            try: 
                print 'INFO: load SynthLab state from file %s' % chooser.get_filename()
                f = shelve.open( chooser.get_filename(), 'r')
                self.loadState(f)
                f.close()
            except IOError: 
                print 'ERROR: failed to load SynthLab state from file %s' % chooser.get_filename()

        chooser.destroy()

    def saveState( self, state ):
        state['types'] = self.synthObjectsParameters.types
        state['controls'] = self.synthObjectsParameters.controlsParameters
        state['sources'] = self.synthObjectsParameters.sourcesParameters
        state['fxs'] = self.synthObjectsParameters.fxsParameters
        state['locations'] = self.locations
        state['connections'] = self.connections
        state['duration'] = self.duration

    def loadState( self, state ):
        self.synthObjectsParameters.types = state['types']
        self.synthObjectsParameters.controlsParameters = state['controls']    
        self.synthObjectsParameters.sourcesParameters = state['sources']
        self.synthObjectsParameters.fxsParameters = state['fxs']
        self.locations = state['locations']
        self.connections = state['connections']
        self.duration = state['duration']
	self.durAdjust.set_value(self.duration)

        self.writeTables( self.synthObjectsParameters.types, self.synthObjectsParameters.controlsParameters, 
                        self.synthObjectsParameters.sourcesParameters, self.synthObjectsParameters.fxsParameters )
        self.synthObjectsParameters.update()
        time.sleep(.01)
        self.allConnections()
        self.controlToSrcConnections()
        time.sleep(.01)
        self.controlToFxConnections()
        time.sleep(.01)
        self.audioConnections()
        time.sleep(.01)
        self.queue_draw()

    def presetCallback( self, widget, data ):
	preset = 'synthFile' + str(data+1)
        f = shelve.open( Constants.TAM_TAM_ROOT + '/Resources/SynthFiles/' + preset, 'r')
        self.loadState(f)
        f.close()

    def initRadioButton( self, labelList, methodCallback, box ):
        for i in range( len( labelList ) ):
	    label = labelList[i]
            if i == 0:
                button = ImageRadioButton( group = None, mainImg_path = GUIConstants.IMAGE_ROOT + label + '.png', altImg_path = GUIConstants.IMAGE_ROOT + label + 'sel.png' )
            else:
                button = ImageRadioButton( group = button, mainImg_path = GUIConstants.IMAGE_ROOT + label + '.png', altImg_path = GUIConstants.IMAGE_ROOT + label + 'sel.png' )
            button.connect( "toggled", methodCallback, i )
            box.pack_start( button, True, True )
