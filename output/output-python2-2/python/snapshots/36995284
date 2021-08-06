#!/usr/bin/python
# Rasmol rocks! But I want to always make cartoons, damn it.
#
import sys
from os import system
from os.path import basename,abspath

filenames = sys.argv[2:]
scriptname = sys.argv[1]

fid = open("/tmp/temp.script",'w')
for filename in filenames:

    absfilename = abspath( filename ).replace(' ','\ ')
    fid.write("zap\nload %s\necho loading ... %s\nscript %s\n" % (absfilename,absfilename,abspath(scriptname)) )
    if not filename == filenames[-1]:
        fid.write("pause\n");
fid.close()

command = "cd /tmp; /Applications/rasmol_32bit -script temp.script"
system(command)

command = "rm -f /tmp/temp.*"
