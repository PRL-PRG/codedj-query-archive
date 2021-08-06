#!/usr/bin/python
# Rasmol rocks! But I want to always make cartoons, damn it.
#
import sys
from os import system
from os.path import basename,abspath

filenames = sys.argv[1:]

#print "-------------- RUNNING rasmol %s ON DOTTY -----------" % basename(filenames[0])
#command = "ssh -Y dotty rm -f toprint/temp.*"
#system(command)

#for filename in filenames:
#    command = "scp %s dotty:toprint/temp.%s.pdb" % (filename,basename(filename))
#    system(command)

fid = open("/tmp/temp.%s.script" % basename(filenames[0]),'w')
for filename in filenames:
    fid.write("zap\nload %s\necho loading ... %s\nrenumber\nwireframe off\ncartoons\ncolor group\nset specular on\n" % (abspath(filename),abspath(filename)) )
    if not filename == filenames[-1]:
        fid.write("pause\n");

fid.close()

#command = "scp /tmp/temp.%s.script dotty:toprint/" % basename(filenames[0])
#system(command)

#command = "ssh -Y dotty 'cd toprint;/net/local/bin/rasmol_32bit -script temp.%s.script'" % basename(filenames[0])
command = "cd /tmp/;/net/local/bin/rasmol -script temp.%s.script" % basename(filenames[0])
system(command)

#command = "ssh -Y dotty rm -f toprint/temp.*"
command = "rm -f /tmp/temp.*"
system(command)
#command = "rm /tmp/temp.%s.script" %  basename(filenames[0])
#system(command)
#print "------------------------------------------------"
