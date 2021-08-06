#!/usr/bin/python

from sys import argv,exit
import string
from os import system
from os.path import basename,dirname,abspath,exists

def Help():
    print
    print argv[0]+' <cluster> <any extra rsync flags>'
    print
    exit()

if len(argv)<2:
    Help()

cluster = argv[1]
clusterlist = [ 'syd','niau','seth','bes','hapy','apep','gebb','ptah','yah','isis','yah','maat','nut','fin' ];
if cluster not in clusterlist:
    print 'Hey, '+cluster+' is not a known cluster.'
    Help()

extra_args = argv[2:]

dir = '.'
clusterdir = abspath(dir).replace('/Users/rhiju/','')
clusterdir = clusterdir.replace('/work/rhiju/','')

#if cluster[:3]=='syd':
#    n = cluster[3]
#    cluster = 'syd'
#    clusterdir = 'work'+n+'/'+clusterdir

command = 'ssh ' + cluster + ' mkdir -p '+clusterdir
print(command)
system(command)

command = 'rsync -avzL '+dir+' '+cluster+':'+clusterdir+' '+string.join(extra_args)
print(command)
system(command)

