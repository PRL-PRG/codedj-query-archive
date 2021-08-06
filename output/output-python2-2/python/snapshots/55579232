#!/usr/bin/env python
import pygtk
pygtk.require( '2.0' )
import gtk

import Config
from Util.ThemeWidgets import *

from Util.CSoundClient import new_csound_client
from Util import NoteDB
from Util.NoteDB import Note
from miniTamTam.RythmGenerator import generator
from SubActivity import SubActivity
    
class Welcome(SubActivity):
    
    def __init__(self, set_mode):
        SubActivity.__init__(self, set_mode)
        
        actVBox = RoundVBox(fillcolor = Config.WS_BCK_COLOR, bordercolor = Config.WS_BCK_COLOR, radius = Config.PANEL_RADIUS)
        actHBox = gtk.HBox()
        
        for activity in ['mini','edit','type','synth']:
            actBtnBox = RoundVBox(fillcolor = Config.WS_PANEL_COLOR, bordercolor = Config.WS_BCK_COLOR, radius = Config.PANEL_RADIUS)
            actBtnBox.set_size_request(200,200)
            actBtnBox.set_border_width(Config.PANEL_SPACING)
            actBtn = ImageButton(Config.IMAGE_ROOT + activity +'Tam.png')
            actBtn.connect('clicked', self.onActivityBtnClicked, activity)
            actBtnBox.pack_start(actBtn,True,False,0)
            actHBox.pack_start(actBtnBox,True,False,0)
            
        title = gtk.Image()
        title.set_from_file(Config.IMAGE_ROOT + 'TamTam.png')
        
        actVBox.pack_start(actHBox,False,False, 100)
        actVBox.pack_start(title,False,False, 30)
        self.add(actVBox)
        self.show_all()
        self.activate_count = 0
        
    def onActivityBtnClicked(self, widget, data):
        self.set_mode(data)

    def onActivate(self, arg):
        def flatten(ll):
            rval = []
            for l in ll:
                rval += l
            return rval
        if self.activate_count == 0:
            i = 0
            csnd = new_csound_client()
            beat = 8
            regularity = 0.77
            reverb = 0.2
            for x in flatten( generator('drum3kit', beat, regularity, reverb) ):
                n = Note(0, x.trackId, i, x)
                i = i + 1
                csnd.loopPlay(n,1) # add as active
            csnd.loopSetNumTicks( beat * Config.TICKS_PER_BEAT)
            csnd.loopSetTick(0)
            csnd.loopStart()
        self.activate_count = self.activate_count + 1
        self.show_all()

    def onDeactivate(self):
        if (self.activate_count == 1):
            csnd = new_csound_client()
            csnd.loopPause()
            csnd.loopClear()
