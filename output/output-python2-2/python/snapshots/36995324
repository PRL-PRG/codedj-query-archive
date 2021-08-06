#!/usr/bin/python

from glob import glob
from sys import argv,exit
from os import system
from os.path import basename, exists
from os import popen

def Help():
    print
    print 'Run from your align_files directory.'
    print ' Usage:  make_alignment_from_fragout.py <casp target number >'
    print
    exit()

if len(argv)<2:
    Help()

fastafiles = argv[1:]

try:
    targetnum = int( fastafiles[0] )
    fastafiles = glob( '/work/casp7/fragout/hom*_t%d_/*fasta.gz' % targetnum )
    fastafiles.sort()
    tag = 't%3d_' % targetnum
except:
    tag = argv[-1]
    fastafiles = fastafiles[:-1]

fastafiles_copied = ''

for fastafile in fastafiles:
    command = 'rsync '+fastafile+' . '
    print(command)
    system(command)
    if fastafile[-3:]=='.gz': fastafile = fastafile[:-3]
    fastafiles_copied += ' ' + basename(fastafile)

lines = popen('zcat '+fastafiles[0]).readlines()
silly_lars_tag = int( lines[0][1:] )

command = 'gunzip -f *.gz'
print(command)
system(command)



real_log_file = glob('/work/casp7/frag/%d/%d/*blast*real.log' % (silly_lars_tag, silly_lars_tag) )
assert( len( real_log_file) > 0)
blastfile = real_log_file[0][:-19]

command = 'cp '+blastfile+' .'
print(command)
system(command)

command = '/users/rhiju/python/blast2align.py '+blastfile+' - '+fastafiles_copied+' -extend > '+ tag+'.align_extended'
print(command)
system(command)

command = '/users/rhiju/python/alignfile2fasta.py '+tag+'.align_extended'
print(command)
system(command)



command = '/users/bqian/bin/muscle -in '+tag+'.align_extended.fasta -out '+tag+'.muscle.fasta'
print(command)
system(command)

command = '/users/rhiju/python/fasta2align.py  '+tag+'.muscle.fasta'
print(command)
system(command)

