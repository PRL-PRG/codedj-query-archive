import pygtk
pygtk.require('2.0')
import gtk

from common.Util.Profiler import TP

import gobject
import time
import shelve
from gettext import gettext as _
import os
import commands

from sugar.graphics.toolcombobox import ToolComboBox
from common.Util.ThemeWidgets import BigComboBox

import common.Util.Instruments
import common.Util.InstrumentDB as InstrumentDB
import common.Config as Config
from common.Util.ThemeWidgets import *
from common.Util.CSoundClient import new_csound_client
from SynthLab.SynthObjectsParameters import SynthObjectsParameters
from SynthLab.SynthLabConstants import SynthLabConstants
from SynthLab.SynthLabToolbars import mainToolbar
from SynthLab.SynthLabToolbars import presetToolbar
from common.Util.Trackpad import Trackpad

as_window = False

class SynthLabMain(gtk.EventBox):
    def __init__( self, activity ):
        gtk.EventBox.__init__(self)
        if as_window:
            color = gtk.gdk.color_parse(Config.PANEL_BCK_COLOR)
            self.modify_bg(gtk.STATE_NORMAL, color)
            self.set_border_width(Config.MAIN_WINDOW_PADDING)
            self.set_keep_above(False)
            self.set_decorated(False)
        self.activity = activity
        self.instrumentDB = InstrumentDB.getRef()
        self.csnd = new_csound_client()
        self.csnd.setMasterVolume( 100.0 ) # csnd expects a range 0-100 for now
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
        self.new = True
        self.selectGate = False
        self.sliderGate = True
        self.duration = 2
        self.viewType = ''
        self.viewParam = ''
        self.curSlider = 1
        self.durString = '%.2f' % self.duration
        self.playingPitch = []
        self.journalCalled = True

        #Toolbars
        self.activity.activity_toolbar.keep.show()
        self._mainToolbar = mainToolbar(self.activity.toolbox, self)
        self._presetToolbar = presetToolbar(self.activity.toolbox, self)
        self.activity.toolbox.add_toolbar(_('Main'), self._mainToolbar)
        self.activity.toolbox.add_toolbar(_('Presets'), self._presetToolbar)
        self.activity.toolbox.set_current_toolbar(1)
        self._mainToolbar.show()
        self._presetToolbar.show()

        loopPointsTable = []
        self.sample_names = [name for i in range( len( self.instrumentDB.instNamed ) ) for name in self.instrumentDB.instNamed.keys() if self.instrumentDB.instNamed[ name ].instrumentId == i ]
        for inst in self.sample_names:
            loopStart = self.instrumentDB.instNamed[ inst ].loopStart
            loopEnd = self.instrumentDB.instNamed[ inst ].loopEnd
            crossDur = self.instrumentDB.instNamed[ inst ].crossDur
            loopPointsTable.extend( [ loopStart, loopEnd, crossDur ] )
        mess = "f5755 0 512 -2 " + " "  .join([str(n) for n in loopPointsTable])
        self.csnd.inputMessage( mess )

        self.lineWidth = 3
        self.lineWidthMUL2 = self.lineWidth*2
        self.lineWidthMUL4 = self.lineWidth*4
        self.lineWidthMUL4SQ = self.lineWidthMUL4*self.lineWidthMUL4
        self.clockStart = 0
        #self.sample_names = [name for i in range( len( Config.INSTRUMENTS ) ) for name in Config.INSTRUMENTS.keys() if Config.INSTRUMENTS[ name ].instrumentId == i ]
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
        self.mainBox = gtk.HBox()
        self.subBox = gtk.HBox()
        self.drawingBox = RoundVBox( 10, Config.PANEL_COLOR, Config.PANEL_COLOR )
        self.drawingBox.set_border_width(0)
        self.infoBox = RoundVBox( 10, Config.TOOLBAR_BCK_COLOR, Config.TOOLBAR_BCK_COLOR )
        self.infoBox.set_border_width(Config.PANEL_SPACING)
        self.infoBox.set_size_request(300, 750)
        self.subBox.pack_start(self.drawingBox, True, True)
        self.subBox.pack_start(self.infoBox, True, True)
        self.mainBox.pack_start(self.subBox)

        menuBox = gtk.HBox()
        self.objComboBox = BigComboBox()
        self.objComboBox.append_item(0, 'Envelope', Config.TAM_TAM_ROOT + '/icons/sl-adsr-menu.svg')
        self.objComboBox.set_active(0)
        self.objComboBox.connect('changed', self.changeObject)
        comboMenu = ToolComboBox(self.objComboBox)
        menuBox.pack_start(comboMenu)
        self.infoBox.pack_start(menuBox, False, False, 5)

        slidersBox = gtk.HBox()

        self.instanceID = 12                    # object number
        self.objectType = self.instanceID / 4   #(control, source, fx, output)
        self.choosenType = 0                    # self.synthObjectsParameters.types[self.instanceID] module as an index
        selectedType = SynthLabConstants.CHOOSE_TYPE[self.objectType][self.choosenType] #module as a string

        slider1Min = SynthLabConstants.TYPES[selectedType][4]
        slider1Max = SynthLabConstants.TYPES[selectedType][5]
        slider2Min = SynthLabConstants.TYPES[selectedType][6]
        if selectedType == 'sample' or selectedType == 'grain':
            slider2Max = len(self.sample_names)
        else:
            slider2Max = SynthLabConstants.TYPES[selectedType][7]
        slider3Min = SynthLabConstants.TYPES[selectedType][8]
        slider3Max = SynthLabConstants.TYPES[selectedType][9]
        slider4Min = SynthLabConstants.TYPES[selectedType][10]
        slider4Max = SynthLabConstants.TYPES[selectedType][11]

        slider1Step = SynthLabConstants.TYPES[selectedType][12][0]
        slider1Snap = SynthLabConstants.TYPES[selectedType][12][1]
        slider2Step = SynthLabConstants.TYPES[selectedType][13][0]
        slider2Snap = SynthLabConstants.TYPES[selectedType][13][1]
        slider3Step = SynthLabConstants.TYPES[selectedType][14][0]
        slider3Snap = SynthLabConstants.TYPES[selectedType][14][1]

        parametersTable = self.synthObjectsParameters.choiceParamsSet[self.objectType]
        tablePos = (self.instanceID % 4)*4
        slider1Init = parametersTable[tablePos]
        slider2Init = parametersTable[tablePos+1]
        slider3Init = parametersTable[tablePos+2]
        slider4Init = parametersTable[tablePos+3]

        sliderTextColor = gtk.gdk.color_parse(Config.WHITE_COLOR)
        sliderHeight = 240

        self.p1Adjust = gtk.Adjustment(slider1Init, slider1Min, slider1Max, slider1Step, slider1Step, 0)
        self.p1Adjust.connect("value-changed", self.sendTables, 1)
        self.slider1 = gtk.VScale(self.p1Adjust)
        self.slider1.connect("button-release-event", self.handleSliderRelease)
        self.slider1.connect("value-changed", self.handleSliderRelease)
        self.slider1.connect("enter-notify-event", self.handleSliderEnter, 1)
        self.slider1.set_digits(slider1Snap)
        self.slider1.set_inverted(True)
        self.slider1.set_size_request(55, sliderHeight)
        self.slider1.modify_fg(gtk.STATE_NORMAL, sliderTextColor)
        slidersBox.pack_start(self.slider1, True, False)

        self.p2Adjust = gtk.Adjustment(slider2Init, slider2Min, slider2Max, slider2Step, slider2Step, 0)
        self.p2Adjust.connect("value-changed", self.sendTables, 2)
        self.slider2 = gtk.VScale(self.p2Adjust)
        self.slider2.connect("button-release-event", self.handleSliderRelease)
        self.slider2.connect("value-changed", self.handleSliderRelease)
        self.slider2.connect("enter-notify-event", self.handleSliderEnter, 2)
        self.slider2.set_digits(slider2Snap)
        self.slider2.set_inverted(True)
        self.slider2.set_size_request(55, sliderHeight)
        self.slider2.modify_fg(gtk.STATE_NORMAL, sliderTextColor)
        slidersBox.pack_start(self.slider2, True, False)

        self.p3Adjust = gtk.Adjustment(slider3Init, slider3Min, slider3Max, slider3Step, slider3Step, 0)
        self.p3Adjust.connect("value-changed", self.sendTables, 3)
        self.slider3 = gtk.VScale(self.p3Adjust)
        self.slider3.connect("button-release-event", self.handleSliderRelease)
        self.slider3.connect("value-changed", self.handleSliderRelease)
        self.slider3.connect("enter-notify-event", self.handleSliderEnter, 3)
        self.slider3.set_digits(slider3Snap)
        self.slider3.set_inverted(True)
        self.slider3.set_size_request(55, sliderHeight)
        self.slider3.modify_fg(gtk.STATE_NORMAL, sliderTextColor)
        slidersBox.pack_start(self.slider3, True, False)

        self.p4Adjust = gtk.Adjustment(slider4Init, slider4Min, slider4Max, .01, .01, 0)
        self.p4Adjust.connect("value-changed", self.sendTables, 4)
        self.slider4 = gtk.VScale(self.p4Adjust)
        self.slider4.connect("button-release-event", self.handleSliderRelease)
        self.slider4.connect("value-changed", self.handleSliderRelease)
        self.slider4.connect("enter-notify-event", self.handleSliderEnter, 4)
        self.slider4.set_digits(2)
        self.slider4.set_inverted(True)
        self.slider4.set_size_request(55, sliderHeight)
        self.slider4.modify_fg(gtk.STATE_NORMAL, sliderTextColor)
        slidersBox.pack_start(self.slider4, True, False)

        self.infoBox.pack_start(slidersBox, False, False, 5)

        self.infoText = 'ADSR envelope apply on the overall signal'
        textBox = gtk.HBox()
        text_color = gtk.gdk.color_parse(Config.WHITE_COLOR)
        text_bg_color = gtk.gdk.color_parse(Config.TOOLBAR_BCK_COLOR)
        textScroller = gtk.ScrolledWindow()
        textScroller.set_policy(gtk.POLICY_NEVER, gtk.POLICY_AUTOMATIC)
        textScroller.set_size_request(270, 301)
        self.textBuf = gtk.TextBuffer(None)
        self.textBuf.set_text(self.infoText)
        self.textViewer = gtk.TextView(self.textBuf)
        self.textViewer.modify_text(gtk.STATE_NORMAL, text_color)
        self.textViewer.modify_base(gtk.STATE_NORMAL, text_bg_color)
        self.textViewer.set_border_window_size(gtk.TEXT_WINDOW_LEFT, 1)
        self.textViewer.set_border_window_size(gtk.TEXT_WINDOW_RIGHT, 1)
        self.textViewer.set_border_window_size(gtk.TEXT_WINDOW_TOP, 1)
        self.textViewer.set_border_window_size(gtk.TEXT_WINDOW_BOTTOM, 1)
        self.textViewer.set_wrap_mode(gtk.WRAP_WORD)
        self.textViewer.set_editable(False)
        self.textViewer.set_overwrite(True)
        self.textViewer.set_cursor_visible(False)
        self.textViewer.set_left_margin(10)
        self.textViewer.set_right_margin(10)
        self.textViewer.set_pixels_above_lines(7)
        self.textViewer.set_justification(gtk.JUSTIFY_LEFT)
        textScroller.add(self.textViewer)
        textBox.pack_start(textScroller, False, False, 10)
        self.infoBox.pack_start(textBox, False, False, 5)

        self.infoLabel = gtk.Label()
        self.infoBox.pack_end(self.infoLabel, False, False, 20)
        textColor = gtk.gdk.color_parse(Config.WHITE_COLOR)
        self.infoLabel.set_justify(gtk.JUSTIFY_LEFT)
        self.infoLabel.modify_fg(gtk.STATE_NORMAL, textColor)

        self.drawingAreaWidth = 900
        self.drawingAreaHeight = 750
        self.separatorY = 660

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
        self.col = gtk.gdk.color_parse(Config.PANEL_COLOR)
        colormap = self.drawingArea.get_colormap()
        self.bgColor = colormap.alloc_color( Config.PANEL_COLOR, True, True )
        self.lineColor = colormap.alloc_color( Config.SL_LINE_COLOR, True, True )
        self.highlightColor = colormap.alloc_color( Config.SL_HIGHLIGHT_COLOR, True, True )
        self.overWireColor = colormap.alloc_color( Config.SL_OVER_WIRE_COLOR, True, True )
        self.overGateColor = colormap.alloc_color( Config.SL_OVER_GATE_COLOR, True, True )
        self.overGateRejectColor = colormap.alloc_color( Config.SL_OVER_GATE_REJECT_COLOR, True, True )
        self.drawingArea.modify_bg(gtk.STATE_NORMAL, self.col)

        self.loadPixmaps()

        self.drawingArea.add_events( gtk.gdk.BUTTON_PRESS_MASK
                                   | gtk.gdk.BUTTON_RELEASE_MASK
                                   | gtk.gdk.POINTER_MOTION_MASK
                                   | gtk.gdk.POINTER_MOTION_HINT_MASK )
        self.drawingArea.connect( "button-press-event", self.handleButtonPress )
        self.drawingArea.connect( "button-release-event", self.handleButtonRelease )
        self.drawingArea.connect( "motion-notify-event", self.handleMotion )
        self.drawingArea.connect("expose-event", self.draw)
        self.drawingBox.pack_start(self.drawingArea, False, False, 0)

        tempFile = 'synthTemp'
        if tempFile in os.listdir(Config.PREF_DIR):
            self.handleLoadTemp()
        else:
            self.presetCallback(None,1)
        self.add(self.mainBox)
        self.show_all()

    def onActivate( self, arg ):
        pass
    def onDeactivate( self ):
        pass

    def onDestroy(self):
        pass

    def select(self, i):
        self.sliderGate = False
        if i == self.instanceID:
            return
        self.new = False
        if self.instanceID > 0:
            self.invalidate_rect( self.bounds[self.instanceID][0], self.bounds[self.instanceID][1]-2, SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE_HIGHLIGHT )
        self.instanceID = i
        self.invalidate_rect( self.bounds[i][0], self.bounds[i][1]-2, SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE_HIGHLIGHT )

        if self.instanceID / 4 != self.objectType:
            self.objectType = self.instanceID / 4
            self.objComboBox.remove_all()
            for i in range(len(SynthLabConstants.CHOOSE_TYPE[self.objectType])):
                self.objComboBox.append_item(i, SynthLabConstants.SYNTHTYPES[self.objectType][i], Config.TAM_TAM_ROOT + '/icons/sl-' + SynthLabConstants.CHOOSE_TYPE[self.objectType][i] + '-menu.svg')

        if self.instanceID != 12:
            self.choosenType = self.synthObjectsParameters.types[self.instanceID]
        else:
            self.choosenType = 0

        self.objComboBox.set_active(self.choosenType)
        #Not sure about this
        self.slider1.grab_focus()
        self.sendTables(self.slider1, 1)
        self.resize()
        self.sliderGate = True

    def changeObject(self, widget):
        self.choosenType = widget.props.value
        if self.sliderGate: self.new = True
        self.resize()
        if self.instanceID != 12:
            self.synthObjectsParameters.setType(self.instanceID, self.choosenType)
            self.invalidate_rect( self.bounds[self.instanceID][0], self.bounds[self.instanceID][1]-2, SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE_HIGHLIGHT )
            if self.new:
                self.writeTables( self.synthObjectsParameters.types,
                            self.synthObjectsParameters.controlsParameters,
                            self.synthObjectsParameters.sourcesParameters,
                            self.synthObjectsParameters.fxsParameters )

    def updateViewer(self):
        selectedType = SynthLabConstants.CHOOSE_TYPE[self.objectType][self.choosenType]
        infoType = SynthLabConstants.SYNTHPARA[selectedType][4]
        infoPara = SynthLabConstants.SYNTHPARA[selectedType][4+self.curSlider]
        self.infoText = infoType + '\n\n' + SynthLabConstants.SYNTHPARA[selectedType][self.curSlider-1] + ': ' + infoPara
        self.textBuf.set_text(self.infoText)

    def recallSliderValue( self, num ):
        if num == 1:
            if SynthLabConstants.SYNTHTYPES[self.objectType][self.choosenType] == SynthLabConstants.NOISE:
                return SynthLabConstants.NOISE_TYPES[int(self.slider1Val)]
            else:
                return '%.2f' % self.slider1Val
        if num == 2:
            if SynthLabConstants.SYNTHTYPES[self.objectType][self.choosenType] == SynthLabConstants.VCO:
                return SynthLabConstants.VCO_WAVEFORMS[int(self.slider2Val)]
            elif SynthLabConstants.SYNTHTYPES[self.objectType][self.choosenType] == SynthLabConstants.SAMPLE or SynthLabConstants.SYNTHTYPES[self.objectType][self.choosenType] == SynthLabConstants.GRAIN:
		sname = self.sample_names[int(self.slider2Val)]
                self.csnd.load_instrument(sname)
                return sname
            elif SynthLabConstants.SYNTHTYPES[self.objectType][self.choosenType] == SynthLabConstants.VOICE:
                return SynthLabConstants.VOWEL_TYPES[int(self.slider2Val)]
            else:
                return '%.2f' % self.slider2Val
        if num == 3:
            if SynthLabConstants.SYNTHTYPES[self.objectType][self.choosenType] == SynthLabConstants.LFO:
                return SynthLabConstants.LFO_WAVEFORMS[int(self.slider3Val)]
            elif SynthLabConstants.SYNTHTYPES[self.objectType][self.choosenType] == SynthLabConstants.TRACKPADX:
                return SynthLabConstants.SCALING_TYPES[int(self.slider3Val)]
            elif SynthLabConstants.SYNTHTYPES[self.objectType][self.choosenType] == SynthLabConstants.TRACKPADY:
                return SynthLabConstants.SCALING_TYPES[int(self.slider3Val)]
            elif SynthLabConstants.SYNTHTYPES[self.objectType][self.choosenType] == SynthLabConstants.FILTER:
                return SynthLabConstants.FILTER_TYPES[int(self.slider3Val)]
            elif SynthLabConstants.SYNTHTYPES[self.objectType][self.choosenType] == SynthLabConstants.RINGMOD:
                return SynthLabConstants.LFO_WAVEFORMS[int(self.slider3Val)]
            else:
                return '%.2f' % self.slider3Val
        if num == 4: return '%.2f' % self.slider4Val

    def resize( self ):
        selectedType = SynthLabConstants.CHOOSE_TYPE[self.objectType][self.choosenType]
        if self.new:
            slider1Init = SynthLabConstants.TYPES[selectedType][0]
            slider2Init = SynthLabConstants.TYPES[selectedType][1]
            slider3Init = SynthLabConstants.TYPES[selectedType][2]
            slider4Init = SynthLabConstants.TYPES[selectedType][3]
        else:
            parametersTable = self.synthObjectsParameters.choiceParamsSet[self.objectType]
            tablePos = (self.instanceID % 4)*4
            slider1Init = parametersTable[tablePos]
            slider2Init = parametersTable[tablePos+1]
            slider3Init = parametersTable[tablePos+2]
            slider4Init = parametersTable[tablePos+3]

        slider1Min = SynthLabConstants.TYPES[selectedType][4]
        slider1Max = SynthLabConstants.TYPES[selectedType][5]
        slider2Min = SynthLabConstants.TYPES[selectedType][6]
        if selectedType == 'sample' or selectedType == 'grain':
            self.sample_names = [name for i in range( len( self.instrumentDB.instNamed ) ) for name in self.instrumentDB.instNamed.keys() if self.instrumentDB.instNamed[ name ].instrumentId == i ]
            slider2Max = len(self.sample_names)
        else:
            slider2Max = SynthLabConstants.TYPES[selectedType][7]
        slider3Min = SynthLabConstants.TYPES[selectedType][8]
        slider3Max = SynthLabConstants.TYPES[selectedType][9]
        slider4Min = SynthLabConstants.TYPES[selectedType][10]
        slider4Max = SynthLabConstants.TYPES[selectedType][11]

        slider1Step = SynthLabConstants.TYPES[selectedType][12][0]
        slider1Snap = SynthLabConstants.TYPES[selectedType][12][1]
        slider2Step = SynthLabConstants.TYPES[selectedType][13][0]
        slider2Snap = SynthLabConstants.TYPES[selectedType][13][1]
        slider3Step = SynthLabConstants.TYPES[selectedType][14][0]
        slider3Snap = SynthLabConstants.TYPES[selectedType][14][1]

        self.p1Adjust.set_all(slider1Init, slider1Min, slider1Max, slider1Step, slider1Step, 0)
        if selectedType == 'mic':
            self.p2Adjust.set_all(self.instanceID-3, slider2Min, slider2Max, slider2Step, slider2Step, 0)
            self.slider2.set_sensitive(False)
        else:
            self.p2Adjust.set_all(slider2Init, slider2Min, slider2Max, slider2Step, slider2Step, 0)
            self.slider2.set_sensitive(True)
        self.p3Adjust.set_all(slider3Init, slider3Min, slider3Max, slider3Step, slider3Step, 0)
        self.p4Adjust.set_all(slider4Init, slider4Min, slider4Max, 0.01, 0.01, 0)

        self.slider1.set_digits(slider1Snap)
        self.slider2.set_digits(slider2Snap)
        self.slider3.set_digits(slider3Snap)

        #self.new = True

    def sendTables( self, widget, data ):
        if self.sliderGate:
            self.curSlider = data
            self.slider1Val = self.p1Adjust.value
            self.slider2Val = self.p2Adjust.value
            self.slider3Val = self.p3Adjust.value
            self.slider4Val = self.p4Adjust.value
            if self.instanceID != 12:
                self.synthObjectsParameters.setType(self.instanceID, self.choosenType)
            sliderListValue = [ self.p1Adjust.value, self.p2Adjust.value, self.p3Adjust.value, self.p4Adjust.value ]
            if self.objectType == 0:
                for i in range(4):
                    self.synthObjectsParameters.setControlParameter((self.instanceID % 4)*4+i, sliderListValue[i])
            elif self.objectType == 1:
                for i in range(4):
                    self.synthObjectsParameters.setSourceParameter((self.instanceID % 4)*4+i, sliderListValue[i])
            elif self.objectType == 2:
                for i in range(4):
                    self.synthObjectsParameters.setFxParameter((self.instanceID % 4)*4+i, sliderListValue[i])
            elif self.objectType == 3:
                for i in range(4):
                    self.synthObjectsParameters.setOutputParameter(i, sliderListValue[i])
            self.updateViewer()
            selectedType = SynthLabConstants.CHOOSE_TYPE[self.objectType][self.choosenType]
            _str = SynthLabConstants.SYNTHTYPES[self.objectType][self.choosenType] + '\n' + SynthLabConstants.SYNTHPARA[selectedType][self.curSlider-1] + ': ' + self.recallSliderValue(self.curSlider)
            self.parameterUpdate(_str)

    def handleSliderRelease(self, widget, data=None):
        if self.instanceID != 12:
            self.writeTables( self.synthObjectsParameters.types, self.synthObjectsParameters.controlsParameters,
                             self.synthObjectsParameters.sourcesParameters, self.synthObjectsParameters.fxsParameters )

    def handleSliderEnter(self, widget, data, slider):
        widget.grab_focus()
        self.sendTables(widget, slider)

        #selectedType = SynthLabConstants.CHOOSE_TYPE[self.objectType][self.choosenType]
        #_str = SynthLabConstants.SYNTHTYPES[self.objectType][self.choosenType] + '\n' + SynthLabConstants.SYNTHPARA[selectedType][self.curSlider-1] + ': ' + self.recallSliderValue(self.curSlider)
        #self.parameterUpdate(_str)

    def onKeyPress(self,widget,event):
        key = event.hardware_keycode

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

    def parameterUpdate( self, string ):
        self.infoLabel.set_text(string)

    def playNote( self, midiPitch, table ):
        cpsPitch = 261.626*pow(1.0594633, midiPitch-36)
        self.recCount += 1
        mess = "i5203." + str(self.recCount) + " 0 " + str(self.duration) + " " + str(cpsPitch) + " " + str(table) + " " + " " .join([str(n) for n in self.synthObjectsParameters.getOutputParameters()])
        self.csnd.inputMessage( mess )
        if self.recCount >= 9: self.recCount = 0

    def handleClose( self, widget, data ):
        if self.journalCalled:
            self.activity.close()
            return
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
        self._mainToolbar.durationSliderAdj.set_value(self.duration)
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
            for i in range(self.objectCount-1,-1,-1):
               if self.bounds[i][0] < event.x < self.bounds[i][2] and self.bounds[i][1] < event.y < self.bounds[i][3]:
                    if self.locations[i][1] == SynthLabConstants.INIT_LOCATIONS[i][1] \
                      and i != self.objectCount-1:
                        gate = False
                    else:
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
                        else:
                            self.select( i )
                    return

            if self.action == "draw-wire": # didn't hit anything
                self.doneWire()
            else:
                # check if we clicked a wire
                i = self.wireUnderLoc( event.x, event.y )
                if i >= 0: self.deleteWire( i )

        elif event.button == 3:
            for i in range(self.objectCount-1,-1,-1):
                if self.bounds[i][0] < event.x < self.bounds[i][2] and self.bounds[i][1] < event.y < self.bounds[i][3]:
                    if i in [4,5,6,7]:
                        if self.synthObjectsParameters.types[i] == 9:
                            snd = i - 3
                            dur = self.synthObjectsParameters.sourcesParameters[(i % 4) * 4]
                            os.system('rm ' + Config.SNDS_DIR + '/labmic' + str(snd))
                            (s1,o1) = commands.getstatusoutput("arecord -f S16_LE -t wav -r 16000 -d " + str(dur) + " " + Config.SNDS_DIR + '/tempMic.wav')
                            (s2, o2) = commands.getstatusoutput("csound " + Config.FILES_DIR + "/cropSynthLab.csd")
                            (s3, o3) = commands.getstatusoutput("mv " + Config.SNDS_DIR + "/micTemp " + Config.SNDS_DIR + "/" + 'labmic' + str(snd))
                            (s4, o4) = commands.getstatusoutput("rm " + Config.SNDS_DIR + "/tempMic.wav")
                            return

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
                    if gate and ( gate != self.wireGate or i != self.wireObj ):
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
                    self.highlightWire( None )
                    return
            # didn't find a gate
            self.highlightGate( None )
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
        if obj == None:
            self.parameterUpdate('')
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
                if True: #obj != 12:
                    if gate[0] == 0:
                        _str = SynthLabConstants.SYNTHTYPES[obj/4][self.typesTable[obj]] + _(': controller output')
                    elif gate[0] == 1:
                        choosen = SynthLabConstants.CHOOSE_TYPE[obj/4][self.typesTable[obj]]
                        parametersTable = self.synthObjectsParameters.choiceParamsSet[obj/4]
                        tablePos = (obj % 4)*4+gate[1]
                        paraVal = '%.2f' % parametersTable[tablePos]
                        _str = SynthLabConstants.SYNTHTYPES[obj/4][self.typesTable[obj]] + '\n' + SynthLabConstants.SYNTHPARA[choosen][gate[1]] + ': ' + paraVal
                        if self.overGateObj == self.instanceID:
                            gateNum = self.overGate[1]+1
                            exec 'self.slider%s.grab_focus()' % str(gateNum)
                            exec 'self.sendTables(self.slider%s, %d)' % (str(gateNum), gateNum)
                    elif gate[0] == 2:
                        _str = SynthLabConstants.SYNTHTYPES[obj/4][self.typesTable[obj]] + _(': sound output')
                    elif gate[0] == 3:
                        if obj != 12:
                            _str = SynthLabConstants.SYNTHTYPES[obj/4][self.typesTable[obj]] + _(': sound input')
                        else:
                            _str = _('Send sound to the speakers')
                    self.parameterUpdate( _str )

    def startDragObject( self, i ):
        self.dragObject = i
        self.dragInitialLoc = (self.locations[i][0],self.locations[i][1])
        self.potentialDisconnect = False
        self.invalidate_rect( self.bounds[i][0], self.bounds[i][1]-2, SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE_HIGHLIGHT )
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

        self.invalidate_rect(self.bounds[self.dragObject][0], self.bounds[self.dragObject][1]-2, SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE_HIGHLIGHT, False )
        if not self.potentialDisconnect:
            for i in self.outputMap[self.dragObject]:
                self.invalidate_rect( self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3], False )
            for i in self.inputMap[self.dragObject]:
                self.invalidate_rect( self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3], False )

        if y > (self.separatorY-5): self.potentialDisconnect = True
        else: self.potentialDisconnect = False

        self.locations[self.dragObject][0] = int( x )
        self.locations[self.dragObject][1] = int( y )
        self.updateBounds(self.dragObject)

        self.invalidate_rect(self.bounds[self.dragObject][0], self.bounds[self.dragObject][1]-2, SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE_HIGHLIGHT, False )
        if not self.potentialDisconnect:
            for i in self.outputMap[self.dragObject]:
                self.invalidate_rect( self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3], False )
            for i in self.inputMap[self.dragObject]:
                self.invalidate_rect( self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3], False )

    def doneDragObject( self ):
        if self.potentialDisconnect:
            self.invalidate_rect( self.bounds[self.dragObject][0], self.bounds[self.dragObject][1]-2, SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE_HIGHLIGHT, False )
            m = self.outputMap[self.dragObject][:]
            m.sort(reverse=True)
            for i in m: self.delConnection( i )
            m = self.inputMap[self.dragObject][:]
            m.sort(reverse=True)
            for i in m: self.delConnection( i )
            self.locations[self.dragObject][0] = SynthLabConstants.INIT_LOCATIONS[self.dragObject][0]
            self.locations[self.dragObject][1] = SynthLabConstants.INIT_LOCATIONS[self.dragObject][1]
            self.updateBounds( self.dragObject )
            #self.invalidate_rect(self.bounds[self.dragObject][0], self.bounds[self.dragObject][1]-2, SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE_HIGHLIGHT )
        else:
            #self.invalidate_rect( self.bounds[self.dragObject][0], self.bounds[self.dragObject][1]-2, SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE_HIGHLIGHT )
            for i in self.outputMap[self.dragObject]:
                self.invalidate_rect( self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3] )
            for i in self.inputMap[self.dragObject]:
                self.invalidate_rect( self.cBounds[i][0], self.cBounds[i][1], self.cBounds[i][2], self.cBounds[i][3] )

        # NOTE: select function invalidates the rect so no need to do it above
        self.select( self.dragObject )

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
        buf.draw_line( self.gc, startX, 1, stopX, 1 )
        buf.draw_line( self.gc, startX, self.separatorY, stopX, self.separatorY )

        # draw objects
        types = self.synthObjectsParameters.getTypes() + [0] # speaker
        self.gc.set_clip_mask( self.clipMask )
        for i in range(self.objectCount):
            if i == self.dragObject or i == self.instanceID:
                continue
            if startX > self.bounds[i][2] or stopX < self.bounds[i][0] or startY > self.bounds[i][3] or stopY < self.bounds[i][1]:
                continue
            type = i >> 2
            self.gc.set_clip_origin( self.bounds[i][0]-SynthLabConstants.PIC_SIZE*type, self.bounds[i][1] )
            buf.draw_drawable( self.gc, self.pixmap[type][types[i]], 0, 0, self.bounds[i][0], self.bounds[i][1], SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE )

        if self.dragObject != self.instanceID:
            i = self.instanceID
            type = i >> 2
            #draw object
            self.gc.set_clip_origin( self.bounds[i][0]-SynthLabConstants.PIC_SIZE*type, self.bounds[i][1] )
            buf.draw_drawable( self.gc, self.pixmap[type][types[i]], 0, 0, self.bounds[i][0], self.bounds[i][1], SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE )
            # draw selectionHighlight
            self.gc.set_clip_origin( self.bounds[i][0]-SynthLabConstants.PIC_SIZE*type, self.bounds[i][1]-82 )
            self.gc.foreground = self.highlightColor
            buf.draw_rectangle( self.gc, True, self.bounds[i][0], self.bounds[i][1]-2, SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE_HIGHLIGHT )
            self.gc.foreground = self.lineColor

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

        if self.screenBufDirty:
            self.predraw( self.screenBuf )

        # draw base
        widget.window.draw_drawable( self.gc, self.screenBuf, startX, startY, startX, startY, event.area.width, event.area.height )

        if self.action == "drag-object":
            # draw dragObject
            types = self.synthObjectsParameters.getTypes()
            self.gc.set_clip_mask( self.clipMask )
            type = self.dragObject >> 2
            self.gc.set_clip_origin( self.bounds[self.dragObject][0]-SynthLabConstants.PIC_SIZE*type, self.bounds[self.dragObject][1] )
            widget.window.draw_drawable( self.gc, self.pixmap[type][types[self.dragObject]], 0, 0, self.bounds[self.dragObject][0], self.bounds[self.dragObject][1], SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE )

            if self.instanceID == self.dragObject:
                # draw selectionHighlight
                self.gc.set_clip_origin( self.bounds[self.dragObject][0]-SynthLabConstants.PIC_SIZE*type, self.bounds[self.dragObject][1]-82 )
                self.gc.foreground = self.highlightColor
                widget.window.draw_rectangle( self.gc, True, self.bounds[self.dragObject][0], self.bounds[self.dragObject][1]-2, SynthLabConstants.PIC_SIZE, SynthLabConstants.PIC_SIZE_HIGHLIGHT )

            self.gc.set_clip_rectangle( self.clearMask )

            # draw wires
            if not self.potentialDisconnect:
                self.gc.foreground = self.lineColor
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
            self.gc.foreground = self.lineColor
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
        time.sleep(0.001)
        mess = "f5201 0 16 -2 " + " "  .join([str(n) for n in sourceParametersTable])
        self.csnd.inputMessage( mess )
        time.sleep(.001)
        mess = "f5202 0 16 -2 " + " "  .join([str(n) for n in fxParametersTable])
        self.csnd.inputMessage( mess )
        time.sleep(.001)
        self.typesTable = typesTable
        lastTable = [0]*12
        for i in range(12):
            if i in self.outputs:
                lastTable[i] = (typesTable[i]+1)
        mess = "f5203 0 16 -2 " + " "  .join([str(n) for n in lastTable]) + " 0 0 0 0"
        self.csnd.inputMessage( mess )
        time.sleep(.001)
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
        if lastTable[4] == 6:
            snd = self.sample_names[int(sourceParametersTable[1])]
            self.csnd.load_instrument(snd)
        if lastTable[5] == 6:
            snd = self.sample_names[int(sourceParametersTable[5])]
            self.csnd.load_instrument(snd)
        if lastTable[6] == 6:
            snd = self.sample_names[int(sourceParametersTable[9])]
            self.csnd.load_instrument(snd)
        if lastTable[7] == 6:
            snd = self.sample_names[int(sourceParametersTable[13])]
            self.csnd.load_instrument(snd)
        time.sleep(.005)

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

    def loadPixmaps( self ):
        win = gtk.gdk.get_default_root_window()
        gc = gtk.gdk.GC( win )
        gc.foreground = self.bgColor
        self.pixmap = [ [], [], [], [] ]

        def loadImg( type, img ):
            pix = gtk.gdk.pixbuf_new_from_file(Config.TAM_TAM_ROOT + '/icons/sl-' + img + '.svg')
            map = gtk.gdk.Pixmap( win, pix.get_width(), pix.get_height() )
            map.draw_rectangle( gc, True, 0, 0, pix.get_width(), pix.get_height() )
            map.draw_pixbuf( gc, pix, 0, 0, 0, 0, pix.get_width(), pix.get_height(), gtk.gdk.RGB_DITHER_NONE )
            self.pixmap[type].append(map)

        for i in range(len(SynthLabConstants.CONTROL_TYPES_PLUS)):
            loadImg( 0, SynthLabConstants.CONTROL_TYPES_PLUS[i] )
        for i in range(len(SynthLabConstants.SOURCE_TYPES_PLUS)):
            loadImg( 1, SynthLabConstants.SOURCE_TYPES_PLUS[i] )
        for i in range(len(SynthLabConstants.FX_TYPES_PLUS)):
            loadImg( 2, SynthLabConstants.FX_TYPES_PLUS[i] )
        loadImg( 3, "speaker" )

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
            if shift > 0:
                bitmap += "%c" % byte
                byte = 0
                shift = 0
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
        f = shelve.open( Config.TAM_TAM_ROOT + '/common/Resources/SynthFiles/' + preset, 'r')
        self.loadState(f)
        f.close()
        self.handleSaveTemp()
