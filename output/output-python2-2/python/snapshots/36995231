#!/usr/bin/python

from sys import argv
from os import popen
import string

pdbfile = argv[1]

lines = open( pdbfile ).readlines()

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

symm_lines = popen('grep SMTRY '+pdbfile ).readlines()
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


for m in range( len( all_rot_matrix ) ):

    fid = open( pdbfile.replace( '.pdb', '_flip%d.pdb' % m ), 'w')

    rot_matrix = all_rot_matrix[ m ]

    for line in lines:
        if ( len(line) > 4 and line[:4]=='ATOM' ) or \
                ( len(line) > 6 and line[:6]=='HETATM' ):
            r = []
            r.append( float(line[30:38]) )
            r.append( float(line[38:46]) )
            r.append( float(line[46:54]) )

            pos = []
            for k in range(3):
                sum = 0.0
                for i in range(3):
                    sum += r[i] * rot_matrix[i][k]
                pos.append( sum )

            fid.write( '%s%8.3f%8.3f%8.3f%s' % \
                           ( line[:30],pos[0],pos[1],pos[2],line[54:] ) )

        else:
            fid.write(line)

    fid.close()



