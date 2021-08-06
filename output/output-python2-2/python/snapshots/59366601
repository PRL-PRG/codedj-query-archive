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

    config = ConfigParser.SafeConfigParser()
    config.read(filename)
    conf = {}
    for sect in config.sections():
        conf[sect] = dict(config.items(sect))

    return conf

def main():
    """
    The function is the client entry point. It read configuration file, load
    all modules and start the client with the appropriate classes for
    application and gui.
    """

    os.chdir(os.path.join(os.getcwd(), os.path.dirname(sys.argv[0])))
    config = readConfiguration("../etc/devclient.cfg")

    module_path = os.path.join(os.getcwd(), config['main']['module_path'])
    sys.path.append(module_path)

    from modules.loader import Loader

    classes = ['Socket', 'Thread', 'Gui', 'Application', 'Parser']
    classes = Loader(config).load(classes)

    # Set current path on module path for external resources like images
    os.chdir(module_path)

    print classes

    classes['Thread'](classes)

if __name__ == '__main__':
    main()





