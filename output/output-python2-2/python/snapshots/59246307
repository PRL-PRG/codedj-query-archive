#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import sys, copy

from pyuac_utils import *
from QRemoteTimereg import *

class ASettings(QSettings):
    """
    Aggiunge una interfaccia più semplice per le array
    Converte in e restituisce valori QVariant
    """
    def getArray(self, prefix, keys):
        res = []
        size = self.beginReadArray(prefix)
        for i in range(size):
            self.setArrayIndex(i)
            item = {}
            for k in keys:
                item[k] = self.value(k)
            res.append(item)
        self.endArray()
        return res

    def setArray(self, prefix, data):
        self.beginWriteArray(prefix)
        for i, item in enumerate(data):
            self.setArrayIndex(i)
            for k, v in item.items():
                self.setValue(k, QVariant(v))
        self.endArray()


class TimeregWindow(QMainWindow):

    def __init__(self, parent, auth):
        debug("TimeregWindow.__init__")
        QMainWindow.__init__(self, parent)
        self._baseproject = AchievoProject()
        self.ui = uic.loadUi("pyuac_edit.ui", self)
        self.remote = QRemoteTimereg(self, auth)
        self.err = QErrorMessage(self)
        self.settings = ASettings("Develer", "PyUAC")
        self.completer = QCompleter([], self)
        self.completer.setCaseSensitivity(Qt.CaseInsensitive)
        self.completer.setCompletionMode(QCompleter.UnfilteredPopupCompletion)
        self.ui.editSmartQuery.setCompleter(self.completer)
        self._completer_list = []
        self._response_projects = []
        self._ppa = {}
        self._connectSlots()
        self._setupGui()
        self._smartQueryEdited("")

    def _setupGui(self):
        debug("TimeregWindow._setupGui")
        self.ui.dateTimeregDate.setDate(QDate.currentDate())
        self.ui.comboTimeWorked.clear()
        for htext in timerange(8, 15):
            self.ui.comboTimeWorked.addItem(htext)
        self.ui.labelExactTime.setText("00:00")
        self.ui.setWindowTitle(self.tr("Time Registration") + " - %s" % self.remote.auth[1])
        self.ui.btnDelete.setText(self.tr("Reset"))
        self.ui.txtRemark.setPlainText("")
        self.ui.txtRemark.setReadOnly(True)
        self.ui.editSmartQuery.setText("")
        self.ui.editSmartQuery.setFocus()
        self.ui.comboPPAlru.clear()
        self.ui.comboPPAlru.addItem("")
        for row in self.settings.getArray("lru", ["ppa-%s" % self.remote.auth[1]]):
            self.ui.comboPPAlru.addItem(row["ppa-%s" % self.remote.auth[1]].toString())

    def _connectSlots(self):
        self.connect(self.ui.editSmartQuery, SIGNAL("textEdited(QString)"),
                     self._smartQueryEdited)
        self.connect(self.ui.editSmartQuery, SIGNAL("returnPressed()"),
                     self.timereg)
        self.connect(self.ui.editSmartQuery.completer(), SIGNAL("activated(const QString&)"),
                     self._completerActivated)
        self.connect(self.ui.comboPPAlru, SIGNAL("activated(const QString&)"),
                     self._updateSmartQuery)
        self.connect(self.ui.btnSave, SIGNAL("clicked()"),
                     self.timereg)
        self.connect(self.ui.btnDelete, SIGNAL("clicked()"),
                     self.delete)
        self.connect(self.ui.btnCancel, SIGNAL("clicked()"),
                     self.cancel)
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

    def _completerActivated(self, smartquery):
        smartquery = unicode(smartquery).strip()
        self._setSmartQuery(smartquery + " ")
        self._smartQueryEdited(smartquery)

    def _updateSmartQuery(self, smartquery):
        debug(u"_updateSmartQuery '%s'" % smartquery)
        self._setSmartQuery(smartquery)
        self._smartQueryEdited(smartquery)

    def _setSmartQuery(self, smartquery):
        debug(u"_setSmartQuery '%s'" % smartquery)
        self.ui.editSmartQuery.setText(smartquery)
        self.ui.editSmartQuery.setFocus()

    def _smartQueryEdited(self, smartquery):
        """ <-- self.ui.editSmartQuery, SIGNAL("textEdited(QString)")
        (Emesso solo per modifiche "umane" e non da programma.)
        Avvia la query al servizio remoto di achievo tramite la cli
        """
        debug(u"_smartQueryEdited: '%s'" % smartquery)
        smartquery = unicode(smartquery).strip()
        self.remote.query(smartquery=smartquery)
        self.notify(self.tr("Searching..."))

    def _projectsChanged(self, projects):
        """ <-- self.remote, SIGNAL("queryOK")
        Aggiorna lo stato interno in funzione dei progetti
        restituiti dalla ricerca in Achievo:
        """
        debug("_projectsChanged %s" % len(projects))

        self._response_projects = projects

        if len(self._response_projects) != 0:
            debug("_baseproject.merge()")
            self._baseproject.merge(self._response_projects)
        #else:
        #    debug("_baseproject.reset()")
        #    self._baseproject.reset()
        debug("_projectsChanged: _baseproject %s" % self._baseproject)
        self._updateGui()
        self.notify(self.tr(""))

    def _updateGui(self):
        """
        Aggiorna lo stato visuale della gui in funzione dello stato interno
        """
        debug("_updateGui")
        self._updateComboBoxes()
        p = self._baseproject
        self._disableAll()

        idx = self.ui.comboProject.findText(p.get("prj") or "")
        self.ui.comboProject.setCurrentIndex(idx)
        self.ui.labelProject.setEnabled(p.get("prj") != None)

        idx = self.ui.comboPhase.findText(p.get("pha") or "")
        self.ui.comboPhase.setCurrentIndex(idx)
        self.ui.labelPhase.setEnabled(p.get("pha") != None)

        idx = self.ui.comboActivity.findText(p.get("act") or "")
        self.ui.comboActivity.setCurrentIndex(idx)
        self.ui.labelActivity.setEnabled(p.get("act") != None)

        idx = self.ui.comboTimeWorked.findText(p.get("hmtime") or "00:00")
        self.ui.comboTimeWorked.setCurrentIndex(idx)

        self.ui.labelExactTime.setText(p.get("in_hmtime") or "00:00")
        self.ui.labelTimeWorked.setEnabled(p.get("hmtime") != "00:00")

        self.ui.txtRemark.setPlainText((p.get("remark") or "").strip())

        self.ui.labelRemark.setEnabled(p.get("remark") != None)
        self.ui.btnSave.setEnabled(p.isComplete())

        #deselezione lru
        self.ui.comboPPAlru.setCurrentIndex(0)

        self.ui.btnDelete.setEnabled(True)
        self.ui.comboProject.setEnabled(True)
        self.ui.comboPhase.setEnabled(True)
        self.ui.comboActivity.setEnabled(True)
        self.ui.comboTimeWorked.setEnabled(True)

    def _disableAll(self):
        self.ui.labelProject.setEnabled(False)
        self.ui.labelPhase.setEnabled(False)
        self.ui.labelActivity.setEnabled(False)
        self.ui.labelTimeWorked.setEnabled(False)
        self.ui.labelRemark.setEnabled(False)
        self.ui.btnSave.setEnabled(False)
        self.ui.btnDelete.setEnabled(False)

    def _updateComboBoxes(self, combo=None, combotext=None):
        """ <-- _updateGui() e dai combo
        Aggiorna i combobox in modo che contengano
        l'unione dei valori visti durante la sessione
        """
        combotext = unicode(combotext)

        debug("_updateComboBoxes %s %s" % (combo, combotext))
        # Aggiorna la lista di progetti, fasi e attività
        # usata per riempire i combobox
        for p in self._response_projects:
            self._ppa.setdefault(p.get("prj"), {})
            self._ppa[p.get("prj")].setdefault(p.get("pha"), {})
            self._ppa[p.get("prj")][p.get("pha")].setdefault(p.get("act"), {})

        project = self._baseproject.get("prj")
        phase = self._baseproject.get("pha")
        activity = self._baseproject.get("act")
        hmtime = self._baseproject.get("hmtime") or ""
        remark = self._baseproject.get("remark") or ""

        if combo == "Project":
            self._baseproject.set("in_prj", combotext)
            self._baseproject.set("prj", combotext)
            self._baseproject.set("pha", None)
            self._baseproject.set("act", None)
            #self._baseproject.set("in_pha", "%")
            #self._baseproject.set("in_act", "%")
        elif combo == "Phase":
            self._baseproject.set("in_pha", combotext)
            self._baseproject.set("pha", combotext)
            self._baseproject.set("act", None)
            #self._baseproject.set("in_act", "%")
        elif combo == "Activity":
            self._baseproject.set("in_act", combotext)
            self._baseproject.set("act", combotext)
        elif combo == "TimeWorked":
            self._baseproject.set("in_hmtime", combotext)
            self._baseproject.set("hmtime", combotext)

        
        def endsWithSpace(msg):
            if msg:
                return msg[-1] == " "
            else:
                return False

        # se il progetto inserito identifica univocamente un nome e
        # la smartquery termina con spazio (questo permette di non
        # interferire brutalmente con le modifiche fatte dall'utente)
        if project != None and project != self._baseproject.get("in_prj"):
            smartquery = unicode(self.ui.editSmartQuery.text())
            if endsWithSpace(smartquery):
                self._baseproject.set("in_prj", project)
                self._setSmartQuery(self._baseproject.getSmartQuery())

        # se ho attivato un combo
        if combo != None:
            self._updateSmartQuery(self._baseproject.getSmartQuery())
            return

        self.ui.comboProject.clear()
        self.ui.comboPhase.clear()
        self.ui.comboActivity.clear()
        self.ui.comboProject.addItems(sorted(self._ppa.keys()))
        if project != None:
            self.ui.comboPhase.addItems(sorted(self._ppa[project].keys()))
            if phase != None:
                self.ui.comboActivity.addItems(sorted(self._ppa[project][phase].keys()))

        def _updateCompleter():
            # La stringa di completamento deve proporre il nome esteso del nodo
            # attivo e mantenere la stringa inserita (se univoca) per ciò che è
            # già stato eseguito <<<< da decidere
            if project == None:
                _completer = self._ppa.keys()
            else:
                if phase == None:
                    _base = self._baseproject.get("prj")+" "
                    _completer = [_base+pha for pha in self._ppa[project].keys()]
                else:
                    if activity == None or not self._baseproject.get("in_act"):
                        _base = self._baseproject.get("prj")+" "+self._baseproject.get("pha")+" "
                        _completer = [_base+act for act in self._ppa[project][phase].keys()]
                    else:
                        _completer = []

            # se ho già scritto hmtime e commento, li aggiungo al completer
            for c, v in enumerate(_completer):
                _completer[c] = " ".join([_completer[c], hmtime, remark]).strip()

            if _completer != self._completer_list:
                self.completer.setModel(QStringListModel(_completer, self.completer))
                self._completer_list = _completer

            if combo == None and self.ui.isVisible():
                #altrimenti compare anche a finestra invisibile...
                self.completer.complete()
                debug("self.completer %s" % _completer)

        _updateCompleter()

    def _timeregStarted(self):
        #debug("_timeregStarted")
        pass

    def _registrationDone(self, eresp):
        #debug("_registrationDone")
        lru = self.settings.getArray("lru", ["ppa-%s" % self.remote.auth[1]])
        new_ppa = self._baseproject.getPPA()+" "
        if new_ppa not in [row["ppa-%s" % self.remote.auth[1]].toString() for row in lru]:
            lru.insert(0, {"ppa-%s" % self.remote.auth[1]: new_ppa})
            if len(lru) > 2:
                lru.pop()
        self.settings.setArray("lru", lru)
        self.emit(SIGNAL("registrationDone"), self._baseproject)
        self._baseproject.reset()
        self.cancel()

    def cancel(self):
        self._setupGui()
        self.ui.close()

    def _timeregErr(self):
        #debug("_timeregError")
        pass

    def _searchStarted(self):
        debug("%s _searchStarted" % __name__)
        #self.ui.btnSave.setEnabled(False)
        #self.ui.comboProject.setEnabled(False)
        #self.ui.comboPhase.setEnabled(False)
        #self.ui.comboActivity.setEnabled(False)
        #self.ui.comboTimeWorked.setEnabled(False)

    def _processError(self, qperror, exitcode):
        debug("_processError %s, %s" % (qperror, exitcode))
        if self.ui.isVisible():
            self.err.showMessage(self.tr("Errore nel processo interfaccia con Achievo:\n") +
                                 "%s, %s" % (qperror, exitcode))

    def _comboProjectActivated(self, combotext):
        self._updateComboBoxes("Project", combotext)

    def _comboPhaseActivated(self, combotext):
        self._updateComboBoxes("Phase", combotext)

    def _comboActivityActivated(self, combotext):
        self._updateComboBoxes("Activity", combotext)

    def _comboTimeWorkedActivated(self, combotext):
        self._updateComboBoxes("TimeWorked", combotext)

    def timereg(self):
        if not self._baseproject.isComplete():
            self.notify(self.tr("Unable to save!"), 1000)
            return
        self.ui.btnSave.setEnabled(False)
        p = self._baseproject
        activitydate = str(self.ui.dateTimeregDate.date().toString("yyyy-MM-dd"))
        p.set("activitydate", activitydate)
        params = dict([(k, p.get(k)) for k in "projectid phaseid activityid hmtime activitydate".split()])
        params["remark"] = p.get("remark")
        if not self._baseproject.isNew():
            debug("-------------> Update")
            params["id"] = self._baseproject.get("id")
        else:
            debug("-------------> New")
        self.remote.timereg(**params)
        self.notify(self.tr("Saving..."))

    def setupEdit(self, project):
        self._baseproject = AchievoProject(project)
        debug("setupEdit %s" % self._baseproject)
        if not self._baseproject.isNew():
            self.ui.btnDelete.setText(self.tr("Delete"))
        self.ui.dateTimeregDate.setDate(QDate.fromString(self._baseproject.get("activitydate"),
                                                         "yyyy-MM-dd"))
        self._updateSmartQuery(self._baseproject.getSmartQuery())
        self.notify(self.tr("Loading..."))

    def delete(self):
        if not self._baseproject.isNew():
            debug("-------------> Delete")
            self.remote.delete(id=self._baseproject.get("id"))
            self.notify(self.tr("Deleting..."))
        else:
            debug("-------------> Reset")
            self.notify(self.tr("Resetting..."))
            self._setupGui()

    def notify(self, msg, timeout=0):
        self.ui.statusBar.showMessage(msg, timeout)

