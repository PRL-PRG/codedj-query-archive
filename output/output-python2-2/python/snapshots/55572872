#!/usr/bin/env python
import pygtk
pygtk.require( '2.0' )
import gtk

import os, signal,math

import Config
from Util.ThemeWidgets import *
from sugar.graphics.toggletoolbutton import ToggleToolButton

from Util.CSoundClient import new_csound_client
from Util import ControlStream
from Util import NoteDB
from Util.NoteDB import Note
from SubActivity import SubActivity
from gettext import gettext as _
    
class Welcome(SubActivity):
    
    def __init__(self, activity, set_mode):
        SubActivity.__init__(self, set_mode)
        self.csnd = new_csound_client()
        self.noteDB = NoteDB.NoteDB()
        first = self.noteDB.addPage( -1, NoteDB.Page(4) )
        self.tooltips = gtk.Tooltips()
        self.activity = activity
        
        self.helpButton = self.activity.activity_toolbar.helpButton = ToggleToolButton('tam-help')
        self.activity.activity_toolbar.insert(self.activity.activity_toolbar.helpButton,2)
        self.activity.activity_toolbar.helpButton.show()
        self.activity.activity_toolbar.helpButton.set_tooltip(_('Help'))
        self.activity.activity_toolbar.helpButton.connect("toggled", self.handleHelp)


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
            if activity == 'mini':
                self.tooltips.set_tip(actBtn,'TamTam Jam')
            elif activity == 'edit':
                self.tooltips.set_tip(actBtn,'TamTam Edit')
            elif activity == 'synth':
                self.tooltips.set_tip(actBtn,'TamTam SynthLab')

            
        title = gtk.Image()
        title.set_from_file(Config.IMAGE_ROOT + 'TamTam.png')

        buttonBox = gtk.HBox()

        loadButton = ImageButton(Config.IMAGE_ROOT + 'load.png')
        loadButton.connect("clicked", self.handleLoad, None)
        buttonBox.pack_start(loadButton, False, False, 275)
        self.tooltips.set_tip(loadButton,'Load TamTamEdit song')

        self.playMode = "TAM"
        self.playStopButton = ImageToggleButton(Config.IMAGE_ROOT + 'miniplay.png', Config.IMAGE_ROOT + 'stop.png')
        self.tooltips.set_tip(self.playStopButton,"Play loaded song")
        self.playStopButton.connect('button-press-event' , self.handlePlayButton)
        buttonBox.pack_start(self.playStopButton, False, False, 275)
 
        
        actVBox.pack_start(actHBox,False,False, 200)
        actVBox.pack_start(title,False,False)
        #actVBox.pack_start(buttonBox, False, False, 100)
        self.add(actVBox)

        self.helpWindow = gtk.Window(gtk.WINDOW_POPUP)
        self.helpWindow.modify_bg(gtk.STATE_NORMAL, gtk.gdk.color_parse("#FFF"))
        self.helpWindow.move( 0, 75 )
        self.helpWindow.resize( 1200, 825 )
        self.helpWindow.set_modal(True)
        self.helpWindow.add_events( gtk.gdk.BUTTON_PRESS_MASK )
        self.helpWindow.connect("button-release-event", lambda w,e: self.helpButton.set_active(False))

        self.show_all()

    def handleLoad(self, widget, data=None):
        chooser = gtk.FileChooserDialog(
                title='Load Tune',
                action=gtk.FILE_CHOOSER_ACTION_OPEN,
                buttons=(gtk.STOCK_CANCEL,gtk.RESPONSE_CANCEL,gtk.STOCK_OPEN,gtk.RESPONSE_OK))

        filter = gtk.FileFilter()
        filter.add_pattern('*.tam')
        filter.add_pattern('*.ogg')
        chooser.set_filter(filter)
        chooser.set_current_folder(Config.TUNE_DIR)

        for f in chooser.list_shortcut_folder_uris():
            chooser.remove_shortcut_folder_uri(f)

        if chooser.run() == gtk.RESPONSE_OK:
            if self.playStopButton.get_active():
                self.playStopButton.set_active( False )

            filename = chooser.get_filename()
            if filename[-4:] == ".ogg":
                self.playMode = "OGG"
                self.playFile = filename
            else: 
                self.playMode = "TAM"
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
        if self.playMode == "OGG":
            if widget.get_active() == True:
                os.kill( self.playPID, signal.SIGKILL )
                if self.csnd:
                    self.csnd.connect(True)
            else:
                if self.csnd:
                    self.csnd.connect(False)
                self.playPID = os.spawnl( os.P_NOWAIT, "/usr/bin/gst-launch-0.10", "gst-launch-0.10", "filesrc", "location="+self.playFile, "!", "oggdemux", "!", "vorbisdec", "!", "audioconvert", "!", "osssink" )
        else:
            if widget.get_active() == True:
                self.csnd.loopPause()
            else:
                self.csnd.loopSetTick( 0 )
                self.csnd.loopStart()

    def handleHelp(self, widget):
        if widget.get_active():
            helpImg = gtk.Image()

            self.imglist = [ i for i in os.listdir(Config.IMAGE_ROOT) 
                    if i[0:8] == 'helpShow']
            self.imglist.sort()
            self.imgpos = 0
            def release(w,e,self):
                imglist = self.imglist
                imgpos = self.imgpos
                if e.button == 1:
                    self.imgpos = ( imgpos + 1 ) % len(imglist)
                else: 
                    self.imgpos = ( imgpos - 1 + len(imglist)) % len(imglist)
                helpImg.set_from_file( Config.IMAGE_ROOT + imglist[self.imgpos])
                return True
            helpImg.set_from_file(Config.IMAGE_ROOT  + self.imglist[self.imgpos])

            hbox = gtk.HBox()
            jj = gtk.EventBox()
            jj.add(helpImg)
            jj.connect("button-release-event", release, self)
            hbox.pack_start( jj, True, False )
            self.helpWindow.add( hbox )
            self.helpWindow.show_all()
        else:
            self.helpWindow.hide()
            self.helpWindow.remove( self.helpWindow.get_children()[0] )
        
    def onActivityBtnClicked(self, widget, data):
        widget.event( gtk.gdk.Event( gtk.gdk.LEAVE_NOTIFY )  ) # fake the leave event
        self.set_mode(data)

    def onActivate(self, arg):
        self.show_all()
        self.helpButton.show()

    def onDeactivate(self):
        if self.playStopButton.get_active():
            self.playStopButton.set_active(False)

        self.helpButton.hide()
