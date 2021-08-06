#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import sys
from PyQt4 import uic
from PyQt4.QtCore import *
from PyQt4.QtGui import *

try:
    from xml.etree import ElementTree as ET
except ImportError:
    try:
        from elementtree import ElementTree as ET
    except ImportError:
        raise ImportError, "ElementTree (or py2.5) needed"

from QRemoteTimereg import RemoteTimereg
from QTimeBrowseWindow import TimeBrowseWindow

import logging
log = logging.getLogger("pyuac.gui")

def debug(msg):
    if __debug__:
        print msg
        log.debug(msg)

class TimeregApplication(QApplication):
    def __init__(self, args):
        QApplication.__init__(self, args)
        self.remote = RemoteTimereg(self,
                                    ["http://www.develer.com/~naufraghi/achievo/",
                                     "matteo", "matteo99"])
        win = TimeBrowseWindow(self)
        win.show()
        sys.exit(self.exec_())

if __name__ == "__main__":
    app = TimeregApplication(sys.argv)
    #2 m a 2:34 prova prova èàò
