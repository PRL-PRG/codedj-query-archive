#!/usr/bin/env python

import pygtk
pygtk.require( '2.0' )
import gtk

import time

import Config
from Util.ThemeWidgets import *
Tooltips = Config.Tooltips

class InstrumentPanel( gtk.EventBox ):
    def __init__(self,setInstrument = None, playInstrument = None, enterMode = False, micRec = None, synthRec = None, rowLen = 8, _instDic = None, force_load = True ):
        gtk.EventBox.__init__(self)
  
        self.setInstrument = setInstrument
        self.playInstrument = playInstrument
        self.micRec = micRec
        self.synthRec = synthRec
        self.rowLen = rowLen
        self.enterMode = enterMode

        self.instDic = _instDic

        self.loaded = False
        self.loadData = {}
        self.loadStage = [0,0,0]
        if force_load: self.load()

    def configure( self, setInstrument = None, playInstrument = None, enterMode = False, micRec = None, synthRec = None, rowLen = 8 ):
  
        self.setInstrument = setInstrument
        self.playInstrument = playInstrument
        self.enterMode = enterMode
        self.micRec = micRec
        
        if self.rowLen != rowLen:
            self.rowLen = rowLen
            self.prepareInstrumentTable( self.category )
        else:
            self.rowLen = rowLen

    def load( self, timeout = -1 ):
        if self.loaded: return True
        if Config.DEBUG > 4: print "InstrumentPanel load", self.loadStage

        if self.loadStage[0] == 0:
            color = gtk.gdk.color_parse(Config.PANEL_BCK_COLOR)
            self.modify_bg(gtk.STATE_NORMAL, color)
            self.loadStage[0] = 1
            if timeout >= 0 and time.time() > timeout: return False

        if self.loadStage[0] == 1:
            self.tooltips = gtk.Tooltips()
            self.loadStage[0] = 2
            if timeout >= 0 and time.time() > timeout: return False

        if self.loadStage[0] == 2:
            self.instTable = None
            self.recstate = False
            self.lastInstrumentWidget = None

            self.mainVBox =  gtk.VBox()
            self.loadStage[0] = 3
            if timeout >= 0 and time.time() > timeout: return False

        if self.loadStage[0] == 3:
            if not self.loadInstrumentList( timeout, self.loadStage ):
                return False
            self.loadStage[0] = 4
            if timeout >= 0 and time.time() > timeout: return False

        if self.loadStage[0] == 4:
            if not self.loadToolbar( timeout, self.loadStage ):
                return False
            self.loadStage[0] = 5
            if timeout >= 0 and time.time() > timeout: return False

        if self.loadStage[0] == 5:
            if self.instDic == None:
                self.instDic = {}
                self.loadStage[0] = 5.1
            else:
                self.loadStage[0] = 6
    
        if self.loadStage[0] == 5.1:
            if not self.loadInstDic( self.instDic, timeout, self.loadStage ):
                return False
            self.loadStage[0] = 6
            if timeout >= 0 and time.time() > timeout: return False
        
        if self.loadStage[0] == 6:
            self.loadInstrumentViewport()
            self.loadStage[0] = 7
            if timeout >= 0 and time.time() > timeout: return False
 
        if self.loadStage[0] == 7:
            self.prepareInstrumentTable()
            self.loadStage[0] = 8
            if timeout >= 0 and time.time() > timeout: return False
        
        self.add(self.mainVBox)
        self.show_all()

        self.loaded = True
        return True

    def loadInstrumentList( self, timeout = -1, loadStage = [0,0,0] ):
       
        if loadStage[1] == 0:
            self.instrumentList = { "all": [], "all.enterMode": [], "percussions.enterMode": [], "lab": [], "mic": [], "kit": [] }
            for category in Config.CATEGORIES:
                self.instrumentList[category] = []
            loadStage[1] = 1
            if timeout >= 0 and time.time() > timeout: return False

        if loadStage[1] == 1:
            keys = Config.INSTRUMENTS.keys()
            for i in range(loadStage[2], len(keys)):
                key = keys[i]
                instrument = Config.INSTRUMENTS[key]
                if key[0:4] != 'drum' and key[0:4] != 'guid' and key[0:3] != 'mic' and key[0:3] != 'lab':
                    self.instrumentList["all"].append( key )
                if key[0:4] != 'drum' and key[0:4] != 'guid' and key[0:3] != 'mic' and key[0:3] != 'lab':
                    self.instrumentList["all.enterMode"].append( key )
                if key[0:4] != 'drum' and key[0:4] != 'guid':
                    self.instrumentList[instrument.category].append( key )
                    if instrument.category == "percussions":
                        self.instrumentList["percussions.enterMode"].append( key )
                if instrument.category == "kit":
                    self.instrumentList["kit"].append( key )
                loadStage[2] += 1
                if timeout >= 0 and time.time() > timeout: return False

            loadStage[1] = 2
            loadStage[2] = 0

        self.instrumentList["mic"].sort()
        self.instrumentList["lab"].sort()

        self.instrumentList["all"] += self.instrumentList["kit"] + self.instrumentList["mic"] + self.instrumentList["lab"]
        self.instrumentList["all.enterMode"] += self.instrumentList["mic"] + self.instrumentList["lab"]
        self.instrumentList["percussions"] += self.instrumentList["kit"]
        self.instrumentList["people"] += self.instrumentList["mic"]
        self.instrumentList["electronic"] += self.instrumentList["lab"]

        loadStage[1] = 0
        return True
 
    def loadToolbar( self, timeout = -1, loadStage = [0,0,0] ):
        if loadStage[1] == 0:
            self.loadData["toolbarBox"] = gtk.HBox()
            self.firstTbBtn = None
            self.loadStage[1] = 1
            if timeout >= 0 and time.time() > timeout: return False

        for i in range(loadStage[1]-1, len(Config.CATEGORIES)):
            category = Config.CATEGORIES[i]
            if loadStage[2] == 0:
                self.loadData["btnBox"] = RoundVBox(fillcolor = Config.CATEGORY_BCK_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
                self.loadData["btnBox"].set_border_width(Config.PANEL_SPACING)
                loadStage[2] = 1
                if timeout >= 0 and time.time() > timeout: return False

            if loadStage[2] == 1:
                self.loadData["btn"] = ImageRadioButton(self.firstTbBtn,Config.IMAGE_ROOT + category + '.png', Config.IMAGE_ROOT + category + 'sel.png', Config.IMAGE_ROOT + category + 'sel.png')
                loadStage[2] = 2
                if timeout >= 0 and time.time() > timeout: return False

            if self.firstTbBtn == None:
                self.firstTbBtn = self.loadData["btn"]
            self.loadData["btn"].connect('clicked',self.handleToolbarBtnPress,category)
            self.loadData["btnBox"].add(self.loadData["btn"])
            self.loadData["toolbarBox"].pack_start(self.loadData["btnBox"],True,True)

            loadStage[2] = 0
            loadStage[1] += 1
            if timeout >= 0 and time.time() > timeout: return False
        
        self.mainVBox.pack_start(self.loadData["toolbarBox"],False,False)

        self.loadData.pop("btn")
        self.loadData.pop("btnBox")
        self.loadData.pop("toolbarBox")
        loadStage[1] = 0
        return True

    def loadInstDic( self, instDic, timeout = -1, loadStage = [0,0,0] ):

        if loadStage[1] == 0:
            self.firstInstButton = None
            self.loadData["len"] = len(self.instrumentList['all'])
            loadStage[1] = 1
            if timeout >= 0 and time.time() > timeout: return False

        
        for i in range( loadStage[1]-1, self.loadData["len"] ):
            instrument = self.instrumentList["all"][i]
            if loadStage[2] == 0:
                self.loadData["instBox"] = RoundVBox(fillcolor = Config.INST_BCK_COLOR, bordercolor = Config.INSTRUMENT_GRID_COLOR, radius = Config.PANEL_RADIUS)
                self.loadData["instBox"].set_border_width(Config.PANEL_SPACING)
                loadStage[2] = 1
                if timeout >= 0 and time.time() > timeout: return False

            if loadStage[2] == 1:
                self.loadData["instButton"] = ImageRadioButton(self.firstInstButton, Config.IMAGE_ROOT + instrument + '.png' , Config.IMAGE_ROOT + instrument + 'sel.png', Config.IMAGE_ROOT + instrument + 'sel.png')
                loadStage[2] = 2
                if timeout >= 0 and time.time() > timeout: return False

            if loadStage[2] == 2:
                self.loadData["instButton"].clickedHandler = self.loadData["instButton"].connect('clicked',self.handleInstrumentButtonClick, instrument)
                self.loadData["instButton"].connect('enter',self.handleInstrumentButtonEnter, instrument)
                loadStage[2] = 3
                if timeout >= 0 and time.time() > timeout: return False

            self.loadData["instBox"].pack_start(self.loadData["instButton"],False,False)
            instDic[instrument] = self.loadData["instBox"]
            if self.firstInstButton == None:
                self.firstInstButton = self.loadData["instButton"]
            loadStage[2] = 0
            if timeout >= 0 and time.time() > timeout: return False

            loadStage[1] += 1

        self.loadData.pop("instBox")
        self.loadData.pop("instButton")
        self.loadData.pop("len")
        loadStage[1] = 0
        return True
  
    def loadInstrumentViewport( self ):
        self.instrumentBox = RoundHBox(fillcolor = Config.INSTRUMENT_GRID_COLOR, bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)

        self.scrollWin = gtk.ScrolledWindow()
        self.scrollWin.set_policy(gtk.POLICY_NEVER,gtk.POLICY_AUTOMATIC)

        self.tableEventBox = gtk.EventBox()
        color = gtk.gdk.color_parse(Config.INSTRUMENT_GRID_COLOR)
        self.tableEventBox.modify_bg(gtk.STATE_NORMAL, color)

        self.scrollWin.add_with_viewport(self.tableEventBox)
        self.tableEventBox.get_parent().set_shadow_type( gtk.SHADOW_NONE )
        self.instrumentBox.pack_start(self.scrollWin,True,True,0)
        self.mainVBox.pack_start(self.instrumentBox)
        self.show_all()
 
    def prepareInstrumentTable(self,category = 'all'):
        
        self.category = category 

        if self.enterMode:
            if category == "all": category = "all.enterMode"
            elif category == "percussions": category = "percussions.enterMode"

        if self.instTable != None:
            for child in self.instTable.get_children()[:]:
                self.instTable.remove(child)
            self.tableEventBox.remove(self.instTable)
            self.instTable.destroy()

        instrumentNum = len(self.instrumentList[category])
        instruments = self.instrumentList[category]
        
        cols = self.rowLen
        if instrumentNum < cols:
            cols = instrumentNum
        rows = (instrumentNum // cols)
        if instrumentNum % cols is not 0:    #S'il y a un reste
            rows = rows + 1
   
        self.instTable = gtk.Table(rows,cols,True)
        self.instTable.set_row_spacings(0)
        self.instTable.set_col_spacings(0)

        for row in range(rows):
            for col in range(cols):
                if row*cols+col >= instrumentNum:
                    break
                instBox = self.instDic[instruments[row*cols+col]]
                self.instTable.attach(instBox, col, col+1, row, row+1, gtk.SHRINK, gtk.SHRINK, 0, 0)
        
        self.tableEventBox.add(self.instTable)
        self.instTable.show_all()
        
    def selectFirstCat(self):
        self.firstTbBtn.set_active(True)
        
    def handleToolbarBtnPress(self, widget, category):
        if widget.get_active(): 
            self.prepareInstrumentTable(category)

    def handleInstrumentButtonClick(self,widget,instrument):
        if widget.get_active() is True and self.recstate == False:
            if self.setInstrument: 
                widget.event( gtk.gdk.Event( gtk.gdk.LEAVE_NOTIFY )  ) # fake the leave event
                self.setInstrument(instrument)
            if self.playInstrument: self.playInstrument(instrument)
            if self.enterMode:
                pass #Close the window
            
    def handleInstrumentButtonEnter(self,widget,instrument):
        if self.enterMode and self.playInstrument: 
            self.playInstrument(instrument)
        
    def handleMicRecButtonClick(self,widget,mic):
        self.recstate = False
        self.setInstrument(mic)
        if self.micRec: self.micRec(mic)
        
    def handleRecButtonPress(self,widget,btn):
        self.recstate = True
        btn.set_active(True)

    def set_activeInstrument(self,instrument, state):
        if len(self.instDic) > 0:
            for key in self.instDic:
                if key == instrument:
                    btn = self.instDic[key].get_children()[0]
                    btn.handler_block(btn.clickedHandler)
                    btn.set_active(state)
                    btn.handler_unblock(btn.clickedHandler)
                
   
class DrumPanel( gtk.EventBox ):
    def __init__(self, setDrum = None):
        gtk.EventBox.__init__(self)
        color = gtk.gdk.color_parse(Config.PANEL_BCK_COLOR)
        self.modify_bg(gtk.STATE_NORMAL, color)
        
        self.setDrum = setDrum
        self.instrumentList = []
        keys = Config.INSTRUMENTS.keys()
        for key in Config.INSTRUMENTS.keys():
            if Config.INSTRUMENTS[key].category == "kit":
                self.instrumentList.append( key )
        self.instrumentList.sort()
        self.drawDrums()
        
    def drawDrums(self):
        firstBtn = None
        btnBox = RoundHBox(fillcolor = '#6F947B', bordercolor = Config.PANEL_BCK_COLOR, radius = Config.PANEL_RADIUS)
        btnBox.set_border_width(Config.PANEL_SPACING)
        self.drums = {}
        for drumkit in self.instrumentList:
            instBox = RoundVBox(fillcolor = Config.INST_BCK_COLOR, bordercolor = Config.PANEL_COLOR, radius = Config.PANEL_RADIUS)
            instBox.set_border_width(Config.PANEL_SPACING)
            self.drums[drumkit] = ImageRadioButton(firstBtn, Config.IMAGE_ROOT + drumkit + '.png' , Config.IMAGE_ROOT + drumkit + 'sel.png', Config.IMAGE_ROOT + drumkit + 'sel.png')
            self.drums[drumkit].clickedHandler = self.drums[drumkit].connect('clicked',self.setDrums,drumkit)
            if firstBtn == None:
                firstBtn = self.drums[drumkit]
            instBox.pack_start(self.drums[drumkit], False, False, 0)
            btnBox.pack_start(instBox, False, False, 0)
        self.add(btnBox)
        self.show_all()
        
    def setDrums(self,widget,data):
        if widget.get_active():
            if self.setDrum: 
                widget.event( gtk.gdk.Event( gtk.gdk.LEAVE_NOTIFY )  ) # fake the leave event
                self.setDrum(data)

    def set_activeInstrument( self, instrument, state ):
        if instrument in self.instrumentList:
            btn = self.drums[instrument]
            btn.handler_block(btn.clickedHandler)
            btn.set_active(state)
            btn.handler_unblock(btn.clickedHandler)

if __name__ == "__main__": 
    win = gtk.Window()
    wc = DrumPanel(None)
    win.add(wc)
    win.show()
    #start the gtk event loop
    gtk.main()
