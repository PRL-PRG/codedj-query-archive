#!/usr/bin/python

from sys import argv,exit
import string
from os import system
from os.path import basename,dirname,abspath,exists


extra_args = argv[1:]

dirs = ['.'] #Current directory

for dir in dirs:

    work_prefix = 'dig22:/work/'
    command = 'ssh dig22 mkdir -p ' + abspath(dir).replace('/Users/','/work/')
    print(command)
    system(command)

    digdir = abspath(dir).replace('/Users/',work_prefix)

    command = 'rsync -avz '+dir+'/'+string.join(extra_args)+' '+digdir
    print(command)
    system(command)

