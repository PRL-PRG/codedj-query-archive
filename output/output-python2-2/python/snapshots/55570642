# -*- coding: utf-8 -*-
import os
from sugar.activity.activity import get_bundle_path
from sugar import env

#QUICKLOAD = os.path.isfile("QUICKLOAD") # skip loading inessential comenents to speed things up

SugarMode = True

print "cwd:", os.getcwd()
if os.path.isfile("DEBUG"):
    f = open("DEBUG")
    l = f.read(10)
    f.close()
    if len(l): DEBUG = int( l )
    else: DEBUG = 99
else:
    DEBUG = 0
print "Debug Level %d" % (DEBUG)


TAM_TAM_ROOT = get_bundle_path()
print 'INFO: loaded TAMTAM_ROOT=%s' % TAM_TAM_ROOT


#PATHS
if os.path.isdir("/usr/share/tamtam/Sounds"):
    SOUNDS_DIR = "/usr/share/tamtam/Sounds"
    LIB_DIR = "/usr/share/tamtam"
else:
    SOUNDS_DIR = "/usr/share/activities/TamTamEdit.activity/common/Resources/Sounds"
    LIB_DIR = "/usr/share/activities/TamTamEdit.activity/common/Resources"
FILES_DIR = TAM_TAM_ROOT + "/common/Resources"
TUNE_DIR='/'
SYNTH_DIR='/'
if SugarMode == True:
    PREF_DIR = env.get_profile_path() + '/tamtam'
    TUNE_DIR=env.get_profile_path() + '/tamtam/tunes'
    SYNTH_DIR=env.get_profile_path() + '/tamtam/synthlab'
    SNDS_DIR=env.get_profile_path() + '/tamtam/snds'
    SNDS_INFO_DIR=env.get_profile_path() + '/tamtam/snds_info'
    SCRATCH_DIR = PREF_DIR + "/.scratch/"
else:
    PREF_DIR = os.getenv('HOME') + '/.tamtam'
    TUNE_DIR= os.getenv('HOME') + '/.tamtam/tunes'
    SYNTH_DIR= os.getenv('HOME') + '/.tamtam/synthlab'
    SNDS_DIR= os.getenv('HOME') + '/.tamtam/snds'
    SNDS_INFO_DIR = os.getenv('HOME') + '/.tamtam/snds_info'
    SCRATCH_DIR = PREF_DIR + "/.scratch/"

#PLUGIN
PLUGIN_DEBUG = PREF_DIR + "/clooper.log"
PLUGIN_VERBOSE = 0
PLUGIN_UNIVORC = TAM_TAM_ROOT + "/common/Resources/tamtamorc.csd"
PLUGIN_KSMPS = 64
PLUGIN_RATE  = 16000

## PLUGIN ALSA PARAMETERS:
PLUGIN_PERIOD = 256 #512
PLUGIN_NPERIODS = 2

##############
## SOUNDS
##############

LOW, MID, HIGH, PUNCH = range( 4 )

INSTRUMENT_TABLE_OFFSET = 5000
INST_FREE = 5000
INST_TIED = 5001
INST_SIMP = 5011
INST_PERC = 5021

CATEGORIES = ['all','animals','concret','keyboard','people','percussions','strings','winds', 'mysounds']

#CSOUND COMMANDS
CSOUND_LOAD_INSTRUMENT = 'f%d 0 0 -1 "%s" 0 0 0'
CSOUND_MIC_RECORD = 'i5201 0 5 %d'
CSOUND_UNLOAD_TABLES = 'i%d 0 0.1 %d' % (INST_FREE, 150) # removed magic number
CSOUND_NOTE_OFF = 'i %s.%s .2 0.01 1. 0. 0. 0.5 %d 0 0 0 0' %('%d','%d',INSTRUMENT_TABLE_OFFSET)
CSOUND_LOAD_LS_INSTRUMENT = 'f4999 0 0 -1 \"%s\" 0 0 0'
CSOUND_PLAY_LS_NOTE = 'i %i 0 -1'
CSOUND_STOP_LS_NOTE = 'i 5022 0 0.5'


#################
## GUI CONSTANTS
#################

