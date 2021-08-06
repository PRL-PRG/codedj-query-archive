#!/usr/bin/python

from sys import argv,exit
import string
from os import popen
from blast import NBAlign

sym_pdbfile    = argv[1]
phaser_pdbfiles = argv[2:]

###################################################################
longer_names={'ALA': 'A', 'ARG': 'R', 'ASN': 'N', 'ASP': 'D',
              'CYS': 'C', 'GLU': 'E', 'GLN': 'Q', 'GLY': 'G',
              'HIS': 'H', 'ILE': 'I', 'LEU': 'L', 'LYS': 'K',
              'MET': 'M', 'PHE': 'F', 'PRO': 'P', 'SER': 'S',
              'THR': 'T', 'TRP': 'W', 'TYR': 'Y', 'VAL': 'V'}


###################################################################
def get_coords_and_sequence( file ):
    # Go through symmetry file,
    # read C-alpha coordinates and sequences
    # of each chain.
    pdb_CA = {}
    sequence = {}
    count  = -1

    newchain = 1

    lines = open( file ).readlines()
    for line in lines:

        if len( line ) > 18 and line[:4]=='ATOM' and line[12:16] ==' CA ':

            if newchain:
                count += 1
                pdb_CA[count] = []
                sequence[count] = ''
                newchain = 0

            x = float( line[30:38])
            y = float( line[38:46])
            z = float( line[46:54])
            pdb_CA[count].append( [x,y,z] )
            res = longer_names[ line[17:20] ]
            sequence[count] += res



        if len( line ) > 3 and line[:3]=='TER':
            newchain = 1

    return (pdb_CA, sequence )


###################################################################
###################################################################
(pdb_CA_sym, sequence_sym ) = get_coords_and_sequence( sym_pdbfile)
#print sequence_sym


def get_matrix_diff( rot_matrix, all_rot_matrix ):
    num_rot_matrix = len( all_rot_matrix)
    stopit = 1

    for j in range( num_rot_matrix ):
        matrix_diff = 0.0
        for k in range(3):
            for l in range(3):
                matrix_diff += abs(rot_matrix[k][l] - all_rot_matrix[j][k][l] )

        if ( matrix_diff < 0.1 ):
            stopit = 0
            break

    return (matrix_diff,stopit)


#Need to test four alternative rotations in unit cell:
all_rot_matrix = []
all_rot_matrix.append(  [ [ 1, 0, 0], [0,  1, 0], [0, 0, 1]] )
all_rot_matrix.append(  [ [ 1, 0, 0], [0, -1, 0], [0, 0,-1]] )
all_rot_matrix.append(  [ [-1, 0, 0], [0,  1, 0], [0, 0,-1]] )
all_rot_matrix.append(  [ [-1, 0, 0], [0, -1, 0], [0, 0, 1]] )

symm_lines = popen('grep SMTRY '+sym_pdbfile ).readlines()
num_symm_operators = len( symm_lines ) /3

for i in range( num_symm_operators ):
    rot_matrix = []
    for j in range( 3 ):
        cols = string.split( symm_lines[ 3*i + j ] )
        rot_matrix.append( [ float( cols[4] ), float( cols[5]), float( cols[6] )])
    (matrix_diff, stopit) = get_matrix_diff( rot_matrix, all_rot_matrix)
    if stopit:
        all_rot_matrix.append( rot_matrix )


#Complete the rotation group?

#Add in inverses
num_rot_matrix = len( all_rot_matrix)
for i in range( num_rot_matrix ):
    #print all_rot_matrix[i]

    rot_matrix =  [ [ 1, 0, 0], [0,  1, 0], [0, 0, 1]]
    for k in range(3):
        for l in range(3):
            rot_matrix[k][l] = all_rot_matrix[i][l][k]

    (matrix_diff,stopit) = get_matrix_diff( rot_matrix, all_rot_matrix )

    if stopit:
        print 'Adding inverse operator: ', rot_matrix
        all_rot_matrix.append( rot_matrix )


