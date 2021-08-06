#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import os, sys

from pyuac_utils import *

from QRemoteTimereg import *
from QTimeregWindow import *

class LoginDialog(QDialog):
    def __init__(self, parent, config):
        QDialog.__init__(self, parent)

        _path = 'pyuac_auth.ui'
        if hasattr(sys, "frozen") and sys.frozen:
            _path = os.path.join(os.path.dirname(sys.executable), _path)

        self.ui = uic.loadUi(_path, self)
        self.settings = QSettings("Develer", "PyUAC")
        _achievouri = self.settings.value("achievouri", QVariant(config["achievouri"])).toString()
        _username = self.settings.value("username", QVariant(config["username"])).toString()
        self.ui.editAchievoUri.setText(_achievouri)
        self.ui.editUsername.setText(_username)
        self.connect(self.ui, SIGNAL("accepted()"), self.login)
        self.connect(self.ui, SIGNAL("rejected()"), self.cancel)
        self.connect(self.ui, SIGNAL("finished()"), self.cancel)
        self.ui.editPassword.setFocus()

    def login(self):
        debug("login")
        self.settings.setValue("achievouri", QVariant(self.ui.editAchievoUri.text()))
        self.settings.setValue("username", QVariant(self.ui.editUsername.text()))
        auth = [self.ui.editAchievoUri.text()]
        auth += [self.ui.editUsername.text()]
        auth += [self.ui.editPassword.text()]
        self.emit(SIGNAL("login"), auth)
        self.ui.close()

    def cancel(self):
        debug("cancel")
        self.emit(SIGNAL("cancel"))
        self.ui.close()


