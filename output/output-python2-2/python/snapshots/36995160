#!/usr/bin/python

from sys import argv
from glob import glob
from os import system
from os.path import basename,dirname,abspath

pdbfiles_in = glob('*.pdb')

pdbfiles_in.sort()

pdbfiles =[]
for pdbfile in pdbfiles_in:
    if pdbfile.find('model')<0 and pdbfile.find('maxsub')<0:
#   if pdbfile.find('maxsub')<0:
        pdbfiles.append( pdbfile )


print pdbfiles

fid = open('order.list','w')

for pdbfile in pdbfiles:
    fid.write( pdbfile+'\n')

fid.close()


oligomer_flag = ''
if argv.count('-oligomer'):
    oligomer_flag = '-oligomer'

system('emacs order.list')

lines = open('order.list','r').readlines()

prefix = basename( dirname( abspath ( pdbfile)))

for i in range( len(lines) ):
    pdbname = lines[i][:-1]
    if i == 0:
        command = 'ln -fs %s %smodel_%d.pdb' % (pdbname,prefix,i+1)
    else:
        command = 'ln -fs %s %smodel_alternate_%d.pdb' % (pdbname,prefix,i+1)
    print command
    system(command)


command = '/work/rhiju/python/pdb2casp.py %smodel*_?.pdb %s -sg ' % (prefix,oligomer_flag)
print command
system(command)

command = 'ls %smodel*_?.casp > cartoons.list' % (prefix)
print command
system(command)

mypath = dirname(abspath(pdbfile))
command = 'ssh zaza mkdir -p '+mypath
print command
system(command)

command = 'rsync -avz . zaza:'+mypath
print command
system(command)

command = 'ssh dotty "cd %s; /users/rhiju/python/make_aligned_pictures_withmaxsub.py cartoons.list"' % mypath
print command
system(command)


command = 'rsync -avz zaza:%s/ . '% mypath
print command
system(command)

command = 'rm cartoons.ps '
print command
system(command)

command = '/work/rhiju/python/combine_new_plots.py cartoons.ps 3 2 ??.ps -l'
print command
system(command)

command = '/work/rhiju/python/printonphaser.py cartoons.ps'
print command
system(command)


command = 'tar cvfz rosetta_%ssubmit.tgz %smodel*_?.casp' % (prefix,prefix)
print command
system(command)

#command = 'ssh zaza copy_submissions_to_bqian.py %d' % int(prefix[1:4])
#print command
#system(command)




