#!/usr/bin/python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import cgi
from libRemoteTimereg import *

docs = """  Use:
    http://domain.com/achievo/ user password [--silent]
    [--silent] silent mode (no prompt, no help messages)"""

def checkParams(params):
    if len(sys.argv[1:]) < 3:
        # less than 3 params is invalid
        return False, False
    elif len(sys.argv[1:]) == 3:
        # exactly 3 params, interactive mode
        return sys.argv[1:4], False
    else:
        # at least 4 params, silent mode
        return sys.argv[1:4], True

def parseCommand(cmdline):
    action_params = cmdline.split("?", 1)
    action = action_params[0]
    params = {}
    if len(action_params) > 1:
        # parse_qsl returns a list like
        # [('par1', 'var'), ('par2', 'var1'), ('par2', 'var2')] that we turn into a dict
        # => {'par1': 'var',
        #     'par2', ['var1','var2']}
        for k, v in cgi.parse_qsl(action_params[1], keep_blank_values=True):
            # str(k) because dict keys well be used as
            # keyword arguments
            k = str(k)
            v = v.decode("utf-8")
            if params.setdefault(k, v) != v:
                if type(params[k]) is not list:
                    params[k] = [params[k], v]
                else:
                    params[k].append(v)
    return action, params

def help(remote):
    res = ["Use one of the defined actions:"]
    res += ["  q: Quit"]
    res += ["  %s: %s" % (action, description) for action, description in remote.actions.items()]
    return "\n".join(res)

exits = "OK CONNECTION_ERROR ACTION_ERROR RESPONSE_ERROR PARAMS_ERROR".split()
def exit(mode, verbose=False):
    if verbose:
        sys.stderr.write(mode)
    sys.exit(exits.index(mode))

def execute(remote, action, params):
    # Try to map the action on the correct method
    func = getattr(remote, action)
    if params:
        eres = func(**params)
    else:
        eres = func()
    res = ET.tostring(eres, "utf-8")
    return res

def serve(params, silent=False):
    """
    This function accepts the user input in http POST format
    and redirects tha call:
      action?param1=var1&param2=var2
    on the *action* method of RemoteTimereg, if this method
    is present in the allowed and documented action dict
    """
    try:
        # Try to set up the class with the provided params
        remote = RemoteTimereg(*params)
    except urllib2.HTTPError:
        exit("CONNECTION_ERROR", True)
    except ExpatError:
        exit("RESPONSE_ERROR", True)

    while True:
        # Loops waiting for input in the form:
        # action?url_encoded=params&other=params
        prompt = (not silent) and "remote: " or ""
        try:
            cmdline = raw_input(prompt).strip()
        except KeyboardInterrupt:
            exit("OK")
        action, params = parseCommand(cmdline)
        if action in remote.actions:
            print execute(remote, action, params)
        elif action == "q":
            exit("OK")
        elif not silent:
            print help(remote)
        else:
            exit("ACTION_ERROR")

if __name__=="__main__":
    params, silent = checkParams(sys.argv[1:])
    if params:
        serve(params, silent=silent)
    else:
        print docs
        exit("PARAMS_ERROR")
