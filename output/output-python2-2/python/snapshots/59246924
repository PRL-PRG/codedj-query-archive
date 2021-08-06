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

import libRemoteTimereg
from QRemoteTimereg import RemoteTimereg

log = logging.getLogger("pyuac.gui")

def debug(msg):
    if __debug__:
        print __name__, msg
        log.debug("%s.%s" % (__name__, msg))

class TimeregWindow(QMainWindow):
    def __init__(self, parent, auth):
        QMainWindow.__init__(self, parent)
        self._baseproject = None
        self.ui = uic.loadUi("pyuac_edit.ui", self)
        self.remote = RemoteTimereg(self, auth)
        self.err = QErrorMessage(self)
        self._last_modified_combo = None
        self._response_projects = []
        self.projects = set()
        self.phases = {None: set()}
        self.activities = {(None,None): set()}
        self._connectSlots()
        self._setupGui()

    def _setupGui(self):
        self.ui.dateTimeregDate.setDate(QDate.currentDate())
        self.ui.comboTimeWorked.clear()
        max_hours = 8
        for hour in range(max_hours):
            for quarter in range(4):
                if hour + quarter > 0:
                    htext = "%02d:%02d" % (hour, 15*quarter)
                    self.ui.comboTimeWorked.addItem(htext)
            if hour == max_hours - 1:
                htext = "%02d:%02d" % (max_hours, 0)
                self.ui.comboTimeWorked.addItem(htext)
        self.ui.labelExactTime.setText("00:00")
        self.ui.setWindowTitle("Time Registration - %s" % self.remote.auth[1])
        self.ui.btnDelete.setText(self.tr("Reset"))
        self.ui.txtRemark.setPlainText("")
        self.ui.txtRemark.setReadOnly(True)
        self.ui.comboSmartQuery.lineEdit().setText("")
        self.ui.comboSmartQuery.setFocus()
        self._smartQueryChanged("")
        self._updateComboBoxes()

    def _disableAll(self):
        self.ui.labelProject.setEnabled(False)
        self.ui.labelPhase.setEnabled(False)
        self.ui.labelActivity.setEnabled(False)
        self.ui.labelTimeWorked.setEnabled(False)
        self.ui.labelRemark.setEnabled(False)
        self.ui.btnSave.setEnabled(False)
        self.ui.btnDelete.setEnabled(False)

    def _connectSlots(self):
        self.connect(self.ui.comboSmartQuery, SIGNAL("editTextChanged(QString)"),
                     self._smartQueryChanged)
        self.connect(self.ui.btnSave, SIGNAL("clicked()"),
                     self.timereg)
        self.connect(self.ui.btnDelete, SIGNAL("clicked()"),
                     self.delete)
        self.connect(self.ui.btnCancel, SIGNAL("clicked()"),
                     self._close)
        self.connect(self.ui.comboProject, SIGNAL("activated(const QString&)"),
                     self._comboProjectActivated)
        self.connect(self.ui.comboPhase, SIGNAL("activated(const QString&)"),
                     self._comboPhaseActivated)
        self.connect(self.ui.comboActivity, SIGNAL("activated(const QString&)"),
                     self._comboActivityActivated)
        self.connect(self.ui.comboTimeWorked, SIGNAL("activated(const QString&)"),
                     self._comboTimeWorkedActivated)
        self.connect(self.remote, SIGNAL("queryStarted"),
                     self._searchStarted)
        self.connect(self.remote, SIGNAL("queryOK"),
                     self._projectsChanged)
        self.connect(self.remote, SIGNAL("timeregStarted"),
                     self._timeregStarted)
        self.connect(self.remote, SIGNAL("timeregOK"),
                     self._registrationDone)
        self.connect(self.remote, SIGNAL("deleteOK"),
                     self._registrationDone)
        self.connect(self.remote, SIGNAL("timeregErr"),
                     self._timeregErr)
        self.connect(self.remote, SIGNAL("processError"),
                     self._processError)

    def _smartQueryChanged(self, smartquery):
        """
        Avvia la query al servizio remoto di achievo tramite la cli
        """
        if smartquery != "" or self._response_projects == []:
            smartquery = "%"+unicode(smartquery)
            self.remote.query(smartquery=smartquery)

    def _updateComboBoxes(self, combo=None):
        """
        Aggiorna i combobox in modo che contengano
        l'unione dei progetti visti durante la sessione
        """
        for p in self._response_projects:
            self.projects.add(p.get("prj"))
            self.phases.setdefault(p.get("prj"), set())
            self.phases[p.get("prj")].add(p.get("pha"))
            self.phases[None].add(p.get("pha"))
            self.activities.setdefault((p.get("prj"), p.get("pha")), set())
            self.activities[(p.get("prj"), p.get("pha"))].add(p.get("act"))
            self.activities[(None,None)].add(p.get("act"))
                    
        if combo == None and len(self._response_projects) >= 1:
            project = p.get("prj")
            phase = p.get("pha")
            activity = p.get("act")
            newtime = p.get("hmtime")
            self.ui.comboProject.clear()
            self.ui.comboProject.addItems(sorted(list(self.projects)))
            self.ui.comboPhase.clear()
            self.ui.comboPhase.addItems(sorted(list(self.phases[project])))
            self.ui.comboActivity.clear()
            self.ui.comboActivity.addItems(sorted(list(self.activities[(project,phase)])))
        else:
            project = unicode(self.ui.comboProject.currentText()) or None
            phase = unicode(self.ui.comboPhase.currentText()) or None
            activity = unicode(self.ui.comboActivity.currentText()) or None
            newtime = unicode(self.ui.comboTimeWorked.currentText())
            
            if combo == "Project" or len(self._response_projects) == 1:
                self.ui.comboPhase.clear()
                self.ui.comboPhase.addItems(sorted(list(self.phases[project])))
            elif combo == "Phase" or len(self._response_projects) == 1:
                self.ui.comboActivity.clear()
                self.ui.comboActivity.addItems(sorted(list(self.activities[(project,phase)])))
            
            smartquery_dict = self._parseSmartQuery()
            smartquery_dict["in_prj"] = project or smartquery_dict["in_prj"]
            smartquery_dict["in_pha"] = phase or smartquery_dict["in_pha"]
            smartquery_dict["in_act"] = activity or smartquery_dict["in_act"]
            smartquery_dict["in_hmtime"] = newtime or smartquery_dict["in_hmtime"]
            self._setSmartQuery(smartquery_dict)

    def _projectsChanged(self, projects):
        """
        Aggiorna l'interfaccia in funzione del numero di progetti
        restituiti dalla ricerca in Achievo:
        n == 1) è possibile procedere con la registrazione
        n == 0) reimposto il campo della smartquery non modificato
                sulla ricerca generica %
                Se scelgo una coppia Phase - Activity che non esiste, mantengo
                il campo associato all'ultimo combo modificato dall'utente
        n >= 2) Non è possibile registrare le ore
        """
        debug("_projectsChanged %s" % len(projects))

        self._response_projects = projects
        self._updateComboBoxes()

        #TODO: spostare projects[0] in _baseproject, in modo che sia il
        #      modello di ciò che viene visualizzato

        if len(self._response_projects) == 1:
            p = self._response_projects[0]

            idx = self.ui.comboProject.findText(p.get("prj"))
            self.ui.comboProject.setCurrentIndex(idx)

            idx = self.ui.comboPhase.findText(p.get("pha"))
            self.ui.comboPhase.setCurrentIndex(idx)

            idx = self.ui.comboActivity.findText(p.get("act"))
            self.ui.comboActivity.setCurrentIndex(idx)

            idx = self.ui.comboTimeWorked.findText(p.get("hmtime") or "00:00")
            self.ui.comboTimeWorked.setCurrentIndex(idx)

            self.ui.labelExactTime.setText(p.get("in_hmtime"))
            self.ui.labelTimeWorked.setEnabled(p.get("hmtime") != "00:00")

            self.ui.txtRemark.setPlainText((p.text or "").strip())

            self.ui.labelRemark.setEnabled(p.text != "")
            self.ui.btnSave.setEnabled(p.get("hmtime") != "00:00" and p.text != "")
            self.ui.labelProject.setEnabled(True)
            self.ui.labelPhase.setEnabled(True)
            self.ui.labelActivity.setEnabled(True)
            self.ui.btnDelete.setEnabled(True)
        else:
            self._disableAll()

    def _timeregStarted(self):
        debug("_timeregStarted")

    def _registrationDone(self, eresp):
        debug("_registrationDone")
        eresp[0].set("activitydate", self._response_projects[0].get("activitydate"))
        self.emit(SIGNAL("registrationDone"), eresp)
        self._baseproject = None
        self._close()

    def _close(self):
        self._setupGui()
        self.ui.close()

    def _timeregErr(self):
        debug("_timeregError")

    def _searchStarted(self):
        debug("_searchStarted")
        self.ui.btnSave.setEnabled(False)

    def _processError(self, qperror, exitcode):
        debug("_processError %s, %s" % (qperror, exitcode))
        if self.ui.isVisible():
            self.err.showMessage(self.tr("Errore nel processo interfaccia con Achievo:\n") +
                                 "%s, %s" % (qperror, exitcode))

    def _parseSmartQuery(self):
        origtext = unicode(self.ui.comboSmartQuery.lineEdit().text())
        return libRemoteTimereg.parseSmartQuery(origtext)

    def _setSmartQuery(self, smartquery_dict):
        keys = "in_prj in_pha in_act in_hmtime in_remark".split()
        print "_setSmartQuery", smartquery_dict
        qstring = " ".join([smartquery_dict.get(k, "") or "" for k in keys]).strip()
        self.ui.comboSmartQuery.setEditText(qstring)
        self.ui.comboSmartQuery.setFocus()
    
    
    def _comboProjectActivated(self, combotext):
        self._updateComboBoxes("Project")

    def _comboPhaseActivated(self, combotext):
        self._updateComboBoxes("Phase")

    def _comboActivityActivated(self, combotext):
        self._updateComboBoxes("Activity")

    def _comboTimeWorkedActivated(self, combotext):
        self._updateComboBoxes("TimeWorked")

    def timereg(self):
        self.ui.btnSave.setEnabled(False)
        p = self._response_projects[0]
        activitydate = str(self.ui.dateTimeregDate.date().toString("yyyy-MM-dd"))
        p.set("activitydate", activitydate)
        params = dict([(k, p.get(k)) for k in "projectid phaseid activityid hmtime activitydate".split()])
        params["remark"] = p.text
        if self._baseproject.get("id") != None:
            params["id"] = self._baseproject.get("id")
        debug(str(params))
        self.remote.timereg(**params)
        self.notify(self.tr("Saving..."))

    def setupEdit(self, project):
        self._baseproject = project
        if self._baseproject.get("id") != None:
            self.ui.btnDelete.setText(self.tr("Delete"))
        self.ui.dateTimeregDate.setDate(QDate.fromString(project.get("activitydate"), "yyyy-MM-dd"))
        smartquery_dict = dict(project.items() + [("remark", project.text)])
        #TODO: i nomi dei campi cambiano troppe volte
        #      questo hack duplica i valori in attesa di rinominate uniformemente
        #      i campi tra php e pyuac e tra timereg, query e timereport
        for k in dict(smartquery_dict):
            smartquery_dict["in_"+k] = smartquery_dict[k]
            del smartquery_dict[k]
        self._setSmartQuery(smartquery_dict)

    def delete(self):
        if self._baseproject.get("id") != None:
             self.remote.delete(id=self._baseproject.get("id"))
             self.notify(self.tr("Deleting..."))
             self._baseproject = None
        else:
             self._setupGui()
    
    def notify(self, msg, timeout=0):
        self.ui.statusBar.showMessage(msg, timeout)


if __name__ == "__main__":
    app = TimeregApplication(sys.argv)
    #2 m a 2:34 prova prova èàò
