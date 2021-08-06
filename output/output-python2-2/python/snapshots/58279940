#!/usr/bin/python

try:
    import optparse
    import os
    import subprocess
    import sys
    
    def main():
        parser = optparse.OptionParser("%prog [options] host")
        parser.add_option("--gw", dest="gateway", nargs=2,
            help="host to proxy through")
        options, args = parser.parse_args()
        if len(args) == 0:
            parser.error("No host specified")
        argv = ["ssh"] + args
        if options.gateway is not None:
            argv.extend(["-o", "ProxyCommand ssh %s nc -w 10 %s ssh" % options.gateway])
        print argv
        rc = subprocess.call(argv)
        if rc != 0:
        	raw_input()
    
    if __name__ == "__main__":
        main()
except KeyboardInterrupt:
    pass
