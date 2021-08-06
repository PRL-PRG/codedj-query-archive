#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id: QTimeBrowseWindow_test.py 21759 2008-06-18 08:42:23Z duplo $
#
# Author: Matteo Bertini <naufraghi@develer.com>

import os, sys

from pyuac_utils import *

from QRemoteTimereg import *
from QTimeregWindow import *

class LoginDialog(QDialog, QAchievoWindow):
    def __init__(self, parent, config):
        QDialog.__init__(self, parent)
        self.__setup__(_path='pyuac_auth.ui')
        _achievouri = self.settings.value("achievouri", QVariant(config["achievouri"])).toString()
        _username = self.settings.value("username", QVariant(config["username"])).toString()
        self.ui.editAchievoUri.setText(_achievouri)
        self.ui.editUsername.setText(_username)
        self.connect(self.ui, SIGNAL("accepted()"), self.login)
        self.connect(self.ui, SIGNAL("rejected()"), self.cancel)
        self.ui.editPassword.setFocus()
        self.ui.show()

    def login(self):
        debug("login")
        self.settings.setValue("achievouri", QVariant(self.ui.editAchievoUri.text()))
        self.settings.setValue("username", QVariant(self.ui.editUsername.text()))
        auth = [self.ui.editAchievoUri.text()]
        auth += [self.ui.editUsername.text()]
        auth += [self.ui.editPassword.text()]
        self.emit(SIGNAL("login"), auth)
        self.ui.editPassword.setText("")
        self.ui.hide()

    def cancel(self):
        debug("cancel")
        self.emit(SIGNAL("cancel"))
        self.ui.close()


