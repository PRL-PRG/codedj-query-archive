#!/usr/bin/python
# -*- coding: utf-8 -*-
""" Parse /etc/gcs.conf for configuration options
"""

import sys

import syck

try:
    config = syck.load(open('/etc/gcs.conf').read())
except:
    print "Can't read /etc/gcs.conf file."
    sys.exit(1)

