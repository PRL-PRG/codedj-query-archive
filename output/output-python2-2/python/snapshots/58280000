#!/usr/bin/python

import os
import socket
import time

os.environ["LD_LIBRARY_PATH"] = "/home/stan/opt/mythtv/lib"
os.environ["PATH"] = "/home/stan/opt/mythtv/bin:/usr/sbin:/usr/bin:/sbin:/bin"

os.system("mythbackend --daemon --logfile /home/stan/opt/mythtv/mythbackend.log")

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

while True:
    try:
        s.connect(("localhost", 6543))
        s.close()
        break
    except socket.error:
        time.sleep(1)

# http://svn.mythtv.org/trac/ticket/3322
try:
    del os.environ["SESSION_MANAGER"]
except KeyError:
    pass

os.system("mythshutdown -u")
os.execvp("mythwelcome", ["mythwelcome"])
