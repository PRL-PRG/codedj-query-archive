#!/usr/bin/python
## look at fa scores

import string

from sys import argv
import sys

file1 = argv[1]

inputfile=open(file1,"r")

firstline=inputfile.readline()
if firstline.find('desc') < 0:
    firstline=inputfile.readline()
labels=string.split(firstline)
i=0
while i < len(labels) :
    print i+1, labels[i]
    i = i +1
