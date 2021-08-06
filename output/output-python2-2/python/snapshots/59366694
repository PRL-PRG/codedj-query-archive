#!/usr/bin/env python
#-*- coding: utf-8 -*-

import os
import sys
import os.path
import ConfigParser

def readConfiguration(filename):

    config = ConfigParser.SafeConfigParser()
    config.read(filename)
    conf = {}
    for sect in config.sections():
        conf[sect] = dict(config.items(sect))

    return conf

if __name__ == '__main__':

    config = readConfiguration("../etc/devclient.cfg")

    sys.path.append(os.path.join(os.getcwd(), config['main']['module_path']))

    from loader import Loader

    classes = ['Socket', 'Thread', 'Gui']
    classes = Loader(config).load(classes)

    print classes

    classes['Thread'](classes)





