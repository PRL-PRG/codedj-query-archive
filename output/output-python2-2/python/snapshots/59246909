#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import getpass

########################## Congiguration ###############################

config = {"achievouri": "https://www.develer.com/groupware/",
          "username": getpass.getuser()}

config = {"achievouri": "http://www.develer.com/~naufraghi/achievo/",
          "username": "matteo"}

########################## Congiguration ###############################

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

from QTimeBrowseWindow import TimeBrowseWindow
from pyuac_utils import *

class TimeregApplication(QApplication):
    def __init__(self, args):
        QApplication.__init__(self, args)
        win = TimeBrowseWindow(config)
        win.show()
        sys.exit(self.exec_())

if __name__ == "__main__":
    app = TimeregApplication(sys.argv)
    #2 m a 2:34 prova prova èàò
