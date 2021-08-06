#!/usr/bin/python
##
## make boinc submit files for homologs
##

from phil import *

NSTRUCT = 10
QUEUE = 500
#commandlinedescription = 'ABINIT'
#args = '-output_silent_gz -silent -increase_cycles 10 -new_centroid_packing -no_filters -nstruct %d' % NSTRUCT #prefix added later
#submitfilename = 'production_homolog.submit'

#commandlinedescription = 'FA_RLX'
#args = '-output_silent_gz -silent -increase_cycles 10  -new_centroid_packing -abrelax  -output_chi_silent -stringent_relax -vary_omega -omega_weight 0.5 -farlx -ex1 -ex2 -termini -short_range_hb_weight 0.50 -long_range_hb_weight 1.0 -no_filters -nstruct %d' % NSTRUCT
#submitfilename = 'fullrelax_nofilters_onehomolog.submit'

#commandlinedescription = 'FA_RLX'
#args = '-output_silent_gz -silent -increase_cycles 10  -new_centroid_packing -abrelax  -output_chi_silent -stringent_relax -vary_omega -omega_weight 0.5 -farlx -ex1 -ex2 -short_range_hb_weight 0.50 -long_range_hb_weight 1.0 -no_filters -nstruct %d' % NSTRUCT
#submitfilename = 'fullrelax_nofilters_onehomolog.submit'

#commandlinedescription = 'FA_RLX'
#args = '-output_silent_gz -silent -increase_cycles 10 -relax_score_filter  -new_centroid_packing -abrelax  -output_chi_silent -stringent_relax -vary_omega -omega_weight 0.5 -farlx -ex1 -ex2 -short_range_hb_weight 0.50 -long_range_hb_weight 1.0 -no_filters -nstruct %d' % NSTRUCT
#submitfilename = 'fullrelax_filters_onehomolog.submit'
#TERMINI_FLAG = 1

commandlinedescription = 'FA_RLX'
args = '-output_silent_gz -silent -increase_cycles 10 -relax_score_filter  -new_centroid_packing -abrelax  -output_chi_silent -stringent_relax -vary_omega -omega_weight 0.5 -farlx -ex1 -ex2 -short_range_hb_weight 0.50 -long_range_hb_weight 1.0 -no_filters -nstruct %d' % NSTRUCT
submitfilename = 'fullrelax_filters.submit'
TERMINI_FLAG = 1

base_dir = argv[1]

id_list = map(lambda x:string.split(x,'/')[-2],
              glob('%s/?????/?????.fasta'%base_dir))
id_list.sort()

old_frag_tag = "_05.200_v1_4"
new_frag_tag = "_05.200_v1_3"

out = open('%s/mapping.txt'%base_dir,'w')
for i in range(len(id_list)):
    id = id_list[i]
    out.write('%s \n' % id)
out.close()

frag_dir = base_dir+'/frags/'
fasta_dir = base_dir+'/fasta/'

#User can also give estimated filters
put_in_filters = 0
if len(argv) > 2:
    put_in_filters = 1
    filter_file = argv[2]
    filter_lines = open(filter_file,'r').readlines()
    filter1 =  {}
    filter2 =  {}
    for line in filter_lines:
        cols = string.split(line)
        filter1[ cols[0]] = cols[1]
        filter2[ cols[0]] = cols[2]

outfilename = base_dir+'/boinc_submit/'+submitfilename
outfile = open(outfilename,'w')

#id_list = ['1fna_']

for id in id_list:

    dirs = glob('%s/%s/h???_/'%(base_dir,id))
    dirs.sort()

    for dir in dirs:
    #for dir in dirs[0:1]:
        hom_id = string.split(dir,'/')[-2]
        n = int(hom_id[1:4])
        prefix = 'hom%03d_' % n
        new_id = prefix + id

        outfile.write('name = '+commandlinedescription+id[1:3]+'_'+new_id+'\n')
        outfile.write('description = homolog '+prefix+' for '+id+' with '+commandlinedescription+' command line\n')
        outfile.write('inputfiles = ')

        # Here come the input files. Run asserts to make sure they all exist!
        for suffix in ['fasta.gz','psipred_ss2.gz']:
            new_file = '%s/%s.%s'%(frag_dir,new_id,suffix)
            #print new_file
            assert(exists(new_file))
            outfile.write(new_file+';')

        for m in [3,9]:
            new_file = '%s/%saa%s%02d%s.gz'%(frag_dir,prefix,id,m,new_frag_tag)
            #print new_file
            assert(exists(new_file))
            outfile.write(new_file+';')

        for suffix in ['pdb.gz']:
            new_file = '%s/%s.%s'%(frag_dir,new_id[:-1],suffix)
            #print new_file
            assert(exists(new_file))
            outfile.write(new_file)

        outfile.write('\n')

        args_withprefix = args+' -protein_name_prefix '+prefix+' -frags_name_prefix '+prefix

        # User may have specified filters...
        if (put_in_filters):
            if filter1.has_key(id):
                args_withprefix += ' -filter1 '+filter1[id]+\
                                  ' -filter2 '+filter2[id]
            else:
                print id,': no filters found?'

        # Use termini option as long as the sequence doesn't start with proline
        # (this bug will be fixed in the next version of BOINC Rosetta)...
        # Note: need a "fasta" directory with unzipped fasta files.
        if (TERMINI_FLAG):
            fasta_file = '%s/%s%s.fasta' % (fasta_dir,prefix,id)
            if exists(fasta_file):
                lines = open(fasta_file).readlines()
                firstletter = lines[1][0]
                if not (firstletter == 'P'):
                    args_withprefix += ' -termini '
            else:
                print 'To use termini, I need to check for an N-terminal proline in a fasta file: ',fasta_file

        outfile.write('arguments = xx '+id[0:4]+' '+id[4]+' '+args_withprefix+'\n')

        resultfile = 'xx'+id[0:4]+'.out.gz'
        outfile.write('resultfiles = '+resultfile+'\n')

        #outfile.write('priority = -10\n') #This is currently poison, even for test jobs.
        outfile.write('Queue = %d\n\n' % QUEUE)

outfile.close()
