#!/usr/bin/python

#Adapted from phil bradley's make_coords_file.py
# Rhiju, Feb 2006.

import string
import sys
from os import popen,system
import pdb
from os.path import basename

if len(sys.argv) < 2:
    print '\n'+'-'*75
    print 'Usage: %s <pdb> <chain> > <coords_file>'
    print '-'*75+'\n\n'
    assert 0==1

pdb_file = sys.argv[1]

chain_defined = 0
chain = ''
if len( sys.argv ) > 2:
    chain = sys.argv[2]
    chain_defined = 1

if chain == '_' or chain == '-':
    chain = ' '


gzipped = 0
if (pdb_file[-3:]=='.gz'):

    copy_pdb_file = '~/'+basename(pdb_file)

    system( 'cp -rf '+pdb_file+' '+copy_pdb_file )
    system( 'gunzip -f '+copy_pdb_file )
    pdb_file = copy_pdb_file.replace('.gz','')
    gzipped = 1


if chain_defined:
    lines = popen('~rhiju/dssp '+pdb_file+' | grep "RESIDUE AA" -A10000 | '+\
                  ' grep "^.[ 0-9][ 0-9][ 0-9][ 0-9]......'+\
                  chain+'"').readlines()
else:
    lines = popen('~rhiju/dssp '+pdb_file+' | grep "RESIDUE AA" -A10000 | '+\
                  ' grep "^.[ 0-9][ 0-9][ 0-9][ 0-9]"' ).readlines()

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


print ss3


if (gzipped):
    'rm -rf '+pdb_file
