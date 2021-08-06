#!/usr/bin/python

from sys import argv
import string
from os import popen

ref_pdb_file = argv[1] #Needs to contain symmetry information!


symm_lines = popen('grep SMTRY '+ref_pdb_file ).readlines()


print symm_lines

num_symm_operators = len( symm_lines ) /3
rot_matrix = {}
trans_vector = {}
for i in range( num_symm_operators ):

    rot_matrix[i] = []
    trans_vector[i] = []

    for j in range( 3 ):
        cols = string.split( symm_lines[ 3*i + j ] )
        rot_matrix[i].append( [ float( cols[4] ), float( cols[5]), float( cols[6] )])
        trans_vector[i].append( float( cols[7] ) )


print rot_matrix
print trans_vector
