#!/usr/bin/python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import sys, cgi, logging

import libRemoteTimereg

ET = libRemoteTimereg.ET

log = logging.getLogger("pyuac.cli")
log.addFilter(logging.Filter("pyuac"))

out = sys.stdout.write

help = """Uso:
    http://domain.com/achievo/ user password [--]
    [--] attiva la modalità a comando singolo
"""

def checkParams(params):
    if len(sys.argv[1:]) < 3:
        #meno di 3 parametri non va
        return False, False
    elif len(sys.argv[1:]) == 3:
        #esattamente tre, parte in modalità interattiva
        return sys.argv[1:4], False
    else:
        #almeno 4, parte in modalità oneshot
        return sys.argv[1:4], True

def serve(params, oneshot=False):
    actions = {"q": "Quit",
               "search": "Search the project",
               "whoami": "Returns login info",
               "timereg": "Register worked time"
            }
    try:
        rt = libRemoteTimereg.RemoteTimereg(*params)
    except libRemoteTimereg.urllib2.HTTPError:
        log.error("Connection Error!!\n")
        sys.exit(1)
    while True:
        #Gira aspettando righe di comando della forma:
        # action?url_encoded=params&other=params
        prompt = (not oneshot) and "remote: " or ""
        msg = raw_input(prompt).strip()
        cmdline = msg.split("?", 1)
        action = cmdline[0]
        params = {}
        if len(cmdline) > 1:
            for k, v in cgi.parse_qsl(cmdline[1]):
                if len(v) == 1:
                    #parse_qsl restituisce sempre array (anche singole) come valore
                    params[str(k)] = v[0]#.decode("UTF-8")
                else:
                    #devo comunque convertire in stringa il nome (orig. unicode)
                    params[str(k)] = v#[i.decode("UTF-8") for i in v]
        if __debug__:
            log.debug("<!--params: \n%s\n-->\n" % str(params))
        if action not in actions:
            if not oneshot:
                print "Usare una delle azioni definite:"
                for action in actions:
                    print "  %s: %s" % (action, actions[action])
            else:
                sys.exit(1)
        else: #action in actions
            if action == "q":
                sys.exit(0)
            try:
                #Cerco di mappare l'azione su un metodo
                func = getattr(rt, action)
                if params:
                    res = func(**params)
                else:
                    res = func()
                if __debug__:
                    log.debug("ASD4 :" + ET.tostring(res, encoding="utf-8"))
                out(ET.tostring(res, encoding="utf-8")+"\n")
            except:
                log.error("Response Error! %s(%s)\n" % (action, params))
                if __debug__:
                    raise
                sys.exit(1)
            if oneshot:
                sys.exit(0)

if __name__=="__main__":
    params, oneshot = checkParams(sys.argv[1:])
    if params:
        serve(params, oneshot=oneshot)
    else:
        out(help)
    sys.exit(0)
