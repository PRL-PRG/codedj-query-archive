#!/usr/bin/python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import sys, cgi, logging, urllib2
import libRemoteTimereg

try:
    from xml.etree import ElementTree as ET
except ImportError:
    try:
        from elementtree import ElementTree as ET
    except ImportError:
        raise ImportError, "ElementTree (or py2.5) needed"

log = logging.getLogger("pyuac.cli")

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

def parseCommand(cmdline):
    action_params = cmdline.split("?", 1)
    action = action_params[0]
    params = {}
    if len(action_params) > 1:
        for k, v in cgi.parse_qsl(action_params[1]):
            if len(v) == 1:
                # parse_qsl restituisce sempre array (anche singole)
                params[str(k)] = v[0]
                # str(k) perchè poi le chiavi del dizionario passeranno
                # per getattr che non accetta unicode (sono nomi di metodi)
            else:
                # devo comunque convertire in stringa il nome (orig. unicode)
                params[str(k)] = v
    if __debug__:
        log.debug("<!--cli params: \n%s\n-->\n" % str(params))
    return action, params

def help(lib):
    res = ["Usare una delle azioni definite:"]
    res += ["  q: Quit"]
    res += ["  %s: %s" % (action, lib.actions[action]) for lib.action in actions]
    return "\n".join(res)

exits = "OK PARAMS_ERROR CONNECTION_ERROR ACTION_ERROR RESPONSE_ERROR".split()
def exit(mode):
    sys.exit(exits.index(mode))

def execute(lib, action, params):
    if __debug__:
        log.debug("cli.%s(%s)" % (action, params))
    #Cerco di mappare l'azione su un metodo
    func = getattr(lib, action)
    if params:
        eres = func(**params)
    else:
        eres = func()
    res = ET.tostring(eres, "utf-8")
    if __debug__:
        log.debug("cli.%s(%s) results: %s" % (action, params, res))
    return res

def serve(params, oneshot=False):
    """
    Questa funzione aspetta l'input dell'utente in forma
    di POST http e redirige la chiamata:
      action?param1=var1&param2=var2
    sul metodo *action* di RemoteTimereg, se questo è
    presente nel dizionario di azioni permesse (e documentate)
    """
    try:
        #Cerca di inizializzare la classe con i parametri forniti
        lib = libRemoteTimereg.RemoteTimereg(*params)
    except urllib2.HTTPError:
        log.error("Connection Error!!\n")
        exit("CONNECTION_ERROR")

    while True:
        #Gira aspettando righe di comando della forma:
        # action?url_encoded=params&other=params
        prompt = (not oneshot) and "remote: " or ""
        cmdline = raw_input(prompt).strip()
        action, params = parseCommand(cmdline)
        if action in lib.actions:
            print execute(lib, action, params)
            if oneshot:
                exit("OK")
        elif action == "q":
            exit("OK")
        elif not oneshot:
            print help()
        else:
            exit("ACTION_ERROR")

if __name__=="__main__":
    params, oneshot = checkParams(sys.argv[1:])
    if params:
        serve(params, oneshot=oneshot)
    else:
        print help
        exit("PARAMS_ERROR")
