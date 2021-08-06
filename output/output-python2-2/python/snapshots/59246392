#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

from pyuac_utils import *
from libRemoteTimereg import *
import pyuac_cli

from PyQt4 import uic
print __name__, "from PyQt4 import uic"
from PyQt4.QtCore import *
print __name__, "from PyQt4.QtCore import *"
from PyQt4.QtGui import *
print __name__, "from PyQt4.QtGui import *"

class QRemoteTimereg(QObject):
    """
    Classe per la gestione asincrona della libreria RemoteAchievo
    La classe è mappata direttamente sui metodi di RemoteAchievo
    tramite codifica urlencode per la chiamata ed xml per la risposta
    """
    def __init__(self, parent, auth):
        QObject.__init__(self, parent)
        self.process = QProcess(self)
        self._waiting = False
        self._resp = ""
        self.auth = auth
        self._actions_params = {}
        self.connect(self.process, SIGNAL("finished(int)"), self._ready)
        self.connect(self.process, SIGNAL("readyReadStandardOutput()"), self._ready)
        self.connect(self.process, SIGNAL("error(QProcess::ProcessError)"),
                     self._error)

    def __getattr__(self, action):
        """
        Imposta per l'esecuzione le azioni definite in RemoteTimereg
        ed avvia sync()
        """
        if action in RemoteTimereg.actions.keys() + ["q"]:
            def _action(**kwargs):
                self._actions_params[action] = self.encode(action, **kwargs)
                self.sync()
            return _action
        else:
            raise AttributeError

    def close(self):
        self.q()

    def encode(self, action, **kwargs):
        """
        Metodo che codifica il dizionario ricevuto con
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

    def execute(self, qstring):
        """
        Avvia il processo e invia la qstring
        viene invocato da sync()
        """
        if self.process.state() == self.process.NotRunning:
            #debug("execute(%s)" % qstring)
            executable = sys.executable
            params = ["-u"]
            if not __debug__:
                params += ["-O"]
            params += ["pyuac_cli.py"]
            self.process.start(executable, params+self.auth+["--silent"])
        if not self._waiting:
            self.process.write(qstring+"\n")
            self._waiting = True
            return True
        else:
            return False

    def sync(self):
        """
        Provvede ad eseguire le query in attesa
        ed emette i segnali adatti alla query avviata:
            whoamiStarted
            queryStarted
            timeregStarted
            timereportStarted
        """
        debug("%s <!-- Sync -->" % __name__)
        for action, cmdline in self._actions_params.items():
            if self.execute(cmdline):
                del self._actions_params[action]
                self.emit(SIGNAL(action+"Started"))

    def _ready(self, exitcode=0):
        """ <-- SIGNAL("finished(int)"), self._ready
        Provvede a emettere i segnali adatti alla risposta ottenuta:
            whoami[OK|Err](ElemetTree)
            query[OK|Err](ElemetTree)
            timereg[OK|Err](ElemetTree)
            timereport[OK|Err](ElemetTree)
            emptyResponse
        """
        debug("%s <!-- Ready -->" % __name__)
        if exitcode != pyuac_cli.exits.index("OK"):
            self._error(5, exitcode)

        self._resp += str(self.process.readAllStandardOutput())
        if self._resp.find("</response>") == -1:
            return

        if self._resp not in ["", "\n"]:
            try:
                eresp = ET.fromstring(self._resp)
            except ExpatError:
                debug("_ready @@@%s@@@" % self._resp)
                raise
            node = eresp.get("node")
            msg = eresp.get("msg")
            self.emit(SIGNAL(node+msg), eresp)
        else:
            self.emit(SIGNAL("emptyResponse"))
        self._resp = ""
        self._whaiting = False
        #appena il processo ha terminato il lavoro controllo la coda con
        self.sync()

    def _error(self, process_error, exitcode=None):
        """ <-- self.process, SIGNAL("error(QProcess::ProcessError)")
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


