
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

        #-- initial settings ----------------------------------
        self.tempo = Config.PLAYER_TEMPO
        self.volume = 50
        self.reverb = 0
        
        self.csnd = new_csound_client()
        for i in range(1,9):
            self.csnd.setTrackVolume( 100, i )
        self.csnd.setMasterVolume( self.volume )
        self.csnd.loopSetTempo( self.tempo )

        #-- Drawing -------------------------------------------
        win = gtk.gdk.get_default_root_window()
        self.gc = gtk.gdk.GC( win )
        colormap = gtk.gdk.colormap_get_system()
        self.colors = { "bg":                 colormap.alloc_color( Config.PANEL_BCK_COLOR ), 
                        "Picker_Bg":          colormap.alloc_color( "#404040" ), 
                        "Picker_Bg_Inactive": colormap.alloc_color( "#808080" ), 
                        #"Picker_Bg":          colormap.alloc_color( style.COLOR_TOOLBAR_GREY.get_html() ), 
                        #"Picker_Bg_Inactive": colormap.alloc_color( style.COLOR_BUTTON_GREY.get_html() ), 
                        "Picker_Fg":          colormap.alloc_color( style.COLOR_WHITE.get_html() ), 
                        "Border_Active":      colormap.alloc_color( "#FF6000" ), 
                        "Border_Inactive":    colormap.alloc_color( "#8D8D8D" ), 
                        "Border_Highlight":   colormap.alloc_color( "#FFFFFF" ), 
                        "Bg_Active":          colormap.alloc_color( "#9400BE" ), 
                        "Bg_Inactive":        colormap.alloc_color( "#DBDBDB" ) }
        
        if True: # load clipmask
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
            self.clipMask = gtk.gdk.bitmap_create_from_data( None, bitmap, pix.get_width(), pix.get_height() )

        #-- Instrument Images ---------------------------------
        self.instrumentImage = {}
        self.instrumentImageActive = {}
        for inst in self.instrumentDB.getSet( "all" ):
            try:
                pix = gtk.gdk.pixbuf_new_from_file(inst.img)
                x = (Block.Block.WIDTH-pix.get_width())//2
                y = (Block.Block.HEIGHT-pix.get_height())//2
                img = gtk.gdk.Pixmap( win, Block.Block.WIDTH, Block.Block.HEIGHT )
                self.gc.foreground = self.colors["Bg_Inactive"]
                img.draw_rectangle( self.gc, True, 0, 0, Block.Block.WIDTH, Block.Block.HEIGHT )
                img.draw_pixbuf( self.gc, pix, 0, 0, x, y, pix.get_width(), pix.get_height(), gtk.gdk.RGB_DITHER_NONE )
                self.instrumentImage[inst.id] = img
                img = gtk.gdk.Pixmap( win, Block.Block.WIDTH, Block.Block.HEIGHT )
                self.gc.foreground = self.colors["Bg_Active"]
                img.draw_rectangle( self.gc, True, 0, 0, Block.Block.WIDTH, Block.Block.HEIGHT )
                img.draw_pixbuf( self.gc, pix, 0, 0, x, y, pix.get_width(), pix.get_height(), gtk.gdk.RGB_DITHER_NONE )
                self.instrumentImageActive[inst.id] = img
            except:
                if Config.DEBUG >= 5: print "JamMain:: file does not exist: " + inst.img
                img = gtk.gdk.Pixmap( win, Block.Block.WIDTH, Block.Block.HEIGHT )
                self.gc.foreground = self.colors["Bg_Inactive"]
                img.draw_rectangle( self.gc, True, 0, 0, Block.Block.WIDTH, Block.Block.HEIGHT )
                self.instrumentImage[inst.id] = img
                img = gtk.gdk.Pixmap( win, Block.Block.WIDTH, Block.Block.HEIGHT )
                self.gc.foreground = self.colors["Bg_Active"]
                img.draw_rectangle( self.gc, True, 0, 0, Block.Block.WIDTH, Block.Block.HEIGHT )
                self.instrumentImageActive[inst.id] = img
 
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

    #==========================================================
    # Get/Set 

    def getDesktop( self ):
        return self.desktop

    def getInstrumentImage( self, id, active = False ):
        if active: return self.instrumentImageActive[id]
        else:      return self.instrumentImage[id]           

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
 
