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

from libRemoteTimereg import *

import time

class TestProcessPerformances():
    
    def __init__(self, auth):
        self._auth = auth
        self._remote = RemoteTimereg()
    
    def testLogin(self):
        print "Login"
        initial_time = time.time()
        self._remote.login(*self._auth)
        final_time = time.time()
        print final_time - initial_time
    
    def testQuery(self):
        query = ""
        print "Query: %s" % query
        initial_time = time.time()
        self._remote.query(query)
        final_time = time.time()
        print final_time - initial_time
    
    def testTimereport(self):
        date = "2008-07-25"
        print "Timereport: %s" % date
        initial_time = time.time()
        self._remote.timereport(date)
        final_time = time.time()
        print final_time - initial_time
        print "Weekly timereport"
        initial_time = time.time()
        for i in range(1, 8):
            self._remote.timereport("2008-07-2%d" %i)
        final_time = time.time()
        print final_time - initial_time

    def testWhoami(self):
        print "Whoami"
        initial_time = time.time()
        self._remote.whoami()
        final_time = time.time()
        print final_time - initial_time


if __name__ == "__main__":
    if len(sys.argv[1:]) == 3:
        test = TestProcessPerformances(sys.argv[1:])
        test.testLogin()
        test.testQuery()
        test.testTimereport()
        test.testWhoami()
    else:
        print "Usage: performance_test.py achievouri user password"