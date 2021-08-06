#!/usr/bin/python

from sys import argv,exit,stdout
from os import popen,system
import string

def Help():
    print
    print
    print 'Usage: cat_outfiles.py <outfile1> <outfile2> ... > <concatenated outfile>'
    print
    print
    exit()

#if len(argv)<3:
#    Help()

outfiles = argv[1:]

command = 'cat '+outfiles[0]
#print(command)
system(command)


for i in range(1, len(outfiles)):
    data = open(outfiles[i],'r')

    line = data.readline() # Skip first two lines
    line = data.readline()

    while line:
        line = data.readline()[:-1]

        description_index = line.find('S_')
        if description_index < 0:
            description_index = line.find('F_')


        if description_index >= 0:
            tag = line[description_index:]

            tagcols = string.split(tag,'_')
            try:
                tagnum = int( tagcols[-1] )
                tagcols[-1] = '%06d' %  (tagnum + 10000*i)
                newtag = string.join( tagcols,'_')

                line = line[:description_index] + newtag
            except:
                continue

        if len(line) < 1: continue

        print line
