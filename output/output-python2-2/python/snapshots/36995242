#!/usr/bin/python
import sys
from os import system


filename = sys.argv[1]
print "-------------- PRINTING %s ON TIGGER -----------" % filename

command = "scp %s tigger:toprint/" % filename
system(command)
command = "ssh tigger lp toprint/%s" % filename
system(command)
command = "ssh tigger rm -f toprint/%s" % filename
system(command)
print "------------------------------------------------"

