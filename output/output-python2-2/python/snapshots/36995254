#!/usr/bin/python
import sys
from os import system


filename = sys.argv[1]
print "-------------- PRINTING %s ON SUMO -----------" % filename

command = "scp %s sumo:toprint/" % filename
system(command)
command = "ssh sumo lpr -Pbiochem toprint/%s" % filename
system(command)
#command = "ssh tigger rm -f toprint/%s" % filename
#system(command)
print "------------------------------------------------"

