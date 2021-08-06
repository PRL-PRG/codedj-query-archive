#!/usr/bin/python

from sys import argv,exit
from os import system
from os.path import exists
from glob import glob

def Help():
    print argv[0],' <mtz file1> <mtz file2> ... fft.script'
    print ' If fft script not given, default in ~rhiju/python/fft.script will be used.'

fft_script_file = '~rhiju/python/fft.script'
if argv[-1][-6:] == 'script':
    fft_script_file = argv[-1]
    del( argv[-1] )

mtz_files = argv[1:]

if not exists(mtz_files[0]): # Need to use glob
    print 'Using glob... '
    mtz_files = glob( mtz_files[0] )

for mtz_file in mtz_files:
    map_file = mtz_file.replace( '.mtz', '.map' )
    if not exists( map_file ):
        command = 'fft HKLIN "%s" MAPOUT "%s" < %s' % (mtz_file, map_file, fft_script_file )
        print command
        system( command )

