#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import sys, logging
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

log = logging.getLogger("pyuac.gui")

def debug(msg):
    if __debug__:
        print __name__, msg
        log.debug("%s.%s" % (__name__, msg))

class TimeregWindow(QMainWindow):
    def __init__(self, parent):
        QMainWindow.__init__(self, parent)
        self.baseproject = None
        self.ui = uic.loadUi("pyuac_edit.ui", self)
        self.remote = parent.remote
        self.err = QErrorMessage(self)
        
        self.projects = None
        self.allProjects = None
        self._connectSlots()
        self._setupGui()

    def _setupGui(self):
        self.ui.dateTimeregDate.setDate(QDate.currentDate())
        self.ui.comboTimeWorked.clear()
        for hour in range(24):
            for quarter in range(4):
                if hour + quarter > 0:
                    htext = "%02d:%02d" % (hour, 15*quarter)
                    self.ui.comboTimeWorked.addItem(htext)
        self.ui.labelExactTime.setText("00:00")
        #fino a quando non sarà attiva la gestione delle modifiche...
        [i.setEnabled(False) for i in (self.ui.txtRemark,
                                       self.ui.btnSave)]
        self.ui.setWindowTitle("Time Registration - %s" % self.remote.auth[1])
        self.ui.txtRemark.setPlainText("")
        self.ui.comboSmartQuery.lineEdit().setText("")
        self.ui.comboSmartQuery.lineEdit().setFocus()
        self._smartQueryChanged("")
 
    def _connectSlots(self):
        self.connect(self.ui.comboSmartQuery, SIGNAL("editTextChanged(QString)"),
                     self._smartQueryChanged)
        self.connect(self.ui.btnSave, SIGNAL("clicked()"),
                     self.timereg)
        self.connect(self.ui.comboProjectPhase, SIGNAL("activated(const QString&)"),
                     self._comboProjectPhaseActivated)
        self.connect(self.ui.comboActivity, SIGNAL("activated(const QString&)"),
                     self._comboActivityActivated)
        self.connect(self.ui.comboTimeWorked, SIGNAL("activated(const QString&)"),
                     self._comboTimeWorkedActivated)
        # Short-circuit Signals (from python to python)
        self.connect(self.remote, SIGNAL("queryStarted"),
                     self._searchStarted)
        self.connect(self.remote, SIGNAL("queryOK"),
                     self._projectsChanged)
        self.connect(self.remote, SIGNAL("timeregStarted"),
                     self._timeregStarted)
        self.connect(self.remote, SIGNAL("timeregOK"),
                     self._registrationDone)
        self.connect(self.remote, SIGNAL("timeregErr"),
                     self._timeregErr)
        self.connect(self.remote, SIGNAL("processError"),
                     self._processError)

    def _smartQueryChanged(self, smartquery):
        smartquery = "%"+unicode(smartquery)
        self.remote.query(smartquery=smartquery)
                      
    def _projectsChanged(self, projects):
        debug("_projectsChanged %s" % len(projects))
        #if len(self.allProjects) < len(projects):
        if not self.allProjects:
            #TODO: mettere in un init, qua ci entro solo alla prima query con %
            self.allProjects = projects #mi stacco dall'oggetto originale
        self.projects = projects

        # ---- Update comboboxes ----
        self.ui.comboProjectPhase.clear()
        self.ui.comboActivity.clear()
        projphases = set()
        activities = set()
        for p in self.allProjects:
            projphases.add("%(project_name)s / %(phase_name)s" % dict(p.items()))
            activities.add(p.get("activity_name"))
        self.ui.comboProjectPhase.addItems(list(projphases))
        self.ui.comboActivity.addItems(list(activities))
        # ^^^^ Update comboboxes ^^^^

        if len(self.projects) == 1:
            p = self.projects[0]
            
            projectphase = "%(project_name)s / %(phase_name)s" % dict(p.items())
            idx = self.ui.comboProjectPhase.findText(projectphase)
            self.ui.comboProjectPhase.setCurrentIndex(idx)

            idx = self.ui.comboActivity.findText(p.get("activity_name"))
            self.ui.comboActivity.setCurrentIndex(idx)
            
            idx = self.ui.comboTimeWorked.findText(p.get("hmtime"))
            self.ui.comboTimeWorked.setCurrentIndex(idx)

            self.ui.labelExactTime.setText(p.get("input_hours") or "00:00")
            self.ui.txtRemark.setPlainText(p.get("remark"))
            self.ui.labelTimeWorked.setEnabled(p.get("hmtime") != "00:00")
            self.ui.labelRemark.setEnabled(p.get("remark") != "")
            self.ui.btnSave.setEnabled(p.get("hmtime") != "00:00" and p.get("remark") != "")
            self.ui.labelProjectPhase.setEnabled(True)
            self.ui.labelActivity.setEnabled(True)
        else:
            self.ui.txtRemark.setPlainText("")
            self.ui.labelProjectPhase.setEnabled(False)
            self.ui.labelActivity.setEnabled(False)
            self.ui.labelTimeWorked.setEnabled(False)
            self.ui.labelRemark.setEnabled(False)
            self.ui.btnSave.setEnabled(False)

    def _timeregStarted(self):
        debug("_timeregStarted")
    
    def _registrationDone(self, eresp):
        debug("_registrationDone")
        self._setupGui()
        self.emit(SIGNAL("registrationDone"), eresp)
        self.baseproject = None
        self.ui.close()

    def _timeregErr(self):
        debug("_timeregError")

    def _searchStarted(self):
        debug("_searchStarted")
        self.ui.btnSave.setEnabled(False)

    def _processError(self, int):
        debug("_processError %s" % int)
        self.err.showMessage(self.tr("Errore nell'avviare il processo interfaccia con Achievo."))
     
    def _comboProjectPhaseActivated(self, combotext):
        #TODO: fattorizzare in qualche modo questi 3
        origtext = unicode(self.ui.comboSmartQuery.lineEdit().text())
        origlist = origtext.split(" ", 4)
        while len(origlist) < 5:
            origlist.append("")
        newproj, newpha = unicode(combotext).split("/")
        origlist[0] = newproj.strip()
        origlist[1] = newpha.strip()
        self.ui.comboSmartQuery.setEditText(" ".join(origlist).strip()+" ")
    
    def _comboActivityActivated(self, combotext):
        origtext = unicode(self.ui.comboSmartQuery.lineEdit().text())
        origlist = origtext.split(" ", 4)
        while len(origlist) < 5:
            origlist.append("")
        newact = unicode(combotext)
        origlist[2] = newact.strip()
        self.ui.comboSmartQuery.setEditText(" ".join(origlist).strip()+" ")

    def _comboTimeWorkedActivated(self, combotext):
        origtext = unicode(self.ui.comboSmartQuery.lineEdit().text())
        origlist = origtext.split(" ", 4)
        while len(origlist) < 5:
            origlist.append("")
        newact = unicode(combotext)
        origlist[3] = newact.strip()
        self.ui.comboSmartQuery.setEditText(" ".join(origlist).strip()+" ")        

    def timereg(self):
        p = self.projects[0]
        params = dict([(k, p.get(k)) for k in "projectid phaseid activityid hmtime remark".split()])
        params["activitydate"] = str(self.ui.dateTimeregDate.date().toString("yyyyMMdd"))
        if self.baseproject != None:
            params["id"] = self.baseproject.get("id")
            self.baseproject.set("activitydate", params["activitydate"])
        debug(str(params))
        self.remote.timereg(**params)
        self.ui.setWindowTitle(self.tr("Time Registration - saving..."))
    
    def edit(self, project):
        self.baseproject = project
        self.ui.dateTimeregDate.setDate(QDate.fromString(project.get("activitydate"), "yyyy-MM-dd"))
        smartquery = "%(project_name)s %(phase_name)s %(activity_name)s %(hmtime)s %(remark)s" % dict(project.items())
        self.ui.comboSmartQuery.setEditText(smartquery)
        

if __name__ == "__main__":
    app = TimeregApplication(sys.argv)
    #2 m a 2:34 prova prova èàò
