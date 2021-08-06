#! /usr/bin/python2

#script to write the file names and directories in the condor format for epmreew

from phil import *

def Help():
    print '\script to write the file names and directories in the condor format for PHASER\n'
    print '\nUsage: <input file list> \n'
    exit()

if len(argv) < 2:
    Help()

list = sys.argv[1]

files = map(lambda x:string.split(x),open(list,'r').readlines())

for file in files:
    sys.stdout.write('Initialdir  = /users/sraman/mr_phaser/rgs4/orig_data_1000/\n')
    sys.stdout.write('Arguments   = %s.script %s_out.log\n'%(file[0][:-4],file[0][:-4]))
#    sys.stdout.write('Output  = ./pose_idealize_%s.log\n'%file[0])
    sys.stdout.write('Queue 1\n')
    sys.stdout.write('\n')