LANGUAGE = 'En'
if os.path.isdir("/usr/share/tamtam/Images"):
    IMAGE_ROOT = '/usr/share/tamtam/Images/'
else:
    IMAGE_ROOT = "/usr/share/activities/TamTamEdit.activity/common/Resources/Images/"
MAIN_WINDOW_PADDING = 5

BG_COLOR = '#404040'
FG_COLOR = '#818286'

NOTE_HEIGHT = 9     # pixels
NOTE_IMAGE_PADDING = 6
NOTE_IMAGE_PADDING_MUL2 = NOTE_IMAGE_PADDING*2
NOTE_IMAGE_TAIL = 1059
NOTE_IMAGE_ENDLENGTH = 12
HIT_HEIGHT = 13    # pixels
HIT_IMAGE_PADDING = 6
HIT_IMAGE_PADDING_MUL2 = HIT_IMAGE_PADDING*2
TRACK_SPACING = 4
TRACK_SPACING_DIV2 = TRACK_SPACING//2
TRACK_COLORS = [ ( "#00290B", "#00E847" ), \
                 ( "#3F0200", "#E72500" ), \
                 ( "#002642", "#0090EA" ), \
                 ( "#313D00", "#F9EF00" ), \
                 ( "#17083B", "#4A00ED" ) ]
#TRACK_COLORS = [ ( "#00591B", "#00E847" ), \
#                 ( "#6F1200", "#E72500" ), \
#                 ( "#004682", "#0090EA" ), \
#                 ( "#716D00", "#F9EF00" ), \
#                 ( "#37187B", "#4A00ED" ) ]
BEAT_COLOR = "#999999"
BEAT_LINE_SIZE = 2
PLAYHEAD_COLOR = "#666666"
PLAYHEAD_SIZE = 2
PLAYHEAD_SIZE_DIV2 = PLAYHEAD_SIZE/2.0
MARQUEE_COLOR = "#FFFFFF"
MARQUEE_SIZE = 2

PAGE_BORDER_SIZE = 2
PAGE_SELECTED_BORDER_SIZE = 5
PAGE_WIDTH = 100
PAGE_HEIGHT = 25

PAGE_THUMBNAIL_WIDTH = 92
PAGE_THUMBNAIL_WIDTH_DIV2 =     PAGE_THUMBNAIL_WIDTH/2
PAGE_THUMBNAIL_HEIGHT = 65

THUMBNAIL_TRACK_RECT = [ ( 2, 4, 83, 10 ), \
                         ( 2, 14, 83, 10 ), \
                         ( 2, 24, 83, 10 ), \
                         ( 2, 34, 83, 10 ), \
                         ( 2, 44, 83, 13 ) ]
THUMBNAIL_DRAG_COLOR = "#000000"
THUMBNAIL_TRACK_COLOR = "#FF0000"
THUMBNAIL_SELECTED_COLOR = "#2266FF"
THUMBNAIL_DISPLAYED_COLOR = "#CC1133"

TOOLBAR_BCK_COLOR = '#404040'
WHITE_COLOR = '#FFFFFF'
### miniTamTam/SYNTHLAB SPECIFIC ###
INST_BCK_COLOR = '#999999'
PANEL_BCK_COLOR =  '#CCCCCC'
PANEL_COLOR = '#CCCCCC'
SL_LINE_COLOR = "#666666"
SL_HIGHLIGHT_COLOR = "#FFFFFF"
SL_OVER_WIRE_COLOR = "#FFFFFF"
SL_OVER_GATE_COLOR = "#00FF18"
SL_OVER_GATE_REJECT_COLOR = "#B30000"
PANEL_RADIUS = 10
PANEL_SPACING = 2

###Instrument Panel###
CATEGORY_BCK_COLOR = '#222222'
INSTRUMENT_GRID_COLOR = '#CCCCCC'

###Welcome Screen Specific###
WS_PANEL_COLOR = '#404040'
WS_BCK_COLOR = '#CCCCCC'

# hardware keycodes for mod keys
MOD_LSHIFT = 50
MOD_RSHIFT = 62
MOD_LCTRL = 37
MOD_RCTRL = 109
MOD_LALT = 64
MOD_RALT = 113



