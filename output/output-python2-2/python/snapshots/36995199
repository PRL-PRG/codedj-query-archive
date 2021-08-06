#!/usr/bin/python
#
#
# ./setup_native_relax_CASP7.py  2hh6.pdb hom001_t283_.fasta.gz  niau -submit
#

import sys
import string
from os.path import exists, abspath
from os import system,getcwd

submit = 0
if sys.argv.count('-submit'):
    pos = sys.argv.index('-submit')
    del(sys.argv[pos])
    submit = 1

getresults = 0
if sys.argv.count('-getresults'):
    pos = sys.argv.index('-getresults')
    del(sys.argv[pos])
    getresults = 1

pdb_file = sys.argv[1]
fasta_file = sys.argv[2]

rsync_to_cluster = 0
if sys.argv.count('-rsync'):
    pos = sys.argv.index('-rsync')
    del(sys.argv[pos])
    rsync_to_cluster = 1

if len(sys.argv) > 3:
    clustername = sys.argv[3]

#make native match fasta
fastaname = string.split( fasta_file, '.')[0]
newchain  = fastaname[-1]
fourlettercode = fastaname[-5:-1]
prefix = fastaname[:-5]

native_pdb_file = prefix+ fourlettercode+'.pdb'
if not exists(native_pdb_file):
    command = 'python /work/rhiju/python/make_native_match_fasta.py '+pdb_file+' A '+fasta_file
    print(command)
    system(command)

#idealize
idealized_native_pdb_file = 'id'+prefix+fourlettercode+'_0001.pdb'
if not exists(idealized_native_pdb_file):
    command = '/work/rhiju/rosetta++/rosetta.gcc id ' + fourlettercode + ' '+newchain+ ' -idealize -nstruct 1 -paths /work/rhiju/paths.txt -s '+native_pdb_file
    print(command)
    system(command)

#nativerelax -- create condor file.
prefix_args = ""

if len(prefix) > 0:
    prefix_args = '-frags_name_prefix '+prefix+' -protein_name_prefix '+prefix

#Check form boinc prefix...
fragfile = 'boinc_'+prefix+'aa'+fourlettercode+newchain+'03_05.200_v1_3'
if exists(fragfile) or exists(fragfile+'.gz'):
    prefix_args = prefix_args.replace('-frags_name_prefix ', '-frags_name_prefix boinc_')


if rsync_to_cluster:
    fid = open('jobNATIVERELAX','w')
    fid.write('universe     = vanilla\n')
    fid.write('\n')
    fid.write('Notify_user  = rhiju@u.washington.edu\n')
    fid.write('notification = Error\n')
    fid.write('\n')
    fid.write('Requirements = ( Memory > 248 )\n')
    fid.write('Log          = condorscript.log\n')
    fid.write('Error        = condorscript.err\n')
    fid.write('Executable   = /users/rhiju/rosetta++/rosetta.gcc\n')
    fid.write('\n')
    fid.write('arguments =  rn %s %s -silent -relax  -output_chi_silent -stringent_relax -vary_omega -omega_weight 0.5 -farlx -ex1 -ex2 -termini -short_range_hb_weight 0.50 -long_range_hb_weight 1.0 -no_filters -nstruct 500 -record_rms_before_relax -s %s %s\n' % (fourlettercode,newchain,idealized_native_pdb_file,prefix_args))
    fid.write('Queue 16\n')
    fid.write('\n')
    fid.close()

    system('cp /work/rhiju/paths.txt .')

    fullpath = abspath( getcwd())
    fullpath = fullpath.replace('/work/','/users/')
    command = 'ssh %s mkdir -p %s'  % (clustername,fullpath)
    print command
    system(command)

    command = 'rsync -avz . %s:%s --exclude="*out" --exclude="*ps"'  % (clustername,fullpath)
    print command
    system(command)

if submit:
    fullpath = abspath( getcwd())
    fullpath = fullpath.replace('/work/','/users/')
    command = 'ssh %s "cd %s; condor_submit jobNATIVERELAX"'  % (clustername,fullpath)
    print command
    system(command)


if getresults:
    fullpath = abspath( getcwd())
    fullpath = fullpath.replace('/work/','/users/')
    command = 'rsync -avz %s:%s/rn*out .'  % (clustername,fullpath)
    print command
    system(command)