rot_group_complete = 0
while not rot_group_complete:
    num_rot_matrix = len( all_rot_matrix )
    rot_group_complete = 0
    stopit = 1

    for i in range( num_rot_matrix ):
        # print all_rot_matrix[i]

        for j in range( num_rot_matrix ):

            rot_matrix =  [ [ 1, 0, 0], [0,  1, 0], [0, 0, 1]]

            #Multiple matrices.
            for k in range(3):
                for l in range(3):
                    pos = 0.0
                    for x in range(3):
                        pos += all_rot_matrix[i][k][x] * all_rot_matrix[j][x][l]

                    rot_matrix[k][l] = pos
            #print 'Testing... ',rot_matrix

            (matrix_diff,stopit) = get_matrix_diff( rot_matrix, all_rot_matrix )

            if stopit:
                print 'Adding operator: ', rot_matrix
                all_rot_matrix.append( rot_matrix )
                break

        if stopit: break

    if not stopit: rot_group_complete = 1


num_rot_matrix = len( all_rot_matrix )

for phaser_pdbfile in phaser_pdbfiles:
    # Assume this is just a first chain.
    (pdb_CA_phaser, sequence_phaser ) = get_coords_and_sequence( phaser_pdbfile)
    print phaser_pdbfile
    #print sequence_sym
    #print sequence_phaser

    shift = {}
    dist2_chain = {}
    num_chains = len( sequence_sym)

    best_dist2 = 9999999
    best_maxsub = -1.0
    numres = len( pdb_CA_phaser[0] )

    for m in range( num_rot_matrix ):

        rot_matrix = all_rot_matrix[m]

        test_phaser = []
        for j in range( numres ):
            pos = []
            for k in range(3):
                sum = 0.0
                for i in range(3):
                    sum += pdb_CA_phaser[0][j][i] * rot_matrix[i][k]
                pos.append( sum )
            test_phaser.append( pos )


        for i in range( num_chains ):

            #        if not sequence_sym[i] == sequence_phaser[0]: continue
            al = NBAlign( sequence_phaser[0], sequence_sym[i] )

            #print al
            if len(al) < 1: continue

            dist2 = 0.0

            # Shift based on first residue in chain?
            # No, based on centroid
            # Why is this so slow?
            shift[ i ] = [0.0,0.0,0.0]
            for k in range( 3 ):
                count = 0
                for j in range( numres ):
                    if not j in al.keys(): continue
                    shift[i][k] += pdb_CA_sym[i][al[j]][k] - test_phaser[j][k]
                    count += 1
                shift[i][k] /= count

                #            print shift[i]


            MAXDIST2 = 4.0
            goodcount = 0
            totcount  = 0
            for j in range( numres ):
                if not j in al.keys(): continue

                local_dist2 = 0.0
                for k in range( 3 ):
                    local_dist2 += pow( ( pdb_CA_sym[i][al[j]][k] - test_phaser[j][k] - shift[i][k] ), 2 )

                dist2 += local_dist2
                totcount += 1
                if (local_dist2 < MAXDIST2):
                    goodcount += 1

            # Make this running count an RMSD.
            dist2 /= numres
            dist2 = pow( dist2, 0.5 )

            if totcount>0: maxsub = goodcount * 1.0/ totcount

            #print 'Chain ', i, ': ',dist2,shift[i]
            dist2_chain[ i ] = dist2

            if maxsub > best_maxsub:
                best_dist2 = dist2
                best_shift = shift[i]
                best_matrix = rot_matrix
                best_maxsub = maxsub

            # print m,i,dist2


    print 'Best distance: ', best_dist2, '  Best maxsub: ', best_maxsub

    symmetry_dist_file = phaser_pdbfile.replace('.pdb','.pdb.symm_dist')
    fid = open( symmetry_dist_file,'w' )
    fid.write(  '%8.5f\n' % best_maxsub )
    fid.close()

