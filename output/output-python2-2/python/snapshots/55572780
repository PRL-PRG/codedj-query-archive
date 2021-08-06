
import pygtk
pygtk.require( '2.0' )
import gtk

import Jam.Block as Block

class Picker( gtk.HBox ):

    INSTRUMENTS = 0
    LOOPS       = 1
    DRUMKITS    = 2

    def __init__( self, owner, type, filter = None ):
        gtk.HBox.__init__( self )

        self.owner = owner

        self.type = type
        self.filter = filter

        self.desktop = owner.getDesktop()

        # temp
        dummy = gtk.Button("dummy")
        dummy.add_events(gtk.gdk.BUTTON_MOTION_MASK)
        dummy.connect( "button-press-event", self.button_press )
        dummy.connect( "button-release-event", self.button_release )
        dummy.connect( "motion-notify-event", self.motion_notify )

        self.pack_start( dummy, False, False )

    def button_press( self, widget, event ):
        alloc = widget.get_allocation()
        if self.type == Picker.INSTRUMENTS:
            blockClass = Block.Instrument
            blockData = []
            loc = ( alloc.x + event.x - blockClass.WIDTH_DIV2, -1 )
        elif self.type == Picker.LOOPS:
            pass
        elif self.type == Picker.DRUMKITS:
            pass
        self.desktop.addBlock( blockClass, blockData, loc, True )

    def button_release( self, widget, event ):
        self.desktop.button_release( widget, event )

    def motion_notify( self, widget, event ):
        self.desktop.motion_notify( widget, event )


