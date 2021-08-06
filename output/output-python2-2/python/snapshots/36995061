#!/usr/bin/python

import sys
from os import popen
from os.path import exists
from glob import glob
import string

def Help():
    print sys.argv[0],' <flexwarp log files>'
    print
    sys.exit()

files = sys.argv[1:]

globfiles = []
for file in files:
    if not exists(file): # Need to use glob
        print 'Using glob... '
        globfiles += glob( file )
    else:
        globfiles += [file]

for logfile in globfiles:
    lines = popen('grep docked '+logfile).readlines()
    print logfile
    if len(lines) > 0:
        bestline = ''
        numresMAX = 0
        for line in lines:
            cols = string.split( line )
            #print cols
            numres = int( cols[-2][1:] )
            if numres >= numresMAX:
                numresMAX = numres
                bestline = line
        print bestline


