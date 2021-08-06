import pygtk
pygtk.require('2.0')
import gtk
from Util.ThemeWidgets import *
import Config
Tooltips = Config.Tooltips()

class LoopSettings( gtk.VBox ):
    def __init__( self, doneHandler, popup ):
        gtk.VBox.__init__( self )
        self.tooltips = gtk.Tooltips()
        self.doneHandler = doneHandler
        self.popup = popup
        self.popup.resize( 545, 378 )

        self.settingsBox = RoundHBox(fillcolor=Config.INST_BCK_COLOR, bordercolor=Config.PANEL_BCK_COLOR)
        self.settingsBox.set_radius(10)
        self.pack_start(self.settingsBox)
        
        self.fixed = gtk.Fixed()
        self.settingsBox.pack_start(self.fixed)
        
        self.controlsBox = gtk.HBox()

        startBox = RoundVBox(fillcolor=Config.PANEL_COLOR, bordercolor=Config.INST_BCK_COLOR)
        startBox.set_border_width(3)
        startBox.set_radius(10)
        self.startAdjust = gtk.Adjustment( 0.5, 0, 1, .1, .1, 0)
        self.GUI['startSlider'] = ImageVScale( Config.TAM_TAM_ROOT + "/Resources/Images/sliderEditVolume.png", self.startAdjust, 7 )
        self.startAdjust.connect("value-changed", self.handleStart)
        self.GUI['startSlider'].set_snap( 0.1 )
        self.GUI['startSlider'].set_inverted(True)
        self.GUI['startSlider'].set_size_request(50, 200)
        self.handleStart( self.startAdjust )
        startBox.pack_start(self.GUI['startSlider'], True, True, 5)
        self.controlsBox.pack_start(startBox)        

        self.fixed.put( self.controlsBox, 0, 0 )
        
        self.show_all()

