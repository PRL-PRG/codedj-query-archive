#!/usr/bin/python

import os
import socket
import subprocess
import time


def daemonize():
    if os.fork() == 0:
        os.setsid()
        if os.fork() == 0:
            os.chdir("/")
            for fd in range(2):
                os.close(fd)
            os.open(os.devnull, os.O_RDWR)
            os.dup2(0, 1)
            os.dup2(0, 2)
        else:
            os._exit(0)
    else:
        os._exit(0)


def main(hostname):
    call = subprocess.check_call
    started = True 
    stopped = True
    while True:
        if socket.gethostbyname(hostname) == "127.0.0.1":
            if started:
                call(["/etc/init.d/openvpn", "stop"])
                started = False
                stopped = True
        else:
            if stopped:
                call(["/etc/init.d/openvpn", "start"])
                started = True
                stopped = False 
        time.sleep(5.0)


if __name__ == "__main__":
    daemonize()
    main("nimitz.saticed.me.uk")

