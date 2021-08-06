
import pygtk
pygtk.require( '2.0' )
import gtk
import pango

import os, sys, shutil, commands
import random

import common.Util.Instruments
import common.Config as Config
from   gettext import gettext as _
import sugar.graphics.style as style

from Jam.Desktop import Desktop
import Jam.Picker as Picker
import Jam.Block as Block
from Jam.Toolbars import JamToolbar, PlaybackToolbar, DesktopToolbar, RecordToolbar


from common.Util.CSoundNote import CSoundNote
from common.Util.CSoundClient import new_csound_client
import common.Util.InstrumentDB as InstrumentDB
from common.Util import NoteDB

from Fillin import Fillin
from RythmGenerator import generator
from common.Generation.GenerationConstants import GenerationConstants
from common.Util.NoteDB import Note, Page

from common.Util import ControlStream

import xdrlib
import time
import gobject
import common.Util.Network as Net
from sugar.presence import presenceservice
from sugar.graphics.xocolor import XoColor

from math import sqrt

class JamMain(gtk.EventBox):

    def __init__(self, activity):
        gtk.EventBox.__init__(self)

        self.activity = activity

        self.instrumentDB = InstrumentDB.getRef()
        self.noteDB = NoteDB.NoteDB()

        #-- initial settings ----------------------------------
        self.tempo = Config.PLAYER_TEMPO
        self.beatDuration = 60.0/self.tempo
        self.ticksPerSecond = Config.TICKS_PER_BEAT*self.tempo/60.0
        self.volume = 0.5

        self.csnd = new_csound_client()
        for i in range(0,9):
            self.csnd.setTrackVolume( 100, i )
        self.csnd.setMasterVolume( self.volume*100 ) # csnd expects a range 0-100 for now
        self.csnd.setTempo( self.tempo )

        self.paused = False

        presenceService = presenceservice.get_instance()
        self.xoOwner = presenceService.get_owner()

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

        xoColorKey = self.xoOwner.props.color
        if not xoColorKey:
            xoColorKey = ( "#8D8D8D,#FFDDEA" )
        xoColor = XoColor( xoColorKey )

        win = gtk.gdk.get_default_root_window()
        self.gc = gtk.gdk.GC( win )
        colormap = gtk.gdk.colormap_get_system()
        self.colors = { "bg":                   colormap.alloc_color( Config.PANEL_BCK_COLOR ),
                        "black":                colormap.alloc_color( style.COLOR_BLACK.get_html() ),
                        #"Picker_Bg":            colormap.alloc_color( "#404040" ),
                        #"Picker_Bg_Inactive":   colormap.alloc_color( "#808080" ),
                        "Picker_Bg":            colormap.alloc_color( style.COLOR_TOOLBAR_GREY.get_html() ),
                        "Picker_Bg_Inactive":   colormap.alloc_color( style.COLOR_BUTTON_GREY.get_html() ),
                        "Picker_Fg":            colormap.alloc_color( style.COLOR_WHITE.get_html() ),
                        "Border_Active":        colormap.alloc_color( xoColor.get_stroke_color() ), #colormap.alloc_color( "#590000" ),
                        "Border_Inactive":      colormap.alloc_color( "#8D8D8D" ),
                        "Border_Highlight":     colormap.alloc_color( "#FFFFFF" ),
                        "Bg_Active":            colormap.alloc_color( xoColor.get_fill_color() ), #colormap.alloc_color( "#FFDDEA" ),
                        "Bg_Inactive":          colormap.alloc_color( "#DBDBDB" ),
                        "Preview_Note_Fill":    colormap.alloc_color( Config.BG_COLOR ),
                        "Preview_Note_Border":  colormap.alloc_color( Config.FG_COLOR ),
                        "Preview_Note_Selected": colormap.alloc_color( style.COLOR_WHITE.get_html() ),
                        "Note_Fill_Active":     lighten( colormap, "#590000" ), # base "Border_Active"
                        "Note_Fill_Inactive":   lighten( colormap, "#8D8D8D" ), # base "Border_Inactive"
                        "Beat_Line":            colormap.alloc_color( "#959595" ) }
        self.colors[    "Note_Border_Active"]   = self.colors["Border_Active"]
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

        pix = gtk.gdk.pixbuf_new_from_file( Config.IMAGE_ROOT+"sampleBG.png" )
        self.sampleBg = gtk.gdk.Pixmap( win, pix.get_width(), pix.get_height() )
        self.sampleBg.draw_pixbuf( self.gc, pix, 0, 0, 0, 0, pix.get_width(), pix.get_height(), gtk.gdk.RGB_DITHER_NONE )
        self.sampleBg.endOffset = pix.get_width()-5
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
            self.prepareInstrumentImage( inst.instrumentId, inst.img )

        #-- Loop Images ---------------------------------------
        self.loopImage = {}       # get filled in through updateLoopImage
        self.loopImageActive = {} #

        #-- Key Images ----------------------------------------
        self.keyImage = {}
        self.keyImageActive = {}
        # use hardware key codes to work on any keyboard layout (hopefully)
        self.valid_shortcuts = { 18:"9", 19:"0", 20:"-", 21:"=",
                                 32:"O", 33:"P", 34:"[", 35:"]",
                                 47:";", 48:"'", 51:"\\",
                                 60:".", 61:"/",
                                 None:" " }
        for key in self.valid_shortcuts.keys():
            self.prepareKeyImage( key )

        #-- Toolbars ------------------------------------------
        self.jamToolbar = JamToolbar( self )
        self.activity.toolbox.add_toolbar( _("Jam"), self.jamToolbar )
        self.playbackToolbar = PlaybackToolbar( self )
        self.activity.toolbox.add_toolbar( _("Playback"), self.playbackToolbar )
        self.desktopToolbar = DesktopToolbar( self )
        self.activity.toolbox.add_toolbar( _("Desktop"), self.desktopToolbar )
        self.recordToolbar = RecordToolbar( self )
        self.activity.toolbox.add_toolbar( _("Record"), self.recordToolbar )

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
                label = gtk.Label( _(name.capitalize()) )
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
        self.keyboardListener = None
        self.recordingNote = None

        self.keyMap = {}

        # default instrument
        self._updateInstrument( self.instrumentDB.instNamed["kalimba"].instrumentId, 0.5 )
        self.instrumentStack = []

        # metronome
        page = NoteDB.Page( 1, local = False )
        self.metronomePage = self.noteDB.addPage( -1, page )
        self.metronome = False

        #-- Drums ---------------------------------------------
        self.drumLoopId = None
        # use dummy values for now
        self.drumFillin = Fillin( 2, 100, self.instrumentDB.instNamed["drum1kit"].instrumentId, 0, 1 )

        #-- Desktops ------------------------------------------
        self.curDesktop = None
        # copy preset desktops
        path = Config.TAM_TAM_ROOT+"/common/Resources/Desktops/"
        filelist = os.listdir( path )
        for file in filelist:
            shutil.copyfile( path+file, Config.SCRATCH_DIR+file )

        #-- Network -------------------------------------------
        self.network = Net.Network()
        self.network.addWatcher( self.networkStatusWatcher )
        self.network.connectMessage( Net.HT_SYNC_REPLY, self.processHT_SYNC_REPLY )
        self.network.connectMessage( Net.HT_TEMPO_UPDATE, self.processHT_TEMPO_UPDATE )
        self.network.connectMessage( Net.PR_SYNC_QUERY, self.processPR_SYNC_QUERY )
        self.network.connectMessage( Net.PR_TEMPO_QUERY, self.processPR_TEMPO_QUERY )
        self.network.connectMessage( Net.PR_REQUEST_TEMPO_CHANGE, self.processPR_REQUEST_TEMPO_CHANGE )

        # sync
        self.syncQueryStart = {}
        self.syncTimeout = None
        self.heartbeatLoop = self.csnd.loopCreate()
        self.syncBeats = 4
        self.csnd.loopSetNumTicks( self.syncBeats*Config.TICKS_PER_BEAT, self.heartbeatLoop )
        self.heartbeatStart = time.time()
        self.csnd.loopStart( self.heartbeatLoop )
        self.curBeat = 0
        self.beatWheelTimeout = gobject.timeout_add( 100, self.updateBeatWheel )

        # data packing classes
        self.packer = xdrlib.Packer()
        self.unpacker = xdrlib.Unpacker("")

        # handle forced networking
        if self.network.isHost():
            self.updateSync()
            self.syncTimeout = gobject.timeout_add( 1000, self.updateSync )
        elif self.network.isPeer():
            self.sendTempoQuery()
            self.syncTimeout = gobject.timeout_add( 1000, self.updateSync )

        #-- Final Set Up --------------------------------------
        self.setVolume( self.volume )
        self.setTempo( self.tempo )
        self.activity.toolbox.set_current_toolbar(1) # JamToolbar
        self.setDesktop( 0, True )


    #==========================================================

    def onActivate( self, arg ):
        pass
    def onDeactivate( self ):
        pass

    def onDestroy( self ):
        #clear up scratch folder
        path = Config.SCRATCH_DIR
        filelist = os.listdir( path )
        for file in filelist:
           os.remove( path+file )


    #==========================================================
    # Playback

    def onKeyPress( self, widget, event ):
        key = event.hardware_keycode

        if key in self.keyMap.keys():
            activate = True
            for block in self.keyMap[key]:
                if block.isActive():
                    activate = False
                    break
            if activate:
                for block in self.keyMap[key]:
                    if not block.isActive():
                        if   block.type == Block.Drum: self.desktop.activateDrum( block )
                        elif block.type == Block.Loop: self.desktop.activateLoop( block )
            else:
                for block in self.keyMap[key]:
                    if block.isActive():
                        if   block.type == Block.Drum: self.desktop.deactivateDrum( block )
                        elif block.type == Block.Loop: self.desktop.deactivateLoop( block )
            return

        if self.key_dict.has_key( key ): # repeated press
            return

        if Config.KEY_MAP_PIANO.has_key( key ):
            pitch = Config.KEY_MAP_PIANO[key]
            inst = self.instrumentDB.instId[self.instrument["id"]]

            if inst.kit: # drum kit
                if pitch in GenerationConstants.DRUMPITCH:
                    pitch = GenerationConstants.DRUMPITCH[pitch]
                csnote = self._playNote( key,
                                         36,
                                         self.instrument["amplitude"]*0.5, # trackVol*noteVol
                                         self.instrument["pan"],
                                         100,
                                         self.instrumentDB.instNamed[inst.kit[pitch]].instrumentId,
                                         self.instrument["reverb"] )
            else:
                if event.state == gtk.gdk.MOD1_MASK:
                    pitch += 5

                if inst.csoundInstrumentId == Config.INST_PERC: #Percussions resonance
                    duration = 60
                else:
                    duration = -1

                csnote = self._playNote( key,
                                         pitch,
                                         self.instrument["amplitude"]*0.5, # trackVol*noteVol
                                         self.instrument["pan"],
                                         duration,
                                         self.instrument["id"],
                                         self.instrument["reverb"] )

            if self.keyboardListener:
                self.keyboardListener.recordNote( csnote.pitch )
                self.recordingNote = True

    def onKeyRelease( self, widget, event ):
        key = event.hardware_keycode

        if self.key_dict.has_key( key ):
            self._stopNote( key )

        if self.recordingNote:
            if self.keyboardListener:
                self.keyboardListener.finishNote()
            self.recordingNote = False

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

        return self.key_dict[key]

    def _stopNote( self, key ):
        csnote = self.key_dict[key]
        if self.instrumentDB.instId[ csnote.instrumentId ].csoundInstrumentId == Config.INST_TIED:
            csnote.duration = .5
            csnote.decay = 0.7
            csnote.tied = False
            self.csnd.play(csnote, 0.3)
        del self.key_dict[key]

    def _updateInstrument( self, id, volume, pan = 0, reverb = 0 ):
        self.instrument = { "id":           id,
                            "amplitude":    volume,
                            "pan":          pan,
                            "reverb":       reverb }


    def pushInstrument( self, instrument ):
        self.instrumentStack.append( self.instrument )
        self.instrument = instrument

    def popInstrument( self ):
        self.instrument = self.instrumentStack.pop()

    def _playDrum( self, id, pageId, volume, reverb, beats, regularity, loopId = None, sync = True ):

        if loopId == None: # create new loop
            if sync: startTick = self.csnd.loopGetTick( self.heartbeatLoop )
            else:    startTick = 0
        else:              # update loop
            startTick = self.csnd.loopGetTick( loopId )
            self.csnd.loopDestroy( loopId )

        loopId = self.csnd.loopCreate()

        # TODO update track volume

        noteOnsets = []
        notePitchs = []
        for n in self.noteDB.getNotesByTrack( pageId, 0 ):
            n.pushState()
            noteOnsets.append( n.cs.onset )
            notePitchs.append( n.cs.pitch )
            n.cs.instrumentId = id
            n.cs.amplitude = volume * n.cs.amplitude # TODO remove me once track volume is working
            n.cs.reverbSend = reverb
            self.csnd.loopPlay( n, 1, loopId = loopId )    #add as active
            n.popState()

        ticks = self.noteDB.getPage( pageId ).ticks

        self.csnd.loopSetNumTicks( ticks, loopId )

        self.drumFillin.setLoopId( loopId )
        self.drumFillin.setProperties( self.tempo, self.instrumentDB.instId[id].name, volume, beats, reverb )
        self.drumFillin.unavailable( noteOnsets, notePitchs )

        self.drumFillin.play()

        while startTick > ticks:
            startTick -= ticks

        # sync to heartbeat
        if sync:
            beatTick = int(startTick) % Config.TICKS_PER_BEAT
            syncTick = self.csnd.loopGetTick( self.heartbeatLoop ) % Config.TICKS_PER_BEAT
            if beatTick > syncTick:
                if beatTick - syncTick < syncTick + Config.TICKS_PER_BEAT - beatTick:
                    startTick = (int(startTick)//Config.TICKS_PER_BEAT)*Config.TICKS_PER_BEAT + syncTick
                else:
                    startTick = (1 + int(startTick)//Config.TICKS_PER_BEAT)*Config.TICKS_PER_BEAT + syncTick
            else:
                if syncTick - beatTick < beatTick + Config.TICKS_PER_BEAT - syncTick:
                    startTick = (int(startTick)//Config.TICKS_PER_BEAT)*Config.TICKS_PER_BEAT + syncTick
                else:
                    startTick = (-1 + int(startTick)//Config.TICKS_PER_BEAT)*Config.TICKS_PER_BEAT + syncTick

            if startTick >= ticks:
                startTick -= ticks
            elif startTick < 0:
                startTick += ticks

        self.csnd.loopSetTick( startTick, loopId )

        if not self.paused:
            self.csnd.loopStart( loopId )

        return loopId

    def _stopDrum( self, loopId ):
        self.drumFillin.stop()
        self.csnd.loopDestroy( loopId )

    def _playLoop( self, id, volume, reverb, tune, loopId = None, force = False, sync = True ):
        if loopId == None: # create new loop
            if sync: startTick = self.csnd.loopGetTick( self.heartbeatLoop )
            else:    startTick = 0
        else:              # update loop
            startTick = self.csnd.loopGetTick( loopId )
            self.csnd.loopDestroy( loopId )

        loopId = self.csnd.loopCreate()

        # TODO update track volume

        inst = self.instrumentDB.instId[id]

        offset = 0
        for page in tune:
            for n in self.noteDB.getNotesByTrack( page, 0 ):
                n.pushState()
                n.cs.instrumentId = id
                n.cs.amplitude = volume * n.cs.amplitude # TODO remove me once track volume is working
                n.cs.reverbSend = reverb
                if inst.kit: # drum kit
                    if n.cs.pitch in GenerationConstants.DRUMPITCH:
                        n.cs.pitch = GenerationConstants.DRUMPITCH[n.cs.pitch]
                n.cs.onset += offset
                self.csnd.loopPlay( n, 1, loopId = loopId )
                n.popState()
            for n in self.noteDB.getNotesByTrack( page, 1 ): # metronome track
                self.csnd.loopPlay( n, 1, loopId = loopId )
            offset += self.noteDB.getPage(page).ticks

        self.csnd.loopSetNumTicks( offset, loopId )

        while startTick > offset:
            startTick -= offset

        # sync to heartbeat
        if sync:
            beatTick = startTick % Config.TICKS_PER_BEAT
            syncTick = self.csnd.loopGetTick( self.heartbeatLoop ) % Config.TICKS_PER_BEAT
            if beatTick > syncTick:
                if beatTick - syncTick < syncTick + Config.TICKS_PER_BEAT - beatTick:
                    startTick = (int(startTick)//Config.TICKS_PER_BEAT)*Config.TICKS_PER_BEAT + syncTick
                else:
                    startTick = (1 + int(startTick)//Config.TICKS_PER_BEAT)*Config.TICKS_PER_BEAT + syncTick
            else:
                if syncTick - beatTick < beatTick + Config.TICKS_PER_BEAT - syncTick:
                    startTick = (int(startTick)//Config.TICKS_PER_BEAT)*Config.TICKS_PER_BEAT + syncTick
                else:
                    startTick = (-1 + int(startTick)//Config.TICKS_PER_BEAT)*Config.TICKS_PER_BEAT + syncTick

            if startTick >= offset:
                startTick -= offset
            elif startTick < 0:
                startTick += offset

        self.csnd.loopSetTick( startTick, loopId )

        if not self.paused or force:
            self.csnd.loopStart( loopId )

        return loopId

    def _stopLoop( self, loopId ):
        self.csnd.loopDestroy( loopId )

    def addMetronome( self, page, period ):
        self.noteDB.deleteNotesByTrack( [ page ], [ 1 ] )

        baseCS = CSoundNote( 0,    # onset
                             36,   # pitch
                             0.2,  # amplitude
                             0.5,  # pan
                             100,  # duration
                             0,    # track
                             self.instrumentDB.instNamed["drum1hatpedal"].instrumentId,
                             reverbSend = 0.5,
                             tied = True,
                             mode = 'mini' )

        stream = []
        offset = 0

        for b in range( self.noteDB.getPage( page ).beats ):
            cs = baseCS.clone()
            cs.instrumentId = self.instrumentDB.instNamed["drum1hatshoulder"].instrumentId
            cs.amplitude = 0.5
            cs.onset += offset

            stream.append( cs )

            onset = period
            while onset < Config.TICKS_PER_BEAT:
                cs = baseCS.clone()
                cs.onset = onset + offset
                stream.append( cs )
                onset += period

            offset += Config.TICKS_PER_BEAT

        self.noteDB.addNotes( [ page, 1, len(stream) ] + stream + [ -1 ] )

    def removeMetronome( self, page ):
        self.noteDB.deleteNotesByTrack( [ page ], [ 1 ] )

    def setPaused( self, paused ):
        if self.paused == paused:
            return

        loops = self.desktop.getLoopIds()

        if self.paused: # unpause
            self.paused = False
            for loop in loops:
                self.csnd.loopStart( loop )
        else:           # pause
            self.paused = True
            for loop in loops:
                self.csnd.loopPause( loop )

    def setStopped( self ):
        for drum in list(self.desktop.drums):
            self.desktop.deactivateDrum(drum)

        for loop in list(self.desktop.loops): # we copy the list using the list() method
            self.desktop.deactivateLoop(loop)



    #==========================================================
    # Generate

    def _generateDrumLoop( self, instrumentId, beats, regularity, reverb, pageId = -1 ):
        def flatten(ll):
            rval = []
            for l in ll:
                rval += l
            return rval

        notes = flatten( generator( self.instrumentDB.instId[instrumentId].name, beats, 0.8, regularity, reverb) )

        if pageId == -1:
            page = Page( beats )
            pageId = self.noteDB.addPage( -1, page )
        else:
            self.noteDB.deleteNotesByTrack( [ pageId ], [ 0 ] )

        if len(notes):
            self.noteDB.addNotes( [ pageId, 0, len(notes) ] + notes + [-1] )

        return pageId

    def _generateTrack( self, instrumentId, page, track, parameters, algorithm ):
        dict = { track: { page: self.noteDB.getCSNotesByTrack( page, track ) } }
        instruments = { page: [ self.instrumentDB.instId[instrumentId].name for i in range(Config.NUMBER_OF_TRACKS) ] }
        beatsOfPages = { page: self.noteDB.getPage(page).beats }

        algorithm( parameters,
                   [ 0.5 for i in range(Config.NUMBER_OF_TRACKS) ],
                   instruments,
                   self.tempo,
                   beatsOfPages,
                   [ track ],
                   [ page ],
                   dict,
                   4)

        # filter & fix input ...WTF!?
        for track in dict:
            for page in dict[track]:
                for note in dict[track][page]:
                    intdur = int(note.duration)
                    note.duration = intdur
                    note.pageId = page
                    note.trackId = track

        # prepare the new notes
        newnotes = []
        for tid in dict:
            for pid in dict[tid]:
                newnotes += dict[tid][pid]

        # delete the notes and add the new
        self.noteDB.deleteNotesByTrack( [ page ], [ track ] )

        self.noteDB.addNotes(
            [ page, track, len(dict[track][page]) ]
          + dict[track][page]
          + [ -1 ] )


    #==========================================================
    # Mic recording
    def micRec(self, widget, mic):
        os.system('rm ' + Config.SNDS_DIR + '/' + mic)
        self.csnd.inputMessage("i5600 0 4")
        (s1,o1) = commands.getstatusoutput("arecord -f S16_LE -t wav -r 16000 -d 4 " + Config.SNDS_DIR + "/tempMic.wav")
        (s2, o2) = commands.getstatusoutput("csound " + Config.FILES_DIR + "/crop.csd")
        (s3, o3) = commands.getstatusoutput("mv " + Config.SNDS_DIR + "/micTemp " + Config.SNDS_DIR + "/" + mic)
        (s4, o4) = commands.getstatusoutput("rm " + Config.SNDS_DIR + "/tempMic.wav")
        self.csnd.load_mic_instrument(mic)


    #==========================================================
    # Loop Settings
    def loopSettingsChannel(self, channel, value):
        self.csnd.setChannel(channel, value)

    def loopSettingsPlayStop(self, state, loop):
        if not state:
            if loop:
                self.loopSettingsPlaying = True
                self.csnd.inputMessage(Config.CSOUND_PLAY_LS_NOTE % 5022)
            else:
                self.csnd.inputMessage(Config.CSOUND_PLAY_LS_NOTE % 5023)
        else:
            if loop:
                self.loopSettingsPlaying = False
                self.csnd.inputMessage(Config.CSOUND_STOP_LS_NOTE)

    def load_ls_instrument(self, soundName):
        self.csnd.load_ls_instrument(soundName)

    #==========================================================
    # Get/Set

    def getVolume( self ):
        return self.volume

    def setVolume( self, volume ):
        self.jamToolbar.volumeSlider.set_value( volume )

    def _setVolume( self, volume ):
        self.volume = volume
        self.csnd.setMasterVolume( self.volume*100 ) # csnd expects a range 0-100 for now

    def getTempo( self ):
        return self.tempo

    def setTempo( self, tempo, quiet = False ):
        self.jamToolbar.setTempo( tempo, quiet )

    def _setTempo( self, tempo, propagate = True ):
        if self.network.isHost() or self.network.isOffline():
            t = time.time()
            percent = self.heartbeatElapsed() / self.beatDuration

        self.tempo = tempo
        self.beatDuration = 60.0/self.tempo
        self.ticksPerSecond = Config.TICKS_PER_BEAT*self.tempo/60.0
        self.csnd.setTempo( self.tempo )

        if self.network.isHost() or self.network.isOffline():
            self.heatbeatStart = t - percent*self.beatDuration
            self.updateSync()
            self.sendTempoUpdate()

    def getInstrument( self ):
        return self.instrument

    def getDesktop( self ):
        return self.desktop

    def _clearDesktop( self, save = True ):
        if self.curDesktop == None:
            return

        if save:
            self._saveDesktop()

        self.desktop._clearDesktop()

        self.curDesktop = None

    def setDesktop( self, desktop, force = False ):
        radiobtn = self.desktopToolbar.getDesktopButton( desktop )
        if force and radiobtn.get_active():
            self._setDesktop( desktop )
        else:
            radiobtn.set_active( True )

    def _setDesktop( self, desktop ):
        self._clearDesktop()

        self.curDesktop = desktop

        TTTable = ControlStream.TamTamTable( self.noteDB, jam = self )

        filename = self.getDesktopScratchFile( self.curDesktop )
        try:
            stream = open( filename, "r" )
            TTTable.parseFile( stream )
            stream.close()
        except IOError, (errno, strerror):
            if Config.DEBUG > 3: print "IOError:: _setDesktop:", errno, strerror

    def getInstrumentImage( self, id, active = False ):
        if active: return self.instrumentImageActive[id]
        else:      return self.instrumentImage[id]

    def getKeyImage( self, key, active = False ):
        if active: return self.keyImageActive[key]
        else:      return self.keyImage[key]

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

    def setKeyboardListener( self, listener ):
        self.keyboardListener = listener

    def mapKey( self, key, block, oldKey = None ):
        if oldKey != None and block in self.keyMap[oldKey]:
            self.keyMap[oldKey].remove( block )

        if key == None:
            return

        if key not in self.keyMap.keys():
            self.keyMap[key] = []

        if block not in self.keyMap[key]:
            self.keyMap[key].append( block )

    #==========================================================
    # Pixmaps

    def prepareInstrumentImage( self, id, img_path ):
        win = gtk.gdk.get_default_root_window()
        try:
            pix = gtk.gdk.pixbuf_new_from_file( img_path )
        except:
            if Config.DEBUG >= 5: print "JamMain:: file does not exist: " + img_path
            pix = gtk.gdk.pixbuf_new_from_file( Config.IMAGE_ROOT + "generic.png" )
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

    def prepareKeyImage( self, key ):
        win = gtk.gdk.get_default_root_window()
        pangolayout = self.create_pango_layout( _(self.valid_shortcuts[key]) )
        fontDesc = pango.FontDescription( "bold" )
        pangolayout.set_font_description( fontDesc )
        extents = pangolayout.get_pixel_extents()
        x = ( Block.Block.KEYSIZE - extents[1][2] ) // 2
        y = ( Block.Block.KEYSIZE - extents[1][3] ) // 2

        pixmap = gtk.gdk.Pixmap( win, Block.Block.KEYSIZE, Block.Block.KEYSIZE )
        self.gc.foreground = self.colors["Border_Inactive"]
        pixmap.draw_rectangle( self.gc, True, 0, 0, Block.Block.KEYSIZE, Block.Block.KEYSIZE )
        self.gc.foreground = self.colors["Bg_Inactive"]
        pixmap.draw_layout( self.gc, x, y, pangolayout )
        self.keyImage[key] = pixmap

        pixmap = gtk.gdk.Pixmap( win, Block.Block.KEYSIZE, Block.Block.KEYSIZE )
        self.gc.foreground = self.colors["Border_Active"]
        pixmap.draw_rectangle( self.gc, True, 0, 0, Block.Block.KEYSIZE, Block.Block.KEYSIZE )
        self.gc.foreground = self.colors["Bg_Active"]
        pixmap.draw_layout( self.gc, x, y, pangolayout )
        self.keyImageActive[key] = pixmap

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

    #==========================================================
    # Load/Save

    def _saveDesktop( self ):
        if self.curDesktop == None:
            return

        filename = self.getDesktopScratchFile( self.curDesktop )
        if os.path.isfile( filename ):
           os.remove( filename )

        try:
            scratch = open( filename, "w" )
            stream = ControlStream.TamTamOStream(scratch)

            self.noteDB.dumpToStream( stream, True )
            self.desktop.dumpToStream( stream )
            stream.sync_beats( self.syncBeats )

            scratch.close()
        except IOError, (errno, strerror):
            if Config.DEBUG > 3: print "IOError:: _saveDesktop:", errno, strerror

    def getDesktopScratchFile( self, i ):
        return Config.SCRATCH_DIR+"desktop%d" % i

    def handleJournalLoad( self, filepath ):

        self._clearDesktop( False )

        TTTable = ControlStream.TamTamTable( self.noteDB, jam = self )

        try:
            stream = open( filepath, "r" )
            TTTable.parseFile( stream )
            stream.close()

            self.setVolume( TTTable.masterVolume )
            self.setTempo( TTTable.tempo )

        except IOError, (errno, strerror):
            if Config.DEBUG > 3: print "IOError:: handleJournalLoad:", errno, strerror

    def handleJournalSave( self, filepath ):

        self._saveDesktop()

        try:
            streamF = open( filepath, "w" )
            stream = ControlStream.TamTamOStream( streamF )

            for i in range(10):
                desktop_file = self.getDesktopScratchFile( i )
                stream.desktop_store( desktop_file, i )

            stream.desktop_set( self.curDesktop )

            stream.master_vol( self.volume )
            stream.tempo( self.tempo )

            streamF.close()

        except IOError, (errno, strerror):
            if Config.DEBUG > 3: print "IOError:: handleJournalSave:", errno, strerror

    #==========================================================
    # Network

    #-- Activity ----------------------------------------------

    def shared( self, activity ):
        if Config.DEBUG: print "miniTamTam:: successfully shared, start host mode"
        self.activity._shared_activity.connect( "buddy-joined", self.buddy_joined )
        self.activity._shared_activity.connect( "buddy-left", self.buddy_left )
        self.network.setMode( Net.MD_HOST )
        self.updateSync()
        self.syncTimeout = gobject.timeout_add( 1000, self.updateSync )

    def joined( self, activity ):
        if Config.DEBUG:
            print "miniTamTam:: joined activity!!"
            for buddy in self.activity._shared_activity.get_joined_buddies():
                print buddy.props.ip4_address

    def buddy_joined( self, activity, buddy ):
        if Config.DEBUG:
            print "buddy joined " + str(buddy)
            try:
                print buddy.props.ip4_address
            except:
                print "bad ip4_address"
        if self.network.isHost():
            if buddy == self.xoOwner:
                return
            if buddy.props.ip4_address:
                self.network.introducePeer( buddy.props.ip4_address )
            else:
                print "miniTamTam:: new buddy does not have an ip4_address!!"

    def buddy_left( self, activity, buddy):
        if Config.DEBUG: print "buddy left"

    #def joined( self, activity ):
    #    if Config.DEBUG: print "miniTamTam:: successfully joined, wait for host"
    #    self.net.waitForHost()

    #-- Senders -----------------------------------------------

    def sendSyncQuery( self ):
        self.packer.pack_float(random.random())
        hash = self.packer.get_buffer()
        self.packer.reset()
        self.syncQueryStart[hash] = time.time()
        self.network.send( Net.PR_SYNC_QUERY, hash)

    def sendTempoUpdate( self ):
        self.packer.pack_int(self.tempo)
        self.network.sendAll( Net.HT_TEMPO_UPDATE, self.packer.get_buffer() )
        self.packer.reset()

    def sendTempoQuery( self ):
        self.network.send( Net.PR_TEMPO_QUERY )

    def requestTempoChange( self, val ):
        self.packer.pack_int(val)
        self.network.send( Net.PR_REQUEST_TEMPO_CHANGE, self.packer.get_buffer() )
        self.packer.reset()

    #-- Handlers ----------------------------------------------

    def networkStatusWatcher( self, mode ):
        if mode == Net.MD_OFFLINE:
            if self.syncTimeout:
                gobject.source_remove( self.syncTimeout )
                self.syncTimeout = None
        if mode == Net.MD_PEER:
            self.updateSync()
            if not self.syncTimeout:
                self.syncTimeout = gobject.timeout_add( 1000, self.updateSync )
            self.sendTempoQuery()

    def processHT_SYNC_REPLY( self, sock, message, data ):
        t = time.time()
        hash = data[0:4]
        latency = t - self.syncQueryStart[hash]
        self.unpacker.reset(data[4:8])
        nextBeat = self.unpacker.unpack_float()
        #print "mini:: got sync: next beat in %f, latency %d" % (nextBeat, latency*1000)
        self.heartbeatStart = t + nextBeat - self.beatDuration - latency/2
        self.correctSync()
        self.syncQueryStart.pop(hash)

    def processHT_TEMPO_UPDATE( self, sock, message, data ):
        self.unpacker.reset(data)
        val = self.unpacker.unpack_int()
        self.setTempo( val, True )
        self.sendSyncQuery()

    def processPR_SYNC_QUERY( self, sock, message, data ):
        self.packer.pack_float(self.nextHeartbeat())
        self.network.send( Net.HT_SYNC_REPLY, data + self.packer.get_buffer(), sock )
        self.packer.reset()

    def processPR_TEMPO_QUERY( self, sock, message, data ):
        self.packer.pack_int(self.tempo)
        self.network.send( Net.HT_TEMPO_UPDATE, self.packer.get_buffer(), to = sock )
        self.packer.reset()

    def processPR_REQUEST_TEMPO_CHANGE( self, sock, message, data ):
        if self.jamToolbar.tempoSliderActive:
            return
        self.unpacker.reset(data)
        val = self.unpacker.unpack_int()
        self.setTempo( val )

    #==========================================================
    # Sync

    def setSyncBeats( self, beats ):
        self.playbackToolbar.setSyncBeats( beats )

    def _setSyncBeats( self, beats ):
        if beats == self.syncBeats:
            return

        self.syncBeats = beats

        ticks = beats * Config.TICKS_PER_BEAT

        curTick = self.csnd.loopGetTick( self.heartbeatLoop )

        self.csnd.loopSetNumTicks( ticks, self.heartbeatLoop )
        while curTick > ticks:
            curTick -= ticks

        self.csnd.loopSetTick( curTick, self.heartbeatLoop )

        self.updateSync()


    def _setBeat( self, beat ):
        curTick = self.csnd.loopGetTick( self.heartbeatLoop )
        beatTick = curTick % Config.TICKS_PER_BEAT

        newTick = beat*Config.TICKS_PER_BEAT + beatTick

        self.csnd.adjustTick( newTick - curTick )

    def updateBeatWheel( self ):
        curTick = self.csnd.loopGetTick( self.heartbeatLoop )
        self.curBeat = int( curTick ) // Config.TICKS_PER_BEAT
        self.playbackToolbar.updateBeatWheel( self.curBeat )
        return True

    def nextHeartbeat( self ):
        delta = time.time() - self.heartbeatStart
        return self.beatDuration - (delta % self.beatDuration)

    def nextHeartbeatInTicks( self ):
        delta = time.time() - self.heartbeatStart
        next = self.beatDuration - (delta % self.beatDuration)
        return self.ticksPerSecond*next

    def heartbeatElapsed( self ):
        delta = time.time() - self.heartbeatStart
        return delta % self.beatDuration

    def heartbeatElapsedTicks( self ):
        delta = time.time() - self.heartbeatStart
        return self.ticksPerSecond*(delta % self.beatDuration)

    def updateSync( self ):
        if self.network.isOffline():
            return False
        elif self.network.isWaiting():
            return True
        elif self.network.isHost():
            self.correctSync()
        else:
            self.sendSyncQuery()
        return True

    def correctSync( self ):
        curTick = self.csnd.loopGetTick( self.heartbeatLoop ) % Config.TICKS_PER_BEAT
        curTicksIn = curTick % Config.TICKS_PER_BEAT
        ticksIn = self.heartbeatElapsedTicks()
        err = curTicksIn - ticksIn
        if err > Config.TICKS_PER_BEAT_DIV2:
            err -= Config.TICKS_PER_BEAT
        elif err < -Config.TICKS_PER_BEAT_DIV2:
            err += Config.TICKS_PER_BEAT
        correct = curTick - err
        if correct > Config.TICKS_PER_BEAT:
            correct -= Config.TICKS_PER_BEAT
        elif correct < 0:
            correct += Config.TICKS_PER_BEAT
        #print "correct:: %f ticks, %f ticks in, %f expected, %f err, correct %f" % (curTick, curTicksIn, ticksIn, err, correct)
        if abs(err) > 0.25:
            self.csnd.adjustTick(-err)
