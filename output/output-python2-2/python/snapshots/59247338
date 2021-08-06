#!/usr/bin/python -O
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

help = """Uso:
    http://domain.com/achievo/ user password [--]
    [--] attiva la modalità a comando singolo
"""

o = sys.stdout.write
e = sys.stderr.write

if len(sys.argv[1:]) < 3:
    o(help)
else:
    try:
        rt = libRemoteTimereg.RemoteTimereg(*sys.argv[1:4])
    except libRemoteTimereg.urllib2.HTTPError:
        e("Connection Error!!\n")
        sys.exit(1)
    while True:
        msg = raw_input().strip().decode("UTF-8")
        cmdline = msg.split(" ", 1)
        if len(cmdline) > 1:
            action, params = cmdline
        else:
            action = cmdline[0]
            params = ""
        if action == "q":
            sys.exit(0)
        try:
            func = getattr(rt, action)
            if params != "":
                o(ET.tostring(func(params)).encode("UTF-8")+"\n")
            else:
                o(ET.tostring(func()).encode("UTF-8")+"\n")
        except AttributeError:
            pass
        except:
            e("Response Error!\n")
            sys.exit(1)
        if sys.argv[-1] == "--":
            sys.exit(0)

