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

import sys
import random
import subprocess
from time import time
from os import getcwd, chdir
from os.path import basename, abspath, normpath, join, dirname

from PyQt4.QtTest import QTest
from PyQt4.QtCore import QTimer, Qt

sys.path.append(join(dirname(abspath(__file__)), '../..'))

import devclient.exception as exception
from devclient.engine import terminateProcess, startProcess
from devclient.conf import loadConfiguration, config
from devclient.storage import Storage, adjustSchema

_DEF_CONFIG_FILE = "../../../etc/devclient.cfg"
cfg_file = normpath(join(dirname(abspath(__file__)), _DEF_CONFIG_FILE))


class StartConnection(object):
    def __init__(self, gui):
        self.f = gui._startConnection

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
    callback.s = gui._startConnection = StartConnection(gui)
    callback.e = gui.displayWarning = EndConnection(gui)
    QTest.mouseClick(gui.button_connect, Qt.LeftButton)

def callback():
    print 'total time:' ,callback.e.time - callback.s.time

def main(cfg_file=cfg_file):

    old_dir = getcwd()
    chdir(join(getcwd(), dirname(sys.argv[0]), dirname(cfg_file)))
    cfg_file = join(getcwd(), basename(cfg_file))
    loadConfiguration(cfg_file)
    config['storage']['path'] = abspath('../data/storage/dbtest.sqlite')
    adjustSchema()  #create the schema must be before adding the connection
    Storage().addConnection([0, 'localhost', 'localhost', 6666])
    sys.path.append(config['servers']['path'])
    sys.path.append(config['resources']['path'])

    chdir(old_dir)
    # this import must stay here, after the appending of resources path to path
    from devclient.gui import Gui

    port = random.randint(2000, 10000)

    p = startProcess(['python',
                      join(config['devclient']['path'], 'core.py'),
                      '--config=%s' % cfg_file,
                      '--port=%d' % port])

    # FIX! To prevent connectionRefused from SocketToGui
    import time
    time.sleep(.5)

    try:
        gui = Gui(port)
        gui.p = subprocess.Popen(['python', '-u','server_test.py'],
                                 stdout=subprocess.PIPE, cwd=dirname(__file__))

        try:
            buf = gui.p.stdout.read(6) # read READY\n from stdout
        except IOError:
            time.sleep(.5)

        Gui.startAction = startAction
        QTimer.singleShot(2000, gui.startAction)
        gui.mainLoop()
    except exception.IPCError:
        terminateProcess(p.pid)
    except Exception, e:
        print 'Fatal Exception:', e
        terminateProcess(p.pid)

if __name__ == '__main__':
    main()