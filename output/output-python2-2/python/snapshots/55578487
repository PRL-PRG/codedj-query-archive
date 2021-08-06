import pickle
import time

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
    _data['page_beats'] = {}

def music_addNotes_fromDict( dict , replace = True):
    
    # {
    dict = { asdf }

def music_setNotes():
    pass

def music_delNotes():
    pass

def music_getNotes( pages, tracks ):
    # unify given pages and tracks into a single note list
    pass

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




