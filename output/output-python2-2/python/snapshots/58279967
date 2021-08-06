#!/usr/bin/python

import os
import urlparse
import subprocess
import sys

import gnomevfs
import gobject


if __name__ == "__main__":
    def callback(monitor_uri, info_uri, event_type):
        if event_type in (gnomevfs.MONITOR_EVENT_CHANGED, 
                          gnomevfs.MONITOR_EVENT_CREATED):
            cmd = ["rsync", "-avz", urlparse.urlsplit(info_uri)[2], dest]
            print cmd
#            subprocess.call(cmd)
    #    print monitor_uri, info_uri, event_type
    dest = sys.argv[1]
    gnomevfs.monitor_add(os.getcwd(), gnomevfs.MONITOR_DIRECTORY, callback)
    try:
        gobject.MainLoop().run()
    except KeyboardInterrupt:
        pass
