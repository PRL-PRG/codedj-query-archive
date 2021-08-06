#!/usr/bin/python

from sys import argv
from os import system

pdbfiles = argv[1:]

for pdbfile in pdbfiles:
    lines = open(pdbfile).readlines()
    numlines = len(lines)
    numlines /= 2
    command = 'head -n %d %s > monomerA_%s' % (numlines, pdbfile, pdbfile)
    print command
    system(command)

