#! /usr/bin/python

from os import system
from os.path import exists
import sys


for i in range( len(sys.argv)/2 ):
    inputfile  = sys.argv[2*i+1]
    outputfile = sys.argv[2*i+2]

    executable = './phaser'
    if not(exists(executable)):
        executable = executable.replace('users','work')
        inputfile  = inputfile.replace('users','work')
        outputfile = outputfile.replace('users','work')

    if not(exists(outputfile)):
        system("%s < %s > %s"%(executable,inputfile,outputfile))

