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


import random
import subprocess
from time import time
from shutil import copy
from sys import argv, path
from optparse import OptionParser
from os import getcwd, chdir, unlink
from os.path import basename, abspath, normpath, join, dirname, exists

from PyQt4.QtTest import QTest
from PyQt4.QtCore import QTimer, Qt

# FIX
path.append(join(dirname(abspath(argv[0])), '../..'))
path.append(join(dirname(abspath(argv[0])), '../../configobj'))

import devclient.storage as storage
import devclient.exception as exception
from devclient.utils import terminateProcess, startProcess
from devclient.conf import loadConfiguration, config

_DEF_CONFIG_FILE = "../../../etc/devclient.cfg"
cfg_file = normpath(join(dirname(abspath(argv[0])), _DEF_CONFIG_FILE))


class StartConnection(object):
    def __init__(self, gui):
        self.f = gui._conn_manager.startConnection

    def __call__(self,*a,**k):
        self.time = time()
        return self.f(*a,**k)


class EndConnection(object):
    def __init__(self, gui):
        self.gui = gui
        self.f = gui.displayWarning

    def __call__(self,*a,**k):
        if self.gui._text['ConnLost'] == unicode(a[1]):
            self.time = time()
            callback()
        return self.f(*a,**k)

def startAction(gui):
    callback.s = gui._conn_manager.startConnection = StartConnection(gui)
    callback.e = gui.displayWarning = EndConnection(gui)
    QTest.mouseClick(gui.button_connect, Qt.LeftButton)

def callback():
    print 'total time:' ,callback.e.time - callback.s.time

def readTestOptions(dirname):
    if exists(join(dirname, 'options.cfg')):
        fd = open(join(dirname, 'options.cfg'))
        data = [r.split('=') for r in fd.readlines()]
        fd.close()
        return dict([(k.strip(), v.strip()) for k,v in data])
    return {}


def main(cfg_file=cfg_file):

    parser = OptionParser()
    parser.add_option('-t', '--test', default='test',
                      help='the test directory (default %default)')
    o, args = parser.parse_args()
    test_options = readTestOptions(o.test)

    old_dir = getcwd()
    chdir(join(getcwd(), dirname(argv[0]), dirname(cfg_file)))
    cfg_file = join(getcwd(), basename(cfg_file))
    loadConfiguration(cfg_file)
    storage.loadStorage()
    conn = [0, 'black_box_test', 'localhost', 6666]
    storage.addConnection(conn)
    default_conn = storage.option('default_connection')
    storage.setOption('default_connection', conn[0])
    path.append(config['servers']['path'])
    path.append(config['resources']['path'])

    chdir(old_dir)
    if exists(join(o.test, 'localhost_server.py')):
        copy(join(o.test, 'localhost_server.py'), config['servers']['path'])

    # this import must stay here, after the appending of resources path to path
    from devclient.gui import Gui

    try:
        gui = Gui(cfg_file)
        cwd = dirname(argv[0]) if dirname(argv[0]) else None
        cmd = ['python', '-u', 'server_test.py']
        if 'delay' in test_options:
            cmd.append('-d')
            cmd.append(test_options['delay'])
        cmd.append(o.test + '/data.txt')
        gui.p = subprocess.Popen(cmd, stdout=subprocess.PIPE, cwd=cwd)

        try:
            buf = gui.p.stdout.read(6) # read READY\n from stdout
        except IOError:
            time.sleep(.5)

        Gui.startAction = startAction
        QTimer.singleShot(2000, gui.startAction)
        gui.mainLoop()
    except exception.IPCError:
        if not o.core_port:
            terminateProcess(p.pid)
    except Exception, e:
        print 'Fatal Exception:', e
        if not o.core_port:
            terminateProcess(p.pid)
    finally:
        storage.deleteConnection(conn)
        storage.setOption('default_connection', default_conn)
        fn = join(config['servers']['path'], 'localhost_server.py')
        if exists(fn):
            unlink(fn)

if __name__ == '__main__':
    main()