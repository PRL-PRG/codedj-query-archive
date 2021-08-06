#!/usr/bin/python

import copy
import optparse
import os
import subprocess
import sys
import termios

# Indexes for termios list.
IFLAG = 0
OFLAG = 1
CFLAG = 2
LFLAG = 3
ISPEED = 4
OSPEED = 5
CC = 6


class Term(object):

    def __init__(self, fd):
        self.fd = fd
        self.orig = termios.tcgetattr(fd)
        
    def setcbreak(self):
        new = copy.copy(self.orig)
        new[LFLAG] = self.orig[LFLAG] & ~(termios.ICANON)
        new[CC][termios.VMIN] = 1
        new[CC][termios.VTIME] = 0
        termios.tcsetattr(self.fd, termios.TCSAFLUSH, new)
   
    def restore(self):
        termios.tcsetattr(self.fd, termios.TCSAFLUSH, self.orig)
        

def main():
    parser = optparse.OptionParser()
    parser.add_option("--debug", dest="debug", action="store_true",
        default=False, help="print debug output")
    parser.add_option("--log", dest="log",
        default=os.path.expanduser("~/papscan.log"), help="log file path")
    parser.add_option("--dryrun", dest="dryrun", action="store_true",
        default=False, help="dont execute actual command")
    options, args = parser.parse_args()

    while True:
        term = Term(sys.stdin)
        sys.stdout.write("scanner preset: ")
        term.setcbreak()
        try:
            preset = sys.stdin.read(1)
        finally:
            term.restore()
        sys.stdout.write("\n")
        cmd = ["bash"]
        if options.debug:
            cmd.append("-x")
        cmd.extend([os.path.join(os.path.dirname(os.path.abspath(__file__)),
                "scan.sh"), preset])
        if options.dryrun or options.debug:
            sys.stdout.write("%s\n" % cmd)
        if not options.dryrun:
            tee = subprocess.Popen(["tee", "-a", options.log],
                stdin=subprocess.PIPE)
            subprocess.call(cmd, stdout=tee.stdin, stderr=subprocess.STDOUT)
            tee.stdin.close()
            tee.wait()
        if os.path.isfile("/usr/share/sounds/question.wav"):
            subprocess.call(["gst-launch-0.10", "playbin",
                "uri=file:///usr/share/sounds/question.wav"],
                stdout=open(os.devnull, "w"),
                stderr=subprocess.STDOUT)


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        pass


