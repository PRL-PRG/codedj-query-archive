from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.GenerationConstants import GenerationConstants
from Framework.Music import *

def note_new( 
        onset,
        pitch, 
        amplitude, 
        pan, 
        duration, 
        trackID, 
        pageID,
        fullDuration = False, 
        instrument = CSoundConstants.FLUTE, 
        attack = 0.002, 
        decay = 0.098, 
        reverbSend = 0.1, 
        filterType = 0, 
        filterCutoff = 1000,
        tied = False,
        overlap = False,
        instrumentFlag = CSoundConstants.FLUTE  ):

    note = {}
    note['onset'] = onset
    note['pitch'] = pitch
    note['amplitude'] = amplitude
    note['pan'] = pan
    note['duration'] = duration
    note['trackID'] = trackID
    note['pageID'] = pageID
    note['fullDuration'] = fullDuration
    note['attack'] = attack
    note['decay'] = decay
    note['reverbSend'] = reverbSend
    note['filterType'] = filterType
    note['filterCutoff'] = filterCutoff
    note['tied'] = tied
    note['overlap'] = overlap
    note['dirty'] = True

    if instrument == 'drum1kit':
        note['instrumentFlag'] = CSoundConstants.DRUM1INSTRUMENTS[ pitch ]
    else:
        note['instrumentFlag'] = instrument

    return note

def note_refresh_play_cmd( note, preVolume, secs_per_tick ):
    if music_trackInstrument_get( note['trackID'] ) == 'drum1kit':
        if GenerationConstants.DRUMPITCH.has_key( note['pitch'] ):
            print note['pitch']
            note['pitch'] = GenerationConstants.DRUMPITCH[ note['pitch'] ]

        note['instrumentFlag'] = CSoundConstants.DRUM1INSTRUMENTS[ note['pitch'] ]
        newPitch = 1
    else:
        note['instrumentFlag'] = music_trackInstrument_get( note['trackID'] )
        newPitch = GenerationConstants.TRANSPOSE[ note['pitch'] - 24 ]

    duration = secs_per_tick * note['duration']
    #print 'hahaha', secs_per_tick, ' ', duration

    # condition for tied notes
    if CSoundConstants.INSTRUMENTS[ note['instrumentFlag'] ].csoundInstrumentID  == 101  and note['tied'] and note['fullDuration']:
        duration = -1.0
    # condition for overlaped notes
    if CSoundConstants.INSTRUMENTS[ note['instrumentFlag'] ].csoundInstrumentID == 102 and note['overlap']:
        duration += 1.0

    newAmplitude = note['amplitude'] * preVolume

    newAttack = duration * note['attack']
    if newAttack <= 0.002:
        newAttack = 0.002

    newDecay = duration * note['decay']
    if newDecay <= 0.002:
        newDecay = 0.002

    note['play_cmd'] = CSoundConstants.PLAY_NOTE_COMMAND_MINUS_DELAY % \
        ( CSoundConstants.INSTRUMENTS[ note['instrumentFlag'] ].csoundInstrumentID, 
            note['trackID'], 
            '%f', #delay,
            duration, 
            newPitch, 
            note['reverbSend'], 
            newAmplitude, 
            note['pan'], 
            CSoundConstants.INSTRUMENT_TABLE_OFFSET + CSoundConstants.INSTRUMENTS[ note['instrumentFlag'] ].instrumentID,
            newAttack,
            newDecay,
            note['filterType'],
            note['filterCutoff'] )

def note_getText( note, preVolume, secs_per_tick, delay ):
    if note['dirty'] :
        note_refresh_play_cmd( note, preVolume, secs_per_tick )
        note['dirty'] = False
    if delay < 0.0 : 
        print 'ERROR: you cant send note with negative delay', delay

    if music_effective_volume_get( note['trackID'] ) > 0.0 :
        #print 'getText', note['trackID'], note['onset'], note['play_cmd'] % float(delay)
        return note['play_cmd'] % float(delay)
    else:
        return ''

from Framework.CSound.CSoundClient import CSoundClient
def note_play(note, preVolume = 1.0, secs_per_tick = 0.1, delay = 0 ):
    CSoundClient.sendText( note_getText( note, preVolume, secs_per_tick, delay))

