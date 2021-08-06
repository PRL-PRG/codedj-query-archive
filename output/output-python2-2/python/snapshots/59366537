#!/usr/bin/env python
#-*- coding: utf-8 -*-

import os
import os.path
import ConfigParser

config = {}

def loadConfiguration(filename):
    """
    Load configuration

    :Parameters:
      filename : str
         the path of config file

    """

    global config

    def changePath(k, v):
        if k == 'path':
            v = os.path.abspath(os.path.join(os.getcwd(), v))
        return v

    cp = ConfigParser.SafeConfigParser()
    cp.read(filename)
    for s in cp.sections():
        config[s] = dict([(k, changePath(k, v)) for k, v in cp.items(s)])
