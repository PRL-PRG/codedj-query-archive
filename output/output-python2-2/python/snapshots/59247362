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
        self._smartquery = ""
        self._connectSlots()
        self._setup_gui()
    def _setup_gui(self):
        self.ui.dateTimeregDate.setDate(QtCore.QDate.currentDate())
        self._slotSmartQueryChanged()
    def _connectSlots(self):
        self.connect(self.ui.comboSmartQuery, QtCore.SIGNAL("editTextChanged(QString)"), self._slotSmartQueryChanged)
    def _slotSmartQueryChanged(self):
        smartquery = unicode(self.ui.comboSmartQuery.lineEdit().text())
        if smartquery == "":
            smartquery = "% % %"
        elif smartquery == self._smartquery:
            return
        self._smartquery = smartquery
        projects = self.rt.search(smartquery)
        self.ui.comboProjectPhase.clear()
        self.ui.comboActivity.clear()
        projphases = set()
        activities = set()
        for p in projects:
            projphases.add("%(project_name)s / %(phase_name)s" % dict(p.record.items()))
            activities.add(p.record.get("activity_name"))
        self.ui.comboProjectPhase.addItems(list(projphases))
        self.ui.comboActivity.addItems(list(activities))
        self.ui.labelProjectPhase.setEnabled(self.ui.comboProjectPhase.count() == 1)
        self.ui.labelActivity.setEnabled(self.ui.comboActivity.count() == 1)
        if len(projects) == 1:
            d = self.ui.timeTimeWorked.dateTimeFromText(unicode(projects[0].record.get("hours")))
            zero = self.ui.timeTimeWorked.dateTimeFromText("0:00")
            self.ui.timeTimeWorked.setDateTime(d)
            self.ui.labelTimeWorked.setEnabled(d > zero)
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
