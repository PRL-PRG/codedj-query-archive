#!/usr/bin/python

from sys import argv,exit
from os import system, popen
from os.path import exists
from glob import glob



def Help():
    print argv[0],' <map file1> <map file2> ... fft.script'
    print ' If fft script not given, default in ~rhiju/python/fft.script will be used.'

THRESHOLDS = []
for i in range( len(argv)-1, 0, -1 ):
    try:
        THRESHOLDS.append( float( argv[i] ) )
        del( argv[i] )
    except:
        continue
THRESHOLDS.reverse()

map_files = argv[1:]

if not exists(map_files[0]): # Need to use glob
    print 'Using glob... '
    map_files = glob( map_files[0] )

for map_file in map_files:

    GSF_EXE = '~rhiju/src/gsf/gsf.gcc'

    command = '%s %s ' % (GSF_EXE, map_file )
    for THRESHOLD in THRESHOLDS:
        command += ' %f' % THRESHOLD
    system( command )
