#!/usr/bin/python

from os import system
#clusterlist = [ 'maat',  'bes', 'hapy', 'niau', 'seth',  'yah', 'gebb', 'ptah', 'apep',  'nut',  'set',   'ra',  'dua', 'atum', 'sumo']
clusterlist = [ 'syd','niau','seth','bes','hapy','apep','gebb','ptah','yah','isis' ];

for cluster in clusterlist:
    cmd = 'rsync -avz /work/rhiju/rosetta_database/ '+cluster+':rosetta_database'
    print(cmd)
    system(cmd)

