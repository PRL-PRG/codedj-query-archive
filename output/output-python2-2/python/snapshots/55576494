import curses
from curses.wrapper import wrapper
import csnd
import sys

#from Util.Sound import Sound

import Config

orc = Config.TAM_TAM_ROOT + '/Resources/univorc.csd'

print 'asdf'
csound = csnd.Csound()
csound.Compile( orc )
perf   = csnd.CsoundPerformanceThread(csound)
perf.Stop()
rval = perf.Join()
print 'rval = %lf'
csound.Reset()
sys.exit(0)

def main(stdscr):
    sound = Sound( 
            Config.TAM_TAM_ROOT + '/Resources/univorc.csd',
            Config.NOTELOOPER_HORIZON,
            Config.PLAYER_TEMPO * Config.TICKS_PER_BEAT / 60.0)
    #sound.setMasterVolume(100.0)

    curses.init_pair(1, curses.COLOR_RED, curses.COLOR_WHITE)
    while True:
        c = stdscr.getch()
        if c == ord('q'): 
            sound.uninit()
            break
        else:
            stdscr.addstr(0,0, "RED ALERT", curses.color_pair(1))
            stdscr.refresh()

if __name__ == "__main__":     
    wrapper(main)

