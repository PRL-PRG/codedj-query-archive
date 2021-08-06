#!/usr/bin/python

from sys import argv,exit
from os import system,getcwd,chdir
from os.path import basename,dirname,abspath,exists
import string

watson_crick_reweight_defined = 0
if (argv.count('-watson_crick_reweight')):
    pos = argv.index('-watson_crick_reweight')
    del(argv[pos])
    watson_crick_reweight = float( argv[pos] )
    del(argv[pos])
    watson_crick_reweight_defined = 1

match_tag = ' -match_YR '
if (argv.count('-match_all')):
    pos = argv.index('-match_all')
    del(argv[pos])
    match_tag = ' -match_all '

excise_SRL = 0
if (argv.count('-excise_SRL')):
    pos = argv.index('-excise_SRL')
    del(argv[pos])
    excise_SRL = 1


fastafiles = argv[1:]

cwd = getcwd()

###############################################################
# Pick fragments in one shot
###############################################################
fid = open('list','w')
for fastafile in fastafiles:
    fid.write(fastafile+'\n')
fid.close()

if excise_SRL:
    command = 'ln -fs /work/rhiju/projects/rna_tests/benchmark1/1ffk_exciseSRL_RNA.pdb 1ffk_RNA.pdb'
else:
    command = 'ln -fs /work/rhiju/projects/rna_tests/benchmark1/1ffk_RNA.pdb .'
print(command)
system(command)


fragmentprefix = ''
watson_crick_reweight_tag = ''
if (watson_crick_reweight_defined):
    watson_crick_reweight_tag = ' -watson_crick_reweight %f ' % watson_crick_reweight
    fragmentprefix = 'wc%3.1f_' % watson_crick_reweight

command = '/work/rhiju/src/rosetta_frozenrotamer/rosetta.gcc '+ \
          ' -paths /work/rhiju/paths.txt -l list -prna -rna_vall 1ffk_RNA.pdb -pick_rna_fragments '+match_tag+'  '+watson_crick_reweight_tag
print(command)
system(command)

command = 'rm 1ffk_RNA.pdb .'
print(command)
system(command)


###############################################################
# Move to individual directories and gzip.
###############################################################
for fastafile in fastafiles:
    fastafile = abspath( fastafile )
    RNAname = basename(fastafile).replace('.fasta','');
    pdbfile = fastafile[:-7] + '_RNA.pdb';
    pairingfile = fastafile[:-6] + '_pairing.pdat';
    fragmentfile = fastafile+'.fragments'

    if not exists(RNAname):
        command = 'mkdir '+RNAname
        print(command)
        system(command)

    chdir( RNAname)
    if not exists(basename(fastafile)+'.gz'):
       command = ' cp '+abspath(fastafile)+' . '
       print(command)
       system(command)

       command = ' gzip -f '+basename(fastafile)
       print(command)
       system(command)

    if not exists(basename(pdbfile)+'.gz'):
       command = ' cp '+abspath(pdbfile)+' . '
       print(command)
       system(command)

       command = ' gzip -f '+basename(pdbfile)
       print(command)
       system(command)

    if exists(pairingfile) and not exists(basename(pairingfile)+'.gz'):
       command = ' cp '+abspath(pairingfile)+' . '
       print(command)
       system(command)

       command = ' gzip -f '+basename(pairingfile)
       print(command)
       system(command)

    newfragmentfile = fragmentprefix + basename( fastafile ).replace('.fasta','_1ffk.fragments')
    if (excise_SRL):
        newfragmentfile = newfragmentfile.replace('_1ffk','_1ffk_exciseSRL')

    if not exists(newfragmentfile+'.gz'):
        command = ' mv '+abspath(fragmentfile)+' '+newfragmentfile
        print(command)
        system(command)

        command = ' gzip -f '+newfragmentfile
        print(command)
        system(command)


    chdir( cwd)

