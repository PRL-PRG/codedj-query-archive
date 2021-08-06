#!/usr/bin/python

import string
from sys import argv,stderr,exit

infile = argv[1]
outfile = argv[2]

inp = open(infile,'r').readlines()
out = open(outfile,'w')

if len(argv)>3:
    S_index = argv[3]
else:
    S_index = inp[1].find('desc')
    print "Looked in SCORE tags, and found index of description: ",S_index
    if S_index == -1:
        exit()

for line in inp:
    if len(line) > S_index:
        if line[S_index] == 'S':
            out.write(line)
