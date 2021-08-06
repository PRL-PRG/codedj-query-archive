#!/usr/bin/python

#Adapted from phil bradley's make_coords_file.py
# Rhiju, Feb 2006.

import string
import sys
from os import popen,system
import pdb

if len(sys.argv) !=3:
    print '\n'+'-'*75
    print 'Usage: %s <pdb> <chain> > <coords_file>'
    print '-'*75+'\n\n'
    assert 0==1

pdb_file = sys.argv[1]
chain = sys.argv[2]

if chain == '_' or chain == '-':
    chain = ' '


lines = popen('/users/pbradley/dssp '+pdb_file+' | grep "RESIDUE AA" -A10000 | '+\
              ' grep "^.[ 0-9][ 0-9][ 0-9][ 0-9]......'+\
              chain+'"').readlines()

lowercase = list('abcdefghijklmnopqrstuvwxyz')

seq = map(lambda x:x[13],lines)

for i in range(len(seq)):
    if seq[i] in lowercase:
        seq[i] = 'C'
seq = string.join(seq,'')

ss = string.join(map(lambda x:x[16],lines),'')

ss3 = ''
for a in ss:
    if a not in [' ','E','B','H','G','I','S','T']:
        sys.stderr.write('undefined ss character? '+a+'\n')
        ss3 = ss3+'L'
    elif a in ['E','B']:
        ss3 = ss3+'E'
    elif a in ['H','G']:
        ss3 = ss3+'H'
    else:
        ss3 = ss3+'L'

assert len(ss3) == len(seq)


ss3_psipred = ''
for i in range(len(seq)):
    Eweight = 0.0
    Cweight = 0.0
    Hweight = 0.0

    if ss3[i]=='E':
        Eweight = 1.0
        ss3_psipred = ss3_psipred+'E'
    if ss3[i]=='H':
        Hweight = 1.0
        ss3_psipred = ss3_psipred+'H'
    if ss3[i]=='L':
        Cweight = 1.0
        ss3_psipred = ss3_psipred+'C'

    print "%4d %s %s   %4.3f  %4.3f  %4.3f"% (i+1, seq[i], ss3_psipred[i],Cweight,Hweight,Eweight)
