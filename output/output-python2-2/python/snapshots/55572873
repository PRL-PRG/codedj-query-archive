import pygtk
pygtk.require('2.0')
import gtk

from Util.Profiler import TP

import gobject
import time
import shelve
from gettext import gettext as _
import os

import Config
from Util.ThemeWidgets import *
from Util.CSoundClient import new_csound_client
from SynthLab.SynthLabParametersWindow import SynthLabParametersWindow
from SynthLab.SynthObjectsParameters import SynthObjectsParameters
from SynthLab.SynthLabConstants import SynthLabConstants
from SynthLab.Parameter import Parameter
from SynthLab.SynthLabToolbars import mainToolbar
from SynthLab.SynthLabToolbars import presetToolbar
from Util.Trackpad import Trackpad
from SubActivity import SubActivity
Tooltips = Config.Tooltips

as_window = False

class SynthLabWindow(SubActivity):
    def __init__( self, activity, set_mode, dummy_to_change_signature ):
        SubActivity.__init__(self, set_mode)
        if as_window:
            color = gtk.gdk.color_parse(Config.PANEL_BCK_COLOR)
            self.modify_bg(gtk.STATE_NORMAL, color)
            self.set_border_width(Config.MAIN_WINDOW_PADDING)
            self.set_keep_above(False)
            self.set_decorated(False)
        self.activity = activity
        self.csnd = new_csound_client()
        self.trackpad = Trackpad( self )
        self.synthObjectsParameters = SynthObjectsParameters()
        self.resetLocations()
        self.objectCount = len(self.locations)
        self.connections = []
        self.initializeConnections()
        self.bounds = []
        for i in range(self.objectCount):
            self.bounds.append([0,0,0,0])
            self.updateBounds(i)
        self.instanceOpen = 0
        self.recordWait = 0
        self.recCount = 0
        self.duration = 2
        self.durString = '%.2f' % self.duration
        self.playingPitch = []
        self.journalCalled = True

        #Toolbars
        self._mainToolbar = mainToolbar(self.activity.toolbox, self)
        self._presetToolbar = presetToolbar(self.activity.toolbox, self)
        self.activity.toolbox.add_toolbar(_('Main'), self._mainToolbar)
        self.activity.toolbox.add_toolbar(_('Presets'), self._presetToolbar)
        self.activity.toolbox.set_current_toolbar(1)
        self._mainToolbar.show()
        self._presetToolbar.show()

        loopPointsTable = []
        sample_names = [name for i in range( len( Config.INSTRUMENTS ) ) for name in Config.INSTRUMENTS.keys() if Config.INSTRUMENTS[ name ].instrumentId == i ]
        for inst in sample_names:
            loopStart = Config.INSTRUMENTS[ inst ].loopStart
            loopEnd = Config.INSTRUMENTS[ inst ].loopEnd
            crossDur = Config.INSTRUMENTS[ inst ].crossDur
            loopPointsTable.extend( [ loopStart, loopEnd, crossDur ] )
        mess = "f5755 0 512 -2 " + " "  .join([str(n) for n in loopPointsTable])
        self.csnd.inputMessage( mess )

        self.lineWidth = 3
        self.lineWidthMUL2 = self.lineWidth*2
        self.lineWidthMUL4 = self.lineWidth*4
        self.lineWidthMUL4SQ = self.lineWidthMUL4*self.lineWidthMUL4
        self.pix = 10
        self.parameterOpen = 0
        self.clockStart = 0
        self.sample_names = [name for i in range( len( Config.INSTRUMENTS ) ) for name in Config.INSTRUMENTS.keys() if Config.INSTRUMENTS[ name ].instrumentId == i ]
        self.tooltips = gtk.Tooltips()
        if as_window:
            self.add_events(gtk.gdk.KEY_PRESS_MASK|gtk.gdk.KEY_RELEASE_MASK)

        self.action = None
        self.dragObject = None
        self.overWire = None
        self.overGate = None
        self.overGateObj = None
        self.overGateReject = False
        self.overGateSize = 32
        self.overGateSizeDIV2 = self.overGateSize//2
        self.overLineWidth = self.lineWidth*2
        self.overLineWidthMUL2 = self.overLineWidth*2

        self.gatePoint = SynthLabConstants.GATE_POINT
        self.gateMap = SynthLabConstants.GATE_MAP
        # look up gate type to find the matching gate type
        self.gateMatch = [ SynthLabConstants.GT_CONTROL_INPUT,
                           SynthLabConstants.GT_CONTROL_OUTPUT,
                           SynthLabConstants.GT_SOUND_INPUT,
                           SynthLabConstants.GT_SOUND_OUTPUT ]

        # set up window
        if as_window:
            self.set_position( gtk.WIN_POS_CENTER_ON_PARENT )
            self.set_title("Synth Lab")
        self.mainBox = gtk.VBox()
        self.subBox = gtk.HBox()
        self.drawingBox = RoundVBox( 10, Config.INST_BCK_COLOR, Config.PANEL_BCK_COLOR )
        self.drawingBox.set_border_width(Config.PANEL_SPACING)
        self.presetBox = RoundVBox( 10, Config.PANEL_COLOR, Config.PANEL_BCK_COLOR )
        self.presetBox.set_border_width(Config.PANEL_SPACING)
        self.presetBox.set_size_request(100, 790)
        self.subBox.pack_start(self.drawingBox, True, True)
        #self.subBox.pack_start(self.presetBox, True, True)
        self.mainBox.pack_start(self.subBox)
        self.commandBox = gtk.HBox()

        self.sliderBox = RoundHBox( 10, Config.PANEL_COLOR, Config.PANEL_BCK_COLOR )
        self.sliderBox.set_border_width(Config.PANEL_SPACING)
        self.commandBox.pack_start(self.sliderBox)
        self.buttonBox = RoundHBox( 10, Config.PANEL_COLOR, Config.PANEL_BCK_COLOR )
        self.buttonBox.set_border_width(Config.PANEL_SPACING)
        self.commandBox.pack_start(self.buttonBox)
        #self.mainBox.pack_start(self.commandBox)

        self.drawingAreaWidth = 1200
        self.drawingAreaHeight = 750
        self.separatorY = 640

        self.clearMask = gtk.gdk.Rectangle(0,0,self.drawingAreaWidth,self.drawingAreaHeight)

        win = gtk.gdk.get_default_root_window()
        self.gc = gtk.gdk.GC( win )
        self.gc.set_line_attributes( self.lineWidth, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_BUTT, gtk.gdk.JOIN_MITER )

        self.dirtyRectToAdd = gtk.gdk.Rectangle()
        self.dirty = False

        self.screenBuf = gtk.gdk.Pixmap( win, self.drawingAreaWidth, self.drawingAreaHeight )
        self.screenBufDirtyRect = gtk.gdk.Rectangle()
        self.screenBufDirty = False

        self.drawingArea = gtk.DrawingArea()
        self.drawingArea.set_size_request( self.drawingAreaWidth, self.drawingAreaHeight )
        self.col = gtk.gdk.color_parse(Config.INST_BCK_COLOR)
        colormap = self.drawingArea.get_colormap()
        self.bgColor = colormap.alloc_color( Config.INST_BCK_COLOR, True, True )
        self.lineColor = colormap.alloc_color( Config.SL_LINE_COLOR, True, True )
        self.overWireColor = colormap.alloc_color( Config.SL_OVER_WIRE_COLOR, True, True )
        self.overGateColor = colormap.alloc_color( Config.SL_OVER_GATE_COLOR, True, True )
        self.overGateRejectColor = colormap.alloc_color( Config.SL_OVER_GATE_REJECT_COLOR, True, True )
        self.drawingArea.modify_bg(gtk.STATE_NORMAL, self.col)
        self.drawingArea.add_events( gtk.gdk.BUTTON_PRESS_MASK
                                   | gtk.gdk.BUTTON_RELEASE_MASK
                                   | gtk.gdk.POINTER_MOTION_MASK
                                   | gtk.gdk.POINTER_MOTION_HINT_MASK )
        self.drawingArea.connect( "button-press-event", self.handleButtonPress )
        self.drawingArea.connect( "button-release-event", self.handleButtonRelease )
        self.drawingArea.connect( "motion-notify-event", self.handleMotion )
        self.drawingArea.connect("expose-event", self.draw)
        self.drawingBox.pack_start(self.drawingArea, False, False, 5)
        self.presets = self.initRadioButton(SynthLabConstants.PRESET, self.presetCallback, self.presetBox)
        self.durLabel = gtk.Image()
        self.durLabel.set_from_file(Config.IMAGE_ROOT + 'dur2.png')
        self.durAdjust = gtk.Adjustment(2, .5, 10, .01, .01, 0)
        self.durAdjust.connect("value-changed", self.handleDuration)
        self.durationSlider = ImageHScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderbutviolet.png", self.durAdjust, 7 )
        self.durationSlider.connect("button-press-event", self.showParameter)
        self.durationSlider.connect("button-release-event", self.hideParameter)
        self.durationSlider.set_size_request(440, 30)
        self.sliderBox.pack_start(self.durationSlider, True, True, 5)
        self.sliderBox.pack_start(self.durLabel, False, padding=10)

