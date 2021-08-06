#!/usr/bin/python

import os
import socket
import subprocess
import sys


def ipcheck(dirname, account_filename, *args):
    subprocess.call(["python", "ipcheck.py"] + list(args) + 
                    ["--acctfile", account_filename], cwd=dirname)


def main(dirname, account_filename):
    hostname = open(os.path.join(dirname, account_filename)).read().split()[-1]
    if socket.gethostbyname(hostname) == "127.0.0.1":
        ipcheck(dirname, account_filename, "-i", "ppp0")
    else:
        ipcheck(dirname, account_filename, "-a", "127.0.0.1")
        

if __name__ == "__main__":
    main(os.path.abspath(os.path.dirname(sys.argv[0])), "acct")
    raw_input()
