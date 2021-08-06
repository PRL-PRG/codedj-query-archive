#!/usr/bin/python

from sys import argv,stderr
from os import system
import string


def Help():
    print argv[0]," <pdb1> <pdb2> ... "
    print '  Shrinks sidechains to gly, ala, or ser, for use in molecular replacement.'
    print '  Note: all pdbs must have the same sequence.'


######################################################################3
# Completely optional -- don't use all residues.
use_subset = 0
subset_residues = []
if argv.count('-subset'):
    use_subset = 1
    pos = argv.index('-subset')
    del argv[pos]

    stderr.write( 'PDBSLICE using a subset of residues: '  )
    goodint = 1
    while goodint:
        try:
            subset_residue = int(argv[pos])
            subset_residues.append( subset_residue )
            del argv[pos]
            stderr.write('%d ' % subset_residue )
        except:
            goodint = 0

    stderr.write( '\n'  )

######################################################################3
pdbfiles = argv[1:]
if len(pdbfiles) < 1:
    Help()

map_res = {'ALA':'ALA','CYS':'SER','ASP':'SER','GLU':'SER','PHE':'SER',\
           'GLY':'GLY','HIS':'SER','ILE':'SER','LYS':'SER','LEU':'SER',\
           'MET':'SER','ASN':'SER','PRO':'SER','GLN':'SER','ARG':'SER',\
           'SER':'SER','THR':'SER','VAL':'SER','TRP':'SER','TYR':'SER'}

backbone_atoms = [' N  ',' CA ',' C  ',' O  ',' CB ']
additional_atom =  {'ALA':'BLAH','CYS':' SG ','ASP':' CG ','GLU':' CG ','PHE':' CG ',\
                    'GLY':'BLAH','HIS':' CG ','ILE':' CG1','LYS':' CG ','LEU':' CG ',\
                    'MET':' CG ','ASN':' CG ','PRO':' CG ','GLN':' CG ','ARG':' CG ',\
                    'SER':' OG ','THR':' OG1','VAL':' CG1','TRP':' CG ','TYR':' CG '}

for pdbfile in pdbfiles:

    new_pdbfile = pdbfile[:-4]+'_stripsidechain.pdb'

    if use_subset: new_pdbfile = new_pdbfile.replace('stripsidechain','stripsidechain_subset')

    fid = open( new_pdbfile,'w')

    lines = open(pdbfile).readlines()
    for line in lines:
        writeout = 0
        if line[:4]=='ATOM':
            old_res = line[17:20]
            new_res = map_res[ old_res ]
            atom = line[12:16]

            if atom in backbone_atoms:
                writeout = 1
                new_atom = atom

            if atom == additional_atom[ old_res ]:
                writeout = 1
                new_atom = ' OG '

            if use_subset and ( int( line[21:26] ) in subset_residues):
                writeout = 1
                new_atom = atom
                new_res = old_res

        if writeout:
            fid.write(line[:12]+new_atom+' '+new_res+line[20:])

    fid.close()

