import pickle
import time
import bisect

from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.Generator import GenerationParameters

_nid = 1
_noteids = {}
_sid = 1
_subset = {}
_notebin = []
_data = {}
_listeners = []

def _newsid(): 
    while _subset.has_key(_sid):
        _sid = _sid + 1
def _newnid(): 
    while _noteids.has_key(_nid):
        _nid = _nid + 1

def music_init():

    #[ volume, ... ]
    _data['track_volume'] = [0.8] * Constants.NUMBER_OF_TRACKS
    _data['track_mute']   = [False] * Constants.NUMBER_OF_TRACKS

    #[ instrument index, ... ]
    track_inst = [ CSoundConstants.FLUTE, CSoundConstants.KOTO, CSoundConstants.GAM, CSoundConstants.GAM,
                   CSoundConstants.GUIT, CSoundConstants.DRUM1KIT, CSoundConstants.DRUM1KIT ]

    _data['track_inst'] = track_inst + [CSoundConstants.FLUTE] * (Constants.NUMBER_OF_TRACKS - len( track_inst) )
    #{ pageId: { [track 0 = note list], [track 2 = note list], ... ] }
    _data['page_notes'] = {}
    #{ pageId: ticks }
    _data['page_ticks'] = {}
    _data['page_beats'] = {}
    _data['tempo'] = Constants.DEFAULT_TEMPO
    _data['tune'] = []

    _subset[0] = _notebin

def music_addListener( fn ):
    _listeners.append(fn)

def music_create(params, wantSubSet = False):
    keys = []

    map( lambda x: x('add', keys), _listeners)

    return 

def music_delete(sid):
    pass

def music_update(sid):
    def setDirty(x): x['dirty'] = True
    map( setDirty, _subset[sid])
    map( lambda x: x('update', sid), _listeners)

def music_forget(sid):
    del _subset[sid]

def music_get(sid):
    return _subset[sid]

def music_filter(test, psid):
    _newsid()
    _subset[_sid] = filter( test, _subset[psid] )
    return _sid


def music_page_start( pid, nbeats):
    _data['page_notes'][pid] = map(lambda i : [], range(Constants.NUMBER_OF_TRACKS))
    _data['page_beats'][pid] = nbeats
    _data['page_ticks'][pid] = nbeats * Constants.TICKS_PER_BEAT


def music_addNotes_fromDict( dict, replace = True ):

    # dict ==  { trackId : { pageId : notelist } }
    noteList = []
    page_notes = _data['page_notes']
    for tid in dict:
        pdict = dict[tid]
        for pid in pdict:
            if len( pdict[pid] ) > 0 :
                if replace: page_notes[pid][tid] = []
                _track = page_notes[pid][tid]
                for note in pdict[pid]:
                    bisect.insort( _track, (note['onset'], note))
                noteList += map( lambda (o,note): note, _track ) #shallow copy!
    _subset[0] += noteList

    map( lambda x: x('add', noteList), _listeners)

def music_addNotes( noteList ):
    for note in noteList:
        bisect.insort( _data['page_notes'][note['pageID']][note['trackID']], (note['onset'], note))
        _subset[0].append(note)
    map( lambda x: x('add', noteList), _listeners)

def music_setNotes( noteList ):
    map( lambda x: x('set', noteList), _listeners)

def music_delNotes( noteList ):
    for note in noteList:
        _subset[0].remove(note)
        _data['page_notes'][note['pageID']][note['trackID']].remove( (note['onset'], note))
    map( lambda x: x('del', noteList), _listeners)

def music_getNotes( pages, tracks ):
    # unify given pages and tracks into a single note list
    notes = []
    offset = 0
    _ticks = _data['page_ticks']
    _notes = _data['page_notes']
    for pid in pages:
        if _notes.has_key(pid):
            for tid in tracks:
                    notes += map( lambda (onset,note) : (onset + offset, note ), _notes[pid][tid])
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

