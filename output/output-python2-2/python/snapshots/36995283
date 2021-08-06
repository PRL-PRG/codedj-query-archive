#!/usr/bin/python

#
# Read through a *.info file made by phil's cluster_info_silent.out
# and make a table of how many members of each cluster came
# from which homolog
#

import string
from os import system,popen,chdir
from os.path import exists
from glob import glob
from whrandom import random
import sys
from math import floor

clusterinfofile = sys.argv[1]

lines = popen('grep CLUSTER_INFO '+clusterinfofile).readlines()

numhomologincluster={}
nummembers = []
homologlist = []
count = 0
for line in lines:
    members = string.split(line,'Members:')
#    cols = string.split(members[0])
#    nummembers.append( int(cols[1]))
    numhomologincluster[count] = {}
    members = string.split(members[-1],',')[1:]
    for i in range(len(members)):
        cols = string.split(members[i],':')
        cols = string.split( cols[0],'/')
        homolog = cols[-1]
        if homolog not in homologlist:
            homologlist.append(homolog)

        if homolog in numhomologincluster[count]:
            numhomologincluster[count][homolog] += 1
        else:
            numhomologincluster[count][homolog] = 1
    count += 1

homologlist.sort()


homologcount = 0
for homolog in homologlist:
    homologcount = homologcount+1
    print "%2d. %s" % (homologcount,homolog)
print
print



homologcount = 0
print "    ",
for homolog in homologlist:
    homologcount = homologcount+1
    print '%2d'%homologcount,
print

for i in range(count):
    homologcount = 0
    print "%02d. " % i,

    for homolog in homologlist:
        homologcount = homologcount+1

        if homolog in numhomologincluster[i]:
            print '%2d'%numhomologincluster[i][homolog],
        else:
            print '%2d'%0,
    print



## print "     ",
## for i in range(count):
##     print '%02d' % i,
## print


## homologcount = 0
## for homolog in homologlist:
##     homologcount = homologcount+1
##     print "%2d. " % homologcount,

##     for i in range(count):
##         if homolog in numhomologincluster[i]:
##             print '%2d'%numhomologincluster[i][homolog],
##         else:
##             print '%2d'%0,
##     print

