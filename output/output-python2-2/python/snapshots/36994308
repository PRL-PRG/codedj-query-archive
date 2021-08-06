#!/usr/bin/python
import sys
from os import system
from os.path import basename

filename = sys.argv[1]
print "-------------- RUNNING rasmol %s ON DOTTY -----------" % filename

command = "scp %s dotty:toprint/" % filename
system(command)
command = "ssh -Y dotty '/net/local/bin/rasmol_32bit toprint/%s'" % basename(filename)
system(command)
command = "ssh -Y dotty rm -f toprint/%s" % basename(filename)
system(command)
print "------------------------------------------------"

