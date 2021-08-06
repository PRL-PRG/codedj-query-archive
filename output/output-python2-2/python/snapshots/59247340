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

EVENT_WORKDONE = QEvent.Type(QEvent.User + 1)

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
        self.ui = uic.loadUi("pyuac_edit.ui", self)
        self.rt = libRemoteTimereg.RemoteTimereg(achievouri, user, password)
        self.worker = Worker(self.rt.search, self)
        self.worker.start()

        self.projects = []
        
        self._connectSlots()
        self._setupGui()

    def _setupGui(self):
        self.ui.dateTimeregDate.setDate(QDate.currentDate())
        self._slotSmartQueryChanged()

    def _connectSlots(self):
        self.connect(self.ui.comboSmartQuery,
                     SIGNAL("editTextChanged(QString)"),
                     self._slotSmartQueryChanged)

    def _slotSmartQueryChanged(self):
        smartquery = unicode(self.ui.comboSmartQuery.lineEdit().text())
        self.worker.process(smartquery)
                    
    def _smartUpdateGui(self, projects):
        self.projects = projects
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
            d = self.ui.timeTimeWorked.dateTimeFromText(p.get("hours"))
            zero = self.ui.timeTimeWorked.dateTimeFromText("0:00")
            self.ui.timeTimeWorked.setDateTime(d)
            self.ui.labelTimeWorked.setEnabled(d > zero)
            self.ui.labelComment.setEnabled(p.get("comment") != "")
            self.ui.txtComment.setPlainText(p.get("comment"))

    def customEvent(self, event):
        if event.type() == EVENT_WORKDONE:
            self._smartUpdateGui(event.msg)

class WorkDoneEvent(QEvent):
    def __init__(self, msg):
        QEvent.__init__(self, EVENT_WORKDONE)
        self.msg = msg

class Worker(QThread):
    def __init__(self, action, receiver):
        QThread.__init__(self)
        self.action = action
        self.receiver = receiver
        self.stopped = False
        self.towork = False
        self._args = []
        self._kwargs = {}
    def process(self, *args, **kwargs):
        self.towork = True
        self._args = args
        self._kwargs = kwargs
    def work(self):
        res = self.action(*self._args, **self._kwargs)
        event = WorkDoneEvent(res)
        QCoreApplication.postEvent(self.receiver, event)
    def stop(self):
        self.stopped = True
    def run(self):
        while not self.stopped:
            if self.towork:
                self.work()
                self.towork = False

if __name__ == "__main__":
    app = TimeregApplication(sys.argv)
