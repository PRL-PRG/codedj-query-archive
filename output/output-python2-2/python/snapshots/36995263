#!/usr/bin/python

import string
import sys
from os import popen,system
import pdb
from blast import NBAlign

infile = sys.argv[1]

lines = popen('grep  \'C4\*\' '+infile).readlines()

sequence =''
for line in lines:
    sequence += string.lower( line[19] )

print '#'+sequence

count = 0
for line in lines:
    cols = string.split( line )
    print count,'1',cols[6],cols[7],cols[8]
    count = count + 1


