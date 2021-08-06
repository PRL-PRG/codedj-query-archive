#!/usr/bin/python
import sys
from os import system
from os.path import basename

filenames = sys.argv[1:]
print "-------------- PRINTING %s ON ZAZA -----------" % filenames[0]

for filename in filenames:
    command = "rsync -avz %s zaza:toprint/" % filename
    system(command)
    command = "ssh zaza lpr -P njord  toprint/%s" % basename(filename)
    system(command)
    command = "ssh zaza rm -f toprint/%s" % basename(filename)
    system(command)
print "------------------------------------------------"

