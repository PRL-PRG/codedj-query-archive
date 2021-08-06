#!/usr/bin/python

from sys import argv, exit, stdout
import string
from os.path import dirname, basename, exists



infile = argv[1]
outfile = argv[2]

lines = open( infile, 'r').readlines()

out = open( outfile, 'w', -1)
 

list = []

for i in range(len(lines)):
    lines[i] = lines[i][:-1]

for i in range(len(lines)):
    for k in range(len(lines)):
        if (k > (i + 1)) and ((k + 1) < len(lines)):
            list.append(lines[i]+" paired to "+lines[k]+" and "+lines[i+1]+" paired to "+lines[k+1])

size = len(list)

step = str(1/float(size))[:7]

print step

for i in range(len(lines)):
    for k in range(len(lines)):
        if (k > (i + 1)) and ((k + 1) < len(lines)):
            out.write("JUMPCODE "+step+" SSPAIR -0.01 A "+lines[i]+" "+lines[k]+" SSPAIR -0.01 A "+lines[i+1]+" "+lines[k+1]+"\n")





