#!/usr/bin/python

from phil import *

numbers = map(str,range(10))
hosts = ['modula','ratfor']

for i in range(1,11):
    host = 'whip%02d'%i
    hosts.append(host)

for host in hosts:
    print 10*'-' + host + 10*'-'
    command = 'ssh %s ps -u rhiju'%host
    system(command)
    print
