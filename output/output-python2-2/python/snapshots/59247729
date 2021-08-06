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
MODES = ("single",  "range", "monthly")

class TimeregWindow(QMainWindow, QAchievoWindow):

    def __init__(self, parent, auth,  mode):
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
            self._mode = mode
        else:
            assert False, "modo non gestito: %s" % self._mode
        self._connectSlots()
        self._setupGui()

    def _setupGui(self):
        if self._mode == "single":
            self._uiSingleMode()
        elif self._mode == "range":
            self._uiRangeMode()
        elif self._mode == "monthly":
            self._uiMonthlyMode()
        else:
            assert False, "modo non gestito: %s" % self._mode
        self.ui.comboTimeWorked.clear()
        for htext in timerange(8, 15):
            self.ui.comboTimeWorked.addItem(htext, QVariant(hmtime2min(htext)))
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
    
    def _uiSingleMode(self):
        """
        Inizializza i valori delle componenti necessarie solamente in modalità 'single'.
        """
        self.ui.stackedWidget.setCurrentIndex(0)
    
    def _uiRangeMode(self):
        """
        Connette le componenti necessarie solamente in modalità 'range' e ne inizializza i valori.
        """
        self.ui.stackedWidget.setCurrentIndex(1)
        self.connect(self.ui.dateFromDateEdit, SIGNAL("dateChanged(const QDate&)"), 
                        self._updateDaysLabel)
        self.connect(self.ui.dateToDateEdit, SIGNAL("dateChanged(const QDate&)"), 
                        self._updateDaysLabel)
        checkBoxes = []
        checkBoxes.append(self.ui.monCheckBox)
        self.connect(self.ui.monCheckBox, SIGNAL("toggled(bool)"), 
                        self._updateDaysLabel)
        checkBoxes.append(self.ui.tueCheckBox)
        self.connect(self.ui.tueCheckBox, SIGNAL("toggled(bool)"), 
                        self._updateDaysLabel)
        checkBoxes.append(self.ui.wedCheckBox)
        self.connect(self.ui.wedCheckBox, SIGNAL("toggled(bool)"), 
                        self._updateDaysLabel)
        checkBoxes.append(self.ui.thuCheckBox)
        self.connect(self.ui.thuCheckBox, SIGNAL("toggled(bool)"), 
                        self._updateDaysLabel)
        checkBoxes.append(self.ui.friCheckBox)
        self.connect(self.ui.friCheckBox, SIGNAL("toggled(bool)"), 
                        self._updateDaysLabel)
        checkBoxes.append(self.ui.satCheckBox)
        self.connect(self.ui.satCheckBox, SIGNAL("toggled(bool)"), 
                        self._updateDaysLabel)
        checkBoxes.append(self.ui.sunCheckBox)
        self.connect(self.ui.sunCheckBox, SIGNAL("toggled(bool)"), 
                        self._updateDaysLabel)
        for i, checkBox in enumerate(checkBoxes):
            checkBox.setText(unicode(QDate.longDayName(i + 1)).capitalize())

    def _uiMonthlyMode(self):
        """
        Connette le componenti necessarie solamente in modalità 'monthly' e ne inizializza i valori.
        """
        self.ui.stackedWidget.setCurrentIndex(2)
        checkBoxes = []
        checkBoxes.append(self.ui.monMonthCheckBox)
        checkBoxes.append(self.ui.tueMonthCheckBox)
        checkBoxes.append(self.ui.wedMonthCheckBox)
        checkBoxes.append(self.ui.thuMonthCheckBox)
        checkBoxes.append(self.ui.friMonthCheckBox)
        checkBoxes.append(self.ui.satMonthCheckBox)
        checkBoxes.append(self.ui.sunMonthCheckBox)
        for i, checkBox in enumerate(checkBoxes):
            checkBox.setText(unicode(QDate.longDayName(i + 1)).capitalize())
        self.connect(self.ui.hoursSpinBox, SIGNAL("valueChanged(int)"),
                     self._spinboxHoursActivated)
        #Rende invisibili le componenti per la scelta del numero di ore
        self.ui.labelTimeWorked.setVisible(False)
        self.ui.comboTimeWorked.setVisible(False)
        self.disconnect(self.ui.comboTimeWorked, SIGNAL("activated(const QString&)"),
                     self._comboTimeWorkedActivated)
        for i in range(12):
            self.ui.monthComboBox.addItem(QDate.longMonthName(i + 1), QVariant(i + 1))
    
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
        self._setSmartQuery(smartquery)
        self._smartQueryEdited(smartquery)

    def _setSmartQuery(self, smartquery):
        """
        Updates the smartquery with no side effects
        """
        self.ui.editSmartQuery.setText(smartquery)

    def _smartQueryEdited(self, smartquery):
        """ <-- self.ui.editSmartQuery, SIGNAL("textEdited(QString)")
        (Emesso solo per modifiche "umane" e non da programma.)
        Avvia la query al servizio remoto di achievo tramite la cli
        """
        smartquery = unicode(smartquery).strip()
        self.ui.hoursSpinBox.blockSignals(True)
        self.remote.query([{"smartquery": smartquery}])
    
    def _projectsChanged(self, projects):
        """ <-- self.remote, SIGNAL("queryOK")
        Aggiorna lo stato interno in funzione dei progetti
        restituiti dalla ricerca in Achievo:
        """
        QApplication.restoreOverrideCursor()
        self._response_projects = projects[0]
        for elem in self._response_projects:
            print elem
            for k, v in elem.items():
                print k, v
        if len(self._response_projects) != 0:
            self._baseproject.merge(self._response_projects)
        self._updateGui()
        self.notify(self.tr(""))

    def _updateGui(self):
        """
        Aggiorna lo stato visuale della gui in funzione dello stato interno
        """
        self._updateComboBoxes()
        p = self._baseproject
        self._disableAll()
        idx = self.ui.comboProject.findText(p.get("prj") or "")
        self.ui.comboProject.setCurrentIndex(idx)
        #self.ui.labelProject.setEnabled(p.get("prj") != None)

        idx = self.ui.comboPhase.findText(p.get("pha") or "")
        self.ui.comboPhase.setCurrentIndex(idx)
        self.ui.labelPhase.setEnabled(p.get("prj") != None)

        idx = self.ui.comboActivity.findText(p.get("act") or "")
        self.ui.comboActivity.setCurrentIndex(idx)
        self.ui.labelActivity.setEnabled(p.get("pha") != None)
        
        if self._mode == "monthly":
            p.set("hmtime", p.get("in_hmtime"))
            self.ui.hoursSpinBox.setValue(int(p.get("in_hmtime").split(":")[0] or 0))
            self.ui.hoursSpinBox.blockSignals(False)
        else:    
            idx = self.ui.comboTimeWorked.findText(p.get("hmtime") or "00:00")
            self.ui.comboTimeWorked.setCurrentIndex(idx)
            self.ui.labelTimeWorked.setEnabled(True)
            
        self.ui.txtRemark.setPlainText((p.get("remark") or "").strip())
        self.ui.labelRemark.setEnabled(True)

        self.ui.btnSave.setEnabled(p.isComplete())

        #deselezione lru
        self.ui.labelProject.setEnabled(self._all_ppa.keys() != [])
        self.ui.comboPPAlru.setCurrentIndex(0)
        self.ui.btnCancel.setEnabled(True)
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
        self.ui.btnCancel.setEnabled(False)

    def _updateComboBoxes(self, combo=None, combotext=None):
        """ <-- _updateGui() e dai combo
        Aggiorna i combobox in modo che contengano
        l'unione dei valori visti durante la sessione
        """
        combotext = unicode(combotext)

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
            elif self._mode != "monthly" and combo == "TimeWorked":
                _bp.set("in_hmtime", combotext)
                _bp.set("hmtime", combotext)
            elif self._mode == "monthly" and combo == "Hours":
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
                        if  self._mode != "monthly" and hmtime not in timerange(24, 15, 1):
                            _base = project+" "+phase+" "+activity+" "
                            _completer = [_base + hmtime for hmtime in timerange(8, 15, 1)]
                        else:
                            _completer = []

            # se ho già scritto l'ora o il commento, lo aggiungo al completer
            if self._mode == "monthly":
                for c, v in enumerate(_completer):
                    _completer[c] = " ".join([_completer[c], str(self.ui.hoursSpinBox.value() or "")]).strip()
            else:
                for c, v in enumerate(_completer):
                    _completer[c] = " ".join([_completer[c], _bp.get("hmtime") or ""]).strip()
            for c, v in enumerate(_completer):
                _completer[c] = " ".join([_completer[c], _bp.get("remark") or ""]).strip()

            _completer.sort()
            if _completer != self._completer_list:
                self.completer.setModel(QStringListModel(_completer, self.completer))
                self._completer_list = _completer

            if combo == None and self.ui.isVisible() and self.ui.editSmartQuery.hasFocus():
                #altrimenti compare anche a finestra invisibile...
                self.completer.complete()
        _updateCompleter()

    def _updateDaysLabel(self):
        """
        Aggiorna il numero di giorni lavorativi e di giorni totali che appare a lato delle dateEdit, in modalità
        'range'
        """
        working, total = daysnumber(self.ui.dateFromDateEdit.date(), self.ui.dateToDateEdit.date(), self._getDays())
        self.ui.daysLabel.setText("Working days: %d, Total days: %d" %(working, total))
    
    def _getDays(self):
        """
        Ritorna una tupla di booleani, dove True sta per lavorativo e False sta per non lavorativo.
        """
        days = []
        if self._mode == "range":
            days.append(self.ui.monCheckBox.isChecked())
            days.append(self.ui.tueCheckBox.isChecked())
            days.append(self.ui.wedCheckBox.isChecked())
            days.append(self.ui.thuCheckBox.isChecked())
            days.append(self.ui.friCheckBox.isChecked())
            days.append(self.ui.satCheckBox.isChecked())
            days.append(self.ui.sunCheckBox.isChecked())
        elif self._mode == "monthly":
            days.append(self.ui.monMonthCheckBox.isChecked())
            days.append(self.ui.tueMonthCheckBox.isChecked())
            days.append(self.ui.wedMonthCheckBox.isChecked())
            days.append(self.ui.thuMonthCheckBox.isChecked())
            days.append(self.ui.friMonthCheckBox.isChecked())
            days.append(self.ui.satMonthCheckBox.isChecked())
            days.append(self.ui.sunMonthCheckBox.isChecked())
        return tuple(days)

    def _timeregStarted(self):
        self._disableAll()
        QApplication.restoreOverrideCursor()
        QApplication.setOverrideCursor(QCursor(Qt.BusyCursor))

    def _registrationDone(self, eresp):
        #self._registrations -= 1
        #if not self._registrations:
        QApplication.restoreOverrideCursor()
        self.emit(SIGNAL("registrationDone"), self._baseproject)
        self._endingRegistrations()
    
    def _endingRegistrations(self):
        username = self.remote.auth[1]
        lru = self.settings.getArray("lru", ["ppa-%s" % username])
        new_ppa = {"ppa-%s" % username: QVariant(self._baseproject.getPPA()+" ")}
        lru.insert(0, new_ppa)
        if new_ppa in lru[1:]:
            lru.pop(lru.index(new_ppa, 1))
        while len(lru) >= LRU_LEN:
            lru.pop()
        self.settings.setArray("lru", lru)
        self._baseproject.reset()
        self._slotClose()
    
    def _timeregErr(self):
        pass

    def _searchStarted(self):
        QApplication.restoreOverrideCursor()
        QApplication.setOverrideCursor(QCursor(Qt.BusyCursor))
        self._disableAll()

    def _comboProjectActivated(self, combotext):
        self._updateComboBoxes("Project", combotext)

    def _comboPhaseActivated(self, combotext):
        self._updateComboBoxes("Phase", combotext)

    def _comboActivityActivated(self, combotext):
        self._updateComboBoxes("Activity", combotext)

    def _comboTimeWorkedActivated(self, combotext):
        self._updateComboBoxes("TimeWorked", combotext)

    def _spinboxHoursActivated(self, value):
        self._updateComboBoxes("Hours", str(value))

    def _multipleInsertionWarning(self, start_day, end_day, days):
        days = daysnumber(start_day, end_day, days)[0]
        if days > 1:
            ret = QMessageBox.warning(self, "TimeregWindow",
                                      "Your changes will affect %d days, are you sure?\n" % days,
                                      QMessageBox.Ok | QMessageBox.Cancel)
            return ret
        return 1
    
    def _checkDate(self):
        invalid = False
        if self._mode == "single":
            if self.ui.singleDateEdit.date() > QDate.currentDate():
                invalid = True
        if self._mode == "range":
            if self.ui.dateFromDateEdit.date() > QDate.currentDate() \
            or self.ui.dateToDateEdit.date() > QDate.currentDate():
                invalid = True
        if self._mode == "montly":
            pass
        if invalid:
            QMessageBox.critical(self, "TimeregWindow",
                                 "Selected date is invalid!",
                                 QMessageBox.Cancel)
        return not invalid

    def timereg(self):
        """
        Metodo chiamato per salvare le ore di lavoro: a sua volta richiama i metodi che elaborano i
        dati in base alla modalità di inserimento.
        """
        if not self._baseproject.isComplete():
            self.notify(self.tr("Unable to save!"), 1000)
            return
        if self._checkDate():
            self.ui.btnSave.setEnabled(False)
            if self._mode == "range":
                ret = self._multipleInsertionWarning(self.ui.dateFromDateEdit.date(),
                                                     self.ui.dateToDateEdit.date(),
                                                     self._getDays())
                if ret == 1:
                    self._rangeTimereg()
                else:
                    self.ui.btnSave.setEnabled(True)
            elif self._mode == "single":
                self._singleTimereg()
            elif self._mode == "monthly":
                year = QDate.currentDate().year()
                month = self.ui.monthComboBox.itemData(self.ui.monthComboBox.currentIndex()).toInt()[0]
                startDay = QDate(year, month, 1)
                endDay = QDate(year,month, 1).addMonths(1).addDays(-1)
                ret = self._multipleInsertionWarning(startDay, endDay, self._getDays())
                if ret == QMessageBox.Ok:
                    self._monthlyTimereg()
                else:
                    self.ui.btnSave.setEnabled(True)
            else:
                assert False, "modo non gestito: %s" % self._mode
            self.notify(self.tr("Saving..."))
    
    def _timereg(self, request_pack):
        self.remote.timereg(request_pack)
    
    def _singleTimereg(self):
        """
        Metodo chiamato da timereg per registrare le ore dalla modalità 'single'.
        """
        activitydate = str(self.ui.singleDateEdit.date().toString("yyyy-MM-dd"))
        p = self._baseproject
        p.set("activitydate", activitydate)
        params = dict([(k, p.get(k)) for k in "projectid phaseid activityid hmtime activitydate".split()])
        params["remark"] = p.get("remark")
        if not self._baseproject.isNew():
            params["id"] = self._baseproject.get("id")
        request_pack = [params]
        self._timereg(request_pack)
    
    def _rangeTimereg(self):
        """
        Metodo chiamato da timereg per registrare le ore dalla modalità 'range'.
        """
        #controllo che impedisce che le date inserite non siano consistenti
        if self.ui.dateFromDateEdit.date() > self.ui.dateToDateEdit.date():
            self.notify(self.tr("From date is after end date!"), 10000)
            return
        request_pack = []
        for date in daterange(self.ui.dateFromDateEdit.date(), self.ui.dateToDateEdit.date(), self._getDays()):
            activitydate = str(date.toString("yyyy-MM-dd"))
            p = self._baseproject
            params = dict([(k, p.get(k)) for k in "projectid phaseid activityid hmtime".split()])
            params["activitydate"] = activitydate
            params["remark"] = p.get("remark")
            if not self._baseproject.isNew():
                params["id"] = self._baseproject.get("id")
            request_pack.append(params)
        self._timereg(request_pack)
    
    def _monthlyTimereg(self):
        """
        Metodo chiamato da timereg per registrare le ore dalla modalità 'montly'.
        """
        year = QDate.currentDate().year()
        month = self.ui.monthComboBox.itemData(self.ui.monthComboBox.currentIndex()).toInt()[0]
        startDay = QDate(year, month, 1)
        endDay = QDate(year,month, 1).addMonths(1).addDays(-1)
        days = daysnumber(startDay, endDay, self._getDays())[0]
        try:
            hours = [hour for hour in divide(self.ui.hoursSpinBox.value(), days)]
        except ValueError:
            ret = QMessageBox.critical(self, "TimeregWindow",
                                      "Time registration must not exceed 24 hours/day!\n",
                                      QMessageBox.Ok)
            return
        request_pack = []
        for date in daterange(startDay, endDay, self._getDays()):
            activitydate = str(date.toString("yyyy-MM-dd"))
            hmtime = hours.pop()
            p = self._baseproject
            params = dict([(k, p.get(k)) for k in "projectid phaseid activityid".split()])
            params["activitydate"] = activitydate
            params["hmtime"] = hmtime
            params["remark"] = p.get("remark")
            if not self._baseproject.isNew():
                params["id"] = self._baseproject.get("id")
            request_pack.append(params)
        self._timereg(request_pack)


    def setupEdit(self, project):
        self._baseproject = AchievoProject(project.data)
        if not self._baseproject.isNew():
            self.ui.btnDelete.setText(self.tr("Delete"))
        self.ui.singleDateEdit.setDate(QDate.fromString(self._baseproject.get("activitydate"), "yyyy-MM-dd"))
        #copia la data in tutte le dateEdit, visibili e non.
        self.ui.dateFromDateEdit.setDate(self.ui.singleDateEdit.date())
        self.ui.dateToDateEdit.setDate(self.ui.singleDateEdit.date())
        self.ui.monthComboBox.setCurrentIndex(self.ui.monthComboBox.findData(
            QVariant(self.ui.singleDateEdit.date().month())) - 1)
        self._updateSmartQuery(self._baseproject.getSmartQuery())
        self.notify(self.tr("Loading..."))

    def delete(self):
        if not self._baseproject.isNew():
            self.remote.delete([{"id": self._baseproject.get("id")}])
            self.notify(self.tr("Deleting..."))
        else:
            self.notify(self.tr("Resetting..."))
            self._setupGui()
    
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
