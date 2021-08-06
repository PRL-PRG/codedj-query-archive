#!/usr/bin/env python
import pygtk
pygtk.require( '2.0' )
import gtk

import Config
from Util.ThemeWidgets import *

from Util.CSoundClient import new_csound_client
from Util import ControlStream
from Util import NoteDB
from Util.NoteDB import Note
from miniTamTam.RythmGenerator import generator
from SubActivity import SubActivity
    
class Welcome(SubActivity):
    
    def __init__(self, set_mode):
        SubActivity.__init__(self, set_mode)
        self.csnd = new_csound_client()
        self.noteDB = NoteDB.NoteDB()
        first = self.noteDB.addPage( -1, NoteDB.Page(4) )

        actVBox = RoundVBox(fillcolor = Config.WS_BCK_COLOR, bordercolor = Config.WS_BCK_COLOR, radius = Config.PANEL_RADIUS)
        actHBox = gtk.HBox()
        
        for activity in ['mini','edit','synth']:
            actBtnBox = RoundVBox(fillcolor = Config.WS_PANEL_COLOR, bordercolor = Config.WS_BCK_COLOR, radius = Config.PANEL_RADIUS)
            actBtnBox.set_size_request(200,200)
            actBtnBox.set_border_width(Config.PANEL_SPACING)
            actBtn = ImageButton(Config.IMAGE_ROOT + activity +'Tam.png' , Config.IMAGE_ROOT + activity +'TamDown.png', Config.IMAGE_ROOT + activity +'TamOver.png' )
            actBtn.connect('clicked', self.onActivityBtnClicked, activity)
            actBtnBox.pack_start(actBtn,True,False,0)
            actHBox.pack_start(actBtnBox,True,False,0)
            
        title = gtk.Image()
        title.set_from_file(Config.IMAGE_ROOT + 'TamTam.png')

        buttonBox = gtk.HBox()

        loadButton = ImageButton(Config.IMAGE_ROOT + 'load.png')
        loadButton.connect("clicked", self.handleLoad, None)
        buttonBox.pack_start(loadButton, False, False, 275)

        playStopButton = ImageToggleButton(Config.IMAGE_ROOT + 'miniplay.png', Config.IMAGE_ROOT + 'stop.png')
        playStopButton.connect('button-press-event' , self.handlePlayButton)
        buttonBox.pack_start(playStopButton, False, False, 275)
 
        
        actVBox.pack_start(actHBox,False,False, 200)
        actVBox.pack_start(title,False,False, 0)
        actVBox.pack_start(buttonBox, False, False, 100)
        self.add(actVBox)
        self.show_all()
        self.activate_count = 0

    def handleLoad(self, widget, data=None):
        chooser = gtk.FileChooserDialog(
                title='Load Tune',
                action=gtk.FILE_CHOOSER_ACTION_OPEN,
                buttons=(gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_OPEN,gtk.RESPONSE_OK))

        filter = gtk.FileFilter()
        filter.add_pattern('*.tam')
        chooser.set_filter(filter)
        chooser.set_current_folder(Config.TUNE_DIR)

        for f in chooser.list_shortcut_folder_uris():
            chooser.remove_shortcut_folder_uri(f)

        if chooser.run() == gtk.RESPONSE_OK:
            self.noteDB.deletePages( self.noteDB.pages.keys() )
            ifile = open(chooser.get_filename(), 'r')
            tuneStream = ControlStream.TamTamTable ( self.noteDB )
            tuneStream.parseFile(ifile)

            self.noteDB.deletePages( self.noteDB.tune[0:1])
            numticks = 0
            page_onset = {}
            notes = []
            for pid in self.noteDB.getTune():
                page_onset[pid] = numticks
                numticks += self.noteDB.getPage(pid).ticks
                notes += self.noteDB.getNotesByPage( pid )
 
            self.csnd.connect(True)
            self.csnd.loopClear()
            for n in notes:
                self.csnd.loopPlay(n, 1)
                self.csnd.loopUpdate(n, NoteDB.PARAMETER.ONSET, n.cs.onset + page_onset[n.page] , 1)
            self.csnd.loopSetNumTicks( numticks )
            self.csnd.loopSetTick( 0 )
            self.csnd.setMasterVolume(float(tuneStream.masterVolume))
            self.csnd.loopSetTempo(float(tuneStream.tempo))
            for i in range(len(tuneStream.tracks_volume)):
                self.csnd.setTrackVolume(float(tuneStream.tracks_volume[i]), i)
        self.csnd.loopPause()
        ifile.close()
        chooser.destroy() 

    def handlePlayButton(self, widget, data):
        if widget.get_active() == True:
            self.csnd.loopPause()
        else:
            self.csnd.loopSetTick( 0 )
            self.csnd.loopStart()
        
    def onActivityBtnClicked(self, widget, data):
        widget.event( gtk.gdk.Event( gtk.gdk.LEAVE_NOTIFY )  ) # fake the leave event
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
            density = 0.8
            regularity = 0.77
            reverb = 0.2
            for x in flatten( generator('drum3kit', beat, density, regularity, reverb) ):
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
