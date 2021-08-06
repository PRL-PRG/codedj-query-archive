
import pygtk
pygtk.require( '2.0' )
import gtk

import random #TEMP
import sets

import Config
from   gettext import gettext as _

from sugar.graphics.palette import Palette, WidgetInvoker

import Util.ControlStream
import Util.InstrumentDB as InstrumentDB

import Jam.Block as Block

class Picker( gtk.HBox ):

    def __init__( self, owner, filter = None ):
        gtk.HBox.__init__( self )

        self.owner = owner
        
        # take drawing setup from owner
        self.gc = owner.gc
        self.colors = owner.colors
        self.clipMask = owner.clipMask

        self.filter = filter

        self.desktop = owner.getDesktop()

        self.scrollLeft = gtk.Button( "<" )
        self.scrollLeft.connect( "clicked", self.doScroll, "left" )
        self.pack_start( self.scrollLeft, False, False )

        self.scrolledWindow = gtk.ScrolledWindow()
        #self.scrolledWindow.modify_bg( gtk.STATE_NORMAL, self.colors["Bg_Active"] )
        self.scrolledWindow.set_policy( gtk.POLICY_ALWAYS, gtk.POLICY_NEVER )
        self.pack_start( self.scrolledWindow )
        self.hadjustment = self.scrolledWindow.get_hadjustment()
        self.hadjustment.connect( "changed", self.scrollChanged )
        self.hadjustment.connect( "value-changed", self.scrollChanged )

        self.scrollRight = gtk.Button( ">" )
        self.scrollRight.connect( "clicked", self.doScroll, "right" )
        self.pack_start( self.scrollRight, False, False )

        self.pickerBox = gtk.HBox()
        self.scrolledWindow.add_with_viewport( self.pickerBox )
        self.pickerBox.get_parent().modify_bg( gtk.STATE_NORMAL, self.colors["Picker_Bg"] )

        # spacers
        self.pickerBox.pack_start( gtk.Label(" "), True, True )
        self.pickerBox.pack_end( gtk.Label(" "), True, True )

        self.show_all()

        self.scroll = {}
        self.scroll[filter] = 0

        self.blocks = []

    def addBlock( self, data, name, block = None ):
        if block == None:
            block = gtk.Button( name )

        # tooltip
        block._palette = Palette( name )
        block._palette.props.invoker = WidgetInvoker(block)
        block._palette.set_property( "position", Palette.AT_CURSOR )

        block.add_events( gtk.gdk.BUTTON_PRESS_MASK
                        | gtk.gdk.BUTTON_RELEASE_MASK
                        | gtk.gdk.ENTER_NOTIFY_MASK
                        | gtk.gdk.LEAVE_NOTIFY_MASK
                        | gtk.gdk.POINTER_MOTION_MASK
                        | gtk.gdk.POINTER_MOTION_HINT_MASK )
        block.connect( "button-press-event", self.button_press )
        block.connect( "button-release-event", self.button_release )
        block.connect( "motion-notify-event", self.motion_notify )
        block.data = data

        self.blocks.append( block )

        if self._testAgainstFilter( block ):
            self.pickerBox.pack_start( block, False, False, 3 )
    
        block.show_all()

        return block

    def getFilter( self ):
        return filter

    def setFilter( self, filter ):
        if filter == self.filter:
            return

        self.scroll[self.filter] = self.hadjustment.get_value()

        self.filter = filter

        for block in self.pickerBox.get_children()[1:-1]: # outside children are spacers
            self.pickerBox.remove( block )

        for block in self.blocks:
            if self._testAgainstFilter( block ):
                self.pickerBox.pack_start( block, False, False, 3 )

        if self.scroll.has_key( filter ):
            self.hadjustment.set_value( self.scroll[filter] )
        else:
            self.hadjustment.set_value( 0 )
            self.scroll[filter] = 0

    def _testAgainstFilter( self, block ):
        return True

    def doScroll( self, widget, data ):
        if data == "left":
            val = max( self.hadjustment.get_property("lower"), self.hadjustment.get_value() - self.hadjustment.get_property("page_increment") )
        else:
            val = min( self.hadjustment.get_property("upper") - self.hadjustment.get_property("page_size"), self.hadjustment.get_value() + self.hadjustment.get_property("page_increment") )

        self.hadjustment.set_value( val )

    def scrollChanged( self, widget ):
        val = self.hadjustment.get_value()
        if val == 0:
            self.scrollLeft.set_sensitive( False )
        else:
            self.scrollLeft.set_sensitive( True )

        if val >= self.hadjustment.get_property( "upper" ) - self.hadjustment.get_property("page_size"):
            self.scrollRight.set_sensitive( False )
        else:
            self.scrollRight.set_sensitive( True )

    def button_press( self, widget, event ):
        pass

    def button_release( self, widget, event ):
        self.desktop.button_release( widget, event )

    def motion_notify( self, widget, event ):
        self.desktop.motion_notify( widget, event )


