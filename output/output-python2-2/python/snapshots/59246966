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
    def __init__(self, parent):
        QMainWindow.__init__(self, parent)
        self._baseproject = None
        self.ui = uic.loadUi("pyuac_edit.ui", self)
        self.remote = parent.remote
        self.err = QErrorMessage(self)
        self._last_modified_combo = None
        self.projects = []
        self.projphases = set()
        self.activities = {None: set()}
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
        self.ui.labelProjectPhase.setEnabled(False)
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
        self.connect(self.ui.comboProjectPhase, SIGNAL("activated(const QString&)"),
                     self._comboProjectPhaseActivated)
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
        smartquery = "%"+unicode(smartquery)
        self.remote.query(smartquery=smartquery)

    def _updateComboBoxes(self, projphase=None):
        """
        Aggiorna i combobox in modo che contengano
        l'unione dei progetti visti durante la sessione
        """
        self.ui.comboProjectPhase.clear()
        self.ui.comboActivity.clear()
        self.projphases = set()
        for p in self.projects:
            projphase = "%(project_name)s / %(phase_name)s" % dict(p.items())
            self.projphases.add(projphase)
            self.activities.setdefault(projphase, set())
            self.activities[projphase].add(p.get("activity_name"))
            self.activities[None].add(p.get("activity_name"))
        self.ui.comboProjectPhase.addItems(sorted(list(self.projphases)))
        self.ui.comboActivity.addItems(sorted(list(self.activities[projphase])))

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

        self.projects = projects
        self._updateComboBoxes()

        #TODO: spostare projects[0] in _baseproject, in modo che sia il
        #      modello di ciò che viene visualizzato

        if len(self.projects) == 1:
            p = self.projects[0]

            projectphase = "%(project_name)s / %(phase_name)s" % dict(p.items())
            idx = self.ui.comboProjectPhase.findText(projectphase)
            self.ui.comboProjectPhase.setCurrentIndex(idx)

            idx = self.ui.comboActivity.findText(p.get("activity_name") or "")
            self.ui.comboActivity.setCurrentIndex(idx)

            idx = self.ui.comboTimeWorked.findText(p.get("hmtime") or "0:00")
            self.ui.comboTimeWorked.setCurrentIndex(idx)

            self.ui.labelExactTime.setText(p.get("input_hmtime"))# or "00:00")
            self.ui.labelTimeWorked.setEnabled(p.get("hmtime") != "0:00")

            self.ui.txtRemark.setPlainText((p.text or "").strip())

            self.ui.labelRemark.setEnabled(p.text != "")
            self.ui.btnSave.setEnabled(p.get("hmtime") != "0:00" and p.text != "")
            self.ui.labelProjectPhase.setEnabled(True)
            self.ui.labelActivity.setEnabled(True)
            self.ui.btnDelete.setEnabled(True)
        elif len(self.projects) == 0:
            if self._last_modified_combo == "projectphase":
                self._comboActivityActivated("%")
            elif self._last_modified_combo == "activity":
                self._comboProjectPhaseActivated("%/%")
        else:
            self._disableAll()

    def _timeregStarted(self):
        debug("_timeregStarted")

    def _registrationDone(self, eresp):
        debug("_registrationDone")
        eresp[0].set("activitydate", self.projects[0].get("activitydate"))
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

    def _processError(self, int):
        debug("_processError %s" % int)
        self.err.showMessage(self.tr("Errore nell'avviare il processo interfaccia con Achievo."))

    def _parseSmartQuery(self):
        origtext = unicode(self.ui.comboSmartQuery.lineEdit().text())
        #origtext = unicode(self.ui.txtRemark.toPlainText())
        return libRemoteTimereg.parseSmartQuery(origtext)

    def _setSmartQuery(self, smartquery_dict):
        keys = "input_project input_phase input_activity input_hmtime input_remark".split()
        print "_setSmartQuery", smartquery_dict
        qstring = " ".join([smartquery_dict.get(k, "") or "" for k in keys]).strip()
        self.ui.comboSmartQuery.setEditText(qstring)
        #self.ui.txtRemark.setPlainText(qstring)

    def _comboProjectPhaseActivated(self, combotext):
        #TODO: fattorizzare meglio questi 3 metodi
        smartquery_dict = self._parseSmartQuery()
        newproj, newpha = unicode(combotext).split("/")
        smartquery_dict["input_project"] = newproj.strip()
        smartquery_dict["input_phase"] = newpha.strip()
        self._last_modified_combo = "projectphase"
        self._updateComboBoxes(unicode(combotext))
        self._setSmartQuery(smartquery_dict)

    def _comboActivityActivated(self, combotext):
        smartquery_dict = self._parseSmartQuery()
        newact = unicode(combotext)
        smartquery_dict["input_activity"] = newact.strip()
        self._last_modified_combo = "activity"
        self._setSmartQuery(smartquery_dict)

    def _comboTimeWorkedActivated(self, combotext):
        smartquery_dict = self._parseSmartQuery()
        newtime = unicode(combotext)
        smartquery_dict["input_hmtime"] = newtime.strip()
        self._setSmartQuery(smartquery_dict)

    def timereg(self):
        p = self.projects[0]
        activitydate = str(self.ui.dateTimeregDate.date().toString("yyyy-MM-dd"))
        p.set("activitydate", activitydate)
        params = dict([(k, p.get(k)) for k in "projectid phaseid activityid hmtime activitydate".split()])
        params["remark"] = p.text
        if self._baseproject.get("id") != None:
            params["id"] = self._baseproject.get("id")
        debug(str(params))
        self.remote.timereg(**params)
        self.ui.setWindowTitle(self.tr("Time Registration - saving..."))

    def setupEdit(self, project):
        self._baseproject = project
        self.ui.btnDelete.setText(self.tr("Delete"))
        self.ui.dateTimeregDate.setDate(QDate.fromString(project.get("activitydate"), "yyyy-MM-dd"))
        smartquery_dict = dict(project.items() + [("remark", project.text)])
        #TODO: i nomi dei campi cambiano troppe volte
        #      questo hack duplica i valori in attesa di rinominate uniformemente
        #      i campi tra php e pyuac e tra timereg, query e timereport
        for k in dict(smartquery_dict):
            smartquery_dict["input_"+k.replace("_name","")] = smartquery_dict[k]
        self._setSmartQuery(smartquery_dict)

    def delete(self):
        if self._baseproject.get("id") != None:
             self.remote.delete(id=self._baseproject.get("id"))
             self.ui.setWindowTitle(self.tr("Time Registration - deleting..."))
             self._baseproject = None
        else:
             self._setupGui()


if __name__ == "__main__":
    app = TimeregApplication(sys.argv)
    #2 m a 2:34 prova prova èàò
