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
        
        actVBox.pack_start(actHBox,False,False, 200)
        actVBox.pack_start(title,False,False)
        self.add(actVBox)

        self.helpWindow = gtk.Window(gtk.WINDOW_POPUP)
        self.helpWindow.modify_bg(gtk.STATE_NORMAL, gtk.gdk.color_parse("#FFF"))
        self.helpWindow.move( 0, 75 )
        self.helpWindow.resize( 1200, 825 )
        self.helpWindow.set_modal(True)
        self.helpWindow.add_events( gtk.gdk.BUTTON_PRESS_MASK )
        self.helpWindow.connect("button-release-event", lambda w,e: self.helpButton.set_active(False))

        self.show_all()

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
        self.helpButton.hide()
