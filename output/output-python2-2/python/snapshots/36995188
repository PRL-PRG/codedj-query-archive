#!/usr/bin/python

from sys import argv
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


for phaser_pdbfile in phaser_pdbfiles:
    # Assume this is just a first chain.
    (pdb_CA_phaser, sequence_phaser ) = get_coords_and_sequence( phaser_pdbfile)
    print phaser_pdbfile
    #    print sequence_phaser


    shift = {}
    dist2_chain = {}
    num_chains = len( sequence_sym)

    #Need to test four alternative rotations in unit cell:
    direction_vector = []
    direction_vector.append(  [ 1, 1, 1] )
    direction_vector.append(  [ 1,-1,-1] )
    direction_vector.append(  [-1, 1,-1] )
    direction_vector.append(  [-1,-1, 1] )

    num_direction_vector = len( direction_vector )

    best_dist2 = 9999999
    for m in range( num_direction_vector):
        numres = len( pdb_CA_phaser[0] )

        #print 'Testing orientation: ',direction_vector[m]

        test_phaser = []
        for j in range( numres ):
            pos = []
            for k in range(3):
                pos.append( pdb_CA_phaser[0][j][k] * direction_vector[m][k] )
            test_phaser.append( pos )


        for i in range( num_chains ):

            #        if not sequence_sym[i] == sequence_phaser[0]: continue
            al = NBAlign( sequence_phaser[0], sequence_sym[i] )

            if len(al) < 1: continue

            dist2 = 0.0

            # Shift based on first residue in chain?
            # No, based on centroid
            shift[ i ] = [0.0,0.0,0.0]
            for k in range( 3 ):
                j = 0

                while (not j in al.keys() and j < numres):
                    j += 1

                if (j == numres):
                    break

                shift[i][k] += pdb_CA_sym[i][al[j]][k] - test_phaser[j][k]


            for j in range( numres ):
                if not j in al.keys(): continue
                for k in range( 3 ):
                    dist2 += pow( ( pdb_CA_sym[i][al[j]][k] - test_phaser[j][k] - shift[i][k] ), 2 )


            dist2 /= numres

            print 'Chain ', i, ': ',dist2, shift[i]
            dist2_chain[ i ] = dist2
            if dist2 <= best_dist2:
                best_dist2 = dist2
                best_shift = shift[i]

    print 'Best distance: ', best_dist2 #, best_shift
