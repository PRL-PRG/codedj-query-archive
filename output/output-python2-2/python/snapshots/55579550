
import pygtk
pygtk.require( '2.0' )
import gtk

from SubActivity import SubActivity

import Config

from Jam.Desktop import Desktop
import Jam.Picker as Picker
    
from Util.CSoundNote import CSoundNote
from Util.CSoundClient import new_csound_client
from Fillin import Fillin
from RythmGenerator import generator
from Util.NoteDB import Note

from math import sqrt

class JamMain(SubActivity):
    
    def __init__(self, activity, set_mode):
        SubActivity.__init__(self, set_mode)

        self.activity = activity

        #-- initial settings ----------------------------------
        self.tempo = Config.PLAYER_TEMPO
        self.volume = 50
        self.reverb = 0
        
        self.csnd = new_csound_client()
        for i in range(1,9):
            self.csnd.setTrackVolume( 100, i )
        self.csnd.setMasterVolume( self.volume )
        self.csnd.setTempo( self.tempo )

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
        
        #-- Keyboard ------------------------------------------
        self.key_dict = {}
        self.nextTrack = 1

        # default instrument
        self._updateInstrument( Config.INSTRUMENTS["kalimba"].instrumentId, 0.5 )

        #-- Drums ---------------------------------------------
        # use dummy values for now
        self.drumFillin = Fillin( 2, 100, Config.INSTRUMENTS["drum1kit"].instrumentId, self.reverb, 1 )

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
                self._playNote( key, 
                                36, 
                                self.instrument["amplitude"], 
                                self.instrument["pan"], 
                                100, 
                                inst.kit[pitch],
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

