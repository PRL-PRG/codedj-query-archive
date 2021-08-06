#!/usr/bin/python

from os import popen,system
from sys import argv,stderr
import string



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

    pdbfiles = argv[1:-1]

    prefix = argv[-1]
    startseq = 1
    endseq = 10000000000


use_excise = 0
excise_residues = []
if argv.count('-excise'):
    use_excise = 1
    pos = argv.index('-excise')
    del argv[pos]

    stderr.write( 'PDBSLICE using a excise of residues: '  )
    goodint = 1
    while goodint:
        try:
            excise_residue = int(argv[pos])
            excise_residues.append( excise_residue )
            del argv[pos]
            stderr.write('%d ' % excise_residue )
        except:
            goodint = 0

    stderr.write( '\n'  )

    pdbfiles = argv[1:-1]

    prefix = argv[-1]
    startseq = 1
    endseq = 10000000000


if not use_excise and not use_subset:
    try:
        pdbfiles = argv[1:-2]
        startseq = int( argv[-2])
        endseq = int( argv[-1])
        prefix = 'truncate_'
    except:
        pdbfiles = argv[1:-3]
        startseq = int( argv[-3])
        endseq = int( argv[-2])
        prefix = argv[-1]

for pdbfile in pdbfiles:
    gzipped = 0
    outid = open(prefix+pdbfile,'w')


    if pdbfile[-2:] == 'gz':
        lines = popen('zcat '+pdbfile).readlines()
    else:
        lines = open(pdbfile).readlines()

    i = 0
    oldresidue = '   '
    for line in lines:
        currentresidue = line[22:26]
        if not currentresidue == oldresidue:
            i += 1
        oldresidue = currentresidue

        if (use_subset and not ( i in subset_residues) ): continue

        try:
            currentresidue_val = int(currentresidue)
            if (use_excise and ( currentresidue_val in excise_residues) ): continue
            if int(currentresidue) < startseq or int(currentresidue) > endseq: continue
        except:
            continue

        outid.write(line)

    outid.close()

    if gzipped:
        command = 'gzip -f '+outfile
        system(command)
