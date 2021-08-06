import pygtk
pygtk.require( '2.0' )
import gtk
import gobject
import os
import random
import time
import xdrlib
import commands

from gettext import gettext as _gettext

from types import *
from math import sqrt
from Util.NoteDB import PARAMETER

import Util.Network
Net = Util.Network # convinience assignment

import Config

from Util.ThemeWidgets import *
from Util.CSoundNote import CSoundNote
from Util import NoteDB
from Util.NoteDB import Note
from Util.CSoundClient import new_csound_client

from Fillin import Fillin
from KeyboardStandAlone import KeyboardStandAlone
from MiniSequencer import MiniSequencer
from Loop import Loop
from RythmGenerator import *
from SynthLab.SynthLabWindow import SynthLabWindow
from Util.Trackpad import Trackpad
from Util.InstrumentPanel import InstrumentPanel

Tooltips = Config.Tooltips

from SubActivity import SubActivity
    
class miniTamTamMain(SubActivity):
    
    def __init__(self, activity, set_mode):
        SubActivity.__init__(self, set_mode)

        self.activity = activity

        self.set_border_width(Config.MAIN_WINDOW_PADDING)

        self.firstTime = False
        self.csnd = new_csound_client()
        self.timeout_ms = 50
        self.instVolume = 50
        self.drumVolume = 0.5
        self.instrument = 'ocarina'
        self.regularity = 0.75
        self.beat = 4
        self.reverb = 0.
        self.tempo = Config.PLAYER_TEMPO
        self.beatDuration = 60.0/self.tempo
        self.ticksPerSecond = Config.TICKS_PER_BEAT*self.tempo/60.0
        self.rythmInstrument = 'drum1kit'
        self.muteInst = False
        self.drumFillin = Fillin( self.beat, self.tempo, self.rythmInstrument, self.reverb, self.drumVolume )
        self.sequencer= MiniSequencer(self.recordStateButton)
        self.loop = Loop(self.beat, sqrt( self.instVolume*0.01 ))
        self.csnd.loopSetTempo(self.tempo)
        self.noteList = []
        time.sleep(0.001)
        self.trackpad = Trackpad( self )
        for i in range(21):
            self.csnd.setTrackVolume( 100, i )

        self.volume = 150
        self.csnd.setMasterVolume(self.volume)
        self.sequencer.beat = self.beat
        self.loop.beat = self.beat 
        self.tooltips = gtk.Tooltips()
        
        self.masterVBox = gtk.VBox()
        self.mainWindowBox = gtk.HBox()
        self.leftBox = gtk.VBox()
        self.leftBox.set_size_request(950,-1)
        self.rightBox = gtk.VBox()
        self.mainWindowBox.pack_start(self.leftBox,False,False)
        self.mainWindowBox.pack_start(self.rightBox,True,True)
        self.masterVBox.pack_start(self.mainWindowBox)
        self.add(self.masterVBox)
       
        self.enableKeyboard()
        self.setInstrument(self.instrument)
        
        self.drawInstrumentButtons()
        self.drawSliders()
        self.drawGeneration()
        self.show_all()
        if 'a good idea' == True:
            self.playStartupSound()

        self.synthLabWindow = None

        self.regenerate()

        self.heartbeatStart = time.time()
        self.syncQueryStart = {}
        self.syncTimeout = None

        self.network = Net.Network()
        self.network.addWatcher( self.networkStatusWatcher )
        self.network.connectMessage( Net.HT_SYNC_REPLY, self.processHT_SYNC_REPLY )
        self.network.connectMessage( Net.HT_TEMPO_UPDATE, self.processHT_TEMPO_UPDATE )
        self.network.connectMessage( Net.PR_SYNC_QUERY, self.processPR_SYNC_QUERY )
        self.network.connectMessage( Net.PR_TEMPO_QUERY, self.processPR_TEMPO_QUERY )

        # data packing classes
        self.packer = xdrlib.Packer()
        self.unpacker = xdrlib.Unpacker("")
    
        #-- handle forced networking ---------------------------------------
        if self.network.isHost():
            self.updateSync()
            self.syncTimeout = gobject.timeout_add( 1000, self.updateSync )
        elif self.network.isPeer():
            self.sendTempoQuery()
            self.syncTimeout = gobject.timeout_add( 1000, self.updateSync )
        #-----------------------------------------------------------------