#!/usr/bin/python

import optparse
import os
import urlparse
import subprocess
import sys

import gnomevfs
import gobject


def call(*args, **kwargs):
    print args[0]#; return 0
    return subprocess.call(*args, **kwargs)


def callback(monitor_uri, trigger_uri, event_type, dest):
    #print monitor_uri, trigger_uri, event_type
    if event_type in (gnomevfs.MONITOR_EVENT_CHANGED,
                      gnomevfs.MONITOR_EVENT_CREATED):
        call(["rsync", "-avz", urlparse.urlsplit(trigger_uri).path, dest])
 

def main():
    parser = optparse.OptionParser("%prog src dest")
    options, args = parser.parse_args()
    if len(args) != 2:
        parser.error("Insufficient arguments")
    src = args[0].rstrip("/")
    dest = args[1]
    os.listdir(src)
    # i.e. like -a but not recursive
    if call(["rsync", "-lptgoDdvz", src + "/", dest]) != 0:
        sys.exit(0)
    gnomevfs.monitor_add(os.path.abspath(src), gnomevfs.MONITOR_DIRECTORY,
                         callback, dest)
    try:
        gobject.MainLoop().run()
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    main()

