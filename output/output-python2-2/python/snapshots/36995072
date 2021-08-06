#!/usr/bin/python
import sys
from os import system
import string

filenames = sys.argv[1:]

command = "ssh nip mkdir -p toprint"
system(command)
for filename in filenames:
    print "-------------- PRINTING %s ON NIP/NJORD -----------" % filename
    command = "rsync -avzL %s nip:toprint/" % filename
    system(command)
    command = "ssh nip lpr -PQueued_Njord_Phaser6350DP  toprint/%s" % filename
    system(command)
    command = "ssh nip rm -f toprint/%s" % filename
    system(command)
print "------------------------------------------------"