class TimeBrowseWindow(QMainWindow, QAchievoWindow):
    def __init__(self, parent, auth=None, config=None):
        QMainWindow.__init__(self, parent)
        self.projects = None
        if config != None:
            self.login = LoginDialog(self, config)
            self.connect(self.login, SIGNAL("login"), self.__auth__)
            self.connect(self.login, SIGNAL("cancel"), self._slotClose)
        elif auth != None:
            self.__auth__(auth)
        else:
            raise TypeError, "Provide auth or config"

    def __auth__(self, auth):
        self.__setup__(auth, 'pyuac_browse.ui')
        self._setupGui()
        self._connectSlots()
        self.ui.show()

    def _connectSlots(self):
        # Short-circuit Signals (from python to python)
        self.connect(self.remote, SIGNAL("timereportStarted"),
                     self._slotTimereportStarted)
        self.connect(self.remote, SIGNAL("timereportOK"),
                     self._slotUpdateTimereport)
        self.connect(self.ui.btnTimereg, SIGNAL("clicked()"),
                     self._slotNewTimereg)
        self.connect(self.ui.btnQuit, SIGNAL("clicked()"),
                     self._slotClose)
        self.connect(self.ui.btnToday, SIGNAL("clicked()"),
                     lambda: self._changeDate(QDate.currentDate()))
        self.connect(self.ui.btnNextDay, SIGNAL("clicked()"),
                     lambda: self._changeDateDelta(1))
        self.connect(self.ui.btnPrevDay, SIGNAL("clicked()"),
                     lambda: self._changeDateDelta(-1))
        self.connect(self.ui.btnNextWeek, SIGNAL("clicked()"),
                     lambda: self._changeDateDelta(7))
        self.connect(self.ui.btnPrevWeek, SIGNAL("clicked()"),
                     lambda: self._changeDateDelta(-7))
        self.connect(self.ui.dateEdit, SIGNAL("dateChanged(const QDate&)"),
                     self._slotTimereport)
        self.connect(self.ui.tableTimereg, SIGNAL("cellDoubleClicked(int,int)"),
                     self._slotTimeEdit)
        self.connect(self.ui.smart_time_edit, SIGNAL("textChanged(const QString)"),
                     self._slotSmartTimeChanged)
        self.connect(self.ui.smart_time_edit, SIGNAL("lostFocus()"),
                     self._slotSmartTimeChanged)

    def _changeDate(self, date):
        if self.ui.dateEdit.date() != date:
            self.ui.dateEdit.setDate(date)
        self._slotTimereport(date)

    def _changeDateDelta(self, numdays):
        date = self.ui.dateEdit.date()
        date = date.addDays(numdays)
        self._changeDate(date)

    def _setupGui(self):
        """
        Reimposta la gui ai volori di default
        (titoli colonne e data attuale)
        """
        self.ui.tableTimereg.setColumnCount(5)
        for c, head in enumerate("Date Project/Phase Activity Time Remark".split()):
            cellHead = QTableWidgetItem(head)
            self.ui.tableTimereg.setHorizontalHeaderItem(c, cellHead)
        self.ui.tableTimereg.horizontalHeader().setStretchLastSection(True)
        self._changeDate(QDate.currentDate())

    def _createTimeregWindow(self):
        editwin = TimeregWindowSelection(self, self.remote.auth)
        self.connect(editwin, SIGNAL("registrationDone"),
                     self._slotRegistrationDone)
        return editwin

    def _slotNewTimereg(self):
        """ <-- self.ui.btnTimereg, SIGNAL("clicked()")
        Imposta la data selezionata nel template ed
        avvia la finestra di inserimento nuova registrazione
        """
        selected_date = unicode(self.ui.dateEdit.date().toString("yyyy-MM-dd"))
        project_template = AchievoProject()
        project_template.set("activitydate", selected_date)
        editwin = self._createTimeregWindow()
        editwin.setupEdit(project_template.data)
        editwin.show()

    def _slotTimeEdit(self, row, column):
        """ <-- self.ui.tableTimereg, SIGNAL("cellDoubleClicked(int,int)")
        Prepara un template con i dati della riga selezionata
        ed avvia la form di modifica
        """
        project_template = AchievoProject()
        print self.projects[row].items()
        for k in project_template.keys:
            project_template.set("in_%s" % k, self.projects[row].get(k))
        for k in "id activitydate".split():
            project_template.set(k, self.projects[row].get(k))
        editwin = self._createTimeregWindow()
        editwin.setupEdit(project_template.data)
        editwin.show()

    def _slotRegistrationDone(self, eresp):
        """ <-- self.edit, SIGNAL("registrationDone")
        Refreshs the window after a time registration
        """
        newdate = QDate.fromString(str(eresp.get("activitydate")), "yyyy-MM-dd")
        if newdate != self.ui.dateEdit.date():
            self.ui.dateEdit.setDate(newdate)
        self._slotTimereport(newdate)

    def _slotTimereport(self, qdate):
        """ <-- self.ui.dateEdit, SIGNAL("dateChanged(const QDate&)")
        Starts the query to update the table contents
        """
        reportdate = qdate.toString("yyyy-MM-dd")
        self.notify(self.tr("Searching..."))
        self.remote.timereport(date=reportdate)

    def _slotTimereportStarted(self):
        self.ui.btnTimereg.setEnabled(False)
        self.ui.tableTimereg.setRowCount(0)

    def _slotUpdateTimereport(self, eprojects):
        """ <-- self.remote, SIGNAL("timereportOK")
        Aggiorna la tabella delle ore registrate
        con la lista dei progetti restituiti da *remote*
        Ha il side-effect di convertire time (minuti) in hmtime (ore:minuti)
        """
        self.projects = []
        self.ui.tableTimereg.setRowCount(len(eprojects))
        total_time = 0
        for r, p in enumerate(eprojects):
            self.projects.append(AchievoProject(p))
            p = self.projects[-1]
            row = []
            row.append(QTableWidgetItem(p.get("activitydate")))
            row.append(QTableWidgetItem("%s / %s" % (p.get("prj"), p.get("pha"))))
            row.append(QTableWidgetItem(p.get("act")))
            hmtime = min2hmtime(int(p.get("time")))
            p.set("hmtime", hmtime)
            total_time += int(p.get("time"))
            row.append(QTableWidgetItem(hmtime))
            row.append(QTableWidgetItem("\n"+p.get("remark")+"\n"))
            for c, cell in enumerate(row):
                self.ui.tableTimereg.setItem(r, c, cell)
                if c != 4:
                    self.ui.tableTimereg.resizeColumnToContents(c)
        self.notify(self.tr("Day total: ") + "%s" % min2hmtime(total_time))
        self.ui.tableTimereg.resizeRowsToContents()
        self.ui.btnTimereg.setEnabled(True)

    def _slotSmartTimeChanged(self, text=None):
        smartime = self.ui.smart_time_edit.text()
        try:
            lapse = parse_wtime(smartime)
        except:
            lapse = "0:00"
        if len(smartime):
            self.ui.time_sum_lbl.setText(lapse)
        else:
            self.ui.time_sum_lbl.setText("0:00")
