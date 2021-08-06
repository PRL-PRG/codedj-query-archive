#!/usr/bin/python

from phil import *

numbers = map(str,range(10))
hosts = ['modula','ratfor']

#for i in range(1,11):
#    host = 'whip%02d'%i
#    hosts.append(host)

hosts = map(lambda x:'whip%02d'%x, [1,2,3,4,5,6,7,8,9,11,14,15])

for host in hosts:
    print 10*'-' + host + 10*'-'
    command = 'ssh %s ps -u rhiju'%host
    system(command)
    print
