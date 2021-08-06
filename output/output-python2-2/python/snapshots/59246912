#!/usr/bin/python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import sys, cgi, urllib2

from xml.parsers.expat import ExpatError
try:
    from xml.etree import ElementTree as ET
except ImportError:
    try:
        from elementtree import ElementTree as ET
    except ImportError:
        raise ImportError, "ElementTree (or py2.5) needed"

import libRemoteTimereg
from pyuac_utils import *

docs = """  Uso:
    http://domain.com/achievo/ user password [--oneshot]
    [--oneshot] attiva la modalità a comando singolo"""

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
        # parse_qsl restutuisce una lista del tipo
        # [('par1', 'var'), ('par2', 'var1'), ('par2', 'var2')] da convertire in
        # => {'par1': 'var',
        #     'par2', ['var1','var2']}
        for k, v in cgi.parse_qsl(action_params[1], keep_blank_values=True):
            # str(k) perchè poi le chiavi del dizionario
            # verranno usate come keyword arguments
            k = str(k)
            v = v.decode("utf-8")
            if params.setdefault(k, v) != v:
                if type(params[k]) is not list:
                    params[k] = [params[k], v]
                else:
                    params[k].append(v)
    return action, params

def help(remote):
    res = ["Usare una delle azioni definite:"]
    res += ["  q: Quit"]
    res += ["  %s: %s" % (action, description) for action, description in remote.actions.items()]
    return "\n".join(res)

exits = "OK CONNECTION_ERROR ACTION_ERROR RESPONSE_ERROR PARAMS_ERROR".split()
def exit(mode, verbose=False):
    if verbose:
        sys.stderr.write(mode)
    sys.exit(exits.index(mode))

def execute(remote, action, params):
    #Cerco di mappare l'azione su un metodo
    func = getattr(remote, action)
    if params:
        eres = func(**params)
    else:
        eres = func()
    res = ET.tostring(eres, "utf-8")
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
        remote = libRemoteTimereg.RemoteTimereg(*params)
    except urllib2.HTTPError:
        exit("CONNECTION_ERROR", True)
    except ExpatError:
        exit("RESPONSE_ERROR", True)

    while True:
        #Gira aspettando righe di comando della forma:
        # action?url_encoded=params&other=params
        prompt = (not oneshot) and "remote: " or ""
        cmdline = raw_input(prompt).strip()
        action, params = parseCommand(cmdline)
        if action in remote.actions:
            print execute(remote, action, params)
            if oneshot:
                exit("OK")
        elif action == "q":
            exit("OK")
        elif not oneshot:
            print help(remote)
        else:
            exit("ACTION_ERROR")

if __name__=="__main__":
    params, oneshot = checkParams(sys.argv[1:])
    if params:
        serve(params, oneshot=oneshot)
    else:
        print docs
        exit("PARAMS_ERROR")
