import string
from glob import glob
from os import popen,system,chdir,remove,getcwd
from os.path import exists
from popen2 import popen2,Popen3
from math import floor,sqrt
from operator import add
from sys import stderr,argv,exit
from whrandom import random
from time import sleep
import whrandom

def run(command,safe=0):
    print command
    if not safe:
        system(command)

def log(s):
    stderr.write(s)
    if s[-1] != '\n':
        stderr.write('\n')

def mkdir(dir):
    if not exists(dir):
        system('mkdir '+dir)

def Increment( D, key, count=1):
    if not D.has_key(key):
        D[key] = 0
    D[key] = D[key] + count

def pick_random(l):
    pos = int( floor( random() * len(l) ) )
    assert pos >= 0 and pos < len(l)
    return l[pos]

def popen_readlines(cmd,timeout):
    whrandom.seed()
    logfile = '/tmp/phil_tmp_%f.log'%(random())
    errfile = '/tmp/phil_tmp_%f.err'%(random())
    new_cmd = '%s > %s 2> %s'%(cmd,logfile,errfile)
    #print 'new_cmd= ', new_cmd
    data = Popen3(new_cmd)
    count = 0
    fail = 0
    while data.poll() < 0:
        increment = count+1
        sleep(increment)
        count+=increment
        if data.poll() < 0 and count > timeout:
            fail = 1
            break


    if exists(errfile):
        lines = open(errfile,'r').readlines()
        if lines:
            print 'ERR: '+string.join(lines,'ERR: ')[:-1]
        remove(errfile)

    if fail or not exists(logfile):
        if exists(logfile):
            lines = open(logfile,'r').readlines()
            if lines:
                print 'LOG: '+string.join(lines,'LOG: ')
            remove(logfile)
        return [],data.poll()
    else:
        lines = open(logfile,'r').readlines()
        remove(logfile)
        return lines,data.poll()


def alive(host):
    cmd = 'ssh %s "(df ./)"'%host
    print 'alive?:',host,cmd
    lines,ret = popen_readlines(cmd,10)
    if lines or ret==0:
        #print lines
        #print ret
        return 1
    else:
        return 0

def random_shuffle(l):
    ll = map(lambda x:[ random(), x], l )
    ll.sort()
    return map(lambda x:x[1], ll )

## returns 0 on success
def ssystem( cmd, timeout ):
    print 'ssystem:',cmd
    lines, ret = popen_readlines( cmd, timeout )
    if ret == 0:
        return 0
    else:
        if lines:
            print 'LOG: '+string.join(lines,'LOG: ')[:-1]
        print 'ssystem failed:',ret
        return 1

def whips_by_usage( MAX_LOAD = 1000 ):
    timeout = 10
    hosts = map(lambda x:'whip%02d'%x, range(1,12) )

    host_list = []
    for host in hosts:
        command = 'ssh %s top -b -n1'%host
        lines,ret = popen_readlines( command, timeout)
        if ret !=0:
            log('host failed: '+host)
            continue
        for line in lines:
            if string.count(line,'load average'):
                try:
                    l = string.split(line)
                    assert l.count('average:')
                    pos = l.index('average:')
                    u1 = float( l[pos+1][:-1] )
                    u2 = float( l[pos+2][:-1] )
                    u3 = float( l[pos+3] )
                    print host, l[pos-1], l[pos], u1, u2, u3
                    if u1>MAX_LOAD or u2>MAX_LOAD or u3>MAX_LOAD:continue
                    host_list.append( [u1, host ] )
                    break
                except:
                    print 'bad line!', host, line[:-1]

    host_list.sort()
    print 'min,max:',host_list[0][0],host_list[-1][0]
    host_list = map(lambda x:x[1],host_list)
    return host_list




if 0:
    #alv= alive('gebb')
    #print alv

    lines,ret = popen_readlines('mkdir /users/pbradley/',5)
    print lines
    print ret