class TimeBrowseWindow(QMainWindow):
    def __init__(self, config):
        QMainWindow.__init__(self)

        _path = 'pyuac_browse.ui'
        if hasattr(sys, "frozen") and sys.frozen:
            _path = os.path.join(os.path.dirname(sys.executable), _path)

        self.ui = uic.loadUi(_path, self)

        self.login = LoginDialog(self, config)
        self.err = QErrorMessage(self)
        self.projects = None
        self._setupGui()
        self._connectSlots()
        self.login.show()

    def _connectSlots(self):
        self.connect(self.login, SIGNAL("login"),
                     self._login)
        self.connect(self.login, SIGNAL("cancel"),
                     self._slotClose)
        self.connect(self.ui.btnTimereg, SIGNAL("clicked()"),
                     self._slotNewTimereg)
        self.connect(self.ui.btnQuit, SIGNAL("clicked()"),
                     self._slotClose)
        self.connect(self.ui.btnToday, SIGNAL("clicked()"),
                     self._setupGui)
        self.connect(self.ui.btnEdit, SIGNAL("clicked()"),
                     self._slotTimeEdit)
        self.connect(self.ui.dateEdit, SIGNAL("dateChanged(const QDate&)"),
                     self._slotTimereport)
        self.connect(self.ui.tableTimereg, SIGNAL("cellDoubleClicked(int,int)"),
                     self._slotTimeEdit)

    def _setupGui(self):
        """ <-- self.ui.btnToday, SIGNAL("clicked()")
        Reimposta la gui ai volori di default
        (titoli colonne e data attuale)
        """
        self.ui.tableTimereg.setColumnCount(5)
        for c, head in enumerate("Date Project/Phase Activity Time Remark".split()):
            cellHead = QTableWidgetItem(head)
            self.ui.tableTimereg.setHorizontalHeaderItem(c, cellHead)
        self.ui.tableTimereg.horizontalHeader().setStretchLastSection(True)
        if self.ui.dateEdit.date() != QDate.currentDate():
            self.ui.dateEdit.setDate(QDate.currentDate())

    def _login(self, auth):
        """ <-- self.login, SIGNAL("login")
        Riceve i valori inseriti nella form di login e completa l'avvio
        """
        debug("_login")
        self.remote = QRemoteTimereg(self, auth)
        self.edit = TimeregWindow(self, auth)
        self._connectRemote()
        self._slotTimereport(QDate.currentDate())
        self.ui.show()

    def _connectRemote(self):
        """
        Connette gli ultimi slot una volta noti i dati di autenticazione
        """
        # Short-circuit Signals (from python to python)
        self.connect(self.edit, SIGNAL("registrationDone"),
                     self._slotRegistrationDone)
        self.connect(self.remote, SIGNAL("timereportStarted"),
                     self._slotTimereportStarted)
        self.connect(self.remote, SIGNAL("timereportOK"),
                     self._slotUpdateTimereport)
        self.connect(self.remote, SIGNAL("processError"),
                     self._slotProcessError)

    def _slotNewTimereg(self):
        """ <-- self.ui.btnTimereg, SIGNAL("clicked()")
        Imposta la data selezionata nel template ed
        avvia la finestra di inserimento nuova registrazione
        """
        selected_date = unicode(self.ui.dateEdit.date().toString("yyyy-MM-dd"))
        project_template = AchievoProject()
        project_template.set("activitydate", selected_date)
        self.edit.setupEdit(project_template.data)
        self.edit.show()

    def _slotClose(self):
        """ <-- self.ui.btnQuit, SIGNAL("clicked()")
            <-- self.login, SIGNAL("cancel")
        Chiude l'applicazione provvedendo a terminare tutti i processi
        """
        self.notify(self.tr("Closing..."))
        if "remote" in dir(self):
            self.remote.close()
            self.edit.remote.close()
        self.ui.close()

    def _slotTimeEdit(self, row=None, column=None):
        """ <-- self.ui.btnEdit, SIGNAL("clicked()")
                self.ui.tableTimereg, SIGNAL("cellDoubleClicked(int,int)")
        Prepara un template con i dati della riga selezionata
        ed avvia la form di modifica
        """
        if row == None:
            row = self.ui.tableTimereg.currentRow()
        project_template = AchievoProject()
        print self.projects[row].items()
        for k in project_template.keys:
            project_template.set("in_%s" % k, self.projects[row].get(k))
        for k in "id activitydate".split():
            project_template.set(k, self.projects[row].get(k))
        #debug("_slotTimeEdit: %s" % project_template)
        self.edit.setupEdit(project_template.data)
        self.edit.show()

    def _slotRegistrationDone(self, eresp):
        """ <-- self.edit, SIGNAL("registrationDone")
        Invocato al termine di una registrazione
        aggiorna la finestra
        """
        #debug("_slotRegistrationDone %s" % eresp.items())
        newdate = QDate.fromString(str(eresp.get("activitydate")), "yyyy-MM-dd")
        if newdate != self.ui.dateEdit.date():
            self.ui.dateEdit.setDate(newdate)
        self._slotTimereport(newdate)

    def _slotTimereport(self, qdate):
        """ <-- self.ui.dateEdit, SIGNAL("dateChanged(const QDate&)")
        Avvia la query per aggiornare il contenuta della tabella
        """
        reportdate = qdate.toString("yyyy-MM-dd")
        self.notify(self.tr("Searching..."))
        self.remote.timereport(date=reportdate)

    def _slotTimereportStarted(self):
        self.ui.btnEdit.setEnabled(False)
        self.ui.btnTimereg.setEnabled(False)
        self.ui.tableTimereg.setRowCount(0)

    def _slotUpdateTimereport(self, eprojects):
        """ <-- self.remote, SIGNAL("timereportOK")
        Aggiorna la tabella delle ore registrate
        con la lista dei progetti restituiti da *remote*
        Ha il side-effect di convertire time (minuti) in hmtime (ore:minuti)
        """
        #debug("_slotUpdateTimereport")
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
        self.ui.btnEdit.setEnabled(len(eprojects) != 0)
        self.ui.btnTimereg.setEnabled(True)

    def _slotProcessError(self, qperror, exitcode):
        """ <-- self.remote, SIGNAL("processError")
        Visualizza un messaggio di errore
        """
        #debug("_slotProcessError %s, %s" % (qperror, exitcode), "warning")
        if exitcode == "RESPONSE_ERROR":
            self.login.show()
        else:
            self.err.showMessage(self.tr("Error contacting Achievo:\n") +
                                 "%s, %s" % (qperror, exitcode))

    def notify(self, msg, timeout=0):
        """
        Visualizza un messaggio nella barra di stato
        """
        self.ui.statusBar.showMessage(msg, timeout)

