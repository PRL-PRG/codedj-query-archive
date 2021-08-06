
import pygtk
pygtk.require( '2.0' )
import gtk

from SubActivity import SubActivity

from Jam.Desktop import Desktop
from Jam.Picker import Picker
    
class JamMain(SubActivity):
    
    def __init__(self, activity, set_mode):
        SubActivity.__init__(self, set_mode)

        self.activity = activity
        

        #======================================================
        # GUI

        if True: # GUI
            self.GUI = {}
            self.GUI["mainVBox"] = gtk.VBox()
            self.add( self.GUI["mainVBox"] )

            #-- Desktop -------------------------------------------
            self.desktop = self.GUI["desktop"] = Desktop( self )
            self.GUI["mainVBox"].pack_start( self.GUI["desktop"] )

            #-- Bank ----------------------------------------------
            self.GUI["bankVBox"] = gtk.VBox()
            self.GUI["mainVBox"].pack_start( self.GUI["bankVBox"], False, False )
            if True: # Tabs
                self.GUI["bankTabs"] = gtk.HBox()
                self.GUI["bankTabs"].set_size_request( -1, 38 )
                self.GUI["bankVBox"].pack_start( self.GUI["bankTabs"], False, False )

                self.GUI["bankInstrumentsTab"] = gtk.RadioButton( None, "Instruments" )
                self.GUI["bankTabs"].pack_start( self.GUI["bankInstrumentsTab"] )
                self.GUI["bankDrumsTab"] = gtk.RadioButton( self.GUI["bankInstrumentsTab"], "Drums" )
                self.GUI["bankTabs"].pack_start( self.GUI["bankDrumsTab"] )
                self.GUI["bankLoopsTab"] = gtk.RadioButton( self.GUI["bankInstrumentsTab"], "Loops" )
                self.GUI["bankTabs"].pack_start( self.GUI["bankLoopsTab"] )

            if True: # Picker
                self.GUI["bankPicker"] = gtk.HBox()
                self.GUI["bankPicker"].set_size_request( -1, 149 )
                self.GUI["bankVBox"].pack_start( self.GUI["bankPicker"], False, False )

                self.GUI["bankScrollLeft"] = gtk.Button( "<" )
                self.GUI["bankPicker"].pack_start( self.GUI["bankScrollLeft"], False, False )

                self.GUI["bankScrolledWindow"] = gtk.ScrolledWindow()
                self.GUI["bankScrolledWindow"].set_policy( gtk.POLICY_ALWAYS, gtk.POLICY_NEVER )
                self.GUI["bankPicker"].pack_start( self.GUI["bankScrolledWindow"] )

                self.GUI["bankScrollRight"] = gtk.Button( ">" )
                self.GUI["bankPicker"].pack_start( self.GUI["bankScrollRight"], False, False )

                self.GUI["pickerInstruments"] = Picker( self, Picker.INSTRUMENTS )
                self.GUI["pickerInstruments"].show_all()

                self.GUI["bankScrolledWindow"].add_with_viewport( self.GUI["pickerInstruments"] )

            self.show_all()

    def onActivate( self, arg ):
        pass

    def onDeactivate( self ):
        pass

    def onDestroy( self ):
        pass

    def getDesktop( self ):
        return self.desktop
