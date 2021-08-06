import pygtk
pygtk.require('2.0')
import gtk

from GUI.Core.KeyMapping import *
from GUI.Core.ThemeWidgets import *
from GUI.GUIConstants import GUIConstants
from SynthLab.SynthLabConstants import SynthLabConstants
from Framework.Constants import Constants
from GUI.Tooltips import Tooltips

class SynthLabParametersWindow( gtk.Window ):
    def __init__( self, instanceID, synthObjectsParameters, writeTables, playNoteFunction ):
        gtk.Window.__init__( self, gtk.WINDOW_TOPLEVEL )
        self.set_type_hint(gtk.gdk.WINDOW_TYPE_HINT_DIALOG)
        self.set_title("SynthLab Parameters")
        self.set_position( gtk.WIN_POS_CENTER )
        self.set_default_size(30, 300)
        self.set_border_width(0)
        self.set_decorated(False)
        self.mainBox = RoundVBox(fillcolor=GUIConstants.PANEL_COLOR, bordercolor=GUIConstants.INST_BCK_COLOR)
        self.mainBox.set_border_width(1)
        self.mainBox.set_radius(10)
        self.typeBox = gtk.HBox(False, 0)
        self.sliderBox = gtk.HBox(False, 0)
        self.add_events(gtk.gdk.KEY_PRESS_MASK)
	self.add_events(gtk.gdk.KEY_RELEASE_MASK)
        self.connect("key-press-event", self.onKeyPress)
	self.connect("key-release-event", self.onKeyRelease)
        self.instanceID = instanceID
        self.objectType = self.instanceID / 4
        self.synthObjectsParameters = synthObjectsParameters
        self.writeTables = writeTables
        self.playNoteFunction = playNoteFunction
	self.playingPitch = []

	self.tooltips = gtk.Tooltips()

        types = SynthLabConstants.CHOOSE_TYPE[self.objectType]
	types2 = SynthLabConstants.CHOOSE_TYPE2[self.objectType]
        self.choosenType = self.synthObjectsParameters.types[self.instanceID]

        self.initRadioButton( types, types2, self.typeCallback, self.typeBox, self.choosenType )
        self.mainBox.pack_start(self.typeBox)

        selectedType = SynthLabConstants.CHOOSE_TYPE[self.objectType][self.choosenType]
        slider1Min = SynthLabConstants.TYPES[selectedType][4]
        slider1Max = SynthLabConstants.TYPES[selectedType][5]
        slider2Min = SynthLabConstants.TYPES[selectedType][6]
        slider2Max = SynthLabConstants.TYPES[selectedType][7]
        slider3Min = SynthLabConstants.TYPES[selectedType][8]
        slider3Max = SynthLabConstants.TYPES[selectedType][9]

        slider1Step = SynthLabConstants.TYPES[selectedType][10][0]
        slider1Snap = SynthLabConstants.TYPES[selectedType][10][1]
        slider2Step = SynthLabConstants.TYPES[selectedType][11][0]
        slider2Snap = SynthLabConstants.TYPES[selectedType][11][1]
        slider3Step = SynthLabConstants.TYPES[selectedType][12][0]
        slider3Snap = SynthLabConstants.TYPES[selectedType][12][1]

        parametersTable = self.synthObjectsParameters.choiceParamsSet[self.objectType]
        tablePos = (self.instanceID % 4)*4
        slider1Init = parametersTable[tablePos]
        slider2Init = parametersTable[tablePos+1]
        slider3Init = parametersTable[tablePos+2]
        slider4Init = parametersTable[tablePos+3]

        self.p1Adjust = gtk.Adjustment(slider1Init, slider1Min, slider1Max, slider1Step, slider1Step, 0)
        self.p1Adjust.connect("value-changed", self.sendTables)
        self.slider1 = ImageVScale(Constants.TAM_TAM_ROOT + '/Resources/Images/sliderbutred.png', self.p1Adjust, 7, snap = slider1Snap)
        self.slider1.set_inverted(True)
        self.slider1.set_size_request(50, 150)
        self.sliderBox.pack_start(self.slider1, True, False)

        self.p2Adjust = gtk.Adjustment(slider2Init, slider2Min, slider2Max, slider2Step, slider2Step, 0)
        self.p2Adjust.connect("value-changed", self.sendTables)
        self.slider2 = ImageVScale(Constants.TAM_TAM_ROOT + '/Resources/Images/sliderbutred.png', self.p2Adjust, 7, snap = slider2Snap)
        self.slider2.set_inverted(True)
        self.slider2.set_size_request(50, 150)
        self.sliderBox.pack_start(self.slider2, True, False)

        self.p3Adjust = gtk.Adjustment(slider3Init, slider3Min, slider3Max, slider3Step, slider3Step, 0)
        self.p3Adjust.connect("value-changed", self.sendTables)
        self.slider3 = ImageVScale(Constants.TAM_TAM_ROOT + '/Resources/Images/sliderbutred.png', self.p3Adjust, 7, snap = slider3Snap)
        self.slider3.set_inverted(True)
        self.slider3.set_size_request(50, 150)
        self.sliderBox.pack_start(self.slider3, True, False)

        self.p4Adjust = gtk.Adjustment(slider4Init, 0, 1, .01, .01, 0)
        self.p4Adjust.connect("value-changed", self.sendTables)
        self.slider4 = ImageVScale(Constants.TAM_TAM_ROOT + '/Resources/Images/sliderbutred.png', self.p4Adjust, 7)
        self.slider4.set_digits(2)
        self.slider4.set_value_pos(2)
        self.slider4.set_inverted(True)
        self.slider4.set_size_request(50, 150)
        self.sliderBox.pack_start(self.slider4, True, False)
	
	tipPos = self.instanceID % 4
	self.tooltips.set_tip(self.slider1, Tooltips.SYNTHPARA[selectedType][0])
	self.tooltips.set_tip(self.slider2, Tooltips.SYNTHPARA[selectedType][1])
	self.tooltips.set_tip(self.slider3, Tooltips.SYNTHPARA[selectedType][2])
	self.tooltips.set_tip(self.slider4, Tooltips.SYNTHPARA[selectedType][3])
        self.mainBox.pack_start(self.sliderBox)

        closeButton = ImageButton(Constants.TAM_TAM_ROOT + '/Resources/Images/close.png' )
        closeButton.connect('clicked', self.destroy )
        self.mainBox.pack_start(closeButton)

        self.add(self.mainBox)
        self.show_all()

    def destroy( self, data=None ):
        self.hide()

    def onKeyPress(self,widget,event):
        midiPitch = KEY_MAP[event.hardware_keycode]
        if midiPitch not in self.playingPitch:
            self.playingPitch.append( midiPitch )
            self.playNoteFunction( midiPitch )
            
    def onKeyRelease( self, widget, event ):
        midiPitch = KEY_MAP[event.hardware_keycode]
        if midiPitch in self.playingPitch:
            self.playingPitch.remove( midiPitch )

    def resize( self ):
        selectedType = SynthLabConstants.CHOOSE_TYPE[self.objectType][self.choosenType]

        slider1Init = SynthLabConstants.TYPES[selectedType][0]
        slider2Init = SynthLabConstants.TYPES[selectedType][1]
        slider3Init = SynthLabConstants.TYPES[selectedType][2]
        slider4Init = SynthLabConstants.TYPES[selectedType][3]

        slider1Min = SynthLabConstants.TYPES[selectedType][4]
        slider1Max = SynthLabConstants.TYPES[selectedType][5]
        slider2Min = SynthLabConstants.TYPES[selectedType][6]
        slider2Max = SynthLabConstants.TYPES[selectedType][7]
        slider3Min = SynthLabConstants.TYPES[selectedType][8]
        slider3Max = SynthLabConstants.TYPES[selectedType][9]

        slider1Step = SynthLabConstants.TYPES[selectedType][10][0]
        slider1Snap = SynthLabConstants.TYPES[selectedType][10][1]
        slider2Step = SynthLabConstants.TYPES[selectedType][11][0]
        slider2Snap = SynthLabConstants.TYPES[selectedType][11][1]
        slider3Step = SynthLabConstants.TYPES[selectedType][12][0]
        slider3Snap = SynthLabConstants.TYPES[selectedType][12][1]

        self.slider1.set_snap(slider1Snap)
        self.slider2.set_snap(slider2Snap)
        self.slider3.set_snap(slider3Snap)

        self.p1Adjust.set_all(slider1Init, slider1Min, slider1Max, slider1Step, slider1Step, 0)
        self.p2Adjust.set_all(slider2Init, slider2Min, slider2Max, slider2Step, slider2Step, 0)
        self.p3Adjust.set_all(slider3Init, slider3Min, slider3Max, slider3Step, slider3Step, 0)
        self.p4Adjust.set_all(slider4Init, 0, 1, 0.01, 0.01, 0)
        
	self.tooltips.set_tip(self.slider1, Tooltips.SYNTHPARA[selectedType][0])
	self.tooltips.set_tip(self.slider2, Tooltips.SYNTHPARA[selectedType][1])
	self.tooltips.set_tip(self.slider3, Tooltips.SYNTHPARA[selectedType][2])
	self.tooltips.set_tip(self.slider4, Tooltips.SYNTHPARA[selectedType][3])

    def typeCallback( self, widget, choosenType ):
        if widget.get_active():
            self.choosenType = choosenType
            self.resize()

    def sendTables( self, data ):
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

        self.writeTables( self.synthObjectsParameters.types, self.synthObjectsParameters.controlsParameters, 
                        self.synthObjectsParameters.sourcesParameters, self.synthObjectsParameters.fxsParameters )

    def initRadioButton( self, labelList, labelList2, methodCallback, box, active ):
        for i in range( len( labelList ) ):
            if i == 0:
                button = ImageRadioButton( group = None, mainImg_path = GUIConstants.IMAGE_ROOT + labelList[i] + '.png', altImg_path = GUIConstants.IMAGE_ROOT + labelList2[i] + '.png' )
            else:
                button = ImageRadioButton( group = button, mainImg_path = GUIConstants.IMAGE_ROOT + labelList[i] + '.png', altImg_path = GUIConstants.IMAGE_ROOT + labelList2[i] + '.png' )
            if i == active:
                button.set_active(True)
            button.connect( "toggled", methodCallback, i )
	    self.tooltips.set_tip(button, labelList[i])
            box.pack_start( button, False, False, 5 )


