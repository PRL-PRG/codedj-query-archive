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
from PyQt4 import QtCore, QtGui, uic
import libRemoteTimereg

class TimeregApplication(QtGui.QApplication):
    def __init__(self, args):
        QtGui.QApplication.__init__(self, args)
        win = TimeregWindow()
        win.show()
        sys.exit(self.exec_())

class TimeregWindow(QtGui.QMainWindow):
    def __init__(self,):
        QtGui.QMainWindow.__init__(self)
        self.ui = uic.loadUi("pyuac_edit.ui", self)
        self.rt = libRemoteTimereg.RemoteTimereg("http://www.develer.com/~naufraghi/achievo/", "matteo", "matteo99")
        self._connectSlots()
        self._setup_gui()
    def _setup_gui(self):
        self.ui.datedTimeregDate.setDate(QtCore.QDate.currentDate())
        self.ui.lstTimeregDay.addItems(["Select e day or","start typing below"])
    def _connectSlots(self):
        self.connect(self.ui.datedTimeregDate, QtCore.SIGNAL("dateChanged(QDate)"), self._slotDateChanged)
        self.connect(self.ui.lstTimeregDay, QtCore.SIGNAL("itemSelectionChanged()"), self._slotTimeregSelected)
    def _slotDateChanged(self):
        qdate = str(self.ui.datedTimeregDate.date().toString("yyyy-MM-dd"))
        self.projects = self.rt.timereport(qdate)
        self.ui.lstTimeregDay.clear()
        self.ui.lstTimeregDay.addItems(map(unicode, self.projects))
    def _slotTimeregSelected(self):
        try:
            selectedid = self.ui.lstTimeregDay.indexFromItem(self.ui.lstTimeregDay.selectedItems()[0]).row()
        except:
            selectedid = None
        if selectedid == None:
            body = "Select a row..."
        else:
            p = self.projects[selectedid]
            #self.ui.txtbrProjectSummary.setPlainText(repr(p))
            body = "<h3>%(activitydate)s - %(time)s minutes</h3>%(remark)s" % dict(p.record.items())
        self.ui.txtbrProjectSummary.setPlainText(body)

if __name__ == "__main__":
    app = TimeregApplication(sys.argv)
