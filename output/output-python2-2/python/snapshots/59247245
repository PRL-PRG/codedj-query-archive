#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import sys, time
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

import libRemoteTimereg

class TimeregApplication(QApplication):
    def __init__(self, args):
        QApplication.__init__(self, args)
        win = TimeregWindow("http://www.develer.com/~naufraghi/achievo/",
                            "matteo", "matteo99")
        win.show()
        sys.exit(self.exec_())
        
class TimeregWindow(QMainWindow):
    def __init__(self, achievouri, user, password):
        QMainWindow.__init__(self)
        self.auth = [achievouri, user, password]
        self.ui = uic.loadUi("pyuac_edit.ui", self)
        self.rt = QProcess(self)
 
        self.projects = []
        self._lastquery = ""
        self._connectSlots()
        self._setupGui()

    def _setupGui(self):
        self.ui.dateTimeregDate.setDate(QDate.currentDate())
        self._slotSmartQueryChanged()

    def _connectSlots(self):
        self.connect(self.ui.comboSmartQuery,
                     SIGNAL("editTextChanged(QString)"),
                     self._slotSmartQueryChanged)
        self.connect(self.rt,
                     SIGNAL("finished(int)"),
                     self._smartUpdateGui)

    def _slotSmartQueryChanged(self):
        if __debug__:
            qDebug("_slotSmartQueryChanged, %s" % self.rt.state())
        smartquery = unicode("search?smartquery=%"+self.ui.comboSmartQuery.lineEdit().text()+"\n")
        if self.rt.state() != self.rt.NotRunning:
            QTimer.singleShot(500, self._slotSmartQueryChanged)
        else:
            if smartquery != self._lastquery:
                self.rt.start("./pyuac_cli.py", self.auth+["--"])
                self.rt.write(smartquery.encode("UTF-8"))
                self._lastquery = smartquery
                    
    def _smartUpdateGui(self):
        if __debug__:
            qDebug("_smartUpdateGui")
        msg = str(self.rt.readAllStandardOutput()).decode("UTF-8")
        if msg == "":
            if __debug__:
                qDebug("No Output!")
            return
        try:
            self.projects = ET.fromstring(msg)
        except libRemoteTimereg.ExpatError:
            print msg
        self.ui.comboProjectPhase.clear()
        self.ui.comboActivity.clear()
        projphases = set()
        activities = set()
        for p in self.projects:
            projphases.add("%(project_name)s / %(phase_name)s" % dict(p.items()))
            activities.add(p.get("activity_name"))
        self.ui.comboProjectPhase.addItems(list(projphases))
        self.ui.comboActivity.addItems(list(activities))
        self.ui.labelProjectPhase.setEnabled(self.ui.comboProjectPhase.count() != 1)
        self.ui.labelActivity.setEnabled(self.ui.comboActivity.count() != 1)
        if len(self.projects) == 1:
            p = self.projects[0]
            d = self.ui.timeTimeWorked.dateTimeFromText(p.get("input_hours"))
            zero = self.ui.timeTimeWorked.dateTimeFromText("0:00")
            self.ui.timeTimeWorked.setDateTime(d)
            self.ui.labelTimeWorked.setEnabled(d > zero)
            self.ui.labelRoundTime.setText(p.get("hmtime"))
            self.ui.labelRemark.setEnabled(p.get("input_remark") != "")
            self.ui.txtRemark.setPlainText(p.get("input_remark"))

if __name__ == "__main__":
    app = TimeregApplication(sys.argv)
