#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Lorenzo Berni <duplo@develer.com>

from libRemoteTimereg import *

import sys

import time

def _main(auth):
    remote = RemoteTimereg()
    t = time.time()
    remote.login(*auth)
    t2 = time.time()
    print "login", t2 - t
    t = time.time()
    remote.query("")
    t2 = time.time()
    print "query", t2 - t
    t = time.time()
    remote.timereport("2008-07-25")
    t2 = time.time()
    print "timereport", t2 - t
    t = time.time()
    for i in range(1, 8):
        remote.timereport("2008-0%d-25" % i)
    t2 = time.time()
    print "weekly timereport", t2 - t
    t = time.time()
    remote.whoami()
    t2 = time.time()
    print "whoami", t2 - t
    
    # TODO: Aggiungere il test di timereg
        
    # timesummary non funziona e dice che non abbiamo i permessi adeguati (ma è
    # stata implementata?)
    #t = time.time()
    #remote.timesummary("2008-06-01", "2008-06-30")
    #t2 = time.time()
    #print "timesummary", t2 - t

    
    

if __name__ == "__main__":
    if len(sys.argv[1:]) == 3:
        _main(sys.argv[1:])
    else:
        print "Usage: performance_test.py achievouri user password"