#        for i in [1,2,3,4,5,6]:
#            recordButton = ImageToggleButton(Config.IMAGE_ROOT + 'synthRecord' + str(i) + '.png', Config.IMAGE_ROOT + 'synthRecord' + str(i) + 'Down.png', Config.IMAGE_ROOT + 'synthRecord' + str(i) + 'Over.png')
#            recordButton.connect("clicked", self.recordSound, i)
#            self.buttonBox.pack_start(recordButton, False, False, 2)
#            self.tooltips.set_tip(recordButton, Tooltips.SL_RECORDBUTTONS[i-1])

#        saveButton = ImageButton(Config.IMAGE_ROOT + 'save.png')
#        saveButton.connect("clicked", self.handleSave, None)
#        self.buttonBox.pack_start(saveButton, False, False, 2)

#        loadButton = ImageButton(Config.IMAGE_ROOT + 'load.png')
#        loadButton.connect("clicked", self.handleLoad, None)
#        self.buttonBox.pack_start(loadButton, False, False, 2)

#        resetButton = ImageButton(Config.IMAGE_ROOT + 'reset.png')
#        resetButton.connect("clicked", self.handleReset, None)
#        self.buttonBox.pack_start(resetButton, False, False, 2)

#        closeButton = ImageButton(Config.IMAGE_ROOT + 'close.png')
#        closeButton.connect("clicked", self.handleClose, None)
#        self.buttonBox.pack_start(closeButton, False, False, 2)

