
import pygtk
pygtk.require( '2.0' )
import gtk

import random #TEMP

import Jam.Block as Block

class Picker( gtk.HBox ):

    def __init__( self, owner, filter = None ):
        gtk.HBox.__init__( self )

        self.owner = owner

        self.filter = filter

        self.desktop = owner.getDesktop()

        self.scrollLeft = gtk.Button( "<" )
        self.pack_start( self.scrollLeft, False, False )

        self.scrolledWindow = gtk.ScrolledWindow()
        self.scrolledWindow.set_policy( gtk.POLICY_ALWAYS, gtk.POLICY_NEVER )
        self.pack_start( self.scrolledWindow )
        self.hadjustment = self.scrolledWindow.get_hadjustment()

        self.scrollRight = gtk.Button( ">" )
        self.pack_start( self.scrollRight, False, False )

        self.pickerBox = gtk.HBox()
        self.scrolledWindow.add_with_viewport( self.pickerBox )

        self.show_all()

        self.scroll = {}
        self.scroll[filter] = 0

        self.blocks = []

    def addBlock( self, name ):
        block = gtk.Button(name)
        block.add_events(gtk.gdk.BUTTON_MOTION_MASK)
        block.connect( "button-press-event", self.button_press )
        block.connect( "button-release-event", self.button_release )
        block.connect( "motion-notify-event", self.motion_notify )

        self.blocks.append( block )

        # TODO test against filter
        self.pickerBox.pack_start( block, False, False )
        block.show()

    def getFilter( self ):
        return filter

    def setFilter( self, filter ):
        # TODO apply filter

        self.scroll[self.filter] = self.hadjustment.get_value()

        if self.scroll.has_key( filter ):
            self.hadjustment.set_value( self.scroll[filter] )
        else:
            self.hadjustment.set_value( 0 )
            self.scroll[filter] = 0

        self.filter = filter

    def button_press( self, widget, event ):
        pass

    def button_release( self, widget, event ):
        self.desktop.button_release( widget, event )

    def motion_notify( self, widget, event ):
        self.desktop.motion_notify( widget, event )


class Instrument( Picker ):
    
    def __init__( self, owner, filter = None ):
        Picker.__init__( self, owner, filter )

        self.type = Instrument

        self.addBlock( "Instrument" )
        
    def button_press( self, widget, event ):
        walloc = widget.get_allocation()
        salloc = self.scrolledWindow.get_allocation()
        loc = ( walloc.x + salloc.x + event.x - self.hadjustment.get_value(), -1 )
        self.desktop.addBlock( Block.Instrument, [], loc, True )


class Drum( Picker ):
    
    def __init__( self, owner, filter = None ):
        Picker.__init__( self, owner, filter )

        self.type = Drum

        self.addBlock( "Drum" )
        
    def button_press( self, widget, event ):
        walloc = widget.get_allocation()
        salloc = self.scrolledWindow.get_allocation()
        loc = ( walloc.x + salloc.x + event.x - self.hadjustment.get_value(), -1 )
        self.desktop.addBlock( Block.Drum, [], loc, True )


class Loop( Picker ):
    
    def __init__( self, owner, filter = None ):
        Picker.__init__( self, owner, filter )

        self.type = Loop

        self.addBlock( "Loop" )
        
    def button_press( self, widget, event ):
        walloc = widget.get_allocation()
        salloc = self.scrolledWindow.get_allocation()
        loc = ( walloc.x + salloc.x + event.x - self.hadjustment.get_value(), -1 )
        self.desktop.addBlock( Block.Loop, { "beats": random.randint(1,8) }, loc, True )


