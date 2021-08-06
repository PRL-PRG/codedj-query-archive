#!/usr/bin/python

import sys
import string
from math import floor,log
from os.path import basename
from os import popen,system

out_file = sys.argv[1]

fractionbest = 0.05
if len(sys.argv) > 2:
    fractionbest = float( sys.argv[2] )

coords_file = '-'
if len(sys.argv) > 3:
    coords_file = sys.argv[3]

EXE = '/users/rhiju/C/cluster_info_silent.out'
command = EXE+' '+out_file+' '+coords_file+' tmp 0,0,0,0 0,0 '
system(command)

EXE = '/users/rhiju/C/cluster_info_silent.out'
command = EXE+' '+out_file+' '+coords_file+' tmp_lowE 0,0,0,0 0,0 -1 '+ str(fractionbest)
system(command)



def getcontactmap(contactfile):
    contactmap = {}
    lines = open(contactfile).readlines()
    for line in lines:
        cols = string.split(line)
        if cols[0] == 'DC':
            res1 = int( cols[1])
            res2 = int( cols[2])
            freq = float(cols[3])
            if not contactmap.has_key(res1):
                contactmap[res1] = {}
            contactmap[res1][res2] = freq
    return contactmap


contactmap      = getcontactmap('tmp.contacts')
contactmap_lowE = getcontactmap('tmp_lowE.contacts')
contactlines = []

MINFREQ = 0.05;
for res1 in contactmap_lowE.keys():
    if contactmap.has_key(res1):

        residues2 = contactmap_lowE[res1].keys()
        residues2.sort()
        for res2 in residues2:
            if contactmap[res1].has_key(res2):
                freq =  contactmap[res1][res2]
                if freq > MINFREQ :
                    freq_lowE = contactmap_lowE[res1][res2]
                    ratio = freq_lowE/freq
                    if ratio>1:
                        logratio = log(ratio)
                        contactline = '%d %d %4.2f' % (res1+1, res2+1, logratio)
                        contactlines.append(contactline)


print len(contactlines)
for contactline in contactlines:
    print contactline


