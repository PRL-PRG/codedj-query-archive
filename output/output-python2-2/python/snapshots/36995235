#!/usr/bin/python

from sys import argv,exit
import string
from os.path import basename,exists
from os import system

one_on_one = 0
if argv.count('-one_on_one'):
    one_on_one = 1
    pos = argv.index('-one_on_one')
    del( argv[pos] )

fourlettercode = argv[1]
chaincode = argv[2]

nterm_file = argv[3]
cterm_file = argv[4]


try:
    patchpoint = int( argv[5])
    patchpoint_defined = 1
except:
    patchpoint_defined = 0


try:
    startlinker = int( argv[6])
    endlinker   = int( argv[7])
    linker_defined = 1
except:
    linker_defined = 0

fa_input_flag = ''
fa_input_abrelax_flag = ''
if argv.count('-fa_input'):
    fa_input_flag = ' -fa_input '
    fa_input_abrelax_flag = ' -fa_input -abrelax -farlx -output_chi_silent'

nterm_files = [nterm_file]
cterm_files = [cterm_file]

condorid = open('jobASSEMBLE'+\
                string.split(string.split(nterm_file,'.pdb')[0],'.list')[0]+\
                string.split(string.split(cterm_file,'.pdb')[0],'.list')[0]\
                ,'w')


if nterm_file[-5:] == '.list':
    nterm_files = open( nterm_file, 'r').readlines()
    nterm_files = [ x[:-1]  for x in nterm_files]

if cterm_file[-5:] == '.list':
    cterm_files = open( cterm_file, 'r').readlines()
    cterm_files = [ x[:-1]  for x in cterm_files]



condorid.write('universe     = vanilla\n');
condorid.write('\n');
condorid.write('Notify_user  = rhiju@u.washington.edu\n');
condorid.write('notification = Error\n');
condorid.write('\n');
condorid.write('Requirements = ( Memory > 248 )\n');
condorid.write('Log          = condorscript.log\n');
condorid.write('Executable   = /users/rhiju/rosetta++/rosetta.gcc\n');
condorid.write('\n');
condorid.write('Output = condor.log\n');
condorid.write('Error = condor.err\n');

countN = 0
for nterm_file in nterm_files:
    countN += 1

    nterm_temp_file = nterm_file
    if patchpoint_defined:
        nterm_temp_file = 'nterm_temp.pdb'
        command = '/users/rhiju/python/termini_truncate_pdb.py %s 1 %d > %s' % (nterm_file,patchpoint,nterm_temp_file)
        print(command)
        system(command)

    countC = 0
    which_cterm_files = cterm_files
    if (one_on_one):
        which_cterm_files = [ cterm_files[ countN - 1] ]
        countC = countN - 1

    for cterm_file in which_cterm_files:
        countC += 1

        outfile_prefix = string.split(basename(nterm_file),'.pdb')[0] + '_' + \
                  string.split(basename(cterm_file),'.pdb')[0]
        outfile = outfile_prefix + '.pdb'

        fid = open( outfile,'w')

        i = 0
        lines = open(nterm_temp_file).readlines()
        oldresidue = '   '
        secstructlines = []
        keep_secstructlines = 0
        for line in lines:
            if line[0:4] == 'ATOM':
                currentresidue = line[23:26]
                if not currentresidue == oldresidue:
                    i += 1
                oldresidue = currentresidue

                fid.write(line)

            if keep_secstructlines:
                secstructlines.append(line)

            if line[0:8] == 'complete':
                keep_secstructlines = 1
                header_line = line


        patchpoint = i

        lines = open(cterm_file).readlines()
        oldresidue = '   '
        keep_secstructlines = 0
        count_secstructlines = i
        for line in lines:
            if line[0:4] == 'ATOM':
                currentresidue = line[23:26]
                if not currentresidue == oldresidue:
                    i += 1
                oldresidue = currentresidue

                line = '%s%3d%s' % (line[0:23], i, line[26:])
                fid.write(line)

            if keep_secstructlines:
                count_secstructlines += 1
                secstructlines.append( '%4d%s' % (count_secstructlines, line[4:]) )

            if line[0:8] == 'complete':
                keep_secstructlines = 1
                header_line = line


        fid.write('TER\n')
#        fid.write(header_line)
#        for line in secstructlines:
#            fid.write(line)

        fid.close()



        #Extend it, dude

        if not linker_defined:
            startlinker = patchpoint-7
            endlinker   = patchpoint+7


        fid = open(outfile_prefix + '.regions','w')
        fid.write('%5d %5d %5d %5d\n' % (startlinker, endlinker, startlinker, endlinker))
        fid.close()

        command = '/users/rhiju/rosetta++/rosetta.gcc zz '+fourlettercode+' '+chaincode+' -assemble -regions -extend -nstruct 1 -protein_name_prefix hom001_ -frags_name_prefix boinc_hom001_ %s -paths /users/rhiju/paths.txt -s %s' % (fa_input_flag,outfile)
        print(command)
        system(command)

        command = 'mv zz'+outfile_prefix+'_0001.pdb '+outfile_prefix+'_EXT.pdb'
        print(command)
        system(command)

        fid = open(outfile_prefix + '_EXT.regions','w')
        old_regions_file = string.split( nterm_file, '.pdb')[0] + '.regions'
        print 'LOOKING FOR ... ' + old_regions_file
        if exists(old_regions_file):
            print 'FOUND IT'
            oldregionlines = open( old_regions_file,'r').readlines()
            for oldregionline in oldregionlines:
                fid.write( oldregionline )
        fid.write('%5d %5d %5d %5d\n' % (startlinker, endlinker, startlinker, endlinker))
        fid.close()

        command = 'cp /users/rhiju/paths.txt .'
        system(command)

        condorid.write('\n')
        ext_pdb_file = outfile_prefix+'_EXT.pdb'
        condorid.write('arguments = %d%d %s %s -silent -nstruct 1000  -regions -s %s -protein_name_prefix hom001_ -frags_name_prefix boinc_hom001_  -assemble -no_filters %s \n' % (countN,countC,fourlettercode,chaincode,ext_pdb_file,fa_input_abrelax_flag) )
        condorid.write('Queue 10\n');

condorid.close()