#        self.tooltips.set_tip(saveButton, Tooltips.SAVE)
#        self.tooltips.set_tip(loadButton, Tooltips.LOAD)
#        self.tooltips.set_tip(resetButton, Tooltips.RESET)
#        self.tooltips.set_tip(closeButton, Tooltips.CLOSE)
#        self.tooltips.set_tip(self.durationSlider, Tooltips.SOUNDDUR + ': ' + self.durString)

        tempFile = 'synthTemp'
        if tempFile in os.listdir(Config.PREF_DIR):
            self.handleLoadTemp()
        else:
            self.presetCallback(self.presets,1)
        self.add(self.mainBox)
        self.show_all()

    def onDestroy(self):
        pass

    def onKeyPress(self,widget,event):
        key = event.hardware_keycode
        #temporary binding
        if key == 50:
            self.handleSave(None, None)

        if key not in Config.KEY_MAP:
            return
        midiPitch = Config.KEY_MAP[key]
        if midiPitch not in self.playingPitch:
            if self.recordWait == 0:
                self.playingPitch.append( midiPitch )
                self.playNote( midiPitch, 0 )
            else:
                self.recordWait = 0
                self.playingPitch.append( midiPitch )
                self.playNote( midiPitch, self.table )
                self.waitRecording()

    def resetRecord( self ):
        gobject.source_remove( self.wait )
        self.recordButton.set_active(False)
        inst = 'lab' + str(self.table-85)
        self.csnd.load_synth_instrument(inst)
        self.table = 0
        return True

    def waitRecording(self):
        self.wait = gobject.timeout_add(int(self.duration*1000) , self.resetRecord )

    def onKeyRelease( self, widget, event ):
        key = event.hardware_keycode
        if key not in Config.KEY_MAP:
            return
        midiPitch = Config.KEY_MAP[key]
        if midiPitch in self.playingPitch:
            self.playingPitch.remove( midiPitch )

    def handleDuration( self, adjustment ):
        self.duration = adjustment.value
        self.durString = '%.2f' % self.duration
        img = int((self.duration - .5) * .5 + 1)
        self.parameterUpdate(self.durString)
        self.tooltips.set_tip(self.durationSlider, Tooltips.SOUNDDUR + ': ' + self.durString)

    def showParameter( self, widget, data=None ):
        if not self.parameterOpen:
            self.parameter = Parameter(self.durString)
            self.parameterOpen = 1

    def hideParameter( self, widget, data=None ):
        if self.parameterOpen and not self.clockStart:
            self.windowCloseDelay = gobject.timeout_add(500, self.closeParameterWindow)
            self.clockStart = 1

    def closeParameterWindow( self ):
        if self.parameterOpen:
            self.parameter.hide()
            self.parameterOpen = 0
            gobject.source_remove( self.windowCloseDelay )
            self.clockStart = 0
            self.tooltips.set_tip(self.durationSlider, Tooltips.SOUNDDUR + ': ' + self.durString)
        return True

    def parameterUpdate( self, durString ):
        if self.parameterOpen:
            self.parameter.update(durString)

    def playNote( self, midiPitch, table ):
        cpsPitch = 261.626*pow(1.0594633, midiPitch-36)
        self.recCount += 1
        mess = "i5203." + str(self.recCount) + " 0 " + str(self.duration) + " " + str(cpsPitch) + " " + str(table) + " " + " " .join([str(n) for n in self.synthObjectsParameters.getOutputParameters()])
        self.csnd.inputMessage( mess )
        if self.recCount >= 9: self.recCount = 0

    def handleClose( self, widget, data ):
        if self.journalCalled:
            self.set_mode('quit')
            return
        if self.instanceOpen:
            self.synthLabParametersWindow.destroy()
        self.set_mode('welcome')
        if as_window:
            self.set_keep_above(False)
            self.hide()

    def resetLocations( self ):
        # deep copy the list
        self.locations = [ loc[:] for loc in SynthLabConstants.INIT_LOCATIONS ]

    def handleReset( self, widget, data = None):
        self.resetLocations()
        self.objectCount = len(self.locations)
        for i in range(self.objectCount):
            self.updateBounds( i )
        self.duration = 2
        self.durAdjust.set_value(self.duration)
        self.connections = []
        self.synthObjectsParameters.__init__()
        self.writeTables( self.synthObjectsParameters.types, self.synthObjectsParameters.controlsParameters, self.synthObjectsParameters.sourcesParameters, self.synthObjectsParameters.fxsParameters )
        self.synthObjectsParameters.update()
        self.initializeConnections()
        self.invalidate_rect( 0, 0, self.drawingAreaWidth, self.drawingAreaHeight )
        time.sleep(.01)
        self.controlToSrcConnections()
        time.sleep(.01)
        self.controlToFxConnections()
        time.sleep(.01)
        self.audioConnections()
        time.sleep(.01)

    def setAction( self, action ):
        self.action = action

    def doneAction( self ):
        if self.action == "drag-object": self.doneDragObject()
        self.action = None

    def handleButtonRelease( self, widget, event ):

        self.highlightWire( None )
        self.highlightGate( None )

        if self.action == "drag-object":
            self.doneAction()
        elif self.action == "draw-wire":
            for i in range(self.objectCount):
                if self.bounds[i][0] < event.x < self.bounds[i][2] and self.bounds[i][1] < event.y < self.bounds[i][3]:
                    if i == self.wireObj:
                        break
                    gate = self.testGates( i, event.x-self.locations[i][0], event.y-self.locations[i][1] )
                    if gate:
                        self.connectWire( i, gate )
                        break
            # if we don't connect the wire here they can try to click it somewhere, so don't end the action

    def handleButtonPress( self, widget, event):
        self.clickLoc = (int(event.x),int(event.y))

        self.highlightWire( None )
        self.highlightGate( None )

        if event.button == 1:
            for i in range(self.objectCount):
                if self.bounds[i][0] < event.x < self.bounds[i][2] and self.bounds[i][1] < event.y < self.bounds[i][3]:
                    gate = self.testGates( i, event.x-self.locations[i][0], event.y-self.locations[i][1] )
                    if gate:
                        if self.action == "draw-wire":
                            self.connectWire( i, gate )
                        else:
                            self.startWire( i, gate )
                    else:
                        if self.action == "draw-wire":
                            self.doneWire()
                        if i != self.objectCount-1:
                            self.startDragObject( i )
                    return
            if self.action == "draw-wire": # didn't hit anything
                self.doneWire()
            else:
                # check if we clicked a wire
                i = self.wireUnderLoc( event.x, event.y )
                if i >= 0: self.deleteWire( i )

        elif event.button == 3:
            for i in range(self.objectCount):
                if self.bounds[i][0] < event.x < self.bounds[i][2] and self.bounds[i][1] < event.y < self.bounds[i][3]:
                    if self.instanceOpen:
                        self.synthLabParametersWindow.destroy()
                    self.synthLabParametersWindow = SynthLabParametersWindow( i, self.synthObjectsParameters, self.writeTables, self.playNote )
                    self.instanceOpen = 1

    def handleMotion( self, widget, event ):

        if event.is_hint:
            x, y, state = widget.window.get_pointer()
            event.x = float(x)
            event.y = float(y)
            event.state = state

        if self.action == "drag-object":
            self.updateDragObject( int(event.x), int(event.y) )
        elif self.action == "draw-wire":
            self.updateWire( int(event.x), int(event.y) )
            for i in range(self.objectCount):
                if self.locations[i] == SynthLabConstants.INIT_LOCATIONS[i] \
                  and i != self.objectCount-1: continue
                if self.bounds[i][0] < event.x < self.bounds[i][2] and self.bounds[i][1] < event.y < self.bounds[i][3]:
                    gate = self.testGates( i, event.x-self.locations[i][0], event.y-self.locations[i][1] )
                    if gate and gate != self.wireGate:
                        if gate[0] == SynthLabConstants.GT_CONTROL_OUTPUT or gate[0] == SynthLabConstants.GT_SOUND_OUTPUT:
                            ok = self.testConnection( i, gate, self.wireObj, self.wireGate )
                        else:
                            ok = self.testConnection( self.wireObj, self.wireGate, i, gate )
                        self.highlightGate( i, gate, not ok )
                    else: self.highlightGate( None )
                    return
            self.highlightGate( None )
        else: # check for mouse overs
            for i in range(self.objectCount):
                if self.locations[i] == SynthLabConstants.INIT_LOCATIONS[i] \
                  and i != self.objectCount-1: continue

                if self.bounds[i][0] < event.x < self.bounds[i][2] and self.bounds[i][1] < event.y < self.bounds[i][3]:
                    gate = self.testGates( i, event.x-self.locations[i][0], event.y-self.locations[i][1] )
                    if gate:
                        self.highlightGate( i, gate )
                    else:
                        self.highlightGate( None )
                        if self.parameterOpen:
                            self.parameter.hide()
                            self.parameterOpen = 0
                    self.highlightWire( None )
                    return
            # didn't find a gate
            self.highlightGate( None )
            if self.parameterOpen:
                self.parameter.hide()
                self.parameterOpen = 0
            # check for wires
            i = self.wireUnderLoc( event.x, event.y )
            if i >= 0: self.highlightWire( i )
            else: self.highlightWire( None )

    def testGates( self, i, x, y ):
        oT = i >> 2
        for gT in range(len(self.gateMap[oT])):
            for n in range(len(self.gateMap[oT][gT])):
                if    self.gateMap[oT][gT][n][0] <= x <= self.gateMap[oT][gT][n][2] \
                  and self.gateMap[oT][gT][n][1] <= y <= self.gateMap[oT][gT][n][3]:
                    return ( gT, n, self.gateMap[oT][gT][n][4], self.gatePoint[oT][gT][n] ) # type, index, wire loc, center loc
        return False

    def startWire( self, obj, gate ):
        self.wireObj = obj
        self.wireGate = gate
        x = gate[2][0] + self.locations[obj][0]
        y = gate[2][1] + self.locations[obj][1]
        self.wirePoint = [ [ x, y ], [ x, y ] ]
        self.wireRect = [ 0, 0, 0, 0 ]
        self.setAction( "draw-wire" )

    def updateWire( self, x, y ):
        if x < 0: x = 0
        elif x > self.drawingAreaWidth: x = self.drawingAreaWidth
        if y < 0: y = 0
        elif y > self.separatorY: y = self.separatorY
        self.invalidate_rect( self.wireRect[0], self.wireRect[1], self.wireRect[2], self.wireRect[3], False )
        if x < self.wirePoint[0][0]: self.wireRect[0], self.wireRect[2] = x-self.lineWidth, self.wirePoint[0][0]-x+self.lineWidthMUL2
        else:                        self.wireRect[0], self.wireRect[2] = self.wirePoint[0][0]-self.lineWidth, x-self.wirePoint[0][0]+self.lineWidthMUL2
        if y < self.wirePoint[0][1]: self.wireRect[1], self.wireRect[3] = y-self.lineWidth, self.wirePoint[0][1]-y+self.lineWidthMUL2
        else:                        self.wireRect[1], self.wireRect[3] = self.wirePoint[0][1]-self.lineWidth, y-self.wirePoint[0][1]+self.lineWidthMUL2
        self.wirePoint[1][0] = x
        self.wirePoint[1][1] = y
        self.invalidate_rect( self.wireRect[0], self.wireRect[1], self.wireRect[2], self.wireRect[3], False )

    def connectWire( self, obj, gate ):
        if gate[0] == SynthLabConstants.GT_CONTROL_OUTPUT or gate[0] == SynthLabConstants.GT_SOUND_OUTPUT:
            bObj, eObj = obj, self.wireObj
            bGate, eGate = gate, self.wireGate
        else:
            bObj, eObj = self.wireObj, obj
            bGate, eGate = self.wireGate, gate

        i = self.newConnection( bObj, bGate, eObj, eGate )
        if i >= 0: # successful connection
            self.invalidate_rect( self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3] )
            self.doneWire()

    def deleteWire( self, i ):
        self.invalidate_rect( self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3] )
        self.delConnection( i )

    def doneWire( self ):
        self.invalidate_rect( self.wireRect[0], self.wireRect[1], self.wireRect[2], self.wireRect[3] )
        self.doneAction()

    def wireUnderLoc( self, x, y ):
        for i in range(len(self.connections)):
            if x < self.cBounds[i][0] or x > self.cBounds[i][4]: continue
            if y < self.cBounds[i][1] or y > self.cBounds[i][5]: continue
            if self.cPoints[i][0] == self.cPoints[i][2]: # vertical line
                if  abs(x-self.cPoints[i][0]) < self.lineWidthMUL4:
                    return i
            else:
                slope = (self.cPoints[i][3]-self.cPoints[i][1])/float(self.cPoints[i][2]-self.cPoints[i][0])
                if abs(slope) < 1:
                    yy = self.cPoints[i][1] + (x-self.cPoints[i][0])*slope
                    if abs(y-yy) < self.lineWidthMUL4:
                        return i
                else:
                    xx = self.cPoints[i][0] + (y-self.cPoints[i][1])/slope
                    if abs(x-xx) < self.lineWidthMUL4:
                        return i
        return -1 # nothing found

    # pass in i = None to clear
    def highlightWire( self, i ):
        if self.overWire != i:
            if self.overWire != None:
                self.invalidate_rect( self.cBounds[self.overWire][0], self.cBounds[self.overWire][1], self.cBounds[self.overWire][2], self.cBounds[self.overWire][3] )
            self.overWire = i
            if self.overWire != None:
                self.invalidate_rect( self.cBounds[self.overWire][0], self.cBounds[self.overWire][1], self.cBounds[self.overWire][2], self.cBounds[self.overWire][3] )

    # pass in obj = None to clear
    def highlightGate( self, obj, gate = None, reject = False ):
        if self.overGateObj != obj or self.overGate != gate or self.overGateReject != reject:
            if self.overGate != None:
                self.invalidate_rect( self.overGateLoc[0], self.overGateLoc[1], self.overGateSize, self.overGateSize )
            self.overGateObj = obj
            self.overGate = gate
            self.overGateReject = reject
            if self.overGate != None:
                oT = self.overGateObj//4
                x = self.locations[self.overGateObj][0] + self.overGate[3][0] - self.overGateSizeDIV2
                y = self.locations[self.overGateObj][1] + self.overGate[3][1] - self.overGateSizeDIV2
                self.overGateLoc = ( x, y )
                self.invalidate_rect( self.overGateLoc[0], self.overGateLoc[1], self.overGateSize, self.overGateSize )
                if obj != 12:
                    choosen = SynthLabConstants.CHOOSE_TYPE[obj/4][self.typesTable[obj]]
                    str = Tooltips.SYNTHTYPES[obj/4][self.typesTable[obj]] + ': ' + Tooltips.SYNTHPARA[choosen][gate[1]]
                    if gate[0] == 1:
                        if self.parameterOpen:
                            self.parameterUpdate( str )
                        else:
                            self.parameter = Parameter( str )
                            self.parameterOpen = 1

    def startDragObject( self, i ):
        self.dragObject = i
        self.dragInitialLoc = (self.locations[i][0],self.locations[i][1])
        self.potentialDisconnect = False
        self.invalidate_rect( self.bounds[i][0], self.bounds[i][1], SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE )
        for i in self.outputMap[self.dragObject]:
            self.invalidate_rect( self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3] )
        for i in self.inputMap[self.dragObject]:
            self.invalidate_rect( self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3] )
        self.setAction( "drag-object" )

    def updateDragObject( self, x, y ):
        delta = [ x-self.clickLoc[0], y-self.clickLoc[1] ]
        x = self.dragInitialLoc[0]+delta[0]
        if x-SynthLabConstants.HALF_SIZE < 0: x = SynthLabConstants.HALF_SIZE
        elif x+SynthLabConstants.HALF_SIZE > self.drawingAreaWidth: x = self.drawingAreaWidth - SynthLabConstants.HALF_SIZE
        y = self.dragInitialLoc[1]+delta[1]
        if y-SynthLabConstants.HALF_SIZE < 0: y = SynthLabConstants.HALF_SIZE
        elif y+SynthLabConstants.HALF_SIZE > self.drawingAreaHeight: y = self.drawingAreaHeight - SynthLabConstants.HALF_SIZE

        self.invalidate_rect(self.bounds[self.dragObject][0], self.bounds[self.dragObject][1], SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE, False )
        if not self.potentialDisconnect:
            for i in self.outputMap[self.dragObject]:
                self.invalidate_rect( self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3], False )
            for i in self.inputMap[self.dragObject]:
                self.invalidate_rect( self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3], False )

        if y > self.separatorY: self.potentialDisconnect = True
        else: self.potentialDisconnect = False

        self.locations[self.dragObject][0] = int( x )
        self.locations[self.dragObject][1] = int( y )
        self.updateBounds(self.dragObject)

        self.invalidate_rect(self.bounds[self.dragObject][0], self.bounds[self.dragObject][1], SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE, False )
        if not self.potentialDisconnect:
            for i in self.outputMap[self.dragObject]:
                self.invalidate_rect( self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3], False )
            for i in self.inputMap[self.dragObject]:
                self.invalidate_rect( self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3], False )

    def doneDragObject( self ):
        if self.potentialDisconnect:
            self.invalidate_rect( self.bounds[self.dragObject][0], self.bounds[self.dragObject][1], SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE, False )
            m = self.outputMap[self.dragObject][:]
            m.sort(reverse=True)
            for i in m: self.delConnection( i )
            m = self.inputMap[self.dragObject][:]
            m.sort(reverse=True)
            for i in m: self.delConnection( i )
            self.locations[self.dragObject][0] = SynthLabConstants.INIT_LOCATIONS[self.dragObject][0]
            self.locations[self.dragObject][1] = SynthLabConstants.INIT_LOCATIONS[self.dragObject][1]
            self.updateBounds( self.dragObject )
            self.invalidate_rect(self.bounds[self.dragObject][0], self.bounds[self.dragObject][1], SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE )
        else:
            self.invalidate_rect( self.bounds[self.dragObject][0], self.bounds[self.dragObject][1], SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE )
            for i in self.outputMap[self.dragObject]:
                self.invalidate_rect( self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3] )
            for i in self.inputMap[self.dragObject]:
                self.invalidate_rect( self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3] )

        self.dragObject = None
        self.handleSaveTemp()

    def updateBounds( self, i ):
        self.bounds[i][0] = self.locations[i][0]-SynthLabConstants.HALF_SIZE
        self.bounds[i][1] = self.locations[i][1]-SynthLabConstants.HALF_SIZE
        self.bounds[i][2] = self.locations[i][0]+SynthLabConstants.HALF_SIZE
        self.bounds[i][3] = self.locations[i][1]+SynthLabConstants.HALF_SIZE

        for c in self.outputMap[i]:
            self.updateConnection( c )
        for c in self.inputMap[i]:
            self.updateConnection( c )

    def updateConnection( self, i ):
        c = self.connections[i]
        oT = c[0][0]//4
        x1 = self.locations[c[0][0]][0] + self.gateMap[oT][c[0][1]][c[0][2]][4][0]
        y1 = self.locations[c[0][0]][1] + self.gateMap[oT][c[0][1]][c[0][2]][4][1]
        oT = c[1][0]//4
        x2 = self.locations[c[1][0]][0] + self.gateMap[oT][c[1][1]][c[1][2]][4][0]
        y2 = self.locations[c[1][0]][1] + self.gateMap[oT][c[1][1]][c[1][2]][4][1]
        self.cPoints[i][0], self.cPoints[i][1], self.cPoints[i][2], self.cPoints[i][3] = ( x1, y1, x2, y2 )
        if x1 > x2: x1, x2 = ( x2, x1 )
        if y1 > y2: y1, y2 = ( y2, y1 )
        self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3], self.cBounds[i][4], self.cBounds[i][5] = ( x1-self.lineWidth, y1-self.lineWidth, x2-x1+self.lineWidthMUL2, y2-y1+self.lineWidthMUL2, x2+self.lineWidth, y2+self.lineWidth )

    def findRecursive( self, obj, target ):
        if obj == target: return True
        for c in self.outputMap[obj]:
            if self.findRecursive( self.straightConnections[c][1], target ):
                return True
        return False

    def testConnection( self, bObj, bGate, eObj, eGate ):
        if self.gateMatch[bGate[0]] != eGate[0]:
            return False # type mismatch

        for c in self.inputMap[eObj]:
            if     self.connections[c][1][1] == eGate[0] \
               and self.connections[c][1][2] == eGate[1] : # same type and port
                if self.connections[c][0][0] == bObj:
                    return False # connections already exists

        if self.findRecursive( eObj, bObj ):
            return False # loop

        return True

    def newConnection( self, bObj, bGate, eObj, eGate ):
        if not self.testConnection( bObj, bGate, eObj, eGate ):
            return -1 # connection failed

        ind = len(self.connections)
        # connection format: [ ( outputObject, gate type, gate num ), ( inputObject, gate type, gate num ) ]
        self.connections.append ( [ ( bObj, bGate[0], bGate[1] ),
                                    ( eObj, eGate[0], eGate[1] ) ] )
        self.straightConnections.append( ( bObj, eObj ) )
        self.outputMap[bObj].append(ind)
        self.inputMap[eObj].append(ind)
        self.outputs.append( bObj )
        self.cPoints.append( [ 0, 0, 0, 0 ] )
        self.cBounds.append( [ 0, 0, 0, 0, 0, 0 ] )
        self.updateConnection( ind )

        self.updateSound()

        self.handleSaveTemp()

        return ind

    def delConnection( self, i ):
        b = self.straightConnections[i][0]
        e = self.straightConnections[i][1]
        self.straightConnections.pop(i)
        self.outputMap[b].remove(i)
        self.inputMap[e].remove(i)
        self.outputs.pop(i)
        self.cPoints.pop(i)
        self.cBounds.pop(i)
        self.connections.pop(i)
        for o in range(self.objectCount):
            for m in range(len(self.outputMap[o])):
                if self.outputMap[o][m] > i: self.outputMap[o][m] -= 1
            for m in range(len(self.inputMap[o])):
                if self.inputMap[o][m] > i: self.inputMap[o][m] -= 1

        self.updateSound()

        self.handleSaveTemp()

    def initializeConnections( self ):
        self.straightConnections = []
        self.outputMap = [ [] for i in self.locations ]
        self.inputMap = [ [] for i in self.locations ]
        self.outputs = []
        self.cPoints = []
        self.cBounds = []
        for i in range(len(self.connections)):
            c = self.connections[i]
            first = c[0][0]
            second = c[1][0]
            self.straightConnections.append([first, second])
            self.outputMap[first].append(i)
            self.inputMap[second].append(i)
            self.outputs.append(first)
            self.cPoints.append( [ 0, 0, 0, 0 ] )
            self.cBounds.append( [ 0, 0, 0, 0, 0, 0 ] )
            self.updateConnection( i )

        self.updateSound()

    def predraw( self, buf ):
        startX = self.screenBufDirtyRect.x
        startY = self.screenBufDirtyRect.y
        stopX = self.screenBufDirtyRect.x + self.screenBufDirtyRect.width
        stopY = self.screenBufDirtyRect.y + self.screenBufDirtyRect.height

        # draw bg
        self.gc.foreground = self.bgColor
        buf.draw_rectangle( self.gc, True, startX, startY, self.screenBufDirtyRect.width, self.screenBufDirtyRect.height )

        # draw separator
        self.gc.foreground = self.lineColor
        buf.draw_line( self.gc, startX, self.separatorY, stopX, self.separatorY )

        # draw objects
        self.gc.set_clip_mask( self.clipMask )
        for i in range(self.objectCount):
            if i == self.dragObject:
                continue
            if startX > self.bounds[i][2] or stopX < self.bounds[i][0] or startY > self.bounds[i][3] or stopY < self.bounds[i][1]:
                continue
            type = i >> 2
            self.gc.set_clip_origin( self.bounds[i][0]-SynthLabConstants.PIC_SIZE*type, self.bounds[i][1] )
            buf.draw_drawable( self.gc, self.pixmap[i], 0, 0, self.bounds[i][0], self.bounds[i][1], SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE )
        self.gc.set_clip_rectangle( self.clearMask )

        # draw wires
        for c in range(len(self.connections)):
            if self.straightConnections[c][0] == self.dragObject or self.straightConnections[c][1] == self.dragObject:
                continue
            if startX > self.cBounds[c][4] or stopX < self.cBounds[c][0] or startY > self.cBounds[c][5] or stopY < self.cBounds[c][1]:
                continue
            buf.draw_line( self.gc, self.cPoints[c][0], self.cPoints[c][1],
                                    self.cPoints[c][2], self.cPoints[c][3] )

        self.screenBufDirty = False

    def draw( self, widget, event ):
        #TP.ProfileBegin("SL::draw")
        startX = event.area.x
        startY = event.area.y
        stopX = event.area.x + event.area.width
        stopY = event.area.y + event.area.height

        self.gc.foreground = self.lineColor

        if self.screenBufDirty:
            self.predraw( self.screenBuf )

        # draw base
        widget.window.draw_drawable( self.gc, self.screenBuf, startX, startY, startX, startY, event.area.width, event.area.height )

        if self.action == "drag-object":
            # draw dragObject
            self.gc.set_clip_mask( self.clipMask )
            type = self.dragObject >> 2
            self.gc.set_clip_origin( self.bounds[self.dragObject][0]-SynthLabConstants.PIC_SIZE*type, self.bounds[self.dragObject][1] )
            widget.window.draw_drawable( self.gc, self.pixmap[self.dragObject], 0, 0, self.bounds[self.dragObject][0], self.bounds[self.dragObject][1], SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE )
            self.gc.set_clip_rectangle( self.clearMask )

            # draw wires
            if not self.potentialDisconnect:
                for c in self.outputMap[self.dragObject]:
                    if startX > self.cBounds[c][4] or stopX < self.cBounds[c][0] or startY > self.cBounds[c][5] or stopY < self.cBounds[c][1]:
                        continue
                    widget.window.draw_line( self.gc, self.cPoints[c][0], self.cPoints[c][1],
                                             self.cPoints[c][2], self.cPoints[c][3] )
                for c in self.inputMap[self.dragObject]:
                    if startX > self.cBounds[c][4] or stopX < self.cBounds[c][0] or startY > self.cBounds[c][5] or stopY < self.cBounds[c][1]:
                        continue
                    widget.window.draw_line( self.gc, self.cPoints[c][0], self.cPoints[c][1],
                                             self.cPoints[c][2], self.cPoints[c][3] )
        elif self.action == "draw-wire":
            # draw the wire
            widget.window.draw_line( self.gc, self.wirePoint[0][0], self.wirePoint[0][1],
                                              self.wirePoint[1][0], self.wirePoint[1][1] )

        # draw highlights
        if self.overWire != None:
            self.gc.foreground = self.overWireColor
            widget.window.draw_line( self.gc, self.cPoints[self.overWire][0], self.cPoints[self.overWire][1],
                                              self.cPoints[self.overWire][2], self.cPoints[self.overWire][3] )
        elif self.overGate != None:
            self.gc.set_line_attributes( self.overLineWidth, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_BUTT, gtk.gdk.JOIN_MITER )
            if self.overGateReject:
                self.gc.foreground = self.overGateRejectColor
                widget.window.draw_line( self.gc, self.overGateLoc[0]+self.overLineWidth, self.overGateLoc[1]+self.overLineWidth, self.overGateLoc[0]+self.overGateSize-self.overLineWidth, self.overGateLoc[1]+self.overGateSize-self.overLineWidth )
                widget.window.draw_line( self.gc, self.overGateLoc[0]+self.overLineWidth, self.overGateLoc[1]+self.overGateSize-self.overLineWidth, self.overGateLoc[0]+self.overGateSize-self.overLineWidth, self.overGateLoc[1]+self.overLineWidth )
            else:
                self.gc.foreground = self.overGateColor
                widget.window.draw_arc( self.gc, False, self.overGateLoc[0]+self.overLineWidth, self.overGateLoc[1]+self.overLineWidth, self.overGateSize-self.overLineWidthMUL2, self.overGateSize-self.overLineWidthMUL2, 0, 23040 )
            self.gc.set_line_attributes( self.lineWidth, gtk.gdk.LINE_SOLID, gtk.gdk.CAP_BUTT, gtk.gdk.JOIN_MITER )

        #print TP.ProfileEndAndPrint("SL::draw")
        return True

    def invalidate_rect( self, x, y, w, h, dirtyScreenBuf = True ):
        self.dirtyRectToAdd.x = x
        self.dirtyRectToAdd.y = y
        self.dirtyRectToAdd.width = w
        self.dirtyRectToAdd.height = h

        if dirtyScreenBuf:
            if self.screenBufDirty:
                self.screenBufDirtyRect = self.screenBufDirtyRect.union(self.dirtyRectToAdd)
            else:
                self.screenBufDirtyRect.x = x
                self.screenBufDirtyRect.y = y
                self.screenBufDirtyRect.width = w
                self.screenBufDirtyRect.height = h
                self.screenBufDirty = True

        if self.drawingArea.window != None:
            self.drawingArea.window.invalidate_rect( self.dirtyRectToAdd, True )

        self.dirty = True

    def writeTables( self, typesTable, controlParametersTable, sourceParametersTable, fxParametersTable ):
        mess = 'f5200 0 16 -2 ' + " ".join([str(n) for n in controlParametersTable])
        self.csnd.inputMessage( mess )
        time.sleep(0.005)
        mess = "f5201 0 16 -2 " + " "  .join([str(n) for n in sourceParametersTable])
        self.csnd.inputMessage( mess )
        time.sleep(.005)
        mess = "f5202 0 16 -2 " + " "  .join([str(n) for n in fxParametersTable])
        self.csnd.inputMessage( mess )
        time.sleep(.005)
        self.typesTable = typesTable
        lastTable = [0]*12
        for i in range(12):
            if i in self.outputs:
                lastTable[i] = (typesTable[i]+1)
        mess = "f5203 0 16 -2 " + " "  .join([str(n) for n in lastTable]) + " 0 0 0 0"
        self.csnd.inputMessage( mess )
        time.sleep(.005)
        if lastTable[4] == 8:
            snd = Config.SOUNDS_DIR + '/' + self.sample_names[int(sourceParametersTable[1])]
            mess = "f5501 0 32768 -1 " + "\"%s\" 0 0 0" % snd
            self.csnd.inputMessage( mess )
        if lastTable[5] == 8:
            snd = Config.SOUNDS_DIR + '/' + self.sample_names[int(sourceParametersTable[5])]
            mess = "f5502 0 32768 -1 " + "\"%s\" 0 0 0" % snd
            self.csnd.inputMessage( mess )
        if lastTable[6] == 8:
            snd = Config.SOUNDS_DIR + '/' + self.sample_names[int(sourceParametersTable[9])]
            mess = "f5503 0 32768 -1 " + "\"%s\" 0 0 0" % snd
            self.csnd.inputMessage( mess )
        if lastTable[7] == 8:
            snd = Config.SOUNDS_DIR + '/' + self.sample_names[int(sourceParametersTable[13])]
            mess = "f5504 0 32768 -1 " + "\"%s\" 0 0 0" % snd
            self.csnd.inputMessage( mess )
        time.sleep(.005)
        self.loadPixmaps(typesTable)
        self.invalidate_rect( 0, 0, self.drawingAreaWidth, self.drawingAreaHeight )

    def recordSound( self, widget, data ):
        if widget.get_active() == True:
            self.recordButton = widget
            self.recordWait = 1
            os.system('rm ' + Config.SNDS_DIR + '/lab' + str(data))
            self.table = 85 + data
        else:
            self.recordWait = 0

    def updateSound( self ):
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
        mess = "f5203 0 16 -2 " + " "  .join([str(n) for n in lastTable]) + " 0 0 0 0"
        self.csnd.inputMessage( mess )
        time.sleep(.01)

    def updateTables( self ):
        self.writeTables( self.synthObjectsParameters.types, self.synthObjectsParameters.controlsParameters, self.synthObjectsParameters.sourcesParameters, self.synthObjectsParameters.fxsParameters )

    def controlToSrcConnections( self ):
        self.contSrcConnections = []
        for i in self.connections:
            if i[0][0] < 4 and 3 < i[1][0] < 8:
                offset = i[1][2]
                self.contSrcConnections.append([i[0][0], i[1][0], offset])
        table = [0 for i in range(16)]
        sources = [source for source in range(4,8) if source in self.outputs]
        for source in sources:
            for entre in range(4):
                value = sum([2**(li[0]+1) for li in self.contSrcConnections if li[1] == source and li[2] == entre], 1)
                table[(source % 4) * 4 + entre] = value
        mess = "f5204 0 16 -2 " + " "  .join([str(n) for n in table])
        self.csnd.inputMessage( mess )

    def controlToFxConnections( self ):
        self.contFxConnections = []
        for i in self.connections:
            if i[0][0] < 4 and 7 < i[1][0] < 12:
                offset = i[1][2]
                self.contFxConnections.append([i[0][0], i[1][0], offset])
        table = [0 for i in range(16)]
        fxs = [fx for fx in range(8,12) if fx in self.outputs]
        for fx in fxs:
            for entre in range(4):
                value = sum([2**(li[0]+1) for li in self.contFxConnections if li[1] == fx and li[2] == entre], 1)
                table[(fx % 4) * 4 + entre] = value
        mess = "f5205 0 16 -2 " + " "  .join([str(n) for n in table])
        self.csnd.inputMessage( mess )

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
        mess = "f5206 0 16 -2 " + " "  .join([str(n) for n in table])
        self.csnd.inputMessage( mess )

    def loadPixmaps( self, typesList ):
        win = gtk.gdk.get_default_root_window()
        gc = gtk.gdk.GC( win )
        gc.foreground = self.bgColor
        self.pixmap = []
        for i in range(13):
            if i < 4:    img = SynthLabConstants.CHOOSE_TYPE_PLUS[0][typesList[i]]
            elif i < 8:  img = SynthLabConstants.CHOOSE_TYPE_PLUS[1][typesList[i]]
            elif i < 12: img = SynthLabConstants.CHOOSE_TYPE_PLUS[2][typesList[i]]
            else:        img = "speaker"
            pix = gtk.gdk.pixbuf_new_from_file(Config.IMAGE_ROOT + img + '.png')
            map = gtk.gdk.Pixmap( win, pix.get_width(), pix.get_height() )
            map.draw_rectangle( gc, True, 0, 0, pix.get_width(), pix.get_height() )
            map.draw_pixbuf( gc, pix, 0, 0, 0, 0, pix.get_width(), pix.get_height(), gtk.gdk.RGB_DITHER_NONE )
            self.pixmap.append(map)
        pix = gtk.gdk.pixbuf_new_from_file(Config.IMAGE_ROOT+'synthlabMask.png')
        pixels = pix.get_pixels()
        stride = pix.get_rowstride()
        channels = pix.get_n_channels()
        bitmap = ""
        byte = 0
        shift = 0
        for j in range(pix.get_height()):
            offset = stride*j
            for i in range(pix.get_width()):
                r = pixels[i*channels+offset]
                if r != "\0": byte += 1 << shift
                shift += 1
                if shift > 7:
                    bitmap += "%c" % byte
                    byte = 0
                    shift = 0
        bitmap += "%c" % byte
        self.clipMask = gtk.gdk.bitmap_create_from_data( None, bitmap, pix.get_width(), pix.get_height() )

    def handleSave(self, widget, data):
        chooser = gtk.FileChooserDialog(title='Save SynthLab Preset',action=gtk.FILE_CHOOSER_ACTION_SAVE, buttons=(gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_SAVE,gtk.RESPONSE_OK))
        filter = gtk.FileFilter()
        filter.add_pattern('*.syn')
        chooser.set_filter(filter)
        chooser.set_current_folder(Config.SYNTH_DIR)

        for f in chooser.list_shortcut_folder_uris():
            chooser.remove_shortcut_folder_uri(f)

        if chooser.run() == gtk.RESPONSE_OK:
            ofilename = chooser.get_filename()
            if ofilename[-4:] != '.syn':
                ofilename += '.syn'
            try:
                print 'INFO: save SynthLab file %s' % chooser.get_filename()
                f = shelve.open(ofilename, 'n')
                self.saveState(f)
                f.close()
            except IOError:
                print 'ERROR: failed to save SynthLab to file %s' % chooser.get_filename()

        chooser.destroy()

    def handleJournalSave(self, file_path):
        f = shelve.open(file_path, 'n')
        self.saveState(f)
        f.close()

    def handleLoad(self, widget, data):
        chooser = gtk.FileChooserDialog(title='Load SynthLab Preset',action=gtk.FILE_CHOOSER_ACTION_OPEN, buttons=(gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_OPEN,gtk.RESPONSE_OK))

        filter = gtk.FileFilter()
        filter.add_pattern('*.syn')
        chooser.set_filter(filter)
        chooser.set_current_folder(Config.SYNTH_DIR)

        for f in chooser.list_shortcut_folder_uris():
            chooser.remove_shortcut_folder_uri(f)

        if chooser.run() == gtk.RESPONSE_OK:
            try:
                print 'INFO: load SynthLab state from file %s' % chooser.get_filename()
                f = shelve.open( chooser.get_filename(), 'r')
                self.loadState(f)
                f.close()
            except IOError:
                print 'ERROR: failed to load SynthLab state from file %s' % chooser.get_filename()

        chooser.destroy()

    def handleJournalLoad(self, file_path):
        f = shelve.open( file_path, 'r')
        self.loadState(f)
        f.close()

    def handleSaveTemp( self ):
        file = Config.PREF_DIR + '/synthTemp'
        f = shelve.open(file, 'n')
        self.saveState(f)
        f.close()

    def handleLoadTemp( self ):
        file = Config.PREF_DIR + '/synthTemp'
        f = shelve.open(file, 'r')
        self.loadState(f)
        f.close()

    def saveState( self, state ):
        state['types'] = self.synthObjectsParameters.types
        state['controls'] = self.synthObjectsParameters.controlsParameters
        state['sources'] = self.synthObjectsParameters.sourcesParameters
        state['fxs'] = self.synthObjectsParameters.fxsParameters
        state['envelope'] = self.synthObjectsParameters.outputParameters
        state['locations'] = self.locations
        state['connections'] = self.connections
        state['duration'] = self.duration

    def tempVerifyLocations(self):
        for l in self.locations:
            l[0] = int(l[0])
            l[1] = int(l[1])

    def tempVerifyConnectionFormat(self):
        for c in self.connections:
            if    c[0][1] > 3 or c[0][2] > 3 \
               or c[1][1] > 3 or c[1][2] > 3:
                print "old format"
                print c
                i = c[0]
                if i[1] == 0 and i[2] == 40:
                    if i[0] < 4: t,n = 0,0 # control output
                    else: t,n = 2,0        # sound output
                else:
                    print "unhandled loc"
                    t,n = i[1],i[2]
                c[0] = ( c[0][0], t, n )
                i = c[1]
                if i[1] == 0 and i[2] == -40: t,n = 3,0
                elif i[1] == 40 and i[2] == -19: t,n = 1,0
                elif i[1] == -25 and i[2] == -40: t,n = 1,0
                elif i[1] == -9 and i[2] == -40: t,n = 1,1
                elif i[1] == 8 and i[2] == -40: t,n = 1,2
                elif i[1] == 25 and i[2] == -40: t,n = 1,3
                else:
                    print "unhandled loc"
                    t,n = i[1],i[2]
                c[1] = ( c[1][0], t, n )

    def loadState( self, state ):
        self.synthObjectsParameters.types = state['types']
        self.synthObjectsParameters.controlsParameters = state['controls']
        self.synthObjectsParameters.sourcesParameters = state['sources']
        self.synthObjectsParameters.fxsParameters = state['fxs']
        self.synthObjectsParameters.outputParameters = state['envelope']
        self.locations = state['locations']
        #self.tempVerifyLocations()
        self.objectCount = len(self.locations)
        for i in range(self.objectCount):
            self.updateBounds( i )
        self.connections = state['connections']
        #self.tempVerifyConnectionFormat()
        self.duration = state['duration']
        self._mainToolbar.durationSliderAdj.set_value(self.duration)

        self.initializeConnections()
        self.controlToSrcConnections()
        time.sleep(.01)
        self.controlToFxConnections()
        time.sleep(.01)
        self.audioConnections()
        time.sleep(.01)
        self.synthObjectsParameters.update()
        self.writeTables( self.synthObjectsParameters.types, self.synthObjectsParameters.controlsParameters, self.synthObjectsParameters.sourcesParameters, self.synthObjectsParameters.fxsParameters )
        time.sleep(.01)
        self.invalidate_rect( 0, 0, self.drawingAreaWidth, self.drawingAreaHeight )

    def presetCallback( self, widget, data ):
        preset = 'synthFile' + str(data)
        f = shelve.open( Config.TAM_TAM_ROOT + '/Resources/SynthFiles/' + preset, 'r')
        self.loadState(f)
        f.close()
        self.handleSaveTemp()

    def initRadioButton( self, labelList, methodCallback, box ):
        for i in range( len( labelList ) ):
            label = labelList[i]
            if i == 0:
                button = ImageRadioButton( group = None, mainImg_path = Config.IMAGE_ROOT + label + '.png', altImg_path = Config.IMAGE_ROOT + label + 'sel.png' )
            else:
                button = ImageRadioButton( group = button, mainImg_path = Config.IMAGE_ROOT + label + '.png', altImg_path = Config.IMAGE_ROOT + label + 'sel.png' )
            button.connect( "toggled", methodCallback, i )
            box.pack_start( button, True, True )
