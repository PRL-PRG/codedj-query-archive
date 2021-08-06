
import os
import sys

import logging
import logging.handlers

import select
import subprocess

def logcall(cmd):
    handler = logging.handlers.TimedRotatingFileHandler("log", "D", 1)
    handler.setFormatter(logging.Formatter("%(asctime)s %(levelname)s %(message)s"))
    logging.root.addHandler(handler)
    logging.root.setLevel(0)

    p = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    stdout_pid = os.fork()
    if stdout_pid == 0:
        for line in p.stdout:
            logging.info(line.rstrip("\n"))
        os._exit(0)
    stderr_pid = os.fork()
    if stderr_pid == 0:
        print "stderr here"
        for line in p.stderr:
            logging.error(line.rstrip("\n"))
        os._exit(0)
    p.wait()
    os.waitpid(stdout_pid, 0)
    os.waitpid(stderr_pid, 0)
#    while True:
#        rlist, _wlist, _xlist = select.select([p.stdout, p.stderr], [], [])
#        if p.stdout in rlist:
#            logging.info(p.stdout.read())
#        if p.stderr in rlist:
#            logging.error(p.stderr.read())
        
        

if __name__ == "__main__":
    logcall(["sh", "-c", "echo two 1>2"])