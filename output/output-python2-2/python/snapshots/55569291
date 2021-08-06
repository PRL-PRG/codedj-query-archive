import pygtk
pygtk.require( '2.0' )
import gtk

from Framework import Note
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.GenerationConstants import GenerationConstants
from GUI.Core.KeyMapping import KEY_MAP


class KeyboardInput:
    def __init__( self , getCurrentTick , getTrackInstruments , getTrackDictionary , getSelectedTrackIDs , mainWindowUpdateCallback , pagePlayerUpdateCallback , getCurrentPageIDCallback ):
        self.active = False
        self.record = False
        self.monophonic = False
        self.key_dict = dict()
        
        self.getCurrentTick = getCurrentTick
        self.getTrackInstruments = getTrackInstruments
        self.getTrackDictionary = getTrackDictionary
        self.getSelectedTrackIDs = getSelectedTrackIDs
        self.mainWindowUpdateCallback = mainWindowUpdateCallback
        self.pagePlayerUpdateCallback = pagePlayerUpdateCallback
        self.getCurrentPageIDCallback = getCurrentPageIDCallback
        
    def onKeyPress(self,widget,event):
        if not self.active:
            return
        if self.record:
            self.monophonic = False
        
        key = event.hardware_keycode 
        # If the key is already in the dictionnary, exit function (to avoir key repeats)
        if self.key_dict.has_key(key):
                return
        # Assign on which track the note will be created according to the number of keys pressed    
        track = len(self.key_dict)+10
        if self.monophonic:
            track = 10
        # If the pressed key is in the keymap
        if KEY_MAP.has_key(key):
            # CsoundNote parameters
            onset = self.getCurrentTick()
            pitch = KEY_MAP[key]
            duration = -1
            instrument = self.getTrackInstruments()[0]
            # get instrument from top selected track if a track is selected
            if self.getSelectedTrackIDs():
                instrument = self.getTrackInstruments()[min(self.getSelectedTrackIDs())]
            
            if instrument == 'drum1kit':
                if GenerationConstants.DRUMPITCH.has_key( pitch ):
                    instrument = CSoundConstants.DRUM1INSTRUMENTS[ GenerationConstants.DRUMPITCH[ pitch ] ]
                else:
                    instrument = CSoundConstants.DRUM1INSTRUMENTS[ pitch ]
                pitch = 36
                duration = 100
            
            if CSoundConstants.INSTRUMENTS[instrument].csoundInstrumentID == 102:
                duration = 100
            
            # Create and play the note
            self.key_dict[key] = Note.note_new(onset = 0, 
                                            pitch = pitch, 
                                            amplitude = 1, 
                                            pan = 0.5, 
                                            duration = duration, 
                                            trackID = track, 
                                            fullDuration = False, 
                                            instrument = instrument, 
                                            instrumentFlag = instrument)
            Note.note_play(self.key_dict[key])
                
    def onKeyRelease(self,widget,event):
        if not self.active:
            return
        key = event.hardware_keycode 
        
        if KEY_MAP.has_key(key):
            self.key_dict[key]['duration'] = 0
            self.key_dict[key]['amplitude'] = 0
            self.key_dict[key]['dirty'] = True
            Note.note_play(self.key_dict[key])
            self.key_dict[key]['duration'] = self.getCurrentTick() - self.key_dict[key]['onset']
            #print "onset",self.key_dict[key].onset
            #print "dur",self.key_dict[key].duration
            if self.record and len( self.getSelectedTrackIDs() ) != 0:
                self.key_dict[key]['amplitude'] = 1
                self.getTrackDictionary()[min(self.getSelectedTrackIDs())][self.getCurrentPageIDCallback()].append(self.key_dict[key])
                self.mainWindowUpdateCallback()
                self.pagePlayerUpdateCallback()
            del self.key_dict[key]
            
    
    def onButtonPress(self,widget,event):
        pass
        
