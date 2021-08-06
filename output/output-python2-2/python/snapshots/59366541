#!/usr/bin/env python
#-*- coding: utf-8 -*-

import os
import sys
import os.path

import conf
from conf import config

def main():
    """
    The function is the client entry point. It loads configuration,
    all modules and starts the client with the appropriate classes for
    application and gui.
    """

    os.chdir(os.path.join(os.getcwd(), os.path.dirname(sys.argv[0])))
    conf.loadConfiguration("../etc/devclient.cfg")

    sys.path.append(config['modules']['path'])

    from modules.loader import Loader

    classes = ['Socket', 'Thread', 'Gui', 'Application', 'Parser']
    classes = Loader().load(classes)

    # Set current path on module path for external resources like images
    os.chdir(config['modules']['path'])

    print classes

    classes['Thread'](classes)

if __name__ == '__main__':
    main()





