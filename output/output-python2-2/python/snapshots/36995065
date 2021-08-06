#!/usr/bin/python

import string
from sys import argv,stderr
from os import popen,system
from os.path import exists,basename
from amino_acids import longer_names

assert( len(argv)>2)
pdbname = argv[1]
chainid = argv[2]

if (pdbname[-4:] != '.pdb' and pdbname[-8:] != '.pdb1.gz'):
    pdbname += '.pdb'

outfile = pdbname

removechain = 0
if argv.count('-nochain'):
    removechain = 1

ignorechain = 0
if argv.count('-ignorechain'):
    ignorechain = 1

netpdbname = '/net/wwpdb/' + pdbname[1:3] + '/' + pdbname
if not exists(netpdbname):
    netpdbname = pdbname

print 'Reading ... '+netpdbname

if netpdbname[-3:]=='.gz':
    lines = popen( 'zcat '+netpdbname,'r').readlines()
else:
    lines = open(netpdbname,'r').readlines()

outfile = string.lower( basename(outfile) )
outfile = outfile.replace('.pdb1.gz','.pdb')

outid = open( outfile, 'w')
print 'Writing ... '+pdbname

#fastafile = pdbname[0:4]+chainid+'.pdb.fasta'
#fastaid = open( fastafile, 'w')
fastaid = stderr
#print 'Writing ... '+fastafile
fastaid.write('>'+pdbname+'\n');

oldresnum = '   '
count = 0;

if chainid == '_':
    chainid = ' '

for i in range(len(lines)):

    line = lines[i]

    if len(line)>5 and line[:6]=='ENDMDL':break #Its an NMR model.

    if ( ((len(line)>22) and chainid == line[21]) or ignorechain):
        line_edit = line
        if line[0:3] == 'TER':
            continue
        elif (line[0:6] == 'HETATM'):
            if (line[17:20]=='MSE'): #Selenomethionine
                line_edit = 'ATOM  '+line[6:17]+'MET'+line[20:]
                if (line_edit[12:14] == 'SE'):
                    line_edit = line_edit[0:12]+' S'+line_edit[14:]
                if len(line_edit)>75:
                    if (line_edit[76:78] == 'SE'):
                        line_edit = line_edit[0:76]+' S'+line_edit[78:]
            else:
                continue # No other hetatms allowed.



        if line_edit[0:4] == 'ATOM' or line_edit[0:6] == 'HETATM':

            if line_edit[13:14]=='P': #Nucleic acid? Skip.
                resnum = line_edit[23:26]
                oldresnum = resnum
                while (resnum == oldresnum):
                    i += 1
                    line = lines[i]
                    resnum = line_edit[23:26]

            resnum = line_edit[23:26]
            if not resnum == oldresnum:
                count = count + 1
                longname = line_edit[17:20]
                if longer_names.has_key(longname):
                    fastaid.write( longer_names[longname] );
                else:
                    fastaid.write( 'X')
            oldresnum = resnum


            if line_edit[16:17] == 'A':
                line_edit = line_edit[:16]+' '+line_edit[17:]

            if line_edit[16:17] != ' ':
                continue

            newnum = '%3d' % count
            line_edit = line_edit[0:23] + newnum + line_edit[26:]
            if removechain:
                line_edit = line_edit[0:21]+' '+line_edit[22:]

            #Fix occupancy
            line_edit = line_edit[:56]+'1.00'+line_edit[60:]

            outid.write(line_edit)


fastaid.write('\n')
outid.close()
fastaid.close()
