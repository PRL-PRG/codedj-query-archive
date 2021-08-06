#!/usr/bin/python
# Rasmol rocks! But I want to always make cartoons, damn it.
#
import sys
from os import system
from os.path import basename

filenames = sys.argv[1:]

print "-------------- RUNNING rasmol %s ON DOTTY -----------" % filenames[0]
command = "ssh -Y dotty rm -f /tmp/temp.*"
system(command)

for filename in filenames:
    command = "scp %s dotty:/tmp/temp.%s.pdb" % (filename,basename(filename))
    system(command)

fid = open("/tmp/temp.script",'w')
for filename in filenames:
    fid.write("zap\nload temp.%s.pdb\necho loading ... %s\nscript /users/rhiju/rasmol/burycomp.txt\n" % (basename(filename),basename(filename)) )
    if not filename == filenames[-1]:
        fid.write("pause\n");
fid.close()

command = "scp /tmp/temp.script dotty:/tmp/"
system(command)

command = "ssh -Y dotty 'cd /tmp;/net/local/bin/rasmol_32bit -script temp.script'"
system(command)

command = "ssh -Y dotty rm -f /tmp/temp.*"
system(command)
command = "rm temp.script"
system(command)
print "------------------------------------------------"