class AchievoProject:
    """
    Classe che decora il progetto xml con alcune metodi di utilità
    """
    keys = "prj pha act hmtime remark".split()

    def __init__(self, eproject=None):
        if eproject == None:
            eproject = ET.fromstring("<record></record>")
        self.data = copy.deepcopy(eproject)

    def isNew(self):
        return self.data.get("id") == None

    def isUnivocal(self):
        for key in self.keys[:3]:
            if self.get(key) in [None, ""]:
                return False
        return True

    def isComplete(self):
        for key in self.keys:
            if self.get(key) in [None, ""]:
                return False
        return True

    def get(self, key):
        if key in ["remark", "in_remark"]:
            return self.data.text
        else:
            if self.data.get(key) != None:
                return self.data.get(key).replace(" ", "_")
            else:
                return self.data.get(key)

    def getPPA(self):
        return " ".join([self.get(k) for k in self.keys[:3]])

    def set(self, key, val):
        if key in ["remark", "in_remark"]:
            self.data.text = val
        else:
            if val != None:
                self.data.set(key, val)
            elif key in self.data.attrib:
                del self.data.attrib[key]

    def items(self):
        return self.data.items() + [("remark", self.data.text)]

    def initems(self):
        return [i for i in self.data.items() + [("in_remark", self.data.text)] if i[0].find("in_") != -1]

    def outitems(self):
        return [i for i in self.data.items() + [("remark", self.data.text)] if i[0].find("in_") == -1]

    def merge(self, others):
        # metto a None tutti gli attributi ambigui
        values = {}
        for c, other in enumerate(others):
            for k, v in other.items()+[("remark", other.text)]:
                values.setdefault(k, set())
                values[k].add(v)
        for k, v in values.items():
            if len(v) == 1:
                self.set(k, list(v)[0])
            else:
                self.set(k, None)
#        for k, v in self.outitems():
#            if v != None:
#                self.set("in_%s" % k, v)

    def reset(self):
        for key in self.data.attrib.keys():
            if key != "id":
                del self.data.attrib[key]

    def getSmartQuery(self):
        smartquery = " ".join([self.get("%s" % key) or self.get("in_%s" % key) or "" for key in self.keys])
        if smartquery.strip() == "":
            return smartquery.strip()
        return smartquery.strip()+" "

    def __str__(self):
        return ET.tostring(self.data, "UTF-8")

if __name__ == "__main__":
    app = TimeregApplication(sys.argv)
    #2 m a 2:34 prova prova èàò
