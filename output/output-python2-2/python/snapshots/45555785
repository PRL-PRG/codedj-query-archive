#!/usr/bin/env python
# -*- coding: koi8-r -*-
# doubleconnect.py: try to connect simultaniously
# to TwistedPythy
# Pythy <the.pythy@gmail.com>

import thread
import telnetlib
import datetime
import time

HOST = "localhost"
PORT = 3000

def connect_to_twisted_pythy(sending_data, myId):
    stdout_mutex.acquire()
    print "send @", datetime.datetime.now(), sending_data
    stdout_mutex.release()
    tn = telnetlib.Telnet(host=HOST, port=PORT)
    tn.write(sending_data+'\r\n')
    data = tn.read_some()
    tn.write('\r\n')
    tn.close()
    stdout_mutex.acquire()
    print "receive @", datetime.datetime.now(), data
    stdout_mutex.release()
    exit_mutexes[myId] = 1
    


stdout_mutex = thread.allocate_lock()
exit_mutexes = [0]*2
sdata = ['12345678901234567890','abcdefghijklmnopqrst']
for d in sdata:
    thread.start_new_thread(connect_to_twisted_pythy, (d,sdata.index(d)))
while 0 in exit_mutexes:
    time.sleep(5)
    pass
