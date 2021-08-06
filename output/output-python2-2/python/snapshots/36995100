#!/usr/bin/python
# Rasmol rocks! But I want to always make cartoons, damn it.
#
import sys
from os import system
from os.path import basename,abspath,exists

filenames = sys.argv[2:]
scriptname = sys.argv[1]

fid = open("/tmp/temp.script",'w')
for filename in filenames:
    fid.write("zap\nload %s\necho loading ... %s\nscript %s\n" % (abspath(filename),abspath(filename),abspath(scriptname)) )
    if not filename == filenames[-1]:
        fid.write("pause\n");
fid.close()

RASMOL_EXE = '/net/local/bin/rasmol'
if not exists( RASMOL_EXE ):
    RASMOL_EXE = '/Applications/rasmol_32BIT'
if not exists( RASMOL_EXE ):
    print
    print 'Cannot find rasmol executable. Edit %s to point to the path!' % sys.argv[0]

command = "cd /tmp; %s -script temp.script" % RASMOL_EXE
system(command)

command = "rm -f /tmp/temp.*"
