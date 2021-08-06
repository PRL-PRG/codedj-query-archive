#!/usr/bin/env python

import pygtk
pygtk.require( '2.0' )
import gtk

import Config
from Util.ThemeWidgets import *
Tooltips = Config.Tooltips

class InstrumentsPanel(gtk.EventBox):
    def __init__(self,setInstrument = None, playInstrument = None, enterMode = False):
        gtk.EventBox.__init__(self)
        
        self.tooltips = gtk.Tooltips()

        self.setInstrument = setInstrument
        self.playInstrument = playInstrument
        self.enterMode = enterMode
        self.instTable = None
        self.instDic = {}
        
        self.generateInstDic()
        
        self.mainVBox = RoundVBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_COLOR, radius = Config.PANEL_RADIUS)
        self.draw_toolbar()
        self.draw_instruments_panel()
        self.draw_mic_lab_box()
        self.add(self.mainVBox)
        self.show_all()
    
    def draw_toolbar(self):
        toolbarBox = RoundHBox(fillcolor = Config.PANEL_COLOR, bordercolor = Config.PANEL_COLOR, radius = Config.PANEL_RADIUS)
        for category in Config.CATEGORIES:
            btn = gtk.Button(label=category)
            btn.connect('clicked',self.handleToolbarBtnPress,category)
            toolbarBox.add(btn)
        
        self.mainVBox.pack_start(toolbarBox,False,False)
        
    def handleToolbarBtnPress(self, widget, category):
            self.draw_instruments_panel(category)
    
    def draw_instruments_panel(self,category = 'all'):
        
        if self.instTable != None:
            for child in self.instTable.get_children():
                self.instTable.remove(child)
            self.instTable.destroy()
        
        instrumentNum = len(self.getInstrumentList(category))
        instruments = self.getInstrumentList(category)
        
        cols = 8
        if instrumentNum < cols:
            cols = instrumentNum
        rows = (instrumentNum // cols)
        if instrumentNum % cols is not 0:    #S'il y a un reste
            rows = rows + 1
        
        self.instTable = gtk.Table(rows,cols,True)
        
        for row in range(rows):
            for col in range(cols):
                if row*cols+col >= instrumentNum:
                    break
                instBox = self.instDic[instruments[row*cols+col]]
                self.instTable.attach(instBox, col, col+1, row, row+1, gtk.SHRINK, 0, gtk.SHRINK, 0)
        
        self.mainVBox.pack_start(self.instTable)
        self.show_all()
                
    def handleInstrumentButtonClick(self,widget,instrument):
        if widget.get_active() is True:
            self.setInstrument(instrument)
            self.playInstrument(instrument)
            if self.enterMode:
                pass #Close the window
            
    def handleInstrumentButtonEnter(self,widget,instrument):
        self.playInstrument(instrument)
        
    def draw_mic_lab_box(self):
        hbox = gtk.HBox()
        
        for n in ['mic1','mic2','mic3','mic4']:
            vbox1 = RoundVBox(fillcolor = Config.INST_BCK_COLOR, bordercolor = Config.PANEL_COLOR, radius = Config.PANEL_RADIUS)
            vbox1.set_border_width(Config.PANEL_SPACING)
            
            micBtn = ImageRadioButton(self.firstInstButton, Config.IMAGE_ROOT + n + '.png' , Config.IMAGE_ROOT + n + 'sel.png', Config.IMAGE_ROOT + n + 'sel.png')
            micRecBtn = ImageButton(Config.IMAGE_ROOT + 'record.png' , Config.IMAGE_ROOT + 'recordhi.png', Config.IMAGE_ROOT + 'recordsel.png')
            self.tooltips.set_tip(micRecBtn,Tooltips.RECMIC)
            
            micBtn.connect('clicked', self.handleInstrumentButtonClick, n)
            micRecBtn.connect('clicked', self.handleMicButtonClick, n)
            micRecBtn.connect('pressed', self.handleRecButtonPress, micBtn)
            
            vbox1.add(micRecBtn)
            vbox1.add(micBtn)
            hbox.add(vbox1)
            
        for n in ['lab1','lab2','lab3','lab4']:
            vbox2 = RoundVBox(fillcolor = Config.INST_BCK_COLOR, bordercolor = Config.PANEL_COLOR, radius = Config.PANEL_RADIUS)
            vbox2.set_border_width(Config.PANEL_SPACING)
            
            synthBtn = ImageRadioButton(self.firstInstButton, Config.IMAGE_ROOT + n + '.png', Config.IMAGE_ROOT + n + 'sel.png', Config.IMAGE_ROOT + n + 'sel.png')
            synthRecBtn = ImageButton(Config.IMAGE_ROOT + 'record.png' , Config.IMAGE_ROOT + 'recordhi.png', Config.IMAGE_ROOT + 'recordsel.png')
            self.tooltips.set_tip(synthRecBtn,Tooltips.RECLAB)
            
            synthBtn.connect('clicked', self.handleInstrumentButtonClick, n)
            synthRecBtn.connect('clicked', self.handleSynthButtonClick, n)
            synthRecBtn.connect('pressed', self.handleRecButtonPress, synthBtn)
            
            vbox2.add(synthRecBtn)
            vbox2.add(synthBtn)
            hbox.add(vbox2)
            
        self.mainVBox.pack_end(hbox,False,False)
        
    def handleMicButtonClick(self,widget,mic):
        self.recstate = False
        
    def handleSynthButtonClick(self,widget,lab):
        self.recstate = False
        
    def handleRecButtonPress(self,widget,btn):
        self.recstate = True
        btn.set_active(True)
        
    def generateInstDic(self):
        self.firstInstButton = None
        for instrument in self.getInstrumentList():
            instBox = RoundVBox(fillcolor = Config.INST_BCK_COLOR, bordercolor = Config.PANEL_COLOR, radius = Config.PANEL_RADIUS)
            instBox.set_border_width(Config.PANEL_SPACING)
            instButton = ImageRadioButton(self.firstInstButton, Config.IMAGE_ROOT + instrument + '.png' , Config.IMAGE_ROOT + instrument + 'sel.png', Config.IMAGE_ROOT + instrument + 'sel.png')
            instButton.connect('clicked',self.handleInstrumentButtonClick, instrument)
            if self.enterMode:
                instButton.connect('enter',self.handleInstrumentButtonEnter, instrument)
            instBox.pack_start(instButton)
            self.instDic[instrument] = instBox
            if self.firstInstButton == None:
                self.firstInstButton = instButton
                
    def getInstrumentList(self,category = 'all'):
        instrumentList = [instrument for instrument in Config.INSTRUMENTS.keys() if instrument[0:4] != 'drum' and instrument[0:3] != 'mic' and instrument[0:3] != 'lab' and instrument[0:4] != 'guid'] + ['drum1kit', 'drum2kit', 'drum3kit']
        if category != 'all':
            instrumentList = [instrument for instrument in Config.INSTRUMENTS.keys() if instrument[0:4] != 'drum' and instrument[0:3] != 'mic' and instrument[0:3] != 'lab' and instrument[0:4] != 'guid' and Config.INSTRUMENTS[instrument].category == category] 
            if category == 'percussions':
                instrumentList = ['drum1kit', 'drum2kit', 'drum3kit'] + instrumentList
        #instrumentList = instrumentList.sort(lambda g,l: cmp(Config.INSTRUMENTS[g].category, Config.INSTRUMENTS[l].category) )    
        return instrumentList
    
if __name__ == "__main__": 
    win = gtk.Window()
    wc = InstrumentsPanel()
    win.add(wc)
    win.show()
    #start the gtk event loop
    gtk.main()