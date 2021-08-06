#!/usr/bin/python

import time

from Framework.Core.Profiler import TaskProfiler

TP = TaskProfiler()


for j in range(1,10):
    t1 = time.time()
    TP.ProfileBegin( "test" )
    print time.time() - t1
    for i in range(1,100000):i=i
    t1 = time.time()
    TP.ProfileEnd( "test" )
    print time.time() - t1

TP.ProfileBegin( "alah" )
for i in range(1,100000):j=i
TP.ProfileEnd("alah")
TP.ProfileBegin( "blah" )
for i in range(1,100000):j=i
TP.ProfileEnd("blah")

print TP.ProfilePrint( "test" )

print TP.PrintAll()
