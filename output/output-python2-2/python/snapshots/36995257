#!/usr/bin/python

from sys import argv
from os import system
from os.path import abspath, dirname,basename,exists
from glob import glob

indir = argv[1]
outdir = argv[2]

#assert( dirname( abspath(outdir)) == '/work/casp7/fragout')

command = 'mkdir '+outdir
print(command)
system(command)

proteinname = basename( abspath(outdir))
fivelettercode = proteinname[-5:]
prefix = proteinname[:-5]

infile = glob(indir+'/*.fasta')[0]
outfile = outdir+'/'+prefix+fivelettercode+'.fasta'
command = 'cp '+infile+' '+outfile
print(command)
system(command)
command = 'gzip -f '+outfile
print(command)
system(command)


infile = glob(indir+'/*.psipred_ss2')[0]
outfile = outdir+'/'+prefix+fivelettercode+'.psipred_ss2'
command = 'cp '+infile+' '+outfile
print(command)
system(command)
command = 'gzip -f '+outfile
print(command)
system(command)

fourlettercode = fivelettercode[:4]
infile = indir+'/'+ fourlettercode+'.pdb'
if exists(infile):
    outfile = outdir+'/'+ prefix + fourlettercode+'.pdb'
    command = 'cp '+infile+' '+outfile
    print(command)
    system(command)
    command = 'gzip -f '+outfile
    print(command)
    system(command)


infile = glob(indir+'/*aa*03_05.200_v1_3')[0]
outfile = outdir+'/'+prefix+'aa'+fivelettercode+'03_05.200_v1_3'
command = '/work/boinc/bin/reduce_fragment_library_size.pl '+infile+' > '+outfile
print(command)
system(command)
command = 'gzip -f '+outfile
print(command)
system(command)

infile = glob(indir+'/*aa*09_05.200_v1_3')[0]
outfile = outdir+'/'+prefix+'aa'+fivelettercode+'09_05.200_v1_3'
command = '/work/boinc/bin/reduce_fragment_library_size.pl '+infile+' > '+outfile
print(command)
system(command)
command = 'gzip -f '+outfile
print(command)
system(command)


infile = glob(indir+'/*aa*03_05.200_v1_3')[0]
outfile = outdir+'/boinc_'+prefix+'aa'+fivelettercode+'03_05.200_v1_3'
command = '/work/boinc/bin/reduce_fragment_library_size.pl '+infile+' > '+outfile
print(command)
system(command)
command = 'gzip -f '+outfile
print(command)
system(command)

infile = glob(indir+'/*aa*09_05.200_v1_3')[0]
outfile = outdir+'/boinc_'+prefix+'aa'+fivelettercode+'09_05.200_v1_3'
command = '/work/boinc/bin/reduce_fragment_library_size.pl '+infile+' 25 > '+outfile
print(command)
system(command)
command = 'gzip -f '+outfile
print(command)
system(command)



# Other secstruct files
infile = glob(indir+'/*.rdb')
if len(infile)>0:
    infile = infile[0]
    outfile = outdir+'/'+prefix+fivelettercode+'.rdb'
    command = 'cp '+infile+' '+outfile
    print(command)
    system(command)
    command = 'gzip -f '+outfile
    print(command)
    system(command)


# Other secstruct files
infile = glob(indir+'/*.prof_rdb')
if len(infile)>0:
    infile = infile[0]
    outfile = outdir+'/'+prefix+fivelettercode+'.prof_rdb'
    command = 'cp '+infile+' '+outfile
    print(command)
    system(command)
    command = 'gzip -f '+outfile
    print(command)
    system(command)

# Other secstruct files
infile = glob(indir+'/*.jufo_ss')
if len(infile)>0:
    infile = infile[0]
    outfile = outdir+'/'+prefix+fivelettercode+'.jufo_ss'
    command = 'cp '+infile+' '+outfile
    print(command)
    system(command)
    command = 'gzip -f '+outfile
    print(command)
    system(command)


