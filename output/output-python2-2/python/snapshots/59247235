#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import sys, time, Queue
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
        self.rt = RemoteTimereg(self, self.auth)
 
        self.projects = []
        self._connectSlots()
        self._setupGui()

    def _setupGui(self):
        self.ui.dateTimeregDate.setDate(QDate.currentDate())
        self.ui.comboSmartQuery.lineEdit().setText("")
        self._slotSmartQueryChanged()
        #fino a quando non sarà attiva la gestione delle modifiche...
        [i.setEnabled(False) for i in (self.ui.txtRemark,
                                       self.ui.timeTimeWorked,
                                       self.ui.btnSave)]

    def _connectSlots(self):
        self.connect(self.ui.comboSmartQuery,
                     SIGNAL("editTextChanged(QString)"),
                     self._slotSmartQueryChanged)
        self.connect(self.rt,
                     SIGNAL("ready()"),
                     self._smartUpdateGui)
        self.connect(self.ui.btnSave,
                     SIGNAL("clicked()"),
                     self.timereg)
        self.connect(self.rt,
                     SIGNAL("timeregging()"),
                     self._enableGui)

    def _enableGui(self):
        self.ui.setEnabled(not self.ui.isEnabled())
        if __debug__:
            qDebug("_enableGui")

    def _slotSmartQueryChanged(self):
#        if __debug__:
#            qDebug("_slotSmartQueryChanged, %s" % self.rt.state())
        smartquery = self.ui.comboSmartQuery.lineEdit().text()
        self.rt.search(smartquery)
                     
    def _smartUpdateGui(self):
        if __debug__:
            qDebug("_smartUpdateGui")
        msg = self.rt.read()

        if msg == "":
            if __debug__:
                qDebug("No Output!")
            return

        if not self.ui.isEnabled():
            self._enableGui()
            self._setupGui()

        self.ui.setWindowTitle("Time Registration - %s" % self.auth[1])

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
            zero = self.ui.timeTimeWorked.dateTimeFromText("00:00")
            self.ui.timeTimeWorked.setDateTime(d)
            self.ui.labelRoundTime.setText(p.get("hmtime"))
            self.ui.txtRemark.setPlainText(p.get("remark"))
            self.ui.labelTimeWorked.setEnabled(d > zero)
            self.ui.labelRemark.setEnabled(p.get("remark") != "")
            self.ui.btnSave.setEnabled(d > zero and p.get("remark") != "")
        else:
            self.ui.txtRemark.setPlainText("")
            self.ui.labelTimeWorked.setEnabled(False)
            self.ui.labelRemark.setEnabled(False)
            self.ui.btnSave.setEnabled(False)
 
    def timereg(self):
        p = self.projects[0]
        params = dict([(k, p.get(k)) for k in "projectid phaseid activityid hmtime remark".split()])
        params["activitydate"] = self.ui.dateTimeregDate.date().toString("yyyyMMdd")
        self.rt.timereg(**params)
        self.ui.setWindowTitle("Time Registration - saving...")


class RemoteTimereg(QObject):
    """
    Classe per la gesitione asincrona della libreria RemoteAchievo
    """
    def __init__(self, parent, auth):
        QObject.__init__(self, parent)
        self.process = QProcess(self)
        self.timer = QTimer(self)
        self.auth = auth
        self.queue = ""
        self._noops = 0
        self._oops = 0
        self.connect(self.timer, SIGNAL("timeout()"), self._sync)
        self.connect(self.process, SIGNAL("finished(int)"), self._ready)

    def read(self):
        return str(self.process.readAllStandardOutput()).decode("UTF-8")

    def search(self, msg):
        if __debug__:
            qDebug("Query")
        self.queue = "search?smartquery=%" + str(msg).encode("UTF-8") + "\n"
        if not self.timer.isActive():
            self._sync()
            self.timer.start(100)

    def _sync(self, timerEvent=None):
        if __debug__:
            qDebug("sync")
        if self.process.state() == self.process.NotRunning:
            if self.queue != "":
                self.process.start("./pyuac_cli.py", self.auth+["--"])
                self.process.write(self.queue)
                self.queue = ""
            else:
                #dopo N loop senza modifiche, ferma il timer
                self._noops += 1
                if self._noops > 5:
                    self.timer.stop()
                    self._noops = 0
        else:
            #dopo NN loop di attesa remina il processo
            self._oops += 1
            if self._oops > 500:
                self.process.terminate()
                self._oops = 0
 
    def _ready(self):
        self.emit(SIGNAL("ready()"))

    def timereg(self, **kwargs):
        if __debug__:
            qDebug("Saving")
        self.queue = ""
        qstring = libRemoteTimereg.urllib.urlencode(kwargs).encode("UTF-8")
        if __debug__:
            qDebug(qstring)
        self.process.start("./pyuac_cli.py", self.auth+["--"])
        self.process.write("timereg?" + qstring + "\n")
        self.emit(SIGNAL("timeregging()"))

if __name__ == "__main__":
    app = TimeregApplication(sys.argv)
    #2 m a 2:34 prova prova
