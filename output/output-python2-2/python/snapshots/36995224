#!/usr/bin/python

from sys import argv
import string
from os import system
from os.path import basename,dirname,abspath,exists


dirs = ['.'] #Current directory

extra_args = argv[1:]

for dir in dirs:
    work_prefix = 'dig29:/work/'

    whipdir = abspath(dir).replace('/Users/',work_prefix)


    command = 'rsync -avz '+whipdir+'/'+string.join(extra_args)+' . '
    print(command)
    system(command)

