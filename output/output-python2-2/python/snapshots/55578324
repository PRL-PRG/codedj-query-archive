import pickle
import time
import bisect

from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.Generator import GenerationParameters

_notebin = []
_data = {}

def music_init():

    #[ volume, ... ]
    _data['track_volume'] = [0.8] * Constants.NUMBER_OF_TRACKS
    _data['track_mute']   = [False] * Constants.NUMBER_OF_TRACKS

    #[ instrument index, ... ]
    track_inst = [
            CSoundConstants.FLUTE,
            CSoundConstants.KOTO,
            CSoundConstants.GAM,
            CSoundConstants.GAM,
            CSoundConstants.GUIT,
            CSoundConstants.DRUM1KIT,
            CSoundConstants.DRUM1KIT ]

    _data['track_inst'] = track_inst + [CSoundConstants.FLUTE] * (Constants.NUMBER_OF_TRACKS - len( track_inst) )

    #{ pageId: { [track 0 = note list], [track 2 = note list], ... ] }
    _data['page_notes'] = {}

    #{ pageId: ticks }
    _data['page_ticks'] = {}
    _data['page_beats'] = {}

    _data['tempo'] = Constants.DEFAULT_TEMPO

    _data['tune'] = []

def music_addPage( pid, nbeats):
    _data['page_notes'][pid] = map(lambda i : [], range(Constants.NUMBER_OF_TRACKS))
    _data['page_beats'][pid] = nbeats
    _data['page_ticks'][pid] = nbeats * Constants.TICKS_PER_BEAT

def music_addNotes_fromDict( dict ):

    global _notebin
    # { trackId : { pageId : notelist } }
    page_notes = _data['page_notes']
    page_ticks = _data['page_ticks']
    for tid in dict:
        pdict = dict[tid]
        for pid in pdict:
            if len( pdict[pid] ) > 0 :
                _track = page_notes[pid][tid]
                for note in pdict[pid]:
                    bisect.insort( _track, (note['onset'], note))
                _notebin += map( lambda (o,note): note, _track ) #shallow copy!

def music_setNotes():
    raise 'not Implemented'

def music_delNotes():
    raise 'not Implemented'

def music_getNotes( pages, tracks ):
    # unify given pages and tracks into a single note list
    notes = []
    offset = 0
    _ticks = _data['page_ticks']
    _notes = _data['page_notes']
    for pid in pages:
        if _notes.has_key(pid):
            for tid in tracks:
                    notes = notes \
                            + map( lambda (onset,note) : (onset + offset, note ),
                                    _notes[pid][tid])
                    #print len(_notes[pid][tid])
            offset = offset + _ticks[pid]
        else:
            print 'WARNING: requesting notes from non-existing page ', pid
        #print len(notes)

    notes.sort()
    return notes

def music_save(f):
    pickle.dump( _data, f )
def music_load(f):
    _data = pickle.load( f )

def music_volume_get(track):
    return _data['track_volume'][track]
def music_volume_set(track, vol):
    _data['track_volume'][track] = vol

def music_mute_get(track):
    return _data['track_mute'][track]
def music_mute_set(track, mute):
    _data['track_mute'][track] = mute

def music_effective_volume_get(track):
    if _data['track_mute'][track]:
        return 0.0
    else:
        return _data['track_volume'][track]

def music_trackInstrument_get(track):
    return _data['track_inst'][track]
def music_trackInstrument_set(track, inst):
    _data['track_inst'][track] = inst

def music_tempo_set( tempo ):
    _data['tempo'] = tempo
def music_tempo_get( ):
    return _data['tempo']

def music_duration_get( pid ):
    print 'pid',pid
    return _data['page_ticks'][pid]
def music_beats_get( pid ):
    return _data['page_beats'][pid]
def music_beats_set( pid, beats ):
    _data['page_beats'][pid] = beats
    _data['page_ticks'][pid] = beats * Constants.TICKS_PER_BEAT

def music_tune_get( ):
    return _data['tune']
def music_tune_set( tune ):
    _data['tune'] = tune

def music_allnotes():
    global _notebin
    return _notebin

