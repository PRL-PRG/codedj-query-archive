#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id$
#
# Author: Lorenzo Berni <duplo@develer.com>

import sys
sys.path.append("../")
import time

from libRemoteTimereg import *

class TestProcessPerformances():
    
    def __init__(self, auth, num=50):
        self._auth = auth
        if num > 0:
            self._num = num
        else:
            num = 1
        self._logged_in = False
        self._remote = RemoteTimereg()
    
    def __getattr__(self, name):
        if name in ["login", "timereport", "query", "whoami"]:
            def wrapper(*paramlist, **paramdict):
                print "Request type: %s\tAverage time:" % name,
                if not self._logged_in:
                    self._remote.login(*self._auth)
                    self._logged_in = True
                timelist = []
                func = getattr(self._remote, name)
                for i in range(self._num):
                    initial_time = time.time()
                    func(*paramlist, **paramdict)
                    timelist.append(time.time() - initial_time)
                average_time = sum(timelist) / self._num
                print "%f sec" % average_time
        return wrapper
    
if __name__ == "__main__":
    if len(sys.argv[1:]) == 3:
        test = TestProcessPerformances(sys.argv[1:])
        test.login(*sys.argv[1:])
        test.query("")
        test.timereport("2008-07-25")
        test.whoami()
    else:
        print "Usage: performance_test.py achievouri user password"
