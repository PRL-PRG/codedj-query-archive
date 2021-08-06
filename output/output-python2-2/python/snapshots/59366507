#!/usr/bin/env python
#-*- coding: utf-8 -*-

import os
import sys
import os.path
import logging

import conf
from conf import config

def setupLogger():
    """
    Setup the root logger from configuration params.
    """

    level = {'CRITICAL': logging.CRITICAL,
             'ERROR': logging.ERROR,
             'WARNING': logging.WARNING,
             'INFO': logging.INFO,
             'DEBUG': logging.DEBUG }

    format = '%(asctime)s %(levelname)s %(message)s'
    datefmt = '%d %b %Y %H:%M:%S'

    if int(config['logger']['log_on_file']):
        log_file = os.path.join(config['logger']['path'],'devclient.log')
        logging.basicConfig(level=level[config['logger']['level']],
                            format=format,
                            datefmt=datefmt,
                            filename=log_file,
                            filemode='a+')
    else:
        logging.basicConfig(level=level[config['logger']['level']],
                            format=format,
                            datefmt=datefmt,
                            stream=sys.stdout)

def main():
    """
    The function is the client entry point. It loads configuration,
    all modules and starts the client with the appropriate classes for
    application and gui.
    """
    os.chdir(os.path.join(os.getcwd(), os.path.dirname(sys.argv[0])))
    conf.loadConfiguration("../etc/devclient.cfg")

    setupLogger()
    logging.debug('*** START DEVCLIENT ***')

    from modules.loader import Loader
    sys.path.append(config['modules']['path'])

    classes = ['Socket', 'Thread', 'Gui', 'Application', 'Parser']
    classes = Loader().load(classes)

    # Set current path on module path for external resources like images
    os.chdir(config['modules']['path'])

    for name, ref in classes.iteritems():
        logging.debug('class: '+ name + ' object: ' + str(ref))

    classes['Thread'](classes)

if __name__ == '__main__':
    main()