########
##   Things that don't belong!
#######
class _ModKeys:
    def __init__( self ):
        self.shiftDown = False
        self.ctrlDown = False
        self.altDown = False

    def keyPress( self, code ):
        if code == MOD_LSHIFT or code == MOD_RSHIFT:  self.shiftDown = True
        elif code == MOD_LCTRL or code == MOD_RCTRL:  self.ctrlDown = True
        elif code == MOD_LALT or code == MOD_RALT:    self.altDown = True

    def keyRelease( self, code ):
        if code == MOD_LSHIFT or code == MOD_RSHIFT:  self.shiftDown = False
        elif code == MOD_LCTRL or code == MOD_RCTRL:  self.ctrlDown = False
        elif code == MOD_LALT or code == MOD_RALT:    self.altDown = False

ModKeys = _ModKeys()


############
## EDIT DEFAULTS
############

#DEFAULTS
PLAYER_TEMPO = 100
PLAYER_TEMPO_LOWER = 30
PLAYER_TEMPO_UPPER = 160
DEFAULT_VOLUME = 50

#NUMERICAL CONSTANTS
NUMBER_OF_POSSIBLE_PITCHES = 25
MINIMUM_PITCH = 24
MAXIMUM_PITCH = MINIMUM_PITCH + NUMBER_OF_POSSIBLE_PITCHES - 1
NUMBER_OF_POSSIBLE_PITCHES_DRUM = 13
PITCH_STEP_DRUM = 2
MINIMUM_PITCH_DRUM = 24
MAXIMUM_PITCH_DRUM = MINIMUM_PITCH_DRUM + PITCH_STEP_DRUM*(NUMBER_OF_POSSIBLE_PITCHES_DRUM - 1)
MINIMUM_NOTE_DURATION = 1 # ticks
MS_PER_MINUTE = 60000.0
TICKS_PER_BEAT = 12
TICKS_PER_BEAT_DIV2 = TICKS_PER_BEAT/2
MAXIMUM_BEATS = 12 # maximum beats per page
NUMBER_OF_TRACKS = 5
NUMBER_OF_PAGES = 2

MINIMUM_AMPLITUDE = 0
MAXIMUM_AMPLITUDE = 1

DEFAULT_GRID = 3
DEFAULT_GRID_DIV2 = DEFAULT_GRID / 2.0


####################
## ToolTips
####################
LANGUAGE = 'en'
exec 'from Resources.tooltips_%s import Tooltips' % LANGUAGE


####################
## KeyMapping
####################

LOOP_KEYS = [17, 18, 19, 20, 21, 32, 33, 34, 35, 45, 46, 47, 48, 51, 60, 61]
# Key = Hardware Keycode Value = Note

KEY_MAP_PIANO = {24:36,    #Q
                 25:38,    #W
                 26:40,    #E
                 27:41,    #R
                 28:43,    #T
                 29:45,    #Y
                 30:47,    #U
                 31:48,    #I

                 11:37,    #2
                 12:39,    #3
                 14:42,    #5
                 15:44,    #6
                 16:46,    #7

                 39:25,    #S
                 40:27,    #D
                 42:30,    #G
                 43:32,    #H
                 44:34,    #J
                 46:37,    #L

                 52:24,    #Z
                 53:26,    #X
                 54:28,    #C
                 55:29,    #V
                 56:31,    #B
                 57:33,    #N
                 58:35,    #M
                 59:36}    #,

KEY_MAP_NOTPIANO = {24:24,    #Q
           25:25,    #W
           26:26,    #E
           27:27,    #R
           28:28,    #T
           29:29,    #Y
           30:30,    #U
           31:31,    #I
           32:32,    #O
           33:33,    #P

           38:34,    #A
           39:35,    #S
           40:36,    #D
           41:37,    #F
           42:38,    #G
           43:39,    #H
           44:40,    #J
           45:41,    #K
           46:42,    #L

           52:43,    #Z
           53:44,    #X
           54:45,    #C
           55:46,    #V
           56:47,    #B
           57:48}    #N

KEY_MAP = KEY_MAP_PIANO
