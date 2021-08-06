#!/usr/bin/python

import string
import sys
from os import popen,system
from os.path import basename,exists
import pdb
from blast import NBAlign

if len(sys.argv) < 2:
    print '\n'+'-'*75
    print 'Usage: %s <fasta_file> <pdb1> <pdb2> ...'
    print '-'*75+'\n\n'
    sys.exit()

fasta_file = sys.argv[1]

pdb_files = sys.argv[2:]

fid = open('list','w')
for pdb_file in pdb_files:
    fid.write( pdb_file+'\n')
fid.close()

pdb_file = pdb_files[0]

chain = ' '


lines = popen('/work/pbradley/dssp '+pdb_file+' | grep "RESIDUE AA" -A10000 | '+\
              ' grep "^.[ 0-9][ 0-9][ 0-9][ 0-9]......'+\
              chain+'"').readlines()

#print string.join(lines,'')

lowercase = list('abcdefghijklmnopqrstuvwxyz')

seq = map(lambda x:x[13],lines)

for i in range(len(seq)):
    if seq[i] in lowercase:
        seq[i] = 'C'
seq = string.join(seq,'')

ss = string.join(map(lambda x:x[16],lines),'')

ss3 = ''
for a in ss:
    if a not in [' ','E','B','H','G','I','S','T']:
        sys.stderr.write('undefined ss character? '+a+'\n')
        ss3 = ss3+'L'
    elif a in ['E','B']:
        ss3 = ss3+'E'
    elif a in ['H','G']:
        ss3 = ss3+'H'
    else:
        ss3 = ss3+'L'

assert len(ss3) == len(seq)

if fasta_file == '-':
    silent_seq = seq
else:
    if fasta_file[-3:] == '.gz':
        lines = popen('zcat '+fasta_file,'r').readlines()
    else:
        lines = open(fasta_file,'r').readlines()


    line = lines[0]

    if line[0] == ">": ## fasta file
        silent_seq = string.join(map(lambda x:string.split(x)[0],
                                     lines[1:]),'')
    elif string.split(line)[0] == 'SEQUENCE:':
        silent_seq = string.split(line)[1]
    else:
        print 'bad silent file type'
        sys.exit()

al = NBAlign(silent_seq,seq)

sys.stderr.write('found dssp secondary structure for %d percent of sequence\n' \
                 %( (len(al.keys())*100)/len(silent_seq)))

coords = pdb.Get_full_coords(pdb_file,silent_seq,chain,0,0)
ca = {}
for pos in coords.keys():
    for a in coords[pos].keys():
        if string.split(a)[0] == 'CA':
            ca[pos] = coords[pos][a]
            break

sys.stderr.write('found coordinates for %d percent of sequence\n' \
                 %( (len(ca.keys())*100)/len(silent_seq)))




al_in_pdbfile = [ al[x] for x in al.keys() ]
align_seq_silent = '-'*min( al_in_pdbfile)
align_seq_pdb    = seq[0: min(al_in_pdbfile)]

for i in range(len(silent_seq)):
    align_seq_silent += silent_seq[i]
    if i in al.keys():
        align_seq_pdb += silent_seq[i]
    else:
        align_seq_pdb += '-'

align_seq_silent += '-' * (len(seq) - max(al_in_pdbfile) - 1)
align_seq_pdb    += seq[max(al_in_pdbfile)+1:]


print 'ALIGN '+ align_seq_silent+' '+fasta_file
print 'ALIGN '+ align_seq_pdb+' '+pdb_file

align_file = basename(pdb_file)+'.align_extended'
fid = open(align_file,'w')
fid.write( 'ALIGN '+ align_seq_silent+' '+fasta_file+'\n')
fid.write( 'ALIGN '+ align_seq_pdb+' '+pdb_file+'\n')
fid.close()

fastaname = string.split( fasta_file, '.')[0]
newchain  = fastaname[-1]
fourlettercode = fastaname[-5:-1]
prefix = fastaname[:-5]

command = 'rm pp'+fourlettercode+'.fasc'+' pp'+fourlettercode+'*pdb'
print(command)
system(command)

boinc_prefix = '';
boinc_frag_file =  'boinc_'+prefix+'aa'+fourlettercode+newchain+'03_05.200_v1_3'
if exists( boinc_frag_file )  or  exists( boinc_frag_file+'.gz' ):
    boinc_prefix = 'boinc_'

if len(prefix) > 0:
    command = '/work/rhiju/rosetta++/rosetta.gcc pp '+fourlettercode+' '+newchain+' -map_sequence '+align_file+' -paths /work/rhiju/paths.txt -score -nstruct 1 -fa_input -l list -omega_weight 0.5 -vary_omega -protein_name_prefix ' + prefix + ' -frags_name_prefix ' + boinc_prefix + prefix
else:
    command = '/work/rhiju/rosetta++/rosetta.gcc pp '+fourlettercode+' '+newchain+' -map_sequence '+align_file+' -paths /work/rhiju/paths.txt -score -nstruct 1 -fa_input -l list -omega_weight 0.5 -vary_omega '
print(command)
system(command)

#command = 'mv pptemp'+ string.split(pdb_file,'.pdb')[0]  + '_0001.pdb* ' +prefix+ fourlettercode+'.pdb'
#print(command)
#system(command)
