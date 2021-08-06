#!/usr/bin/python
#
# Coordinate systems seem to be off in PDB's
# vs. phaser output
#
from sys import argv

sym_pdbfile    = argv[1]
phaser_pdbfile = argv[2]

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
print sequence_sym

# Assume this is just a first chain.
(pdb_CA_phaser, sequence_phaser ) = get_coords_and_sequence( phaser_pdbfile)
print sequence_phaser


shift = {}
dist2_chain = {}
for i in range( len( sequence_sym) ):

    if not sequence_sym[i] == sequence_phaser[0]: continue

    dist2 = 0.0

    # Shift based on first residue in chain?
    # No, based on centroid
    shift[ i ] = [0.0,0.0,0.0]
    numres = len( pdb_CA_sym[i] )
    for k in range( 3 ):
        for j in range( numres  ):
            shift[i][k] += pdb_CA_sym[i][j][k] - pdb_CA_phaser[0][j][k]
        shift[i][k] /= numres

    for j in range( numres  ):
        for k in range( 3 ):
            dist2 += pow( ( pdb_CA_sym[i][j][k] - pdb_CA_phaser[0][j][k] - shift[i][k] ), 2 )

    dist2 /= numres

    print 'Chain ', i, ': ',dist2
    dist2_chain[ i ] = dist2


##########################
# Which one to align to?
# Well, it needs to have the same
# rotation as phaser reference.
# And it should be near middle of
# the pack (near first chain).
##########################
which_chain = 0
best_dist_to_first_chain = 999999.99999
for i in range( len( sequence_sym) ):
    if ( dist2_chain[i] < 1.0 ):
        dist_to_first_chain = 0.0
        for k in range( 3 ):
            dist_to_first_chain += pow( shift[0][k] - shift[i][k], 2)
        if (dist_to_first_chain < best_dist_to_first_chain):
            best_dist_to_first_chain = dist_to_first_chain
            which_chain = i


def translate_pdb( file, shift ):
    lines = open( file ).readlines()

    file_new = file.replace('.pdb','_translate.pdb')
    fid = open ( file_new, 'w' )

    for line in lines:
        if len(line) > 7 and ( line[:4]  == 'ATOM' or  line[:4] == 'HETATM' ):
            x = float( line[30:38]) - shift[0]
            y = float( line[38:46]) - shift[1]
            z = float( line[46:54]) - shift[2]

            line =  '%s%8.3f%8.3f%8.3f%s' % \
                (line[:30],x,y,z,line[54:])
        fid.write( line )


translate_pdb( sym_pdbfile, shift[ which_chain] )

