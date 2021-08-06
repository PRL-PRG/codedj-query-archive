#!/usr/bin/python

from sys import argv,exit
from os import getcwd,chdir,system
from os.path import abspath,exists,basename,dirname
from glob import glob
import string


pdbfile = argv[1]
segmentfile = argv[2]


##########################################################3
#Idealize pdbfile
print 25*'*'
print '   Idealizing pdb file '
print 25*'*'

idealized_pdbfile =  pdbfile.replace('.pdb','_ideal.pdb')
newpdbfile =  pdbfile.replace('.pdb','_0001.pdb')

if not exists( idealized_pdbfile):
    command = '/work/rhiju/rosetta++/rosetta.gcc -idealize -s %s -nstruct 1 -paths /work/rhiju/paths.txt' % pdbfile
    print command
    system( command )

    command = 'mv %s %s' % (newpdbfile, idealized_pdbfile)
    print command
    system( command )


##########################################################3
#Slicing pdbfile
print 25*'*'
print '   Slice out segment pdbs '
print 25*'*'

lines = open(segmentfile).readlines()
count = 0
for line in lines:
    count += 1

    cols = string.split( line )
    segment_start = int(cols[0])
    segment_end   = int(cols[1])

    prefix = 'helix%d_' % count

    command = '/work/rhiju/python/pdbslice.py %s %d %d %s' % \
              (idealized_pdbfile,segment_start,segment_end,prefix)
    print command
    system( command )



##########################################################3
#Pairwise pdbs
print 25*'*'
print '   Creating pairwise PDBs'
print 25*'*'

for i in range(1,count+1):
    for j in range(i+1,count+1):
        prefixAB = "H%dH%d_" % (i,j)
        helixApdb = 'helix%d_%s' % (i,idealized_pdbfile)
        helixBpdb = 'helix%d_%s' % (j,idealized_pdbfile)

        pairpdbfile = prefixAB+pdbfile
        fid = open( pairpdbfile, 'w' )

        linesA = open(helixApdb).readlines()
        linesB = open(helixBpdb).readlines()

        for line in linesA: fid.write(line)
        fid.write('TER\n')
        for line in linesB: fid.write(line)
        fid.write('TER\n')

        fid.close()

        pairpdb_unbound_file = pairpdbfile.replace('.pdb','.unbound.pdb')
        command = 'cp %s %s' % (pairpdbfile, pairpdb_unbound_file)
        print(command)
        system(command)

        fourlettercode = "H%dH%d" % (i,j)
        newpdbfile = fourlettercode+'.pdb'
        command = 'cp %s %s' % (pairpdbfile, newpdbfile)
        print(command)
        system(command)

        newpdbfile = fourlettercode+'.unbound.pdb'
        command = 'cp %s %s' % (pairpdbfile, newpdbfile)
        print(command)
        system(command)


##########################################################3
# Condor script
print 25*'*'
print '   Creating condor script'
print 25*'*'


command = 'cp /work/rhiju/paths.txt .'
system(command)
print(command)

fid = open('jobDOCK','w')

fid.write('########\n')
fid.write('# Condor script file for running rosetta runs\n')
fid.write('# Adapted from a script from B. Qian.\n')
fid.write('########\n')
fid.write('\n')
fid.write('universe     = vanilla\n')
fid.write('\n')
fid.write('Notify_user  = rhiju@u.washington.edu\n')
fid.write('notification = Error\n')
fid.write('\n')
fid.write('Log          = condorscript.log\n')
fid.write('\n')
fid.write('Executable   = /users/rhiju/rosetta++/rosetta.gcc\n')
fid.write('\n')


for i in range(1,count+1):
    for j in range(i+1,count+1):
        pairpdbfile = "H%dH%d.unbound.pdb" % (i,j)

        fourlettercode = "H%dH%d" % (i,j)

        fid.write('dd %s _ -s %s -dock -pose -dock_mcm -randomize1 -randomize2 -ex1 -ex2aro_only -dock_rtmin -dock_score_norepack -no_filters -output_all -accept_all -nstruct 10000 -silent  -pose_silent_out -use_score12 -seed_offset $(Process)\n' % (fourlettercode,pairpdbfile))
        fid.write('Queue 8')

fid.close()




##########################################################3
# Boinc submit
ralph_submit_file = 'PAIRWISEDOCK.%s.ralph.submit' % pdbfile.replace('.pdb','')
boinc_submit_file = 'PAIRWISEDOCK.%s.boinc.submit' % pdbfile.replace('.pdb','')

print 25*'*'
print '   Creating boinc and ralph scripts: '
print 25*'*'
print ralph_submit_file
print boinc_submit_file

fid = open(ralph_submit_file,'w')

for i in range(1,count+1):
    for j in range(i+1,count+1):
        prefixAB = "H%dH%d_" % (i,j)
        pairpdbfile = prefixAB+pdbfile
        pairpdb_unbound_file = pairpdbfile.replace('.pdb','.unbound.pdb')

        fourlettercode = string.split( pdbfile, '.pdb')[-2][-4:]

        fid.write('name = %s_PAIRWISE_DOCK_MCM\n' % pairpdbfile.replace('.pdb','') )
        fid.write('description = %s pairwise docking\n' % pairpdbfile)
        fid.write('inputfiles = ')
        fid.write( abspath(pairpdbfile)+';' )
        fid.write( abspath(pairpdb_unbound_file)+';' )
        fid.write( '/work/rhiju/description_DOCK.txt\n')

        fid.write('arguments = dd %s _ -s %s -dock -pose -dock_mcm -randomize1 -randomize2 -ex1 -ex2aro_only -dock_rtmin -dock_score_norepack -no_filters -output_all -accept_all -nstruct 100 -silent  -pose_silent_out -output_silent_gz -use_score12 -protein_name_prefix %s -description_file description_DOCK.txt\n' % (fourlettercode,pairpdb_unbound_file,prefixAB))

        fid.write('resultfiles = dd%s.out.gz\n' % fourlettercode );
        fid.write('Queue = 20\n')
        fid.write('\n')

fid.close()


fid = open(boinc_submit_file,'w')

for i in range(1,count+1):
    for j in range(i+1,count+1):
        prefixAB = "H%dH%d_" % (i,j)
        pairpdbfile = prefixAB+pdbfile
        pairpdb_unbound_file = pairpdbfile.replace('.pdb','.unbound.pdb')

        fourlettercode = string.split( pdbfile, '.pdb')[-2][-4:]

        fid.write('name = %s_PAIRWISE_DOCK_MCM\n' % pairpdbfile.replace('.pdb','') )
        fid.write('description = %s pairwise docking\n' % pairpdbfile)
        fid.write('inputfiles = ')
        fid.write( abspath(pairpdbfile)+';' )
        fid.write( abspath(pairpdb_unbound_file)+';' )
        fid.write( '/work/rhiju/description_DOCK.txt\n')

        fid.write('arguments = dd %s _ -s %s -dock -pose -dock_mcm -randomize1 -randomize2 -ex1 -ex2aro_only -dock_rtmin -dock_score_norepack -no_filters -output_all -accept_all -nstruct 100 -silent  -pose_silent_out -output_silent_gz -use_score12 -protein_name_prefix %s -description_file description_DOCK.txt\n' % (fourlettercode,pairpdb_unbound_file,prefixAB))

        fid.write('resultfiles = dd%s.out.gz\n' % fourlettercode );
        fid.write('Queue = 1000\n')
        fid.write('\n')

fid.close()