class Instrument( Picker ):
    
    def __init__( self, owner, filter =  ( "all" ) ):
        Picker.__init__( self, owner, filter )

        self.type = Instrument

        self.instrumentDB = InstrumentDB.getRef()

        for inst in self.instrumentDB.getSet( "all" ):
            self.addBlock( inst.id )

    def addBlock( self, id ):
        # match data structure of Block.Instrument
        data = { "name": _(Config.INSTRUMENTSID[id].name),
                 "id":   id } 

        win = gtk.gdk.get_default_root_window()
        width = Block.Instrument.WIDTH
        height = Block.Instrument.HEIGHT
        pixmap = gtk.gdk.Pixmap( win, width, height )

        self.gc.set_clip_rectangle( gtk.gdk.Rectangle( 0, 0, width, height ) )
        
        # draw bg
        self.gc.foreground = self.colors["Picker_Bg"]
        pixmap.draw_rectangle( self.gc, True, 0, 0, width, height )

        self.gc.set_clip_mask( self.clipMask )
        
        # draw border
        self.gc.foreground = self.colors["Border_Inactive"]
        self.gc.set_clip_origin( -Block.Instrument.MASK_START, 0 )
        pixmap.draw_rectangle( self.gc, True, 0, 0, width, height )

        # draw block
        inst = self.owner.getInstrumentImage( data["id"] )
        self.gc.set_clip_origin( -Block.Instrument.MASK_START, -height )
        pixmap.draw_drawable( self.gc, inst, 0, 0, 0, 0, width, height )      

        image = gtk.Image()
        image.set_from_pixmap( pixmap, None )

        block = gtk.EventBox()
        block.modify_bg( gtk.STATE_NORMAL, self.colors["Picker_Bg"] )
        block.add( image )

        Picker.addBlock( self, data, data["name"], block )

    def _testAgainstFilter( self, block ):
        if "all" in self.filter:
            return True

        for label in self.instrumentDB.getInstrument( block.data["id"] ).labels:
            if label in self.filter:
                return True

        return False
        
    def button_press( self, widget, event ):
        walloc = widget.get_allocation()
        salloc = self.scrolledWindow.get_allocation()
        loc = ( walloc.x + salloc.x + event.x - self.hadjustment.get_value(), -1 )
        self.desktop.addBlock( Block.Instrument, widget.data, loc, True )


class Drum( Picker ):
    
    def __init__( self, owner, filter = None ):
        Picker.__init__( self, owner, filter )

        self.type = Drum

        self.instrumentDB = InstrumentDB.getRef()

        for inst in self.instrumentDB.getSet( "kit" ):
            self.addBlock( inst.id )

    def addBlock( self, id ):
        # match data structure of Block.Drum
        data = { "name":       _(Config.INSTRUMENTSID[id].name),
                 "id":         id }

        win = gtk.gdk.get_default_root_window()
        width = Block.Drum.WIDTH
        height = Block.Drum.HEIGHT
        pixmap = gtk.gdk.Pixmap( win, width, height )

        self.gc.set_clip_rectangle( gtk.gdk.Rectangle( 0, 0, width, height ) )
        
        # draw bg
        self.gc.foreground = self.colors["Picker_Bg"]
        pixmap.draw_rectangle( self.gc, True, 0, 0, width, height )

        self.gc.set_clip_mask( self.clipMask )
        
        # draw border
        self.gc.foreground = self.colors["Border_Inactive"]
        self.gc.set_clip_origin( -Block.Drum.MASK_START, 0 )
        pixmap.draw_rectangle( self.gc, True, 0, 0, width, height )

        # draw block
        inst = self.owner.getInstrumentImage( data["id"] )
        self.gc.set_clip_origin( -Block.Drum.MASK_START, -height )
        pixmap.draw_drawable( self.gc, inst, 0, 0, 0, 0, width, height )      

        image = gtk.Image()
        image.set_from_pixmap( pixmap, None )

        block = gtk.EventBox()
        block.modify_bg( gtk.STATE_NORMAL, self.colors["Picker_Bg"] )
        block.add( image )

        Picker.addBlock( self, data, data["name"], block ) 
        
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


