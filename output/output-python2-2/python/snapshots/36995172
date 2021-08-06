#!/usr/bin/python

from sys import argv
from os import system,popen
from os.path import basename
import string

trimN = 0
trimC = 0
for i in range(len(argv)):
    try:
        trimN = int( argv[i] )
        del( argv[i] )
        break
    except:
        continue

for i in range(len(argv)):
    try:
        trimC = int( argv[i] )
        del( argv[i] )
        break
    except:
        continue

use_align_file = 0
for i in range(len(argv)):
    if argv[i].find( 'align_extended')>0:
        align_file =  argv[i]
        use_align_file = 1
        del( argv[i] )
        break

maxsubtag ='-ms' # Don't calculate pseudomaxsubs
if argv.count('-maxsub'):
    i = argv.index('-maxsub')
    del( argv[i] )
    maxsubtag = ''


chaintag ='A' # Don't calculate pseudomaxsubs
if argv.count('-chain'):
    i = argv.index('-chain')
    del( argv[i] )
    chaintag = argv[i]
    del( argv[i] )


nativepdb = argv[1]
outfiles = argv[2:]


for outfile in outfiles:

    if not use_align_file:
        nativecoords = nativepdb.replace('.pdb','') + '.coords'
        command = 'make_coords_file.py  %s %s %s > %s' % (nativepdb,chaintag,outfile,nativecoords)
        print(command)
        system(command)

        command = 'cluster_info_silent.out  %s %s %s %s -trimN %d -trimC %d 0,0,0,0 0,0' % (outfile,nativecoords,basename(outfile),maxsubtag,trimN,trimC)
        print(command)
        system(command)
    else: #Oh man this is a pain in the ass....
        nativecoords = nativepdb.replace('.pdb','') + '.coords'

        if align_file[-3:] == '.gz':
            lines = popen('zcat '+align_file).readlines()
        else:
            lines = open(align_file).readlines()
        fastafile = 'temp.'+basename(outfile)+'.fasta'
        fid = open(fastafile,'w')
        fid.write('>blah\n')
        targetsequence = string.split(lines[0])[1]
        fid.write( targetsequence+'\n' )
        fid.close()

        command = 'make_coords_file.py  %s A %s > %s' % (nativepdb,fastafile,nativecoords)
        print(command)
        system(command)

        temp_align_file = 'temp.'+basename(outfile)+'.align_extended'
        fid = open(temp_align_file,'w')
        fid.write('ALIGN '+targetsequence+' NATIVE\n' )
        outfile_seq = string.split(popen('head -n 1 '+outfile).readlines()[0]) [-1]
        print outfile_seq
        for line in lines:
            seq = ''
            seq_with_dashes = string.split(line)[1]
            for char in seq_with_dashes:
                if not char == '-': seq+=char
            print seq
            if seq == outfile_seq:
                fid.write( 'ALIGN ' + seq_with_dashes + ' ' + outfile + '\n')
                break
        fid.close()

        command = 'cluster_info_silent.out  %s %s %s %s  -trimN %d -trimC %d 0,0,0,0 0,0' % (temp_align_file,nativecoords,maxsubtag,basename(outfile),trimN,trimC)
        print(command)
        system(command)

        command = 'rm %s %s ' % (temp_align_file, fastafile)
        print(command)
        system(command)

    if len(maxsubtag) < 1: #Maxsubs were calculated.
        command = '/users/rhiju/python/make_new_plot.py  %s.contacts -e -nopath' % (basename(outfile))
        print(command)
        system(command)


