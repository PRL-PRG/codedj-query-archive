
import pygtk
pygtk.require( '2.0' )
import gtk

from SubActivity import SubActivity

from Jam.Desktop import Desktop
import Jam.Picker as Picker
    
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
                self.GUI["bankInstrumentsTab"].connect( "clicked", lambda w: self.setPicker( Picker.Instrument, None ) )
                self.GUI["bankTabs"].pack_start( self.GUI["bankInstrumentsTab"] )
                self.GUI["bankDrumsTab"] = gtk.RadioButton( self.GUI["bankInstrumentsTab"], "Drums" )
                self.GUI["bankDrumsTab"].connect( "clicked", lambda w: self.setPicker( Picker.Drum ) )
                self.GUI["bankTabs"].pack_start( self.GUI["bankDrumsTab"] )
                self.GUI["bankLoopsTab"] = gtk.RadioButton( self.GUI["bankInstrumentsTab"], "Loops" )
                self.GUI["bankLoopsTab"].connect( "clicked", lambda w: self.setPicker( Picker.Loop ) )
                self.GUI["bankTabs"].pack_start( self.GUI["bankLoopsTab"] )

            if True: # Picker
                self.GUI["bankPicker"] = gtk.HBox()
                self.GUI["bankPicker"].set_size_request( -1, 149 )
                self.GUI["bankVBox"].pack_start( self.GUI["bankPicker"], False, False )

                self.pickers = {}
                self.pickerScroll = {}
                for type in [ Picker.Instrument, Picker.Drum, Picker.Loop ]:
                    self.pickers[type] = type( self )

            self.show_all()

            self.curPicker = None
            self.setPicker( Picker.Instrument )

    def onActivate( self, arg ):
        pass

    def onDeactivate( self ):
        pass

    def onDestroy( self ):
        pass

    def getDesktop( self ):
        return self.desktop

    def setPicker( self, type, filter = None ):
        if self.curPicker == type:
            if self.pickers[self.curPicker].getFilter() == filter:
                return
            self.pickers[self.curPicker].setFilter( filter )
        else:
            if self.curPicker != None:
                self.GUI["bankPicker"].remove( self.pickers[self.curPicker] )

            self.GUI["bankPicker"].pack_start( self.pickers[type] )
            self.curPicker = type

