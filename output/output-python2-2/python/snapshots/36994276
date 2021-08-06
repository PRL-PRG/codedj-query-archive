#!/usr/bin/python
##
## make boinc submit files for homologs
##

from phil import *
import sys
from os.path import dirname, basename, abspath
import string

NSTRUCT = 30
QUEUE = 10

select_mode = 0

mode='-assemble'
if argv.count(mode):
    commandlinedescription = 'CASP7_ASSEMBLE_SAVE_ALL_OUT'
    args = '-output_silent_gz -silent -increase_cycles 1  -new_centroid_packing -abrelax -minimize_exclude_helix -output_chi_silent -stringent_relax -vary_omega -omega_weight 0.5 -farlx -ex1 -ex2 -termini -short_range_hb_weight 0.50 -long_range_hb_weight 1.0 -rg_reweight 0.5 -rsd_wt_helix 0.5 -rsd_wt_loop 0.5 -no_filters -nstruct %d -assemble -regions -fa_input' % NSTRUCT
    descriptionfile = '/work/rhiju/CASP7/casp7.description.shorter.txt';
    select_mode = 1
    pos = argv.index(mode); del(argv[pos])
    start_structure_list = argv[pos:]
    del(argv[pos:])

mode='-assemble_centroid'
if argv.count(mode):
    commandlinedescription = 'CASP7_ASSEMBLE_CENTROID_SAVE_ALL_OUT'
    args = '-output_silent_gz -silent -increase_cycles 1  -new_centroid_packing  -termini -rg_reweight 0.5 -rsd_wt_helix 0.5 -rsd_wt_loop 0.5 -no_filters -nstruct %d -assemble -regions' % NSTRUCT
    descriptionfile = '/work/rhiju/CASP7/casp7.description.shorter.txt';
    select_mode = 1
    pos = argv.index(mode); del(argv[pos])
    start_structure_list = argv[pos:]
    del(argv[pos:])

if argv.count('-queue'):
    i = argv.index('-queue')
    del argv[i]
    QUEUE = int(argv[i])
    del argv[i]

set_priority = 0
if argv.count('-priority'):
    i = argv.index('-priority')
    del argv[i]
    PRIORITY = int(argv[i])
    del argv[i]
    set_priority = 1

if argv.count('-increase_cycles'):
    i = argv.index('-increase_cycles')
    del argv[i]
    increase_cycles = float(argv[i])
    del argv[i]
    args = args.replace('-increase_cycles 1','-increase_cycles %3.1f' % increase_cycles)


put_in_filters = 0
if argv.count('-filter'):
    i = argv.index('-filter')
    del argv[i]
    put_in_filters = 1
    args += ' -relax_score_filter'


outfile = sys.stdout;
start_structure_list.sort()

fasta = argv[1]
frag_dir = dirname(abspath(fasta))+'/'
fasta = basename(fasta)

if fasta[-3:] == '.gz' :
    fasta = fasta[:-3]
    gzipped = 1

assert(gzipped == 1)

fivelettercode = fasta[-11:-6]
fourlettercode = fasta[-11:-7]
chaincode = fasta[-7:-6]

prefix_defined = 0
prefix = fasta[:-11]
if len(prefix)>0:
   prefix_defined = 1

for start_structure in start_structure_list:
    gzipped = 0
    start_structure_tag = string.split( basename(start_structure), '.pdb')[0]
    outfile.write('name = '+fivelettercode+'_'+commandlinedescription+'_'+prefix+'_'+start_structure_tag+'\n')
    if prefix_defined:
        outfile.write('description = homolog '+prefix+' for '+fivelettercode+' with '+commandlinedescription+' command line\n')
    else:
        outfile.write('description = '+fivelettercode+' with '+commandlinedescription+' command line\n')

    outfile.write('inputfiles = ')

    # Here come the input files. Run asserts to make sure they all exist!
    for suffix in ['fasta.gz','psipred_ss2.gz']:
        new_file = '%s/%s%s.%s'%(frag_dir,prefix,fivelettercode,suffix)
        #print new_file
        assert(exists(new_file))
        outfile.write(new_file+';')

    new_frag_tag = "_05.200_v1_3"
    for m in [3,9]:
        new_file = '%s/%saa%s%02d%s.gz'%(frag_dir,'boinc_'+prefix,fivelettercode,m,new_frag_tag)
        assert(exists(new_file))
        outfile.write(new_file+';')

    for suffix in ['pdb.gz']:
        new_file = '%s/%s.%s'%(frag_dir,fourlettercode,suffix)
        if (exists(new_file)):
            outfile.write(new_file+';')

    assert(exists(start_structure))
    outfile.write('%s;' % start_structure)

    regions_file = start_structure.replace( '.pdb','.regions')
    assert(exists(regions_file))
    outfile.write('%s;' % regions_file)

    outfile.write('%s' % descriptionfile)
    outfile.write('\n')

    if prefix_defined:
        args_withprefix = args+' -protein_name_prefix '+prefix+' -frags_name_prefix '+'boinc_'+prefix
    else:
        args_withprefix = args+' -frags_name_prefix boinc_'


    assert( exists(descriptionfile))
    args_withprefix += ' -description_file '+ basename(descriptionfile)

    args_withprefix += ' -s %s' % basename(start_structure)

    if (put_in_filters):
        filterfile =  prefix+fivelettercode+'filters.txt'
        if exists(filterfile):
            filters = open( prefix+fivelettercode+'filters.txt') .readlines()
            filter1 = int(float(string.split(filters[0])[0]))
            filter2 = int(float(string.split(filters[0])[1]))
            args_withprefix += ' -filter1 %d  -filter2 %d' % (filter1, filter2)
        else:
            print '************  '+filterfile+' NOT FOUND'
            sys.exit()

    outfile.write('arguments = xx '+fourlettercode+' '+chaincode+' '+args_withprefix+'\n')

    resultfile = 'xx'+fourlettercode+'.out.gz'
    outfile.write('resultfiles = '+resultfile+'\n')

    if set_priority:
        outfile.write('priority = %d\n' % PRIORITY)

    outfile.write('Queue = %d\n\n' % QUEUE)

outfile.close()
