#!/usr/bin/python

from os import system
#clusterlist = [ 'nut', 'maat',  'bes', 'hapy', 'niau', 'seth',  'yah', 'gebb', 'ptah', 'apep',  'set',   'ra',  'dua', 'atum', 'dotty']
clusterlist = [ 'maat','niau','seth','bes','hapy','apep','gebb','ptah','yah','isis' ];

for cluster in clusterlist:
    cmd = 'rsync -avzL ~rhiju/rosetta++/rosetta.gcc '+cluster+':rosetta++/rosetta.gcc'
    print(cmd)
    system(cmd)

