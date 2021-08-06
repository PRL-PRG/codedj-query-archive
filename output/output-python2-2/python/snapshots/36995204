#!/usr/bin/python

import sys
import string
from os import system
from os.path import basename,abspath

system( 'source ~/.bashrc' )

datafile = sys.argv[1]
seqfile  = sys.argv[2]
modelfile = sys.argv[3]

N_ASU = 1
if len(sys.argv) > 4:
    N_ASU = int( sys.argv[4])

workdir = basename(modelfile).replace('.pdb','')
command = 'mkdir -p '+workdir
print( command )
system(command)

workdir = abspath( workdir )
workdir = workdir.replace( '/work/rhiju/', '~rhiju/' )
workdir = workdir.replace( '/Users/rhiju/', '~rhiju/' )
workdir = workdir.replace( '/users/rhiju/', '~rhiju/' )
workdir = workdir.replace( '/work1/rhiju/', '~rhiju/' )

for infile in [datafile, seqfile, modelfile]:
    command = 'rsync -avzL '+infile+' '+workdir+'/'+basename(infile)
    print( command )
    system(command)

lines = open(seqfile).readlines()
lines = map( lambda x:x[:-1], lines[2:])
numres = len( string.join( lines ) )


command = 'auto_tracing.sh datafile %s  residues %d workdir %s fp F sigfp SIGF modelin %s seqin %s cgr %d' % \
    (basename(datafile), numres, workdir, basename(modelfile), basename(seqfile), N_ASU)

print command
system( command )
