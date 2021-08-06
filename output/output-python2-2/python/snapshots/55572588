
import pygtk
pygtk.require( '2.0' )
import gtk

import random #TEMP
import sets

import Config

import Util.ControlStream

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

    def addBlock( self, data, name = "TEMPORARY ARG" ):
        block = gtk.Button(name)
        block.add_events(gtk.gdk.BUTTON_MOTION_MASK)
        block.connect( "button-press-event", self.button_press )
        block.connect( "button-release-event", self.button_release )
        block.connect( "motion-notify-event", self.motion_notify )
        block.data = data

        self.blocks.append( block )

        # TODO test against filter
        self.pickerBox.pack_start( block, False, False )
        block.show()

        return block

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

        # TEMP
        self.addBlock( Config.INSTRUMENTS["kalimba"].instrumentId )
        self.addBlock( Config.INSTRUMENTS["flute"].instrumentId )

    def addBlock( self, id ):
        # match data structure of Block.Instrument
        data = { "name": Config.INSTRUMENTSID[id].name,
                 "id":   id } 
        Picker.addBlock( self, data, Config.INSTRUMENTSID[id].name )
        
    def button_press( self, widget, event ):
        walloc = widget.get_allocation()
        salloc = self.scrolledWindow.get_allocation()
        loc = ( walloc.x + salloc.x + event.x - self.hadjustment.get_value(), -1 )
        self.desktop.addBlock( Block.Instrument, widget.data, loc, True )


class Drum( Picker ):
    
    def __init__( self, owner, filter = None ):
        Picker.__init__( self, owner, filter )

        self.type = Drum

        # TEMP
        self.addBlock( Config.INSTRUMENTS["drum1kit"].instrumentId )
        self.addBlock( Config.INSTRUMENTS["drum2kit"].instrumentId )
        self.addBlock( Config.INSTRUMENTS["drum3kit"].instrumentId )
        self.addBlock( Config.INSTRUMENTS["drum4kit"].instrumentId )

    def addBlock( self, id ):
        # match data structure of Block.Drum
        data = { "name":       Config.INSTRUMENTSID[id].name,
                 "id":         id }
        Picker.addBlock( self, data, Config.INSTRUMENTSID[id].name )
        
    def button_press( self, widget, event ):
        walloc = widget.get_allocation()
        salloc = self.scrolledWindow.get_allocation()
        loc = ( walloc.x + salloc.x + event.x - self.hadjustment.get_value(), -1 )
        self.desktop.addBlock( Block.Drum, widget.data, loc, True )


class Loop( Picker ):
    
    def __init__( self, owner, filter = None ):
        Picker.__init__( self, owner, filter )

        self.type = Loop

        self.addBlock( {}, "Loop" )
        
    def _loadFile( self, path ):
        try:
            oldPages = sets.Set( self.owner.noteDB.getTune() )

            ifile = open( path, 'r' )
            ttt = ControlStream.TamTamTable ( self.owner.noteDB )
            ttt.parseFile( ifile )
            ifile.close()

            curPages = sets.Set( self.owner.noteDB.getTune() )
            newPages = curPages.difference( oldPages )

            if len(newPages) != 1:
                print "ERROR: bad loop file, contains more than one page (or none)"
                return -1

            return newPages.pop() # new pageId
            
        except OSError,e:
            print 'ERROR: failed to open file %s for reading\n' % ofilename


 

    def _scanDirectory( self, path ):
        pass    

    def button_press( self, widget, event ):
        walloc = widget.get_allocation()
        salloc = self.scrolledWindow.get_allocation()
        loc = ( walloc.x + salloc.x + event.x - self.hadjustment.get_value(), -1 )
        self.desktop.addBlock( Block.Loop, { "beats": random.randint(1,8) }, loc, True )


