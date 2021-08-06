#!/usr/bin/python

from sys import argv,exit
from os import popen, system
from os.path import basename,exists
import string
import commands
from glob import glob

def Help():
    print
    print 'Usage: '+argv[0]+' <silent out file 1> < silent file 2> ... <N> '
    print '  Will extract N decoys with lowest score from each silent file.'
    print '  If you want to select based on another column, say 12 (Rg), the'
    print '    last arguments should be -12 <N>  (for lowest Rg) or +12 <N>'
    print '    (for highest Rg).'
    print

    exit()


if len(argv)<2:
    Help()

replace_names = 1
if argv.count('-no_replace_names'):
    pos = argv.index('-no_replace_names')
    del( argv[pos] )
    replace_names = 0


extract_first_chain = 0
if argv.count('-extract_first_chain'):
    pos = argv.index('-extract_first_chain')
    del( argv[pos] )
    extract_first_chain = 1

use_start_pdb = 0
if argv.count('-start_pdb'):
    pos = argv.index('-start_pdb')
    del( argv[pos] )
    start_pdb_file = argv[ pos ]
    del( argv[pos] )
    use_start_pdb = 1

try:
    NSTRUCT = int(argv[-1])
    del(argv[-1])
except:
    NSTRUCT = 2

scorecol_defined = 0
try:
    scorecol = int(argv[-1])
    del(argv[-1])
    scorecol_defined = 1
except:
    scorecol = -1


REVERSE = ''
if scorecol > 0:
    REVERSE = ' --reverse '

#Another possibility... user supplies -rms or +rms
scorecol_name_defined = 0
if not scorecol_defined:
    scorecol_name = argv[-1]
    if scorecol_name[0] == '-':
        scorecol_name_defined = 1
        scorecol_name = scorecol_name[1:]
        del( argv[-1] )
        REVERSE = ''
    if scorecol_name[0] == '+':
        scorecol_name_defined = 1
        scorecol_name = scorecol_name[1:]
        REVERSE = '-r'
        del( argv[-1] )

infiles = argv[1:]


for infile in infiles:
    tags = []

    scoretags = string.split( popen('head -n 2 '+infile).readlines()[1] )
    scoretag=''
    if scorecol_defined:
        scoretag = scoretags[ abs(scorecol) ]

    if scorecol_name_defined:
        scorecol_names = string.split( scorecol_name,',' )
        scorecols = []
        for s in scorecol_names:
            assert( scoretags.count( s ))
            scorecol = scoretags.index( s )
            scorecols.append( scorecol )
        scoretag = scorecol_name
    else:
        scorecols  = [scorecol]

    assert(infile[-3:] == 'out')
#    lines = popen('grep SCORE '+infile+' |  sort -k %d -n %s | head -n %d' % (abs(SCORECOL)+1, REVERSE, NSTRUCT+1) ).readlines()


    # Check if this run appeared to use -termini
    terminiflag = ''
    fid = open( infile, 'r')
    line = 'ATOM'
    while (line.count('ATOM') or line.count('SCORE') or
           line.count('SEQU') or line.count('JUMP') or line.count('FOLD')):
        line = fid.readline()
    if line.count('AAV'):
        terminiflag = ' -termini '


    # Make the list of decoys to extract
    lines = popen( 'grep SCORE '+infile+' | grep -v NATIVE').readlines()

    score_plus_lines = []
    for line in lines:
        cols = string.split( line )
        score = 0.0
        try:
            for scorecol in scorecols: score += float( cols[ abs(scorecol) ] )
        except:
            continue
        if REVERSE: score *= -1
        score_plus_lines.append( ( score, line ))

    score_plus_lines.sort()
    lines = map( lambda x:x[-1], score_plus_lines[:NSTRUCT] )

    templist_name = 'temp.%s.list'% basename(infile)

    fid = open(templist_name,'w')
    count = 0
    for line in lines:
        cols = string.split(line)
        tag = cols[-1]
        if tag.find('desc') < 0:
            fid.write(tag+'\n')
            tags.append(tag)
            count = count+1
        if count >= NSTRUCT:
            break
    outfilename = infile

    fid.close()

    startpdbflag = ''
    if (use_start_pdb): startpdbflag = '-start_pdb '+start_pdb_file

    extract_first_chain_tag = ''
    if (extract_first_chain): extract_first_chain_tag = ' -extract_first_chain '

    #Set up bonds file?
    softlink_bonds_file = 0
    wanted_bonds_file = infile+'.bonds'
    wanted_rot_templates_file = infile+'.rot_templates'
    bonds_files = glob( '*.bonds')
    if len( bonds_files ) > 0:
        if not exists( wanted_bonds_file ):
            softlink_bonds_file = 1
            system( 'ln -fs '+bonds_files[0]+' '+wanted_bonds_file )
            system( 'ln -fs '+bonds_files[0].replace('.bonds','.rot_templates') \
                    +' '+wanted_rot_templates_file )


    EXE = '/work/rhiju/rosetta++/rosetta.gcc'
    if not exists( EXE ):
        EXE = 'rm boinc* ros*txt; ~rhiju/rosetta++/rosetta.mactelboincgraphics '

    command = '%s -extract -l %s -paths ~rhiju/paths.txt -s %s %s %s '% (EXE, templist_name,outfilename, terminiflag, startpdbflag+extract_first_chain_tag)



    # Check if this is an RNA run.
    fid = open( infile, 'r')
    line = fid.readline(); # Should be the sequence.
    if (line.count('a') or line.count('c') or
        line.count('g') or line.count('u')):
        command  += ' -enable_dna -enable_rna '
#        command = command.replace('rosetta++','rosetta_rna')

    # Check if this is full atom.
    lines = popen('head -n 8 '+outfilename).readlines()
    if len(string.split(lines[6])) > 10:
        command += ' -fa_input'


    # Hey this could be a new mini RNA file
    if (scoretags.count('rna_torsion') or scoretags.count('rna_base_axis') ):
        MINI_EXE = '~rhiju/src/mini/bin/rna_test.linuxgccrelease'
        command = '%s -database ~rhiju/minirosetta_database/ -in::file::silent %s -tags %s  -extract' % \
                  ( MINI_EXE, outfilename, string.join( tags ) )

    print(command)
    system(command)


    if outfilename.find('t343')>0:
        command = '/work/rhiju/python/extract_t343.py %s %s' % (outfilename,
                                                                 string.join(tags,' '))
        print(command)
        system(command)


    count = 1
    if replace_names:
        for tag in tags:
            if scorecol_defined or scorecol_name_defined:
                command = 'mv %s.pdb %s.%s.%d.pdb' % (tag,basename(infile),scoretag,count)
            else:
                command = 'mv %s.pdb %s.%d.pdb' % (tag,basename(infile),count)
            print(command)
            system(command)
            count += 1

    command = 'rm '+templist_name
    print(command)
    system(command)

    if (softlink_bonds_file):
        #system( 'rm '+wanted_bonds_file+' '+wanted_rot_templates_file )
        print ' WARNING! WARNING'
        print ' Found a .bonds and .rot_templates file and used it!'

