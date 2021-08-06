#!/usr/bin/python

from sys import argv
import string
from glob import glob
from os.path import exists

sol_files = argv[1:]
llg_lines = []

if not exists(sol_files[0]): # Need to use glob
    print 'Using glob... '
    sol_files = glob( sol_files[0] )

for sol_file in sol_files:
    lines = open( sol_file).readlines()

    for line in lines[0:3]:
        cols = string.split( line )
        if len(cols)>5:
            llgcol = cols[-1]
#            llgcol = cols[-4]
            if len(llgcol)>4 and llgcol[3]=='=':
                llg = float(llgcol[4:])
                llg_lines.append( (llg, line[:-1],sol_file) )

llg_lines.sort()

for llg_line in llg_lines[-10:]:
    print llg_line[2],'==>',llg_line[1]




