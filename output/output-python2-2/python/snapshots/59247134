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

ZEROtime = QDateTime.fromString("00:00", "HH:mm")

def debug(msg):
    if __debug__:
        qDebug("#-#-#-#-# "+msg.replace(r"%%", r"%").replace(r"%", r"%%"))

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
        self.rt = RemoteTimereg(self, self.auth)
        self.err = QErrorMessage(self)
 
        self.projects = []
        self._connectSlots()
        self._setupGui()

    def _setupGui(self):
        self.ui.dateTimeregDate.setDate(QDate.currentDate())
        self.ui.comboSmartQuery.lineEdit().setText("")
        self.ui.txtRemark.setPlainText("")
        self.ui.timeTimeWorked.setDateTime(ZEROtime)
        self.ui.labelRoundTime.setText("00:00")
        self._smartQueryChanged()
        #fino a quando non sarà attiva la gestione delle modifiche...
        [i.setEnabled(False) for i in (self.ui.txtRemark,
                                       self.ui.timeTimeWorked,
                                       self.ui.btnSave)]
        self.ui.setWindowTitle("Time Registration - %s" % self.auth[1])

    def _connectSlots(self):
        self.connect(self.ui.comboSmartQuery,
                     SIGNAL("editTextChanged(QString)"),
                     self._smartQueryChanged)
        self.connect(self.ui.btnSave,
                     SIGNAL("clicked()"),
                     self.timereg)
        self.connect(self.rt,
                     SIGNAL("searchStarted()"),
                     self._searchStarted)
        self.connect(self.rt,
                     SIGNAL("searchDone(PyObject *)"),
                     self._projectsChanged)
        self.connect(self.rt,
                     SIGNAL("timeregStarted()"),
                     self._timeregStarted)
        self.connect(self.rt,
                     SIGNAL("timeregDone()"),
                     self._timeregDone)
        self.connect(self.rt,
                     SIGNAL("timeregError()"),
                     self._timeregError)
        self.connect(self.rt,
                     SIGNAL("processError(int)"),
                     self._processError)

    def _smartQueryChanged(self):
        smartquery = unicode(r"%"+self.ui.comboSmartQuery.lineEdit().text())
        self.rt.search(smartquery)
                     
    def _projectsChanged(self, projects):
        debug("_projectsChanged %s" % len(projects))
        self.projects = projects

        # ---- Update comboboxes ----
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
        # ^^^^ Update comboboxes ^^^^

        if len(self.projects) == 1:
            p = self.projects[0]
            worked_time = self.ui.timeTimeWorked.dateTimeFromText(p.get("input_hours"))
            self.ui.timeTimeWorked.setDateTime(worked_time)
            self.ui.labelRoundTime.setText(p.get("hmtime"))
            self.ui.txtRemark.setPlainText(p.get("remark"))
            self.ui.labelTimeWorked.setEnabled(worked_time > ZEROtime)
            self.ui.labelRemark.setEnabled(p.get("remark") != "")
            self.ui.btnSave.setEnabled(worked_time > ZEROtime and p.get("remark") != "")
        else:
            self.ui.txtRemark.setPlainText("")
            self.ui.labelTimeWorked.setEnabled(False)
            self.ui.labelRemark.setEnabled(False)
            self.ui.btnSave.setEnabled(False)
 
    def _timeregStarted(self):
        debug("_timeregStarted")
    
    def _timeregDone(self):
        debug("_timeregDone")
        self._setupGui()

    def _timeregError(self):
        debug("_timeregError")

    def _searchStarted(self):
        debug("_searchStarted")

    def _processError(self, int):
        debug("_processError %s" % int)
        self.err.showMessage("Errore nell'avviare il processo interfaccia con Archievo.")
     
    def timereg(self):
        p = self.projects[0]
        params = dict([(k, p.get(k)) for k in "projectid phaseid activityid hmtime remark".split()])
        params["activitydate"] = self.ui.dateTimeregDate.date().toString("yyyyMMdd")
        debug(str(params))
        self.rt.timereg(**params)
        self.ui.setWindowTitle("Time Registration - saving...")

if __name__ == "__main__":
    app = TimeregApplication(sys.argv)
    #2 m a 2:34 prova prova èàò
