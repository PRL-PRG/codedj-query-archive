#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id$
#
# Author: Lorenzo Berni <duplo@develer.com>

from PyQt4.QtCore import *
from PyQt4.QtGui import *
from PyQt4.uic import loadUi
from QRemoteTimereg import QAchievoWindow
from pyuac_utils import *
import sys


class TimeCalculator(QWidget):

    def __init__(self, parent = None):
        QWidget.__init__(self, parent)
        self.ui = QAchievoWindow.loadUi("time_calculator.ui", self)
        self.connect(self.ui.smartTimeEdit, SIGNAL("textEdited(QString)"), self._slotSmartTimeEdited)
        self.ui.show()
    
    def _slotSmartTimeEdited(self, smartime):
        try:
            lapse = parse_wtime(smartime)
            lapse = lapse[: lapse.find(":", 3)]
        except:
            lapse = "0:00"
        if len(smartime):
            self.ui.smartTimeResult.setText(QString(lapse))


if __name__ == "__main__":
    app = QApplication(sys.argv)
    tc = TimeCalculator()
    sys.exit(app.exec_())
