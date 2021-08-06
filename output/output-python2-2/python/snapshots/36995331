#!/usr/bin/python

from sys import argv
import string
from os import system

native_pdb_file = argv[1]
segment_file = argv[2]




#################################
# Set up condor submission file
#################################
condorid = open( 'jobCHUNK_' + native_pdb_file.replace('.pdb','') , 'w')


condorid.write('universe     = vanilla\n')
condorid.write('\n')
condorid.write('Notify_user  = rhiju@u.washington.edu\n')
condorid.write('notification = Error\n')
condorid.write('\n')
condorid.write('Log          = condorscript.log\n')
condorid.write('\n')
condorid.write('Executable   = rosetta.gcc\n')
condorid.write('\n')
condorid.write('GetEnv       = True\n')
condorid.write('\n')
condorid.write('Output = logtmp.out\n')
condorid.write('Error = logtmp.err\n')
condorid.write('\n')


###############################
# Loop through segments
###############################
segment_lines = open( segment_file ).readlines()

count = 0

for line in segment_lines:

    count += 1

    ############################
    #Keep a record of segment boundaries too.
    ############################
    pair_list = string.split( line[:-1] )

    #Parse lines defining segments, jumps, etc.

    i = 0
    jump_list = []
    segment_list = []
    non_watson_crick = 0
    while i < len( pair_list):

        tertiary_contact = 0
        chain_end = 0

        if pair_list[i] == 'T': #Tertiary contact.
            i += 1
            tertiary_contact = 1

            try:
                blah = int( pair_list[i] )
            except:
                non_watson_crick = 1
                non_watson_crick_info = string.join( pair_list[i:i+3])
                i += 3

            jump_tertiary_contact =  [ min(int( pair_list[i] ), int( pair_list[i+1]) ),
                                       max(int( pair_list[i] ), int( pair_list[i+1]) ) ]

            i += 2
            continue

        if pair_list[i] == 'X': #Chain ends.
            i += 1
            chain_end = 1

        if not chain_end:
            jump_list.append( [ min(int( pair_list[i] ), int( pair_list[i+1]) ),
                                max(int( pair_list[i] ), int( pair_list[i+1]) ) ] )

        segment_list.append( int( pair_list[i] ) )
        segment_list.append( int( pair_list[i+1] ) )

        i += 2 # Next pair.


    assert( len(segment_list) % 2 == 0 )
    segment_list.sort()

    num_segments = len( segment_list ) / 2
    segment_start = []
    segment_end = []

    for i in range( num_segments ):
        segment_start.append( segment_list[ 2 * i ]  )
        segment_end.append(   segment_list[ 2 * i + 1 ]  )

    ############################
    # Extract segments
    ############################

    lines = open( native_pdb_file ).readlines()

    new_pdb_file = 'chunk%03d_%s' % (count, native_pdb_file )
    fid = open(new_pdb_file ,'w')

    oldresidue = '    '
    i = 0
    renumber_i = 0
    writeout = 0
    renumber_map = {}
    for line in lines:
        currentresidue = line[22:26]
        if not currentresidue == oldresidue:
            i = int( currentresidue)
            writeout = 0
            for k in range( num_segments):
                if ( i >= segment_start[k] and i <= segment_end[k] ):
                    writeout = 1
                    renumber_i += 1
                    renumber_map[ i ] = renumber_i
                    break


        if (writeout):
            line = '%s %4d%s' % (line[:21], renumber_i, line[26:])
            fid.write( line )
        oldresidue = currentresidue

    fid.close()

    ############################
    # Create fasta file
    ############################
    assert( new_pdb_file[-8:]  == '_RNA.pdb' )
    new_fasta_file = new_pdb_file.replace('_RNA.pdb','_.fasta')
    command = '~rhiju/python/pdb2fasta.py %s > %s' % (new_pdb_file, new_fasta_file)
    system(command)

    ############################
    # What jumps and chainbreaks to apply?
    ############################
    new_pair_file = new_pdb_file.replace('_RNA.pdb','__pairing.pdat')
    fid = open( new_pair_file, 'w')

    jump_list.sort()

    if tertiary_contact: jump_list.append( jump_tertiary_contact )
    jump_list.reverse() # Should longest range pair at the end.

    print jump_list
    print segment_list

    num_jumps =  num_segments - 1

    pairings_exist = 0

    chainbreak_list = segment_end

    for i in range( num_jumps ):
        pair1    = renumber_map[ jump_list[i][0] ]
        pair2    = renumber_map[ jump_list[i][1] ]

        for k in range(len(chainbreak_list)):
            if ( renumber_map[chainbreak_list[k]] >= pair1 and
                 renumber_map[chainbreak_list[k]] < pair2 ):
                cutpoint = renumber_map[ chainbreak_list[k] ]
                del chainbreak_list[k]
                break


        base_pair_info = "W W A"
        if non_watson_crick: base_pair_info = non_watson_crick_info

        fid.write( '%d %d %d %s\n' % (pair1,pair2,cutpoint,base_pair_info) )

        pairings_exist = 1

    fid.close()

    if not pairings_exist: system("rm "+new_pair_file)

    #############################################
    # Additional constraints?
    #############################################

    new_contact_file = new_pdb_file.replace('_RNA.pdb','__basepairs.contacts')
    fid = open( new_contact_file, 'w')

    # OUTER PAIR gives a constraint ... are ther any?
    for i in range( num_jumps, len(jump_list) ):
        contacts_exist = 1
        pair1    = renumber_map[ jump_list[i][0] ]
        pair2    = renumber_map[ jump_list[i][1] ]

        fid.write( '%d %d 1 1\n' % (pair1,pair2) )

    fid.close()

    if not contacts_exist: system("rm "+new_contact_file)

    print 'Created ',new_pdb_file,' and associated files!'

    fourlettercode = new_pdb_file.replace('_RNA.pdb','')[-4:]
    prefix         = new_pdb_file.replace('_RNA.pdb','')[:-4]

    pairing_file_tag  = ''
    if pairings_exist:
        pairing_file_tag = ' -pairing_file %s ' % new_pair_file


    contact_file_tag  = ''
    if contacts_exist:
        contact_file_tag = ' -rna_contact_file %s ' % new_contact_file


    condorline = '\narguments =  %02d %s _  -prna -nstruct 5 -cycles 50000 -scale_axis_stagger_by_xy_score -do_2mers_in_last_half -do_1mers_in_last_round -frag_match 2  -harmonic_constraint  %s %s  -n %s -chainbreak_weight 0.0 -protein_name_prefix %s -vall_torsions 1ffk_5sRNA.vall_torsions \n' % (count, fourlettercode, contact_file_tag, pairing_file_tag, new_pdb_file, prefix )

    condorid.write( condorline )
    condorid.write('Queue 5\n')

