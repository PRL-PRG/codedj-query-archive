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
    """
    Questa funzione aspetta l'input dell'utente in forma
    di POST http e redirige la chiamata:
      action?param1=var1&param2=var2
    sul metodo *action* di RemoteTimereg, se questo è
    presente nel dizionario di azioni permesse (e documentate)
    """
    actions = {"q": "Quit",
               "search": "Search the project",
               "whoami": "Returns login info",
               "timereg": "Register worked time",
               "timereport": "Report time registered in the provided date"
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
                    #parse_qsl restituisce sempre array (anche singole)
                    params[str(k)] = v[0]
                else:
                    #devo comunque convertire in stringa il nome (orig. unicode)
                    params[str(k)] = v
        if __debug__:
            log.debug("<!--cli params: \n%s\n-->\n" % str(params))
        if action not in actions:
            if not oneshot:
                print "Usare una delle azioni definite:"
                for action in actions:
                    print "  %s: %s" % (action, actions[action])
            else: # if oneshot
                sys.exit(1)
        else: #action in actions
            if action == "q":
                sys.exit(0)
            else:
                #Cerco di mappare l'azione su un metodo
                if __debug__:
                    log.debug("cli.%s(%s)" % (action, params))
                try:
                    func = getattr(rt, action)
                except:
                    log.error("Attribute Error! %s(%s)\n" % (action, params))
                    sys.exit(1)
                try:
                    if params:
                        eres = func(**params)
                    else:
                        eres = func()
                except:
                    log.error("Call Error! %s(%s)\n" % (action, params))
                    sys.exit(1)
                try:
                    res = libRemoteTimereg.emsgDump(eres)
                except:
                    log.error("Response Error! %s(%s)\n" % (action, params))
                    sys.exit(1)
                if __debug__:
                    log.debug("cli.%s(%s) results: %s" % (action, params, res))
                out(res+"\n")
                if oneshot:
                    sys.exit(0)

if __name__=="__main__":
    params, oneshot = checkParams(sys.argv[1:])
    if params:
        serve(params, oneshot=oneshot)
    else:
        out(help)
        sys.exit(1)
