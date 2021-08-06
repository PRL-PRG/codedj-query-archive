#!/usr/bin/env python

import pygtk
pygtk.require( '2.0' )
import gtk

import Config
from Util.ThemeWidgets import *
Tooltips = Config.Tooltips

class InstrumentPanel(gtk.EventBox):
    def __init__(self,setInstrument = None, playInstrument = None, enterMode = False, micRec = None, synthRec = None):
        gtk.EventBox.__init__(self)
        color = gtk.gdk.color_parse(Config.PANEL_BCK_COLOR)
        self.modify_bg(gtk.STATE_NORMAL, color)
        
        self.tooltips = gtk.Tooltips()

        self.setInstrument = setInstrument
        self.playInstrument = playInstrument
        self.micRec = micRec
        self.synthRec = synthRec
        self.enterMode = enterMode
        self.instrumentBox = None
        self.recstate = False
        self.instDic = {}
        
        self.mainVBox =  gtk.VBox()
        self.draw_toolbar()
        self.generateInstDic()
        self.draw_instruments_panel()
        
        self.add(self.mainVBox)
        self.show_all()
    
    def draw_toolbar(self):
        toolbarBox = gtk.HBox()
        firstBtn = None
        for category in Config.CATEGORIES:
            btnBox = RoundVBox(fillcolor = '#6F947B', bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
            btnBox.set_border_width(Config.PANEL_SPACING)
            btn = ImageRadioButton(firstBtn,Config.IMAGE_ROOT + category + '.png', Config.IMAGE_ROOT + category + 'sel.png', Config.IMAGE_ROOT + category + 'sel.png')
            if firstBtn == None:
                firstBtn = btn
            btn.connect('clicked',self.handleToolbarBtnPress,category)
            btnBox.add(btn)
            toolbarBox.pack_start(btnBox,True,True)
        
        self.mainVBox.pack_start(toolbarBox,False,False)
        
    def handleToolbarBtnPress(self, widget, category):
            self.draw_instruments_panel(category)
    
    def draw_instruments_panel(self,category = 'all'):

        if self.instrumentBox != None:
            for child in self.instTable.get_children():
                self.instTable.remove(child)
            for child in self.scrollWin.get_children():
                self.scrollWin.remove(child)
            for child in self.instrumentBox.get_children():
                self.instrumentBox.remove(child)
            self.instrumentBox.destroy()
        
        self.instrumentBox = RoundHBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)

        instrumentNum = len(self.getInstrumentList(category))
        instruments = self.getInstrumentList(category)
        
        cols = 8
        if instrumentNum < cols:
            cols = instrumentNum
        rows = (instrumentNum // cols)
        if instrumentNum % cols is not 0:    #S'il y a un reste
            rows = rows + 1
        
        self.scrollWin = gtk.ScrolledWindow()
        self.scrollWin.set_policy(gtk.POLICY_NEVER,gtk.POLICY_AUTOMATIC)
    
        self.instTable = gtk.Table(rows,cols,True)
        self.instTable.set_row_spacings(0)
        self.instTable.set_col_spacings(0)

        for row in range(rows):
            for col in range(cols):
                if row*cols+col >= instrumentNum:
                    break
                instBox = self.instDic[instruments[row*cols+col]]
                self.instTable.attach(instBox, col, col+1, row, row+1, gtk.SHRINK, gtk.SHRINK, 0, 0)
        
        tableEventBox = gtk.EventBox()
        color = gtk.gdk.color_parse(Config.PANEL_COLOR)
        tableEventBox.modify_bg(gtk.STATE_NORMAL, color)
        tableEventBox.add(self.instTable)
        self.scrollWin.add_with_viewport(tableEventBox)
        self.instrumentBox.pack_start(self.scrollWin,True,True,0)
        self.mainVBox.pack_start(self.instrumentBox)
        self.show_all()
        
    def handleInstrumentButtonClick(self,widget,instrument):
        if widget.get_active() is True and self.recstate == False:
            if self.setInstrument: self.setInstrument(instrument)
            if self.playInstrument: self.playInstrument(instrument)
            if self.enterMode:
                pass #Close the window
            
    def handleInstrumentButtonEnter(self,widget,instrument):
        if self.playInstrument: self.playInstrument(instrument)
        
    def handleMicRecButtonClick(self,widget,mic):
        self.recstate = False
        self.setInstrument(mic)
        if self.micRec: self.micRec(mic)
        
    def handleSynthRecButtonClick(self,widget,lab):
        self.recstate = False
        self.setInstrument(lab)
        if self.synthRec: self.synthRec(lab)
        
    def handleRecButtonPress(self,widget,btn):
        self.recstate = True
        btn.set_active(True)
        
    def generateInstDic(self):
        self.firstInstButton = None
        for instrument in self.getInstrumentList('all'):
            if instrument[0:3] == 'lab' or instrument[0:3] == 'mic':
                vbox = RoundVBox(fillcolor = Config.INST_BCK_COLOR, bordercolor = Config.PANEL_COLOR, radius = Config.PANEL_RADIUS)
                vbox.set_border_width(Config.PANEL_SPACING)
                
                Btn = ImageRadioButton(self.firstInstButton, Config.IMAGE_ROOT + instrument + '.png' , Config.IMAGE_ROOT + instrument + 'sel.png', Config.IMAGE_ROOT + instrument + 'sel.png')
                if self.firstInstButton == None:
                   self.firstInstButton = Btn
                RecBtn = ImageButton(Config.IMAGE_ROOT + 'record.png' , Config.IMAGE_ROOT + 'recordsel.png', Config.IMAGE_ROOT + 'recordhi.png')
                self.tooltips.set_tip(RecBtn,Tooltips.RECMIC)
                if instrument[0:3] == 'lab':
                    self.tooltips.set_tip(RecBtn,Tooltips.RECLAB)
                    
                Btn.connect('clicked', self.handleInstrumentButtonClick, instrument)
                if instrument[0:3] == 'mic':
                    RecBtn.connect('clicked', self.handleMicRecButtonClick, instrument)
                if instrument[0:3] == 'lab':
                    RecBtn.connect('clicked', self.handleSynthRecButtonClick, instrument)
                RecBtn.connect('pressed', self.handleRecButtonPress, Btn)
                
                vbox.pack_start(RecBtn,False,False)
                vbox.pack_start(Btn,False,False)
                self.instDic[instrument] = vbox

            else:    
                instBox = RoundVBox(fillcolor = Config.INST_BCK_COLOR, bordercolor = Config.PANEL_COLOR, radius = Config.PANEL_RADIUS)
                instBox.set_border_width(Config.PANEL_SPACING)
                instButton = ImageRadioButton(self.firstInstButton, Config.IMAGE_ROOT + instrument + '.png' , Config.IMAGE_ROOT + instrument + 'sel.png', Config.IMAGE_ROOT + instrument + 'sel.png')
                instButton.connect('clicked',self.handleInstrumentButtonClick, instrument)
                if self.enterMode:
                    instButton.connect('enter',self.handleInstrumentButtonEnter, instrument)
                instBox.pack_start(instButton,False,False)
                self.instDic[instrument] = instBox
                if self.firstInstButton == None:
                    self.firstInstButton = instButton

                
    def getInstrumentList(self,category = 'all'):
        instrumentList = [instrument for instrument in Config.INSTRUMENTS.keys() if instrument[0:4] != 'drum' and instrument[0:4] != 'guid' and instrument[0:3] != 'mic' and instrument[0:3] != 'lab'] + Config.DRUMKITS + ['mic1', 'mic2', 'mic3', 'mic4', 'lab1', 'lab2', 'lab3', 'lab4']
        
        if self.enterMode:
            instrumentList = [instrument for instrument in Config.INSTRUMENTS.keys() if instrument[0:4] != 'drum' and instrument[0:4] != 'guid' and instrument[0:3] != 'mic' and instrument[0:3] != 'lab'] + ['mic1', 'mic2', 'mic3', 'mic4', 'lab1', 'lab2', 'lab3', 'lab4']
  
        if category != 'all':
            instrumentList = [instrument for instrument in Config.INSTRUMENTS.keys() if instrument[0:4] != 'drum' and instrument[0:4] != 'guid' and Config.INSTRUMENTS[instrument].category == category] 
            if category == 'percussions' and not self.enterMode:
                instrumentList = Config.DRUMKITS + instrumentList
            if category == 'people':
                instrumentList = instrumentList + ['mic1', 'mic2', 'mic3', 'mic4']
            if category == 'electronic':
                instrumentList = instrumentList + ['lab1', 'lab2', 'lab3', 'lab4']
        #instrumentList = instrumentList.sort(lambda g,l: cmp(Config.INSTRUMENTS[g].category, Config.INSTRUMENTS[l].category) )    
        return instrumentList
    
if __name__ == "__main__": 
    win = gtk.Window()
    wc = InstrumentPanel()
    win.add(wc)
    win.show()
    #start the gtk event loop
    gtk.main()
