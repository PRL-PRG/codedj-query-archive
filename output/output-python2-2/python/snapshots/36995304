#!/usr/bin/python

from sys import argv,exit
from os import getcwd,chdir,system
from os.path import abspath,exists,basename,dirname
from glob import glob
import string

def Help():
    print
    print argv[0],' <out_file1> <out_file2> ... <align_file> <hom001 fasta_file> '
    print
    print ' The outfile names must contain the words hom001, hom002, etc.'
    print ' The align_file needs to have the aligned sequences in the same order.'
    print ' The fasta file must be gzipped, and present in the same directory'
    print '  as boinc-ready fragment files and psipred files.'
    print
    exit()

NUM_EXTRACT_DECOYS = 50

if len(argv) < 4:
    Help()

out_files = argv[1:-2]
align_file = abspath(argv[-2])
fasta_file = abspath(argv[-1]) # Must also be in same directory with fragments and psipred

#########################
# First extract decoys!
#########################
print 50*'*'
print 'Extracting Decoys'
print 50*'*'

CWD = getcwd()

homnames = []
for out_file in out_files[1:]: #Skip hom001
# Back to starting directory
    chdir(CWD)

    out_file = abspath( out_file )
    hompos = out_file.find('hom')
    assert( hompos > -1 ) # Filename must contain hom!!!

    homname = out_file[hompos:hompos+6]
    print 'DOING... ',homname
    homnames.append( homname )


    #Make a directory
    if not exists(homname):
        command = 'mkdir '+homname
        system(command)
        print(command)

    if not exists( homname+'/original_sequence' ):
        command = 'mkdir '+homname+'/original_sequence'
        system(command)
        print(command)

    chdir( homname )

#Check if extraction has already been carried out!
    pdbnames = glob( 'original_sequence/*.pdb' )
    if len(pdbnames) < NUM_EXTRACT_DECOYS:

        command = '/work/rhiju/looprelax_scripts/extract_lowscore_decoys.py %s %d -no_replace_names' %(out_file,NUM_EXTRACT_DECOYS)
        print(command)
        system(command)

        pdbnames = glob( '*.pdb' )
        for pdbname in pdbnames:
            command = 'mv '+pdbname+' original_sequence/'+homname+'_'+pdbname
            print(command)
            system(command)


#########################
# Create Zones Files
#########################
print 50*'*'
print 'Creating Zones Files'
print 50*'*'
for homname in homnames:
    chdir(CWD)
    chdir( homname )
    print 'DOING ...', homname

    homnum = int( homname[3:6] )

    #Specialized fasta file necessary for Bin's script.
    #This is a little roundabout, but it works.
    align_fasta_name = homname+'_align.fasta'
    tempid = open(align_fasta_name,'w')

    lines = open(align_file).readlines()
    for num in [1, homnum]:
        cols = string.split( lines[ num - 1] )
        tempid.write( '>ALIGN %d %s\n' % (num,cols[2]) )
        tempid.write(cols[1]+'\n') # The sequence
    tempid.close()

    zone_file_name = homname+'.zones'
    command = 'perl /work/rhiju/looprelax_scripts/fastaAln2zones.pl  '+align_fasta_name+' '+zone_file_name
    print(command)
    system(command)




#########################
# Create Mapped PDB's
#########################
print 50*'*'
print 'Creating Mapped PDBs'
print 50*'*'
for homname in homnames:
    chdir(CWD)
    chdir( homname )
    print 'DOING ...', homname

    #Grab the fasta.
    command = 'cp -rf '+fasta_file+' . '
    print(command)
    system(command)


    local_fasta_file = basename(fasta_file)
    if fasta_file[-3:] == '.gz':
        command = 'gunzip -f '+local_fasta_file
        print(command)
        system(command)
        local_fasta_file = local_fasta_file[:-3]

    zone_file_name = homname+'.zones'

    #Alright, do the mapbacks
    if not exists('mapback'):
        command = 'mkdir mapback'
        system(command)
        print(command)

    pdb_files =  glob('original_sequence/*.pdb')
    for pdb_file in pdb_files:
        output_pdb_file = 'mapback/mapback_'+basename(pdb_file)
        if not exists(output_pdb_file) and not exists(output_pdb_file+'.gz'):
            command = 'perl /work/rhiju/looprelax_scripts//createTemplate.pl  -zonesfile %s -fastafile %s  -parentpdb %s -outpdb %s -takeoffpad F' % \
                      (zone_file_name, local_fasta_file, pdb_file, output_pdb_file)
            print(command)
            system(command)





#########################**
# Create Obligate LoopFile!
#########################**
print 50*'*'
print 'Creating Obligate Loop Files'
print 50*'*'
#local_fasta_file is defined above.
for homname in homnames:
    chdir(CWD)
    chdir( homname )
    print 'DOING ...', homname

    zone_file_name = homname+'.zones'

    pdb_files =  glob('mapback/mapback*.pdb')
    for pdb_file in pdb_files:
        obligate_loop_file_name = pdb_file[:-4]+'.obligate_loopfile'
        if not exists(obligate_loop_file_name):
            command = 'perl /work/rhiju/looprelax_scripts/zones2loopfile.pl -query '+local_fasta_file+' -zones '+zone_file_name+' > '+obligate_loop_file_name
            print(command)
            system(command)





#########################**
# Create (DSSP) LoopFile!
#########################**
print 50*'*'
print 'Creating DSSP Loop Files'
print 50*'*'
#local_fasta_file is defined above.
for homname in homnames:
    chdir(CWD)
    chdir( homname )
    print 'DOING ...', homname

    pdb_files =  glob('mapback/mapback*.pdb')
    for pdb_file in pdb_files:
        loop_file_name = pdb_file[:-4]+'.loopfile'
        if not exists(loop_file_name):
            command = 'perl /work/rhiju/looprelax_scripts/dssp2loopfile.pl  -pdbfile %s  -fastafile %s  -outfile %s' % (pdb_file, local_fasta_file, loop_file_name)

            print(command)
            system(command)


#########################**
# gzip
#########################**
print 50*'*'
print 'Gzipping PDBs, Loop files, Obligate loop files'
print 50*'*'
#local_fasta_file is defined above.
for homname in homnames:
    chdir(CWD)
    chdir( homname )
    print 'DOING ...', homname

    pdb_files =  glob('mapback/mapback*.pdb')
    for pdb_file in pdb_files:
        command = 'gzip -rf '+pdb_file
        print(command)
        system(command)

    loop_files =  glob('mapback/mapback*loopfile')
    for loop_file in loop_files:
        command = 'gzip -rf '+loop_file
        print(command)
        system(command)

#########################**
# Boinc submit files
#########################**
print 50*'*'
print 'Making files for boinc submission'
print 50*'*'
chdir(CWD)

command = 'make_boinc_submit_startstructure.py '+fasta_file+' -loop_relax '
pdb_files = glob('*/mapback/*pdb.gz')
pdb_files.sort()
prefix = basename(fasta_file).replace('.fasta.gz','')

command_make_boinc_submit = command
for pdb_file in pdb_files:
    command_make_boinc_submit += ' '+pdb_file
command_make_boinc_submit += ' -queue 50 > MAPBACK_'+prefix+'.boinc.submit'
print( command_make_boinc_submit )
system( command_make_boinc_submit)


command_make_ralph_submit = command
for pdb_file in pdb_files[0:5]+pdb_files[-5:]:
    command_make_ralph_submit += ' '+pdb_file
command_make_ralph_submit += ' -queue 10 > MAPBACK_'+prefix+'.ralph.submit'
print( command_make_ralph_submit )
system( command_make_ralph_submit)



