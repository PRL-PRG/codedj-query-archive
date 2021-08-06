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
from QTimeregWindow import TimeregWindow

log = logging.getLogger("pyuac.gui")

def debug(msg):
    if __debug__:
        print __name__, msg
        log.debug("%s.%s" % (__name__, msg))

class LoginDialog(QDialog):
    def __init__(self, parent, config):
        QDialog.__init__(self, parent)
        self.ui = uic.loadUi("pyuac_auth.ui", self)
        self.ui.editAchievoUri.setText(config["achievouri"])
        self.ui.editUsername.setText(config["username"])
        self.connect(self.ui.buttonBox, SIGNAL("accepted()"), self.login)
        self.connect(self.ui.buttonBox, SIGNAL("rejected"), self.cancel)

    def login(self):
        auth = [self.ui.editAchievoUri.text()]
        auth += [self.ui.editUsername.text()]
        auth += [self.ui.editPassword.text()]
        self.emit(SIGNAL("login"), auth)
        self.close()
        
    def cancel(self):
        self.emit(SIGNAL("cancel"))
        self.ui.close()

class TimeBrowseWindow(QMainWindow):
    def __init__(self, config):
        QMainWindow.__init__(self)
        self.ui = uic.loadUi("pyuac_browse.ui", self)
        self.login = LoginDialog(self, config)
        self.login.show()
        self.err = QErrorMessage(self)
        self.projects = None
        self._connectSlots()

    def _connectSlots(self):
        self.connect(self.login, SIGNAL("login"),
                     self._login)
        self.connect(self.login, SIGNAL("cancel"),
                     self._close)
        self.connect(self.ui.btnTimereg, SIGNAL("clicked()"),
                     self._timereg)
        self.connect(self.ui.btnQuit, SIGNAL("clicked()"),
                     self._close)
        self.connect(self.ui.btnToday, SIGNAL("clicked()"),
                     self._setupGui)
        self.connect(self.ui.btnEdit, SIGNAL("clicked()"),
                     self._timeedit)
        self.connect(self.ui.dateEdit, SIGNAL("dateChanged(const QDate&)"),
                     self._timereport)
        self.connect(self.ui.tableTimereg, SIGNAL("cellDoubleClicked(int,int)"),
                     self._timeedit)

    def _login(self, auth):
        #TODO: la classe Timereg si "aspetta" un attributo *remote*
        #      nella classe parent... non é tanto carino
        self.remote = RemoteTimereg(self, auth)
        self.edit = TimeregWindow(self)
        self._connectRemote()
        self._setupGui()
    
    def _connectRemote(self):
        # Short-circuit Signals (from python to python)
        self.connect(self.edit, SIGNAL("registrationDone"),
                     self._registrationDone)
        self.connect(self.remote, SIGNAL("timereportOK"),
                     self._updateTimereport)

    def _setupGui(self):
        self.ui.tableTimereg.setColumnCount(5)
        for c, head in enumerate("Date Project/Phase Activity Time Remark".split()):
            cellHead = QTableWidgetItem(head)
            self.ui.tableTimereg.setHorizontalHeaderItem(c, cellHead)
        self.ui.tableTimereg.horizontalHeader().setStretchLastSection(True)
        self.ui.dateEdit.setDateTime(QDateTime.currentDateTime())

    def _timereg(self):
        self.edit.show()
        
    def _close(self):
        self.remote._close()
        self.ui.close()

    def _timeedit(self, row=None, column=None):
        debug("_timeedit Editing projects")
        if row == None:
          row = self.ui.tableTimereg.currentRow()
        self.edit.setupEdit(self.projects[row])
        self.edit.show()

    def _registrationDone(self, eresp):
        debug("_registrationDone %s" % ET.tostring(eresp))
        newdate = QDate.fromString(str(eresp[0].get("activitydate")), "yyyy-MM-dd")
        if newdate != self.ui.dateEdit.date():
            self.ui.dateEdit.setDate(newdate)
        self._timereport(newdate)

    def _timereport(self, qdate):
        reportdate = qdate.toString("yyyy-MM-dd")
        self.remote.timereport(date=reportdate)

    def _updateTimereport(self, eprojects):
        debug("_updateTimereport")
        self.projects = eprojects
        self.ui.tableTimereg.setRowCount(0)
        self.ui.tableTimereg.setRowCount(len(eprojects))
        for r, p in enumerate(eprojects):
            row = []
            row.append(QTableWidgetItem(p.get("activitydate")))
            row.append(QTableWidgetItem("%(project_name)s / %(phase_name)s" %\
                                        dict(p.items())))
            row.append(QTableWidgetItem(p.get("activity_name")))
            hmtime = "%02d:%02d" % (int(p.get("time"))/60, int(p.get("time"))%60)
            p.set("hmtime", hmtime)
            row.append(QTableWidgetItem(hmtime))
            row.append(QTableWidgetItem(p.text))
            for c, cell in enumerate(row):
                self.ui.tableTimereg.setItem(r, c, cell)
                if c != 4:
                    self.ui.tableTimereg.resizeColumnToContents(c)
        self.ui.tableTimereg.resizeRowsToContents()
        self.ui.btnEdit.setEnabled(len(eprojects) != 0)

