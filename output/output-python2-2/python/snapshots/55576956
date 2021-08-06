from Framework.Constants import Constants 
from Framework.CSound.CSoundClient import CSoundClient
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.GenerationConstants import GenerationConstants

class NoteStdAlone:
    def __init__( self, client,
                        onset, 
                        pitch, 
                        amplitude, 
                        pan, 
                        duration, 
                        trackID, 
                        fullDuration = False, 
                        instrument = CSoundConstants.FLUTE, 
                        attack = 0.005, 
                        decay = 0.095, 
                        reverbSend = 0.1, 
                        filterType = 0, 
                        filterCutoff = 1000,
                        tied = False,
                        overlap = False,
                        instrumentFlag = CSoundConstants.FLUTE  ):
        self.csnd = client
        self.onset = onset
        self.pitch = pitch
        self.amplitude = amplitude
        self.pan = pan
        self.duration = duration
        self.trackID = trackID
        self.instrument = instrument
        self.fullDuration = fullDuration
        self.attack = attack
        self.decay = decay
        self.reverbSend = reverbSend
        self.filterType = filterType
        self.filterCutoff = filterCutoff
        self.tied = tied
        self.overlap = overlap
        if self.instrument == 'drum1kit':
            self.instrumentFlag = CSoundConstants.DRUM1INSTRUMENTS[ self.pitch ]
        else:
            self.instrumentFlag = self.instrument

    def play( self ):
        self.csnd.sendText( self.getText(120) )
        
    def getText( self, tempo ):
        if self.instrument[ 0: 4 ] == 'drum':
            if GenerationConstants.DRUMPITCH.has_key( self.pitch ):
                self.pitch = GenerationConstants.DRUMPITCH[ self.pitch ]

            if self.instrument == 'drum1kit':
                self.instrumentFlag = CSoundConstants.DRUM1INSTRUMENTS[ self.pitch ]
            if self.instrument == 'drum2kit':
                self.instrumentFlag = CSoundConstants.DRUM2INSTRUMENTS[ self.pitch ]
            if self.instrument == 'drum3kit':
                self.instrumentFlag = CSoundConstants.DRUM3INSTRUMENTS[ self.pitch ]
            newPitch = 1
        else:
            self.instrumentFlag = self.instrument
            newPitch = pow( GenerationConstants.TWO_ROOT_TWELVE, self.pitch - 36 )

        oneTickDuration = (Constants.MS_PER_MINUTE / 1000)  / tempo / Constants.TICKS_PER_BEAT

        newDuration = oneTickDuration * self.duration

        # condition for tied notes
        if CSoundConstants.INSTRUMENTS[ self.instrumentFlag ].csoundInstrumentID  == 101  and self.tied and self.fullDuration:
            newDuration = -1
        # condition for overlaped notes
        if CSoundConstants.INSTRUMENTS[ self.instrumentFlag ].csoundInstrumentID == 102 and self.overlap:
            newDuration = oneTickDuration * self.duration + 1.

        if True: newAmplitude = self.amplitude * 0.8
        else : newAmplitude = self.amplitude * music_volume_get( self.trackID )

        newAttack = newDuration * self.attack
        if newAttack <= 0.002:
            newAttack = 0.002

        newDecay = newDuration * self.decay
        if newDecay <= 0.002:
            newDecay = 0.002

	loopStart = CSoundConstants.INSTRUMENTS[ self.instrumentFlag ].loopStart
	loopEnd = CSoundConstants.INSTRUMENTS[ self.instrumentFlag ].loopEnd
	crossDur = CSoundConstants.INSTRUMENTS[ self.instrumentFlag ].crossDur
        return CSoundConstants.PLAY_NOTE_COMMAND % ( CSoundConstants.INSTRUMENTS[ self.instrumentFlag ].csoundInstrumentID, 
                                                     self.trackID, 
                                                     0,
                                                     newDuration, 
                                                     newPitch, 
                                                     self.reverbSend, 
                                                     newAmplitude, 
                                                     self.pan, 
                                                     CSoundConstants.INSTRUMENT_TABLE_OFFSET + CSoundConstants.INSTRUMENTS[ self.instrumentFlag ].instrumentID,
                                                     newAttack,
                                                     newDecay,
                                                     self.filterType,
                                                     self.filterCutoff,
						     loopStart,
						     loopEnd,
						     crossDur )        
