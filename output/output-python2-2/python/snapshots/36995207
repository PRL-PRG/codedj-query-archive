#!/usr/bin/python

from sys import argv
from os import popen
import string

silentfile = argv[1]

lines = popen('head -n 100000 '+silentfile).readlines()

totres   = 0
totres_H = 0
totres_E = 0
for line in lines:
    cols = string.split(line)
    if len(cols)>1:
        if cols[1]=='L' or cols[1]=='E' or cols[1]=='H':
            totres += 1
        if cols[1]=='H':
            totres_H += 1
        if cols[1]=='E':
            totres_E += 1

alpha_fraction = totres_H/ (1.0* totres)
beta_fraction = totres_E/ (1.0* totres)

print alpha_fraction , beta_fraction
