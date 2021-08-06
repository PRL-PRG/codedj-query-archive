#!/usr/bin/python
##
## make boinc submit files for homologs
##

from phil import *
import sys
from os.path import dirname, basename, abspath
import string

NSTRUCT = 30
QUEUE = 30

select_mode = 0

mode='-shortrelax'
if argv.count(mode):
    commandlinedescription = 'CASP7_ABRELAX_SHORTRELAX_SAVE_ALL_OUT'
    args = '-output_silent_gz -silent -increase_cycles 10  -new_centroid_packing -abrelax -output_chi_silent -stringent_relax -vary_omega -omega_weight 0.5 -farlx -ex1 -ex2 -termini -short_range_hb_weight 0.50 -long_range_hb_weight 1.0 -no_filters -rg_reweight 0.5 -rsd_wt_helix 0.5 -rsd_wt_loop 0.5 -output_all -accept_all -nstruct %d -relax_score_filter' % NSTRUCT
    descriptionfile = '/work/rhiju/CASP7/t287_/frags/casp7.description.shorter.txt';
    select_mode = 1
    pos = argv.index(mode); del(argv[pos])

mode='-abrelax'
if argv.count(mode):
    commandlinedescription = 'CASP7_ABRELAX_SAVE_ALL_OUT'
    args = '-output_silent_gz -silent -increase_cycles 10  -new_centroid_packing -abrelax -output_chi_silent -stringent_relax -vary_omega -omega_weight 0.5 -farlx -ex1 -ex2 -termini -short_range_hb_weight 0.50 -long_range_hb_weight 1.0 -no_filters -rg_reweight 0.5 -rsd_wt_helix 0.5 -rsd_wt_loop 0.5 -output_all -accept_all -nstruct %d -relax_score_filter' % NSTRUCT
    descriptionfile = '/work/rhiju/CASP7/t287_/frags/casp7.description.shorter.txt';
    select_mode = 1
    pos = argv.index(mode); del(argv[pos])

mode='-abinitio'
if argv.count(mode):
    commandlinedescription = 'CASP7_ABINITIO_SAVE_ALL_OUT'
    args = '-output_silent_gz -silent -increase_cycles 10  -new_centroid_packing -termini -rg_reweight 0.5 -rsd_wt_helix 0.5 -rsd_wt_loop 0.5 -output_all -accept_all -nstruct %d ' % NSTRUCT
    descriptionfile = '/work/rhiju/CASP7/casp7.description.graphicsproblem.txt';
    select_mode = 1
    pos = argv.index(mode); del(argv[pos])

put_in_filters = 0
if argv.count('-filter'):
    i = argv.index('-filter')
    del argv[i]
    put_in_filters = 1

shortrelax = 0
if argv.count('-shortrelax'):
    i = argv.index('-shortrelax')
    del argv[i]
    shortrelax = 1

put_in_barcode = 0
if argv.count('-barcode'):
    i = argv.index('-barcode')
    del argv[i]
    barcode_file_first = abspath(argv[i])
    del argv[i]
    put_in_barcode = 1
    defined_barcode_prefix = 0

if argv.count('-queue'):
    i = argv.index('-queue')
    del argv[i]
    QUEUE = int(argv[i])
    del argv[i]

if argv.count('-increase_cycles'):
    i = argv.index('-increase_cycles')
    del argv[i]
    increase_cycles = float(argv[i])
    del argv[i]
    args = args.replace('-increase_cycles 10','-increase_cycles %3.1f' % increase_cycles)

fasta_list = argv[1:]

outfile = sys.stdout;


fasta_list.sort()

for fasta in fasta_list:
    prefix_defined = 0
    gzipped = 0

    frag_dir = dirname(abspath(fasta))+'/'
    fasta = basename(fasta)

    if fasta[-3:] == '.gz' :
        fasta = fasta[:-3]
        gzipped = 1

    assert(gzipped == 1)

    fivelettercode = fasta[-11:-6]
    fourlettercode = fasta[-11:-7]
    chaincode = fasta[-7:-6]

    prefix = fasta[:-11]
    if len(prefix)>0:
       prefix_defined = 1

    if put_in_barcode and not defined_barcode_prefix:
        defined_barcode_prefix = 1
        barcode_prefix_start = barcode_file_first.find(prefix)
        barcode_prefix_end = barcode_prefix_start + len(prefix)
        assert(barcode_prefix_start >= 0)


    outfile.write('name = '+fivelettercode+'_'+commandlinedescription+'_'+prefix+'\n')
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

    if (put_in_barcode):
        barcode_file = barcode_file_first[:barcode_prefix_start] + prefix + barcode_file_first[barcode_prefix_end:]
        assert( exists(barcode_file))
        outfile.write(barcode_file+';')

    if prefix == 'hom001_':
        new_file = '/work/rhiju/CASP7/bioinfo/outfiles/%s.3djury.out.gz'%(fivelettercode)
        assert (exists(new_file))
        outfile.write(new_file+';')

    outfile.write('%s' % descriptionfile)
    outfile.write('\n')

    if prefix_defined:
        args_withprefix = args+' -protein_name_prefix '+prefix+' -frags_name_prefix '+'boinc_'+prefix
    else:
        args_withprefix = args+' -frags_name_prefix boinc_'


    assert( exists(descriptionfile))
    args_withprefix += ' -description_file '+ basename(descriptionfile)


    if prefix == 'hom001_':
        new_file = '%s.3djury.out'%(fivelettercode)
        args_withprefix += ' -server_models ' + new_file

    if (shortrelax):
        args_withprefix += ' -filter1 -9999 -filter2 -9999'

    if (put_in_barcode):
        barcode_file = basename(barcode_file)
        if barcode_file[-3:] == '.gz':
            barcode_file = barcode_file[:-3]
        args_withprefix += ' -barcode_mode 3 -barcode_file %s -output_flavor' % barcode_file

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

    #outfile.write('priority = -10\n') #This is currently poison, even for test jobs.
    outfile.write('Queue = %d\n\n' % QUEUE)

outfile.close()
