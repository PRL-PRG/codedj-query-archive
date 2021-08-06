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

import libRemoteTimereg

class TimeregApplication(QApplication):
    def __init__(self, args):
        QApplication.__init__(self, args)
        win = TimeregWindow()
        win.show()
        sys.exit(self.exec_())
        
class TimeregWindow(QMainWindow):
    def __init__(self,):
        QMainWindow.__init__(self)
        self.ui = uic.loadUi("pyuac_edit.ui", self)
        self.rt = libRemoteTimereg.RemoteTimereg("http://www.develer.com/~naufraghi/achievo/", "matteo", "matteo99")

        self._projphaseact = ""
        self._hours = ""
        self._comment = ""
        self.projects = []
        
        self._connectSlots()
        self._setupGui()

    def _setupGui(self):
        self.ui.dateTimeregDate.setDate(QDate.currentDate())
        self._slotSmartQueryChanged()

    def _connectSlots(self):
        self.connect(self.ui.comboSmartQuery, SIGNAL("editTextChanged(QString)"), self._slotSmartQueryChanged)

    def _slotSmartQueryChanged(self):
        smartquery = unicode(self.ui.comboSmartQuery.lineEdit().text())
        projphaseact, self.hours, self.comment = self._parseSmartQuery(smartquery)
        if projphaseact != self._projphaseact:
            self.projects = self.rt.search(projphaseact)
            self._projphaseact = projphaseact
        self._smartUpdateGui()
            
    def _parseSmartQuery(self, smartquery):
        if smartquery == "":
            #Se vuota converte in "trova tutto"
            smartquery = "% % %"
        smartdict = libRemoteTimereg.parseSmartQuery(smartquery)
        projphaseact = "%(project)s %(phase)s %(activity)s" % smartdict
        return projphaseact, smartdict["hours"], smartdict["comment"]
        
    def _smartUpdateGui(self):
        self.ui.comboProjectPhase.clear()
        self.ui.comboActivity.clear()
        projphases = set()
        activities = set()
        for p in self.projects:
            projphases.add("%(project_name)s / %(phase_name)s" % dict(p.record.items()))
            activities.add(p.record.get("activity_name"))
        self.ui.comboProjectPhase.addItems(list(projphases))
        self.ui.comboActivity.addItems(list(activities))
        self.ui.labelProjectPhase.setEnabled(self.ui.comboProjectPhase.count() != 1)
        self.ui.labelActivity.setEnabled(self.ui.comboActivity.count() != 1)
        if len(self.projects) == 1:
            d = self.ui.timeTimeWorked.dateTimeFromText(self.hours)
            zero = self.ui.timeTimeWorked.dateTimeFromText("0:00")
            self.ui.timeTimeWorked.setDateTime(d)
            self.ui.labelTimeWorked.setEnabled(d > zero)
            self.ui.labelComment.setEnabled(self.comment != "")
            self.ui.txtComment.setPlainText(self.comment)
                
if __name__ == "__main__":
    app = TimeregApplication(sys.argv)
