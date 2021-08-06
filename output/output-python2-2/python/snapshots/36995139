#!/usr/bin/python

from sys import argv
from os import system,chdir,getcwd
from os.path import basename,abspath,exists
from glob import glob

indir = argv[1]
outdir = argv[2]

rsync_to_cluster = 0
if len(argv)>3:
    clustername = argv[3]
    rsync_to_cluster = 1

command = 'mkdir -p '+ outdir
print command
system(command)

fastafile = basename( glob( indir+'/*fasta')[0] )


fourlettercode = fastafile[0:4]
chain = fastafile[4]
fivelettercode = fastafile[0:5]

infile = indir+'/'+ fivelettercode+'.fasta'
command = 'rsync -avz '+infile+' '+outdir
print command
system(command)

infile = indir+'/'+ fivelettercode+'.psipred_ss2'
command = 'rsync -avz '+infile+' '+outdir
print command
system(command)

infile = indir+'/'+ fourlettercode+'.pdb'
command = 'rsync -avz '+infile+' '+outdir
print command
system(command)

infile = indir+'/'+ 'aa'+fivelettercode+'03_05.200_v1_3'
command = 'rsync -avz '+infile+' '+outdir
print command
system(command)

infile = indir+'/'+ 'aa'+fivelettercode+'09_05.200_v1_3'
command = 'rsync -avz '+infile+' '+outdir
print command
system(command)

command = 'rsync -avz /users/rhiju/paths.txt '+outdir
print command
system(command)

chdir(outdir)

pdbfile = fourlettercode+'.pdb'
idealizedpdbfile = 'idealized_'+pdbfile

if not exists(idealizedpdbfile):
    command = '/users/rhiju/rosetta++/rosetta.gcc id %s %s -idealize -nstruct 1 -score -fa_input -s  %s' % (fourlettercode,chain,pdbfile)
    print command
    system(command)

    command = 'mv id'+fourlettercode+'_0001.pdb '+idealizedpdbfile
    print command
    system(command)

fid = open('jobABRELAX','w')
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
fid.write('arguments =  ax %s %s -silent  -new_centroid_packing -abrelax  -output_chi_silent -stringent_relax -vary_omega -omega_weight 0.5 -farlx -ex1 -ex2 -termini -short_range_hb_weight 0.50 -long_range_hb_weight 1.0 -rg_reweight 0.5 -rsd_wt_helix 0.5 -rsd_wt_loop 0.5 -output_all -accept_all -nstruct 100000 -record_rms_before_relax\n' % (fourlettercode,chain))
fid.write('Queue 48\n')
fid.write('\n')
fid.close()

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
fid.write('arguments =  rn %s %s -silent -relax  -output_chi_silent -stringent_relax -vary_omega -omega_weight 0.5 -farlx -ex1 -ex2 -termini -short_range_hb_weight 0.50 -long_range_hb_weight 1.0 -no_filters -nstruct 500 -record_rms_before_relax -s %s -fa_input\n' % (fourlettercode,chain,idealizedpdbfile))
fid.write('Queue 8\n')
fid.write('\n')
fid.close()


if rsync_to_cluster:
    fullpath = abspath( getcwd())
    fullpath = fullpath.replace('/work/','/users/')
    command = 'ssh %s mkdir -p %s'  % (clustername,fullpath)
    print command
    system(command)

    command = 'rsync -avz . %s:%s'  % (clustername,fullpath)
    print command
    system(command)

#    command = 'ssh %s "cd %s; condor_submit jobNATIVERELAX"'  % (clustername,fullpath)
#    print command
#    system(command)

