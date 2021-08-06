#!/usr/bin/python

import string
from sys import argv, stdout
from os import system


command = "~rhiju/python/superimpose.py  "+ string.join(argv[1:])+" > temp.sup"
system(command)

lines = open('temp.sup').readlines()

model_num = 0
model_lines = {}
model_lines[0] = []
for line in lines:
    cols = string.split( line )
    if cols[0]=='MODEL': continue
    if cols[0]=='ENDMDL':
        model_num += 1
        model_lines[model_num] = []
        continue
    model_lines[model_num].append( line )

num_models = model_num - 1
num_lines = len( model_lines[0] )
for i in range( num_lines ) :
    sumx = 0.0
    sumy = 0.0
    sumz = 0.0
    for j in range( num_models ):
        x = float( model_lines[j][i][29:37] )
        sumx += x
        y = float( model_lines[j][i][38:45] )
        sumy += y
        z = float( model_lines[j][i][46:53] )
        sumz += z

    sumx /= num_models
    sumy /= num_models
    sumz /= num_models

    stdout.write( "%s %8.3f%8.3f%8.3f%s" % \
        (model_lines[0][i][0:29], sumx,sumy,sumz,model_lines[0][i][54:]) )
