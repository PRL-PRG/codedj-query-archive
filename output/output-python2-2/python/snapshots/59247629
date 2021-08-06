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
    
    def __init__(self, auth, num=50):
        self._auth = auth
        self._num = num
        self._remote = RemoteTimereg()
    
    def testLogin(self):
        print "Login"
        timelist = []
        for i in range(self._num):
            initial_time = time.time()
            self._remote.login(*self._auth)
            final_time = time.time()
            timelist.append(final_time - initial_time)
        print sum(timelist)/self._num

    def testQuery(self):
        query = ""
        print "Query: %s" % query
        timelist = []
        for i in range(self._num):
            initial_time = time.time()
            self._remote.query(query)
            final_time = time.time()
            timelist.append(final_time - initial_time)
        print sum(timelist)/self._num

    def testTimereport(self):
        date = "2008-07-25"
        print "Timereport: %s" % date
        timelist = []
        for i in range(self._num):
            initial_time = time.time()
            self._remote.timereport(date)
            final_time = time.time()
            timelist.append(final_time - initial_time)
        print sum(timelist)/self._num
        print "Weekly timereport"
        timelist = []
        for i in range(self._num):
            initial_time = time.time()
            for i in range(1, 8):
                self._remote.timereport("2008-07-2%d" %i)
            final_time = time.time()
            timelist.append(final_time - initial_time)
        print sum(timelist)/self._num

    def testWhoami(self):
        print "Whoami"
        timelist = []
        for i in range(self._num):
            initial_time = time.time()
            self._remote.whoami()
            final_time = time.time()
            timelist.append(final_time - initial_time)
        print sum(timelist)/self._num


if __name__ == "__main__":
    if len(sys.argv[1:]) == 3:
        test = TestProcessPerformances(sys.argv[1:])
        test.testLogin()
        test.testQuery()
        test.testTimereport()
        test.testWhoami()
    else:
        print "Usage: performance_test.py achievouri user password"