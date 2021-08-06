#!/usr/bin/python

from sys import argv,exit
import string
from os import system
from os.path import basename,dirname,abspath,exists

def Help():
    print
    print argv[0]+' <cluster> <submit file>'
    print  'You may want to run rsync_to_cluster.py before this.'
    print
    exit()

if len(argv)<3:
    Help()

cluster = argv[1]
submitfile = argv[2]
clusterlist = [ 'fin','syd','niau','seth','bes','hapy','apep','gebb','ptah','yah','isis','yah','maat','nut' ];
if cluster not in clusterlist:
    print 'Hey, '+cluster+' is not a known cluster.'
    Help()


# log condor submission event (so I don't forget which cluster this went on.)
fid = open('condor_submit.log','a')
fid.write('%s  %s \n' % (cluster,submitfile));
fid.close()


dir = '.'
clusterdir = abspath(dir).replace('/Users/rhiju/','')
clusterdir = clusterdir.replace('/work/rhiju/','')

#if cluster[:3]=='syd':
#    n = cluster[3]
#    cluster = 'syd'
#    clusterdir = 'work'+n+'/'+clusterdir

command = 'ssh %s "cd %s; condor_submit %s" ' % \
          (cluster,clusterdir,submitfile)
print(command)
system(command)


