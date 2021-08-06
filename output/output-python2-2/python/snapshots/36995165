#!/usr/bin/python

import sys
import os
import shutil
import re
from tempfile import NamedTemporaryFile
from os import system, remove, getcwd
from os.path import basename, exists, isdir, dirname, abspath
from glob import glob
from string import join
from phil import whips_by_usage

# configuration information
#working_dir       = '/work/tex/sg_targets/working'
working_dir       = getcwd()
pick_homs_blast   = '/work/tex/sg_targets/bin/pick_homs_blast.py'
pick_homs_wrapper = '/work/tex/sg_targets/bin/pick_homs_wrapper.py'
fragpicker        = '/work/tex/fragpicker/fragpicker.pl'
MAX_LOAD          = 4.0

# begin main program flow.
fastas = sys.argv[1:]
for fasta_file in fastas:
    fasta_file = fasta_file.strip()
    e  = fasta_file.split('.')
    id = e[0]
    # Make a subdirectory in working_dir for this fasta, put the .fasta file in
    # it.
    wd = working_dir + '/' + id
    if not os.path.isdir(wd):
        os.makedirs(wd)
    shutil.copyfile( fasta_file, wd + '/' + basename(fasta_file))

    # Execute the pick_homs_blast.py script.
    #cmd = pick_homs_blast + ' ' + working_dir + ' -master '
    cmd = pick_homs_blast + ' ' + wd + '/' + basename(fasta_file)
    print cmd
    system(cmd)

    # Read in a list of the .fasta homolog files. Rename them using hom001-hom00N naming scheme,
    # and put each of them in their own subdirectory.
    files = glob( wd + '/*blast*.fasta' )
    new_files = []
    for file in files:
        print file
        bd = dirname(file)
        bn = basename(file)

        list1 = bn.split('.')
        list2 = list1[3].split('_')

        hom_number = list2[1][1:]
        new_id     = 'hom' + hom_number + '_' + id
        new_name   = bd + '/' + new_id + '.fasta'
        os.rename( file, new_name )

        # Make a directory for the new file. Move everything in there, and then pick fragments.
        new_dir = bd + '/' + new_id
        os.makedirs( new_dir )
        os.rename( new_name, new_dir + '/' + basename(new_name) )
        new_name = new_dir + '/' + basename(new_name)
        new_files.append( new_name )

    # Finally, make the fragments. Use Phil's trickeration for parallelizing
    # fragment picking jobs across the whips.
    while new_files:
        print new_files
        hosts = whips_by_usage( MAX_LOAD )
        for host in hosts:
            if not new_files:
                break # exit if we've processed all new_files
            fasta_file = abspath(new_files[0])
            del new_files[0]

            logfile = wd + '/' + basename(fasta_file) + 'fragpicker.log'
            cmd = 'ssh %s "(nice -n +19 %s -boinc -size 200 -fasta %s &> %s &)"'\
                    %(host,fragpicker,fasta_file,logfile)
            print cmd
            system(cmd)
