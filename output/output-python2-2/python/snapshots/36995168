#! /usr/bin/python

from os import system
from os.path import exists
import sys

inputfile = sys.argv[1]
outputfile = sys.argv[2]

executable = '/users/rhiju/phaser'
if not(exists(executable)):
    executable = '/work/rhiju/phaser'

system("%s < %s > %s"%(executable,inputfile,outputfile))

