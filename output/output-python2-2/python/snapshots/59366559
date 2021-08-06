#!/usr/bin/env python
#-*- coding: utf-8 -*-

import os
import sys
import os.path
import ConfigParser

def readConfiguration(filename):
    """
    Read configuration file

    :Parameters:
      filename : str
         the path of config file

    :return: a dictionary of the form [section][option]
    """

    def changePath(k, v):
        if k == 'path':
            v = os.path.abspath(os.path.join(os.getcwd(), v))
        return v

    config = ConfigParser.SafeConfigParser()
    config.read(filename)
    conf = {}
    for s in config.sections():
        conf[s] = dict([(k, changePath(k, v)) for k, v in config.items(s)])

    return conf

def main():
    """
    The function is the client entry point. It reads configuration file, loads
    all modules and starts the client with the appropriate classes for
    application and gui.
    """
    
    os.chdir(os.path.join(os.getcwd(), os.path.dirname(sys.argv[0])))
    config = readConfiguration("../etc/devclient.cfg")

    sys.path.append(config['modules']['path'])

    from modules.loader import Loader

    classes = ['Socket', 'Thread', 'Gui', 'Application', 'Parser']
    classes = Loader(config).load(classes)

    # Set current path on module path for external resources like images
    os.chdir(config['modules']['path'])

    print classes

    classes['Thread'](classes, config)

if __name__ == '__main__':
    main()





