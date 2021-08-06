#!/usr/bin/python

from os import popen, system
import string

lines =  popen( 'condor_status | grep vm1' ).readlines()

clusternames = []
for line in lines:
    clustername = string.split( line, '@' )[1]
    clustername = string.split( clustername, '.' )[0]
    #print clustername
    clusternames.append( clustername )

username = popen( 'whoami' ).readlines()[0][:-1]
#print username

CURRENT_DATABASE_LOCATION = '/work/%s/minirosetta_database' % username
NEW_DATABASE_LOCATION = '/scratch/USERS/%s/' % username

#rsync_command = '"rsync -avzL --delete %s %s"' % ( CURRENT_DATABASE_LOCATION, NEW_DATABASE_LOCATION )
rsync_command = '"rsync --bwlimit=500 -avzL %s %s"' % ( CURRENT_DATABASE_LOCATION, NEW_DATABASE_LOCATION )
#rsync_command = '"cp -rf %s/scoring/weights/rna_hires.wts %s/minirosetta_database/scoring/weights"' % ( CURRENT_DATABASE_LOCATION, NEW_DATABASE_LOCATION )



count = 0
for clustername in clusternames:
    count += 1
    ssh_command =  'ssh '+clustername+' '+rsync_command
    print 'RSYNCING AT %10s. %4d  out of %4d... ' % (clustername, count, len(clusternames) )
    print( ssh_command )
    system( ssh_command )

