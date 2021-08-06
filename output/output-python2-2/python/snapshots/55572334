
import pygtk
pygtk.require( '2.0' )
import gtk

from SubActivity import SubActivity

import Config
from   gettext import gettext as _
import sugar.graphics.style as style

from Jam.Desktop import Desktop
import Jam.Picker as Picker
import Jam.Block as Block
from Jam.Toolbars import DesktopToolbar

    
from Util.CSoundNote import CSoundNote
from Util.CSoundClient import new_csound_client
import Util.InstrumentDB as InstrumentDB
from Util import NoteDB

from Fillin import Fillin
from RythmGenerator import generator
from Generation.GenerationConstants import GenerationConstants
from Util.NoteDB import Note

from math import sqrt

class JamMain(SubActivity):
    
    def __init__(self, activity, set_mode):
        SubActivity.__init__(self, set_mode)

        self.activity = activity

        self.instrumentDB = InstrumentDB.getRef()
        self.noteDB = NoteDB.NoteDB()

        #-- initial settings ----------------------------------
        self.tempo = Config.PLAYER_TEMPO
        self.volume = 50
        self.reverb = 0
        
        self.csnd = new_csound_client()
        for i in range(1,9):
            self.csnd.setTrackVolume( 100, i )
        self.csnd.setMasterVolume( self.volume )
        self.csnd.setTempo( self.tempo )

        #-- Drawing -------------------------------------------
        def darken( colormap, hex ):
            hexToDec = { "0":0, "1":1, "2":2, "3":3, "4":4, "5":5, "6":6, "7":7, "8":8, "9":9, "A":10, "B":11, "C":12, "D":13, "E":14, "F":15, "a":10, "b":11, "c":12, "d":13, "e":14, "f":15 }
            r = int( 0.7*(16*hexToDec[hex[1]] + hexToDec[hex[2]]) )
            g = int( 0.7*(16*hexToDec[hex[3]] + hexToDec[hex[4]]) )
            b = int( 0.7*(16*hexToDec[hex[5]] + hexToDec[hex[6]]) )
            return colormap.alloc_color( r*256, g*256, b*256 )
        def lighten( colormap, hex ):
            hexToDec = { "0":0, "1":1, "2":2, "3":3, "4":4, "5":5, "6":6, "7":7, "8":8, "9":9, "A":10, "B":11, "C":12, "D":13, "E":14, "F":15, "a":10, "b":11, "c":12, "d":13, "e":14, "f":15 }
            r = 255 - int( 0.7*(255-(16*hexToDec[hex[1]] + hexToDec[hex[2]])) )
            g = 255 - int( 0.7*(255-(16*hexToDec[hex[3]] + hexToDec[hex[4]])) )
            b = 255 - int( 0.7*(255-(16*hexToDec[hex[5]] + hexToDec[hex[6]])) )
            return colormap.alloc_color( r*256, g*256, b*256 )

        win = gtk.gdk.get_default_root_window()
        self.gc = gtk.gdk.GC( win )
        colormap = gtk.gdk.colormap_get_system()
        self.colors = { "bg":                   colormap.alloc_color( Config.PANEL_BCK_COLOR ), 
                        "Picker_Bg":            colormap.alloc_color( "#404040" ), 
                        "Picker_Bg_Inactive":   colormap.alloc_color( "#808080" ), 
                        #"Picker_Bg":            colormap.alloc_color( style.COLOR_TOOLBAR_GREY.get_html() ), 
                        #"Picker_Bg_Inactive":   colormap.alloc_color( style.COLOR_BUTTON_GREY.get_html() ), 
                        "Picker_Fg":            colormap.alloc_color( style.COLOR_WHITE.get_html() ), 
                        "Border_Active":        colormap.alloc_color( "#590000" ), 
                        "Border_Inactive":      colormap.alloc_color( "#8D8D8D" ), 
                        "Border_Highlight":     colormap.alloc_color( "#FFFFFF" ), 
                        "Bg_Active":            colormap.alloc_color( "#FFDDEA" ), 
                        "Bg_Inactive":          colormap.alloc_color( "#DBDBDB" ),
                        "Note_Fill_Active":     lighten( colormap, "#590000" ),  # base "Border_Active"
                        "Note_Fill_Inactive":   lighten( colormap, "#8D8D8D" ) } # base "Border_Inactive"
        self.colors[    "Note_Border_Active"] =   self.colors["Border_Active"]
        self.colors[    "Note_Border_Inactive"] = self.colors["Border_Inactive"]


        if True: # load block clipmask
            pix = gtk.gdk.pixbuf_new_from_file(Config.IMAGE_ROOT+'jam-blockMask.png')
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
            self.blockMask = gtk.gdk.bitmap_create_from_data( None, bitmap, pix.get_width(), pix.get_height() )
        
        self.sampleNoteHeight = 7
        if True: # load sample note clipmask
            pix = gtk.gdk.pixbuf_new_from_file(Config.IMAGE_ROOT+'sampleNoteMask.png')
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
            self.sampleNoteMask = gtk.gdk.bitmap_create_from_data( None, bitmap, pix.get_width(), pix.get_height() )
            self.sampleNoteMask.endOffset = pix.get_width()-3

        self.loopPitchOffset = 4
        self.loopTickOffset = 13
        self.pitchPerPixel = float(Config.NUMBER_OF_POSSIBLE_PITCHES-1) / (Block.Loop.HEIGHT - 2*self.loopPitchOffset - self.sampleNoteHeight)
        self.pixelsPerPitch = float(Block.Loop.HEIGHT - 2*self.loopPitchOffset - self.sampleNoteHeight)/(Config.MAXIMUM_PITCH - Config.MINIMUM_PITCH)
        self.pixelsPerTick = Block.Loop.BEAT/float(Config.TICKS_PER_BEAT)
        self.ticksPerPixel = 1.0/self.pixelsPerTick

        #-- Instrument Images ---------------------------------
        self.instrumentImage = {}
        self.instrumentImageActive = {}
        for inst in self.instrumentDB.getSet( "All" ):
            self.prepareInstrumentImage( inst.id, inst.img )

        #-- Loop Images ---------------------------------------
        self.loopImage = {}       # get filled in through updateLoopImage 
        self.loopImageActive = {} #

        #-- Toolbars ------------------------------------------
        self.desktopToolbar = DesktopToolbar( self )
        self.activity.toolbox.add_toolbar( _("Desktop"), self.desktopToolbar )

        #-- GUI -----------------------------------------------
        if True: # GUI
            self.modify_bg( gtk.STATE_NORMAL, self.colors["bg"] ) # window bg
            
            self.GUI = {}
            self.GUI["mainVBox"] = gtk.VBox()
            self.add( self.GUI["mainVBox"] )

            #-- Desktop -------------------------------------------
            self.desktop = self.GUI["desktop"] = Desktop( self )
            self.GUI["mainVBox"].pack_start( self.GUI["desktop"] )

            #-- Bank ----------------------------------------------
            separator = gtk.Label( " " )
            separator.set_size_request( -1, style.TOOLBOX_SEPARATOR_HEIGHT )
            self.GUI["mainVBox"].pack_start( separator, False )
            self.GUI["notebook"] = gtk.Notebook()
            self.GUI["notebook"].set_scrollable( True )
            self.GUI["notebook"].modify_bg( gtk.STATE_NORMAL, self.colors["Picker_Bg"] )            # active tab
            self.GUI["notebook"].modify_bg( gtk.STATE_ACTIVE, self.colors["Picker_Bg_Inactive"] )   # inactive tab
            self.GUI["notebook"].props.tab_vborder = style.TOOLBOX_TAB_VBORDER
            self.GUI["notebook"].props.tab_hborder = style.TOOLBOX_TAB_HBORDER
            self.GUI["notebook"].set_size_request( -1, 160 )
            self.GUI["notebook"].connect( "switch-page", self.setPicker )
            self.GUI["mainVBox"].pack_start( self.GUI["notebook"], False, False )
            self.pickers = {}
            self.pickerScroll = {}
            for type in [ Picker.Instrument, Picker.Drum, Picker.Loop ]:
                self.pickers[type] = type( self )

            def prepareLabel( name ):
                label = gtk.Label( _(name) )
                label.set_alignment( 0.0, 0.5 )
                label.modify_fg( gtk.STATE_NORMAL, self.colors["Picker_Fg"] )
                label.modify_fg( gtk.STATE_ACTIVE, self.colors["Picker_Fg"] )
                return label
                
            self.GUI["notebook"].append_page( self.pickers[Picker.Drum], prepareLabel("Drum Kits") )
            self.GUI["notebook"].append_page( self.pickers[Picker.Loop], prepareLabel("Loops") )

            sets = self.instrumentDB.getLabels()[:]
            sets.sort()
            for set in sets:
                page = gtk.HBox()
                page.set = set
                self.GUI["notebook"].append_page( page, prepareLabel( set ) )

            self.show_all()

            self.GUI["notebook"].set_current_page( 0 )

        #-- Keyboard ------------------------------------------
        self.key_dict = {}
        self.nextTrack = 1

        # default instrument
        self._updateInstrument( Config.INSTRUMENTS["kalimba"].instrumentId, 0.5 )

        #-- Drums ---------------------------------------------
        # use dummy values for now
        self.drumFillin = Fillin( 2, 100, Config.INSTRUMENTS["drum1kit"].instrumentId, self.reverb, 1 )

    #==========================================================
    # SubActivity Handlers 

    def onActivate( self, arg ):
        pass

    def onDeactivate( self ):
        pass

    def onDestroy( self ):
        pass

    #==========================================================
    # Playback 

    def onKeyPress( self, widget, event ):
        key = event.hardware_keycode

        if self.key_dict.has_key( key ): # repeated press
            return

        if Config.KEY_MAP_PIANO.has_key( key ):
            pitch = Config.KEY_MAP_PIANO[key]
            inst = Config.INSTRUMENTS[self.instrument["name"]]

            if inst.kit: # drum kit
                if pitch in GenerationConstants.DRUMPITCH:
                    pitch = GenerationConstants.DRUMPITCH[pitch]
                print inst.kit
                self._playNote( key, 
                                36, 
                                self.instrument["amplitude"], 
                                self.instrument["pan"], 
                                100, 
                                inst.kit[pitch].instrumentId,
                                self.instrument["reverb"] ) 
            else:
                if event.state == gtk.gdk.MOD1_MASK:
                    pitch += 5
                
                if inst.csoundInstrumentId == Config.INST_PERC: #Percussions resonance
                    duration = 60 
                else:
                    duration = -1

                self._playNote( key, 
                                pitch,
                                self.instrument["amplitude"], 
                                self.instrument["pan"], 
                                duration,
                                self.instrument["id"], 
                                self.instrument["reverb"] ) 
 
    def onKeyRelease( self, widget, event ):
        key = event.hardware_keycode

        if self.key_dict.has_key( key ): 
            self._stopNote( key )

    def _playNote( self, key, pitch, amplitude, pan, duration, instrumentId, reverb ):
        self.key_dict[key] = CSoundNote( 0, # onset
                                         pitch,
                                         amplitude,
                                         pan,
                                         duration,
                                         self.nextTrack,
                                         instrumentId,
                                         reverbSend = reverb,
                                         tied = True,
                                         mode = 'mini' )
        self.nextTrack += 1
        if self.nextTrack > 8:
            self.nextTrack = 1
        self.csnd.play(self.key_dict[key], 0.3)

    def _stopNote( self, key ):
        csnote = self.key_dict[key]
        if Config.INSTRUMENTSID[ csnote.instrumentId ].csoundInstrumentId == Config.INST_TIED:
            csnote.duration = .5
            csnote.decay = 0.7
            csnote.tied = False
            self.csnd.play(csnote, 0.3)
        del self.key_dict[key]
 
    def _updateInstrument( self, id, volume ): 
        self.instrument = { "name":         Config.INSTRUMENTSID[id].name,
                            "id":           id,
                            "amplitude":    sqrt( self.volume*volume*0.1 ),
                            "pan":          0.5,
                            "reverb":       self.reverb }

    def _playDrum( self, id, volume, beats, regularity, seed ):
        def flatten(ll):
            rval = []
            for l in ll:
                rval += l
            return rval

        noteOnsets = []
        notePitchs = []
        i = 0
        self.noteList= []
        self.csnd.loopClear()
        for x in flatten( generator( Config.INSTRUMENTSID[id].name, beats, 0.8, regularity, self.reverb) ):
            x.amplitude = x.amplitude * volume 
            noteOnsets.append(x.onset)
            notePitchs.append(x.pitch)
            n = Note(0, x.trackId, i, x)
            self.noteList.append( (x.onset, n) )
            i = i + 1
            self.csnd.loopPlay(n,1)                    #add as active
        self.csnd.loopSetNumTicks( beats * Config.TICKS_PER_BEAT )

        self.drumFillin.setProperties( self.tempo, Config.INSTRUMENTSID[id].name, volume, beats, self.reverb ) 
        self.drumFillin.unavailable( noteOnsets, notePitchs )

        self.drumFillin.play()
        self.csnd.loopSetTick(0)
        self.csnd.loopStart()
        
    def _stopDrum( self ):
        self.drumFillin.stop()
        self.csnd.loopPause()

    def _playLoop( self, id, volume, tune ):
        loopId = self.csnd.loopCreate()
        
        offset = 0
        print "------------", loopId, tune
        temp = []
        for page in tune:
            for n in self.noteDB.getNotesByTrack( page, 0 ):
                temp.append( n )
                n.pushState()
                n.cs.instrumentId = id
                n.cs.onset += offset
                self.csnd.loopPlay( n, 1, loopId = loopId )
                n.popState()
            offset += self.noteDB.getPage(page).ticks

        print temp

        self.csnd.loopSetNumTicks( offset, loopId )
        
        # TODO update for beat syncing

        self.csnd.loopStart( loopId )

        return loopId

    def _stopLoop( self, loopId ):
        print "===============", loopId
        self.csnd.loopDestroy( loopId )

    #==========================================================
    # Get/Set 

    def getDesktop( self ):
        return self.desktop

    def getInstrumentImage( self, id, active = False ):
        if active: return self.instrumentImageActive[id]
        else:      return self.instrumentImage[id]           

    def getLoopImage( self, id, active = False ):
        if active: return self.loopImageActive[id]
        else:      return self.loopImage[id]

    def setPicker( self, widget, pagePointer, page_num ):
        page = self.GUI["notebook"].get_nth_page( page_num )
        if page == self.pickers[Picker.Drum]:
            pass
        elif page == self.pickers[Picker.Loop]:
            pass
        else:
            self.pickers[Picker.Instrument].setFilter( ( page.set ) )
            parent = self.pickers[Picker.Instrument].get_parent()
            if parent != page:
                if parent != None:
                    parent.remove( self.pickers[Picker.Instrument] )
                page.add( self.pickers[Picker.Instrument] )

    #==========================================================
    # Pixmaps 

    def prepareInstrumentImage( self, id, img_path ):
        try:
            win = gtk.gdk.get_default_root_window()
            pix = gtk.gdk.pixbuf_new_from_file( img_path )
            x = (Block.Block.WIDTH-pix.get_width())//2
            y = (Block.Block.HEIGHT-pix.get_height())//2
            img = gtk.gdk.Pixmap( win, Block.Block.WIDTH, Block.Block.HEIGHT )
            self.gc.foreground = self.colors["Bg_Inactive"]
            img.draw_rectangle( self.gc, True, 0, 0, Block.Block.WIDTH, Block.Block.HEIGHT )
            img.draw_pixbuf( self.gc, pix, 0, 0, x, y, pix.get_width(), pix.get_height(), gtk.gdk.RGB_DITHER_NONE )
            self.instrumentImage[id] = img
            img = gtk.gdk.Pixmap( win, Block.Block.WIDTH, Block.Block.HEIGHT )
            self.gc.foreground = self.colors["Bg_Active"]
            img.draw_rectangle( self.gc, True, 0, 0, Block.Block.WIDTH, Block.Block.HEIGHT )
            img.draw_pixbuf( self.gc, pix, 0, 0, x, y, pix.get_width(), pix.get_height(), gtk.gdk.RGB_DITHER_NONE )
            self.instrumentImageActive[id] = img
        except:
            if Config.DEBUG >= 5: print "JamMain:: file does not exist: " + img_path
            img = gtk.gdk.Pixmap( win, Block.Block.WIDTH, Block.Block.HEIGHT )
            self.gc.foreground = self.colors["Bg_Inactive"]
            img.draw_rectangle( self.gc, True, 0, 0, Block.Block.WIDTH, Block.Block.HEIGHT )
            self.instrumentImage[id] = img
            img = gtk.gdk.Pixmap( win, Block.Block.WIDTH, Block.Block.HEIGHT )
            self.gc.foreground = self.colors["Bg_Active"]
            img.draw_rectangle( self.gc, True, 0, 0, Block.Block.WIDTH, Block.Block.HEIGHT )
            self.instrumentImageActive[id] = img
    
    def _drawNotes( self, pixmap, beats, notes, active ):
        self.gc.set_clip_mask( self.sampleNoteMask )
        for note in notes: # draw N notes
            x = self.ticksToPixels( note.cs.onset )
            endX = self.ticksToPixels( note.cs.onset + note.cs.duration ) - 3 # include end cap offset
            width = endX - x
            if width < 5: 
                width = 5
                endX = x + width
            y = self.pitchToPixels( note.cs.pitch )
            # draw fill
            if active: self.gc.foreground = self.colors["Note_Fill_Active"]
            else:      self.gc.foreground = self.colors["Note_Fill_Inactive"]
            self.gc.set_clip_origin( x, y-self.sampleNoteHeight )
            pixmap.draw_rectangle( self.gc, True, x+1, y+1, width+1, self.sampleNoteHeight-2 )
            # draw border
            if active: self.gc.foreground = self.colors["Note_Border_Active"]
            else:      self.gc.foreground = self.colors["Note_Border_Inactive"]
            self.gc.set_clip_origin( x, y )
            pixmap.draw_rectangle( self.gc, True, x, y, width, self.sampleNoteHeight )
            self.gc.set_clip_origin( endX-self.sampleNoteMask.endOffset, y )
            pixmap.draw_rectangle( self.gc, True, endX, y, 3, self.sampleNoteHeight )
 
    def updateLoopImage( self, id ):
        page = self.noteDB.getPage( id )

        win = gtk.gdk.get_default_root_window()
        width = Block.Loop.WIDTH[page.beats]
        height = Block.Loop.HEIGHT

        self.gc.set_clip_rectangle( gtk.gdk.Rectangle( 0, 0, width, height ) )

        pixmap = gtk.gdk.Pixmap( win, width, height )
        self.gc.foreground = self.colors["Bg_Inactive"]
        pixmap.draw_rectangle( self.gc, True, 0, 0, width, height )
        self._drawNotes( pixmap, page.beats, self.noteDB.getNotesByTrack( id, 0 ), False )
        self.loopImage[id] = pixmap

        self.gc.set_clip_rectangle( gtk.gdk.Rectangle( 0, 0, width, height ) )

        pixmap = gtk.gdk.Pixmap( win, width, height )
        self.gc.foreground = self.colors["Bg_Active"]
        pixmap.draw_rectangle( self.gc, True, 0, 0, width, height )
        self._drawNotes( pixmap, page.beats, self.noteDB.getNotesByTrack( id, 0 ), True )
        self.loopImageActive[id] = pixmap

    def ticksToPixels( self, ticks ):
        return self.loopTickOffset + int(round( ticks * self.pixelsPerTick ))
    def pitchToPixels( self, pitch ):
        return self.loopPitchOffset + int(round( ( Config.MAXIMUM_PITCH - pitch ) * self.pixelsPerPitch ))


