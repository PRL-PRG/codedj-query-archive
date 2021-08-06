#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import sys

import libRemoteTimereg

ET = libRemoteTimereg.ET

help = """
Expected input:
    http://domain.com/achievo/ user password
"""

o = sys.stdout.write
e = sys.stderr.write

if len(sys.argv[1:]) != 3:
    o(help)
else:
    try:
        rt = libRemoteTimereg.RemoteTimereg(*sys.argv[1:])
    except libRemoteTimereg.urllib2.HTTPError:
        e("Connection Error!!\n")
        sys.exit(1)
    while True:
        msg = raw_input("AchievoRemote: ")
        action = msg.split(" ")[0]
        params = " ".join(msg.split(" ")[1:])
        if action == "q":
            sys.exit(0)
        try:
            func = getattr(rt, action)
            if params != "":
                o(ET.tostring(func(params))+"\n")
            else:
                o(ET.tostring(func())+"\n")
        except AttributeError:
            pass
        except:
            e("Response Error!\n")
            sys.exit(1)
