import pickle
import time
import bisect

from Framework.Constants import Constants
from Framework.CSound.CSoundConstants import CSoundConstants
from Framework.Generation.Generator import GenerationParameters

_data = {}

def music_init():
    #[ volume, ... ]
    _data['track_volume'] = {}

    #[ instrument index, ... ]
    _data['track_inst'] = {}

    #{ pageId: { [track 0 = note list], [track 2 = note list], ... ] }
    _data['page_notes'] = {}
    #{ pageId: ticks }
    _data['page_ticks'] = {}

def music_addNotes_fromDict( dict , replace = True):

    def new_page(pid):
        page_notes[pid] = [[]] * Constants.NUMBER_OF_TRACKS
        page_ticks[pid] = 4 * 12  #TODO

    if not replace : raise 'not Implemented'

    # { trackId : { pageId : notelist } }
    page_notes = _data['page_notes']
    page_ticks = _data['page_ticks']
    for tid in dict:
        pdict = dict[tid]
        for pid in pdict:
            if len( pdict[pid] ) > 0 :
                if not page_notes.has_key(pid):
                    new_page(pid)
                _track = page_notes[pid][tid]
                for note in pdict[pid]:
                    bisect.insort( _track, (note.onset, note))

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
        for tid in tracks:
            notes = notes \
                    + map( lambda (onset,note) : (onset + offset, note ),
                            _notes[pid][tid])
        offset = offset + _ticks[pid]

    return notes

def music_save(f):
    pickle.dump( _data, f )
def music_load(f):
    _data = pickle.load( f )

def music_volume_get(track):
    return _data['track_volume'][track]
def music_volume_set(track, vol):
    _data['track_volume'][track] = vol

def music_trackInstrument_get(track):
    return _data['track_inst'][track]
def music_trackInstrument_set(track, vol):
    _data['track_inst'][track] = vol




