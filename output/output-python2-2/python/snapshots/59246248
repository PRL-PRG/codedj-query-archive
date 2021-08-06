#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import sys, getpass

########################## Congiguration ###############################

config = {"achievouri": "http://www.develer.com/groupware/",
          "username": getpass.getuser()}

#config = {"achievouri": "http://www.develer.com/~naufraghi/achievo/",
#          "username": "matteo"}

########################## Congiguration ###############################

from QTimeBrowseWindow import *

class TimeregApplication(QApplication):
    def __init__(self, args):
        print "TimeregApplication(QApplication).__init__"
        QApplication.__init__(self, args)
        win = TimeBrowseWindow(config)
        win.show()
        sys.exit(self.exec_())

if __name__ == "__main__":
    app = TimeregApplication(sys.argv)
    #2 m a 2:34 prova prova èàò
