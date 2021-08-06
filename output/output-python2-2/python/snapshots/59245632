#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id: QTimeregWindow2.py 21760 2008-06-18 10:26:17Z duplo $
#
# Author: Matteo Bertini <naufraghi@develer.com>

import os, sys, copy

from pyuac_utils import *
from QRemoteTimereg import *
from daterange import *

LRU_LEN = 10

#modalità che può assumere la finestra di dialogo
MODES = ("normal",  "range")

class TimeregWindow(QMainWindow, QAchievoWindow):

    def __init__(self, parent, auth,  mode):
        debug("TimeregWindow.__init__")
        QMainWindow.__init__(self, parent)
        self.__setup__(auth, 'pyuac_edit.ui')

        self.completer = QCompleter([], self)
        self.completer.setCaseSensitivity(Qt.CaseInsensitive)
        self.completer.setCompletionMode(QCompleter.UnfilteredPopupCompletion)
        self.ui.editSmartQuery.setCompleter(self.completer)
        self._completer_list = []

        self._baseproject = AchievoProject()
        self._response_projects = []
        self._all_ppa = {}
        self._projects = set()
        if mode in MODES:
            self.mode = mode
        else:
            self.mode = "range"
        self._connectSlots()
        self._setupGui()

    def _setupGui(self):
        debug("TimeregWindowSelection._setupGui")
        if self.mode == "normal":
            self._normalMode()
        elif self.mode == "range":
            self._rangeMode()
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
        self.notify(self.tr("Type something in the smartquery field or use combos."))
        self._smartQueryEdited("")

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
                     self._slotClose)
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

    def _completerActivated(self, smartquery):
        smartquery = unicode(smartquery).strip()
        self._setSmartQuery(smartquery + " ")
        self._smartQueryEdited(smartquery)

    def _updateSmartQuery(self, smartquery):
        """
        Updates the smartquery starting a new query
        """
        debug(u"_updateSmartQuery '%s'" % smartquery)
        self._setSmartQuery(smartquery)
        self._smartQueryEdited(smartquery)

    def _setSmartQuery(self, smartquery):
        """
        Updates the smartquery with no side effects
        """
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
        self.ui.labelTimeWorked.setEnabled(p.get("hmtime") != "00:00")

        self.ui.labelExactTime.setText(p.get("in_hmtime") or "00:00")

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

        _bp = self._baseproject

        def _updateBaseproject(combo, combotext):
            if combo == "Project":
                _bp.set("in_prj", combotext)
                _bp.set("prj", combotext)
                _bp.set("pha", None)
                _bp.set("act", None)
            elif combo == "Phase":
                _bp.set("in_pha", combotext)
                _bp.set("pha", combotext)
                _bp.set("act", None)
            elif combo == "Activity":
                _bp.set("in_act", combotext)
                _bp.set("act", combotext)
            elif combo == "TimeWorked":
                _bp.set("in_hmtime", combotext)
                _bp.set("hmtime", combotext)
            # se ho attivato un combo
            if combo != None:
                self._updateSmartQuery(_bp.getSmartQuery())
                return
        _updateBaseproject(combo, combotext)

        # Aggiorna la lista di progetti, fasi e attività
        # usata per riempire i combobox
        def _updatePpa():
            _ppa = {}
            for p in self._response_projects:
                self._projects.add(p.get("prj"))
                self._all_ppa.setdefault(p.get("prj"), {})
                self._all_ppa[p.get("prj")].setdefault(p.get("pha"), {})
                self._all_ppa[p.get("prj")][p.get("pha")].setdefault(p.get("act"), {})
                _ppa.setdefault(p.get("prj"), {})
                _ppa[p.get("prj")].setdefault(p.get("pha"), {})
                _ppa[p.get("prj")][p.get("pha")].setdefault(p.get("act"), {})
            if _bp.isUnivocal():
                _ppa = self._all_ppa
            return _ppa
        _ppa = _updatePpa()

        def _updateSmartquery():
            def endsWithSpace(msg):
                return msg[-1:] == " "
            # se il progetto inserito identifica univocamente un nome e
            # la smartquery termina con spazio (questo permette di non
            # interferire brutalmente con le modifiche fatte dall'utente)
            if _bp.get("prj") != None:
                smartquery = unicode(self.ui.editSmartQuery.text())
                if _bp.get("prj") != _bp.get("in_prj"):
                    _bp.set("in_prj", _bp.get("prj"))
                    if endsWithSpace(smartquery):
                        self._setSmartQuery(_bp.getSmartQuery())
                if _bp.get("pha") != None:
                    if _bp.get("pha") != _bp.get("in_pha"):
                        _bp.set("in_pha", _bp.get("pha"))
                        if endsWithSpace(smartquery):
                            self._setSmartQuery(_bp.getSmartQuery())
                    if _bp.get("act") != None and _bp.get("act") != _bp.get("in_act"):
                        _bp.set("in_act", _bp.get("act"))
                        if endsWithSpace(smartquery):
                            self._setSmartQuery(_bp.getSmartQuery())
        _updateSmartquery()

        def _updateCombos():
            self.ui.comboProject.clear()
            self.ui.comboPhase.clear()
            self.ui.comboActivity.clear()
            self.ui.comboProject.addItems([""]+sorted(list(self._projects)))
            if _bp.get("prj") != None:
                self.ui.comboPhase.addItems([""]+sorted(_ppa[_bp.get("prj")].keys()))
                if _bp.get("pha") != None:
                    prj = _bp.get("prj")
                    pha = _bp.get("pha")
                    self.ui.comboActivity.addItems([""]+sorted(_ppa[prj][pha].keys()))
        _updateCombos()

        def _updateCompleter():
            project = _bp.get("prj")
            phase = _bp.get("pha")
            activity = _bp.get("act")
            hmtime = _bp.get("hmtime")
            if project == None:
                _completer = [pro for pro in _ppa.keys()]
            else:
                if phase == None:
                    _base = project+" "
                    _completer = [_base + pha for pha in _ppa[project].keys()]
                else:
                    if activity == None or not _bp.get("in_act"):
                        _base = project+" "+phase+" "
                        _completer = [_base + act for act in _ppa[project][phase].keys()]
                    else:
                        if hmtime not in timerange(8, 15, 1):
                            _base = project+" "+phase+" "+activity+" "
                            _completer = [_base + hmtime for hmtime in timerange(8, 15, 1)]
                        else:
                            _completer = []

            # se ho già scritto il commento, lo aggiungo al completer
            for c, v in enumerate(_completer):
                _completer[c] = " ".join([_completer[c], _bp.get("remark") or ""]).strip()

            _completer.sort()
            if _completer != self._completer_list:
                self.completer.setModel(QStringListModel(_completer, self.completer))
                self._completer_list = _completer

            if combo == None and self.ui.isVisible():
                #altrimenti compare anche a finestra invisibile...
                self.completer.complete()
                #debug("self.completer %s" % _completer)
        _updateCompleter()

    def _timeregStarted(self):
        #debug("_timeregStarted")
        pass

    def _registrationDone(self, eresp):
        #debug("_registrationDone")
        username = self.remote.auth[1]
        lru = self.settings.getArray("lru", ["ppa-%s" % username])
        new_ppa = {"ppa-%s" % username: QVariant(self._baseproject.getPPA()+" ")}
        lru.insert(0, new_ppa)
        if new_ppa in lru[1:]:
            lru.pop(lru.index(new_ppa, 1))
        while len(lru) >= LRU_LEN:
            lru.pop()
        self.settings.setArray("lru", lru)
        self.emit(SIGNAL("registrationDone"), self._baseproject)
        self._baseproject.reset()
        self._slotClose()

    def _timeregErr(self):
        #debug("_timeregError")
        pass

    def _searchStarted(self):
        debug("%s _searchStarted" % __name__)
        self._disableAll()

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
            
        #operazioni necessarie solamente nella modalità di immissione multipla
        #controllo delle date
        if self.ui.dateTimeregDateFrom.date() > self.ui.dateTimeregDateTo.date():
            self.notify(self.tr("From date is after end date!"),  10000)
            return
        #fine controllo delle date
        
        #controllo dei giorni lavorativi
        days = []
        days.append(self.ui.monCheckBox.isChecked())
        days.append(self.ui.tueCheckBox.isChecked())
        days.append(self.ui.wedCheckBox.isChecked())
        days.append(self.ui.thuCheckBox.isChecked())
        days.append(self.ui.friCheckBox.isChecked())
        days.append(self.ui.satCheckBox.isChecked())
        days.append(self.ui.sunCheckBox.isChecked())
        days = tuple(days)
        #fine controllo dei giorni lavorativi
        #fine delle operazioni necessarie solamente nella modalità di immissione multipla
        
        self.ui.btnSave.setEnabled(False)
        for date in daterange(self.ui.dateTimeregDateFrom.date(),  self.ui.dateTimeregDateTo.date(),  days):
            p = self._baseproject
            activitydate = str(date.toString("yyyy-MM-dd"))
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
        self.ui.dateTimeregDateFrom.setDate(QDate.fromString(self._baseproject.get("activitydate"),
                                                         "yyyy-MM-dd"))
        self.ui.dateTimeregDateTo.setDate(QDate.fromString(self._baseproject.get("activitydate"),
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

    #metodi per la personalizzazione dell'interfaccia a seconda della modalità
    def _normalMode(self):
        self.ui.daterangeGroupBox.setVisible(False)
    
    def _rangeMode(self):
        self.ui.dateTimeregDateFrom.setDate(QDate.currentDate())
        self.ui.dateTimeregDateTo.setDate(QDate.currentDate())
        self.ui.dateGroupBox.setVisible(False)
    
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

    def isSync(self):
        for key in self.keys[:3]:
            if self.get(key) != self.get("in_%s" % key):
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
