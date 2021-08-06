
import pygtk
pygtk.require( '2.0' )
import gtk

import os

import random #TEMP
import sets

import Config
from   gettext import gettext as _

from sugar.graphics.palette import Palette, WidgetInvoker

from Util import ControlStream
from Util import InstrumentDB

from Jam import Block

class Picker( gtk.HBox ):

    def __init__( self, owner, filter = None ):
        gtk.HBox.__init__( self )

        self.owner = owner
        
        # take drawing setup from owner
        self.gc = owner.gc
        self.colors = owner.colors
        self.blockMask = owner.blockMask

        self.filter = filter

        self.desktop = owner.getDesktop()

        self.scrollLeft = gtk.Button( "<" )
        self.scrollLeft.connect( "clicked", self.doScroll, "left" )
        self.pack_start( self.scrollLeft, False, False )

        self.scrolledWindow = gtk.ScrolledWindow()
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
        block.connect( "button-press-event", self.on_button_press )
        block.connect( "button-release-event", self.on_button_release )
        block.connect( "motion-notify-event", self.on_motion_notify )
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

    def on_button_press( self, widget, event ):
        pass

    def on_button_release( self, widget, event ):
        self.desktop.on_button_release( widget, event )

    def on_motion_notify( self, widget, event ):
        self.desktop.on_motion_notify( widget, event )


class Instrument( Picker ):
    
    def __init__( self, owner, filter =  ( "All" ) ):
        Picker.__init__( self, owner, filter )

        self.type = Instrument

        self.instrumentDB = InstrumentDB.getRef()

        for inst in self.instrumentDB.getSet( "All" ):
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

        self.gc.set_clip_mask( self.blockMask )
        
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
        if "All" in self.filter:
            return True

        for label in self.instrumentDB.getInstrument( block.data["id"] ).labels:
            if label in self.filter:
                return True

        return False
        
    def on_button_press( self, widget, event ):
        walloc = widget.get_allocation()
        salloc = self.scrolledWindow.get_allocation()
        loc = ( walloc.x + salloc.x + event.x - self.hadjustment.get_value(), -1 )

        block = self.desktop.addBlock( Block.Instrument, widget.data, loc, True )
        self.desktop.activateInstrument( block )


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

        self.gc.set_clip_mask( self.blockMask )
        
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
        
    def on_button_press( self, widget, event ):
        walloc = widget.get_allocation()
        salloc = self.scrolledWindow.get_allocation()
        loc = ( walloc.x + salloc.x + event.x - self.hadjustment.get_value(), -1 )
        self.desktop.addBlock( Block.Drum, widget.data, loc, True )


