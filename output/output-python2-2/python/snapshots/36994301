#!/usr/bin/python
# Rasmol rocks! But I want to always make cartoons, damn it.
#
import sys
from os import system
from os.path import basename

script = sys.argv[1]
filename = sys.argv[2]

print "-------------- RUNNING rasmol %s ON DOTTY -----------" % filename
command = "ssh -Y dotty rm -f /tmp/temp.*"
system(command)

command = "scp %s dotty:/tmp/temp.%s" % (filename,basename(filename))
system(command)

command = "scp %s dotty:/tmp/" % script
system(command)

command = "ssh -Y dotty 'cd /tmp;/net/local/bin/rasmol_32bit -script %s temp.%s'" % (script,basename(filename))
system(command)

command = "ssh -Y dotty rm -f /tmp/temp.* /tmp/%s "% script
system(command)
print "------------------------------------------------"


