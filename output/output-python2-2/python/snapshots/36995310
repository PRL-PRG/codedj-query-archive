#!/usr/bin/python

from sys import argv,exit
import string
from os import system
from os.path import basename,dirname,abspath,exists


extra_args = argv[1:]

dirs = ['.'] #Current directory

for dir in dirs:

    work_prefix = '/Volumes/WORK/'
    if exists(work_prefix):
        command = 'mkdir -p '+whipdir
        print(command)
        system(command)
    else:
        print
        print 'Hey, you need to mount /Volumes/WORK!! Alternatively type in your password: '
        work_prefix = 'whip03:/work/'
        command = 'ssh whip03 mkdir -p ' + abspath(dir).replace('/Users/','/work/')
        print(command)
        system(command)

    whipdir = abspath(dir).replace('/Users/',work_prefix)



    command = 'rsync -avz '+dir+'/ '+whipdir+' '+string.join(extra_args)
    print(command)
    system(command)