class Loop( Picker ):
    
    def __init__( self, owner, filter = None ):
        Picker.__init__( self, owner, filter )

        self.type = Loop

        self.presetLoops = self._scanDirectory( Config.FILES_DIR+"/Loops" )
        
    def _loadFile( self, fullpath, filename ):
        if filename[-4:] != ".ttl": 
            if Config.DEBUG >= 3: print "WARNING: incorrect extension on loop file: " + filename
            return -1
        try:
            oldPages = sets.Set( self.owner.noteDB.getTune() )

            ifile = open( fullpath, 'r' )
            ttt = ControlStream.TamTamTable ( self.owner.noteDB )
            ttt.parseFile( ifile )
            ifile.close()

            curPages = sets.Set( self.owner.noteDB.getTune() )
            newPages = curPages.difference( oldPages )

            if len(newPages) != 1:
                print "ERROR: bad loop file, contains more than one page (or none)"
                return -1
            
            id = newPages.pop() # new pageId

            self.owner.noteDB.getPage( id ).local = False # flag as a global page
            
            self.addBlock( id, filename[:-4] )

            return id
            
        except OSError,e:
            print 'ERROR: failed to open file %s for reading\n' % ofilename
            return -1

    def _scanDirectory( self, path ):
        dirlist = os.listdir( path )
        ids = []
        for fpath in dirlist:
            id = self._loadFile( path+"/"+fpath, fpath )
            if id != -1: ids.append(id)
        return ids

    def addBlock( self, id, name ):
        # match data structure of Block.Loop
        data = { "name": _(name),
                 "id":   id } 

        self.owner.updateLoopImage( data["id"] )
        loop = self.owner.getLoopImage( data["id"] )
        
        page = self.owner.noteDB.getPage( id )

        win = gtk.gdk.get_default_root_window()
        width = Block.Loop.WIDTH[page.beats]
        height = Block.Loop.HEIGHT
        pixmap = gtk.gdk.Pixmap( win, width, height )

        self.gc.set_clip_rectangle( gtk.gdk.Rectangle( 0, 0, width, height ) )
        
        # draw bg
        self.gc.foreground = self.colors["Picker_Bg"]
        pixmap.draw_rectangle( self.gc, True, 0, 0, width, height )

        self.gc.set_clip_mask( self.blockMask )
        self.gc.foreground = self.owner.colors["Border_Inactive"]
 
        #-- draw head -----------------------------------------

        # draw border
        self.gc.set_clip_origin( -Block.Loop.MASK_START, 0 )
        pixmap.draw_rectangle( self.gc, True, 0, 0, Block.Loop.HEAD, height )

        # draw block
        self.gc.set_clip_origin( -Block.Loop.MASK_START, -height )
        pixmap.draw_drawable( self.gc, loop, 0, 0, 0, 0, Block.Loop.HEAD, height )      

        #-- draw beats ----------------------------------------
  
        beats = page.beats - 1 # last beat is drawn with the tail
        curx = Block.Loop.HEAD
        while beats > 3:
            # draw border
            self.gc.set_clip_origin( curx-Block.Loop.MASK_BEAT, 0 )
            pixmap.draw_rectangle( self.gc, True, curx, 0, Block.Loop.BEAT_MUL3, height )

            # draw block
            self.gc.set_clip_origin( curx-Block.Loop.MASK_BEAT, -height )
            pixmap.draw_drawable( self.gc, loop, curx, 0, curx, 0, Block.Loop.BEAT_MUL3, height )      

            curx += Block.Loop.BEAT_MUL3
            beats -= 3

        if beats:
            w = Block.Loop.BEAT*beats 

            # draw border
            self.gc.set_clip_origin( curx-Block.Loop.MASK_BEAT, 0 )
            pixmap.draw_rectangle( self.gc, True, curx, 0, w, height )

            # draw block
            self.gc.set_clip_origin( curx-Block.Loop.MASK_BEAT, -height )
            pixmap.draw_drawable( self.gc, loop, curx, 0, curx, 0, w, height )      

            curx += w 

        #-- draw tail -----------------------------------------

        # draw border
        self.gc.set_clip_origin( curx-Block.Loop.MASK_TAIL, 0 )
        pixmap.draw_rectangle( self.gc, True, curx, 0, Block.Loop.TAIL, height )

        # draw block
        self.gc.set_clip_origin( curx-Block.Loop.MASK_TAIL, -height )
        pixmap.draw_drawable( self.gc, loop, curx, 0, curx, 0, Block.Loop.TAIL, height )      

        image = gtk.Image()
        image.set_from_pixmap( pixmap, None )

        block = gtk.EventBox()
        block.modify_bg( gtk.STATE_NORMAL, self.colors["Picker_Bg"] )
        block.add( image )

        Picker.addBlock( self, data, data["name"], block ) 
       
    def on_button_press( self, widget, event ):
        walloc = widget.get_allocation()
        salloc = self.scrolledWindow.get_allocation()
        loc = ( walloc.x + salloc.x + event.x - self.hadjustment.get_value(), -1 )

        data = {}
        for key in widget.data.keys():
            data[key] = widget.data[key]

        newid = self.owner.noteDB.duplicatePages( [ data["id"] ] )[data["id"]] 
        self.owner.updateLoopImage( newid )
        data["id"] = newid

        block = self.desktop.addBlock( Block.Loop, data, loc, True )


