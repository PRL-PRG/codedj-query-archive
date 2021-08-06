#!/usr/bin/env python
#-*- coding: utf-8 -*-

import os
import sys
import os.path
import ConfigParser

def readConfiguration(filename):
    """
    Read configuration file and return a dictionary of the form [section][option]
    """

    config = ConfigParser.SafeConfigParser()
    config.read(filename)
    conf = {}
    for sect in config.sections():
        conf[sect] = dict(config.items(sect))

    return conf

if __name__ == '__main__':

    os.chdir(os.path.join(os.getcwd(), os.path.dirname(sys.argv[0])))
    config = readConfiguration("../etc/devclient.cfg")

    module_path = os.path.join(os.getcwd(), config['main']['module_path'])
    sys.path.append(module_path)

    from loader import Loader

    classes = ['Socket', 'Thread', 'Gui', 'Application', 'Parser']
    classes = Loader(config).load(classes)

    # Set current path on module path for external resources like images
    os.chdir(module_path)

    print classes

    classes['Thread'](classes)





