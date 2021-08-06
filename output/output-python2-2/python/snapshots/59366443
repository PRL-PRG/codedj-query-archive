#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright (C) 2007 Gianni Valdambrini, Develer S.r.l (http://www.develer.com)
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.
#
# Author: Gianni Valdambrini gvaldambrini@develer.com

__version__ = "$Revision$"[11:-2]
__docformat__ = 'restructuredtext'

import os
import sys
import os.path
import logging

import conf
from conf import config
from modules.thread import Thread

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

    # Set current path on module path for external resources like images
    os.chdir(config['modules']['path'])

    Thread()

if __name__ == '__main__':
    main()





