#!/usr/bin/python

from sys import argv
from os.path import exists
from os import system,getcwd

#infiles = argv[1:]

CWD = getcwd()

local_id = open( 'jobMINI','w')
local_id.write( 'ssh fin "cd '+CWD)

count = -1

QUEUE = 20


NUM_DECOYS = 200;

for i in range( QUEUE ):
    count += 1
    outdir = 'OUTFILES/%d' % count
    command =  'mkdir -p '+outdir
    system( command )

    outfile = outdir+'/'+'tltr_dopenativefrags.out'

    command = './rna_test.linuxgccrelease -database ~/minirosetta_database -native tltr_RNA.pdb -fasta tltr_.fasta -params_file tltr_.prm -nstruct %d -cycles 5000 -out::file::silent %s -minimize_rna -vall_torsions 1jj2_2r8s.torsions ' %  ( NUM_DECOYS, outfile )

    script_file = 'MINI%d.sh' % count
    fid = open( script_file, 'w' )
    fid.write( 'cd '+CWD+'\n' )
    fid.write( command +'\n' )
    fid.close()

    local_id.write( ';qsub '+script_file )

local_id.write('"\n')

