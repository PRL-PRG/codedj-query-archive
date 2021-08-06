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

try:
    from xml.etree import ElementTree as ET
except ImportError:
    try:
        from elementtree import ElementTree as ET
    except ImportError:
        raise ImportError, "ElementTree (or py2.5) needed"

import libRemoteTimereg, pyuac_cli
log = logging.getLogger("pyuac.gui")

def debug(msg):
    if __debug__:
        log.debug("%s.%s" % (__name__, msg))
        print __name__, msg

class RemoteTimereg(QObject):
    """
    Classe per la gestione asincrona della libreria RemoteAchievo
    La classe è mappata direttamente sui metodi di RemoteAchievo
    tramite codifica urlencode per la chiamata ed xml per la risposta
    """
    def __init__(self, parent, auth):
        QObject.__init__(self, parent)
        self.process = QProcess(self)
        self.auth = auth
        self._actions_params = {}
        self.connect(self.process, SIGNAL("finished(int)"), self._ready)
        self.connect(self.process, SIGNAL("error(QProcess::ProcessError)"),
                     self._error)

    def _close(self):
        #blocco in chiusura ed aspetto la terminazione del processo
        self.process.waitForFinished()

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
        qstring = urllib.urlencode(kwargs, doseq=True)
        #debug("_encode "+qstring)
        return action + "?" + qstring

    def _execute(self, qstring):
        """
        Avvia il processo e invia la qstring
        """
        if self.process.state() == self.process.NotRunning:
            debug("_execute(%s)" % qstring)
            executable = sys.executable
            params = ["-u"]
            if not __debug__:
                params += ["-O"]
            params += ["pyuac_cli.py"]
            #debug("Executing: %s" % " ".join([executable]+self.auth+["--oneshot"]))
            self.process.start(executable, params+self.auth+["--oneshot"])
            self.process.write(qstring+"\n")
            return True
        else:
            return False

    def __getattr__(self, action):
        debug(action)
        if action in libRemoteTimereg.RemoteTimereg.actions:
            def _action(**kwargs):
                self._actions_params[action] = self._encode(action, **kwargs)
                self._sync()
            return _action
        else:
            raise AttributeError

    def _sync(self):
        """
        Provvede ad eseguire le query in attesa
        ed emette i segnali adatti alla query avviata:
            whoamiStarted
            queryStarted
            timeregStarted
            timereportStarted
        """
        debug("Sync")
        for action, cmdline in self._actions_params.items():
            if self._execute(cmdline):
                del self._actions_params[action]
                self.emit(SIGNAL(action+"Started"))

    def _ready(self, exitcode):
        """
        Provvede a emettere i segnali adatti alla risposta ottenuta:
            whoami[OK|Err](ElemetTree)
            query[OK|Err](ElemetTree)
            timereg[OK|Err](ElemetTree)
            timereport[OK|Err](ElemetTree)
            emptyResponse
        """
        debug("Ready")
        if exitcode != pyuac_cli.exits.index("OK"):
            self._error(5, exitcode)
        resp = str(self.process.readAllStandardOutput())
        if resp != "":
            eresp = ET.fromstring(resp)
            node = eresp.get("node")
            msg = eresp.get("msg")
            self.emit(SIGNAL(node+msg), eresp)
        else:
            self.emit(SIGNAL("emptyResponse"))
        self._sync()

    def _error(self, process_error, exitcode=None):
        """
        Emette processError con parametri:
            QProcess::ProcessError decodificato come stringa
            pyuac_cli.error decodificato come stringa
        """
        process_errors = "FailedToStart Crashed Timedout WriteError ReadError UnknownError"
        process_error = process_errors.split()[process_error]
        if exitcode != None:
            exitcode = pyuac_cli.exits[exitcode]
        debug("Err(%s, %s): %s" % (process_error, exitcode, str(self.process.readAllStandardError())))
        self.emit(SIGNAL("processError"), process_error, exitcode)


