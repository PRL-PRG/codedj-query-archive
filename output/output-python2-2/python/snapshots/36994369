#!/usr/bin/python

from os import system
import sys
import string

#clusterlist = [ 'nut', 'maat',  'bes', 'hapy', 'niau', 'seth',  'yah', 'gebb', 'ptah', 'apep',  'set',   'ra',  'dua', 'atum', 'dotty']
clusterlist = [ 'niau','seth','bes','hapy','apep','gebb','ptah','yah','isis' ];

command = sys.argv[1:]
command = string.join(command)

for cluster in clusterlist:
    cmd = 'ssh '+cluster+' '+command
    print(cmd)
    system(cmd)

