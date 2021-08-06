#!/usr/bin/python
import sys
from os import system
from os.path import basename

filenames = sys.argv[1:]
print "-------------- PRINTING %s ON BADGER -----------" % filenames[0]

for filename in filenames:
    command = "rsync -avz %s badger:toprint/" % filename
    system(command)
    command = "ssh badger lpr -P phaser  toprint/%s" % basename(filename)
    system(command)
    command = "ssh badger rm -f toprint/%s" % basename(filename)
    system(command)
print "------------------------------------------------"

