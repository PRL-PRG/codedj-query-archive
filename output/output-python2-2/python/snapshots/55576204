import Config
from Util.CSoundClient import CSoundClient
from Generation.GenerationConstants import GenerationConstants

class NoteStdAlone:
    def __init__( self, client,
                        onset, 
                        pitch, 
                        amplitude, 
                        pan, 
                        duration, 
                        trackId, 
                        fullDuration = False, 
                        instrument = Config.FLUTE, 
                        attack = 0.005, 
                        decay = 0.095, 
                        reverbSend = 0.1, 
                        filterType = 0, 
                        filterCutoff = 1000,
                        tied = False,
                        overlap = False,
                        instrumentFlag = Config.FLUTE  ):
        self.csnd = client
        self.onset = onset
        self.pitch = pitch
        self.amplitude = amplitude
        self.pan = pan
        self.duration = duration
        self.trackId = trackId
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
            self.instrumentFlag = Config.DRUM1INSTRUMENTS[ self.pitch ]
        else:
            self.instrumentFlag = self.instrument

    def play( self ):
        self.csnd.sendText( self.getText(120) )
        
    def getText( self, tempo ):
        if self.instrument[ 0: 4 ] == 'drum':
            if GenerationConstants.DRUMPITCH.has_key( self.pitch ):
                self.pitch = GenerationConstants.DRUMPITCH[ self.pitch ]

            if self.instrument == 'drum1kit':
                self.instrumentFlag = Config.DRUM1INSTRUMENTS[ self.pitch ]
            if self.instrument == 'drum2kit':
                self.instrumentFlag = Config.DRUM2INSTRUMENTS[ self.pitch ]
            if self.instrument == 'drum3kit':
                self.instrumentFlag = Config.DRUM3INSTRUMENTS[ self.pitch ]
            newPitch = 1
        else:
            self.instrumentFlag = self.instrument
            newPitch = pow( GenerationConstants.TWO_ROOT_TWELVE, self.pitch - 36 )

        oneTickDuration = (Config.MS_PER_MINUTE / 1000)  / tempo / Config.TICKS_PER_BEAT

        newDuration = oneTickDuration * self.duration

        # condition for tied notes
        if Config.INSTRUMENTS[ self.instrumentFlag ].csoundInstrumentId  == 101  and self.tied and self.fullDuration:
            newDuration = -1
        # condition for overlaped notes
        if Config.INSTRUMENTS[ self.instrumentFlag ].csoundInstrumentId == 102 and self.overlap:
            newDuration = oneTickDuration * self.duration + 1.

        if True: newAmplitude = self.amplitude * 0.8
        else : newAmplitude = self.amplitude * music_volume_get( self.trackId )

        newAttack = newDuration * self.attack
        if newAttack <= 0.002:
            newAttack = 0.002

        newDecay = newDuration * self.decay
        if newDecay <= 0.002:
            newDecay = 0.002

	loopStart = Config.INSTRUMENTS[ self.instrumentFlag ].loopStart
	loopEnd = Config.INSTRUMENTS[ self.instrumentFlag ].loopEnd
	crossDur = Config.INSTRUMENTS[ self.instrumentFlag ].crossDur
        return Config.PLAY_NOTE_COMMAND % ( Config.INSTRUMENTS[ self.instrumentFlag ].csoundInstrumentId, 
                                                     self.trackId, 
                                                     0,
                                                     newDuration, 
                                                     newPitch, 
                                                     self.reverbSend, 
                                                     newAmplitude, 
                                                     self.pan, 
                                                     Config.INSTRUMENT_TABLE_OFFSET + Config.INSTRUMENTS[ self.instrumentFlag ].instrumentId,
                                                     newAttack,
                                                     newDecay,
                                                     self.filterType,
                                                     self.filterCutoff,
						     loopStart,
						     loopEnd,
						     crossDur )        
