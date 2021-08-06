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
import os.path
from time import strftime
from traceback import print_exc
from os.path import dirname, join, abspath, normpath
from sys import path, argv, platform, exc_info

import exception
from constants import PROJECT_NAME
from conf import loadConfiguration, config

cfg_file = normpath(dirname(abspath(__file__)) + "/../../etc/devclient.cfg")


def save_exception():
    """
    Save a detailed traceback of a fatal exception.
    """

    def extract_stack():
        tb = exc_info()[2]
        while tb.tb_next:
            tb = tb.tb_next

        stack = []
        f = tb.tb_frame
        while f:
            stack.append(f)
            f = f.f_back
        stack.reverse()

        return stack

    fd = open('exception.txt', 'a+')
    fd.write("%s FATAL EXCEPTION %s %s\n" % ('*' * 23,
                                             strftime("%Y-%m-%d %H:%M"),
                                             '*' * 23))
    print_exc(file=fd)
    fd.write("\n%s\n" % ('+' * 80, ))
    stack = extract_stack()

    for frame in stack:
        fd.write("\nFrame %s in %s at line %s\n" % (frame.f_code.co_name,
                                                    frame.f_code.co_filename,
                                                    frame.f_lineno))
        for key, value in frame.f_locals.items():
            if key not in ('__doc__', '__docformat__', '__builtins__'):
                try:
                    fd.write("%s = %s\n" % (key, value))
                except:
                    fd.write("%s = <UNKNOWN VALUE>\n" % key)

    fd.write("%s\n" % ('*' * 80, ))
    fd.close()

def main(argv=argv, cfg_file=cfg_file, update=1):
    """
    The function is the client entry point.
    """

    start_dir = os.getcwd()
    os.chdir(join(start_dir, dirname(argv[0]), dirname(cfg_file)))
    cfg_file = join(os.getcwd(), os.path.basename(cfg_file))
    loadConfiguration(cfg_file)
    os.chdir(start_dir)
    path.append(config['servers']['path'])
    path.append(config['resources']['path'])
    path.append(config['configobj']['path'])

    # this import must stay here, after the appending of configobj path to path
    import storage
    storage.loadStorage()

    # this import must stay here, after the appending of resources path to path
    from gui import Gui
    try:
        gui = Gui(cfg_file)
        if not update:
            gui.displayWarning(PROJECT_NAME, gui._text['UpdateFail'])
        gui.mainLoop()
    except exception.IPCError:
        save_exception()
    except Exception, e:
        print 'Fatal Exception:', e
        save_exception()

