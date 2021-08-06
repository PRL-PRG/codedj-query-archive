#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import sys, urllib, logging
from PyQt4 import uic
from PyQt4.QtCore import *
from PyQt4.QtGui import *

import libRemoteTimereg
log = logging.getLogger("pyuac.gui")

def debug(msg):
    if __debug__:
        qDebug("#-#-#-#-# "+msg.replace(r"%%", r"%").replace(r"%", r"%%"))

class RemoteTimereg(QObject):
    """
    Classe per la gestione asincrona della libreria RemoteAchievo
    La classe espone due metodi:
    * search(smartquery)
      che emette searchStarted() e searchDone(PyObject *)
      searchDone restituiesce un oggetto ElementTree
    * timereg(**kwargs)
      accetta un dizionario di valori necessari per registrare
      un blocco di ore lavorate
      emette timeregStarted() e timeregDone()
    """
    def __init__(self, parent, auth):
        QObject.__init__(self, parent)
        self.process = QProcess(self)
        self.timer = QTimer(self)
        self.auth = auth
        self._query = ""
        self._timereg = ""
        self._oops = 0
        self.connect(self.timer, SIGNAL("timeout()"), self._sync)
        self.connect(self.process, SIGNAL("finished(int)"), self._ready)

    def _encode(self, action, **kwargs):
        """
        metodo che codifica il dizionario ricevuto con
        urllib.urlencode() e restituisce una stringa
          action?param1=var1&param2=var2
        compatibile con il metodo
        cgi.parse_qs[l]() che restituisce il dizionario
        originale
        """
        for k, v in kwargs.items():
            kwargs[k] = unicode(v).strip().encode("utf-8") #se v è un QString
        qstring = urllib.urlencode(kwargs)
        debug("**"+qstring)
        return action + "?" + qstring

    def _execute(self, qstring):
        """
        Avvia il processo e invia la qstring
        Se execute viene rinviata NN volte, termina il processo in esecuzione
        """
        if self.process.state() == self.process.NotRunning:
            debug("_execute(%s) %s" % (self.process.state(), qstring))
            self.process.start("./pyuac_cli.py", self.auth+["--"])
            self.process.write(qstring+"\n")
            self._oops = 0
            return True
        else:
            self._oops += 1
            if self._oops > 20:
                debug("terminate")
                self.process.terminate()
                self._oops = 0
            return False

    def search(self, query):
        debug("Search")
        self._query = self._encode("search", smartquery=query)
        self._sync()

    def timereg(self, **kwargs):
        debug("Timereg")
        self._timereg = self._encode("timereg", **kwargs)
        self._sync()

    def _sync(self):
        """
        Provvede ad eseguire le query in attesa
        ed emette i segnali adatti alla query avviata
        """
        debug("Sync")
        if self._query != "" and self._execute(self._query):
            self._query = ""
            self.emit(SIGNAL("searchStarted()"))
        elif self._timereg != "" and self._execute(self._timereg):
            self._timereg = ""
            self.emit(SIGNAL("timeregStarted()"))
        else:
            pass

    def _ready(self):
        """
        Provvede a emettere i segnali adatti alla risposta ottenuta
        """
        debug("Ready")
        resp = str(self.process.readAllStandardOutput()).decode("utf-8")
        debug("Err: "+str(self.process.readAllStandardError()))
        if resp == "":
            return
        self.eresp = libRemoteTimereg.msgParse(resp)
        node = self.eresp.get("node")
        msg = self.eresp.get("msg")
        if node == "query":
            self.emit(SIGNAL("searchDone(PyObject *)"), self.eresp)
        elif node == "timereg" and msg == "OK":
            self.emit(SIGNAL("timeregDone()"))
        else:
            self.emit(SIGNAL("timeregError()"))
        self._sync()


