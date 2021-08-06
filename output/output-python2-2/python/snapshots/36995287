#!/usr/bin/python

import sys
import string
from math import sqrt,acos

files = sys.argv[4:]
atomtype = sys.argv[1:4]

atom1_defined = 0
atom2_defined = 0
atom3_defined = 0

for file in files:

    lines = open( file, 'r' ).readlines()
    for line in lines:
        cols = string.split(line)
        if len(cols)<3:  continue
        if cols[2] == atomtype[0]:
            N_atom = (float(cols[5]), float(cols[6]), float(cols[7]))
            atom1_defined = 1
        if cols[2] == atomtype[1]:
            CA_atom = (float(cols[5]), float(cols[6]), float(cols[7]))
            atom2_defined = 1
        if cols[2] == atomtype[2]:
            C_atom = (float(cols[5]), float(cols[6]), float(cols[7]))
            atom3_defined = 1
        if atom1_defined and atom2_defined and atom3_defined:
            #slow, but does the job.
            dotproduct = (
                (N_atom[0] - CA_atom[0])*(C_atom[0] - CA_atom[0]) +
                (N_atom[1] - CA_atom[1])*(C_atom[1] - CA_atom[1]) +
                (N_atom[2] - CA_atom[2])*(C_atom[2] - CA_atom[2]))
            dotproduct /= sqrt(
                (N_atom[0] - CA_atom[0])*(N_atom[0] - CA_atom[0]) +
                (N_atom[1] - CA_atom[1])*(N_atom[1] - CA_atom[1]) +
                (N_atom[2] - CA_atom[2])*(N_atom[2] - CA_atom[2]));
            dotproduct /= sqrt(
                (C_atom[0] - CA_atom[0])*(C_atom[0] - CA_atom[0]) +
                (C_atom[1] - CA_atom[1])*(C_atom[1] - CA_atom[1]) +
                (C_atom[2] - CA_atom[2])*(C_atom[2] - CA_atom[2]));

            angle = acos( dotproduct ) * 180/(3.141592654);
            print angle
            atom1_defined = 0
            atom2_defined = 0
            atom3_defined = 0
