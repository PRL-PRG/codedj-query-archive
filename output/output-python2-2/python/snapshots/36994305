#!/usr/bin/python
# Rasmol rocks! But I want to always make cartoons, damn it.
#
import sys
from os import system
from os.path import basename

filenames = sys.argv[1:]

print "-------------- RUNNING rasmol %s ON DOTTY -----------" % basename(filenames[0])
command = "ssh -Y dotty rm -f toprint/temp.*"
system(command)

for filename in filenames:
    command = "scp %s dotty:toprint/temp.%s.pdb" % (filename,basename(filename))
    system(command)

fid = open("/tmp/temp.%s.script" % basename(filenames[0]),'w')
for filename in filenames:
    fid.write("zap\nload temp.%s.pdb\necho loading ... %s\ncartoons\nselect not backbone or *.ca\nwireframe 50\nrestrict not hydrogen\nselect all\ncolor temperature\nselect hetero\nspacefill temperature\nselect all\nset specular on\npause\nspacefill\n" % (basename(filename),basename(filename)) )
    if not filename == filenames[-1]:
        fid.write("pause\n");

fid.close()

command = "scp /tmp/temp.%s.script dotty:toprint/" % basename(filenames[0])
system(command)

command = "ssh -Y dotty 'cd toprint;/net/local/bin/rasmol_32bit -script temp.%s.script'" % basename(filenames[0])
system(command)

command = "ssh -Y dotty rm -f toprint/temp.*"
system(command)
command = "rm /tmp/temp.%s.script" %  basename(filenames[0])
system(command)
print "------------------------------------------------"
