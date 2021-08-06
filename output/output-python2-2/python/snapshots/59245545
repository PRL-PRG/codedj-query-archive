#!/usr/bin/env python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id: QRemoteTimereg2.py 21759 2008-06-18 08:42:23Z duplo $
#
# Author: Matteo Bertini <naufraghi@develer.com>

import sys, os

from pyuac_utils import *
from libRemoteTimereg import *
import pyuac_cli

from PyQt4 import uic
print __name__, "from PyQt4 import uic"
from PyQt4.QtCore import *
print __name__, "from PyQt4.QtCore import *"
from PyQt4.QtGui import *
print __name__, "from PyQt4.QtGui import *"


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


class QAchievoWindow:
    """
    Base class for Achievo GUI
    """
    @staticmethod
    def loadUi(_path, _parent):
        if hasattr(sys, "frozen") and sys.frozen:
            _path = os.path.join(os.path.dirname(sys.executable), _path)
        else:
            _path = os.path.join(os.path.dirname(__file__), _path)
        return uic.loadUi(_path, _parent)

    def __setup__(self, auth=None, _path=None):
        if _path != None:
            self.ui = QAchievoWindow.loadUi(_path, self)
        if auth != None:
            self.remote = QRemoteTimereg(self, auth)
            self.connect(self.remote, SIGNAL("processError"),
                         self._slotProcessError)
        self.err = QErrorMessage(self)
        self.settings = ASettings("Develer", "PyUAC")

    def notify(self, msg, timeout=0):
        """
        Visualizza un messaggio nella barra di stato
        """
        try:
            self.ui.statusBar.showMessage(msg, timeout)
        except AttributeError:
            pass

    def _slotClose(self):
        print "Closing..."
        if "remote" in dir(self):
            self.remote.close()
        self.close()

    def _slotProcessError(self, process_error, exitcode, errstr):
        """ <-- self.remote, SIGNAL("processError")
        Visualizza un messaggio di errore
        """
        if exitcode == "OK":
            self._slotClose()
        else:
            self.emit(SIGNAL("processError"), process_error, exitcode, errstr)
            sep = ["-"*20]
            msg  = [process_error] + sep
            msg += [exitcode] + sep
            msg += [errstr]  
            self.err.showMessage(self.tr("Error contacting Achievo: ") + "\n".join(msg).replace("\n","<br>\n"))


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
        self.login(achievouri=auth[0], user=auth[1], password=auth[2])

    def __getattr__(self, action):
        """
        Imposta per l'esecuzione le azioni definite in RemoteTimereg
        ed avvia sync()
        """
        if action in RemoteTimereg.actions.keys() + ["q"]:
            def _action(**kwargs):
                if action == "timereg":
                    if not self._actions_params[action]:
                        self._actions_params[action] = []
                    self._actions_params[action].append(self._encode(action, **kwargs))
                else:
                    self._actions_params[action] = self._encode(action, **kwargs)
                self._sync()
            return _action
        else:
            raise AttributeError

    def close(self):
        self.q()

    def _encode(self, action, **kwargs):
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

    def _execute(self, qstring):
        """
        Avvia il processo e invia la qstring.
        Viene invocato da sync()
        """
        if self.process.state() == self.process.NotRunning:
            if not hasattr(sys, "frozen") or not sys.frozen:
                executable = sys.executable
                params = []
                if not __debug__:
                    params += ["-O"]
                pyuac_cli = os.path.join(os.path.dirname(__file__), "pyuac_cli.py")
                params += [pyuac_cli]
                self.process.start(executable, params+["--silent"])
            else:
                executable = os.path.join(os.path.dirname(sys.executable), "pyuac_cli")
                params = ["--silent"]
                self.process.start(executable, params)
        if not self._waiting:
            self.process.write(qstring+"\n")
            self._waiting = True
            return self._waiting
        else:
            return False

    def _sync(self):
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
            if action == "timereg":
                for cmd in cmdline:
                    if self._execute(cmdline):
                        self.emit(SIGNAL(action+"Started"))
                del self._actions_params[action]
            else:
                if self._execute(cmdline):
                    del self._actions_params[action]
                    self.emit(SIGNAL(action+"Started"))

    def _ready(self, exitcode=None):
        """ <-- self.process, SIGNAL("finished(int)")
            <-- self.process, SIGNAL("readyReadStandardOutput()")
        Provvede a emettere i segnali adatti alla risposta ottenuta:
            whoami[OK|Err](ElemetTree)
            query[OK|Err](ElemetTree)
            timereg[OK|Err](ElemetTree)
            timereport[OK|Err](ElemetTree)
            emptyResponse
        """
        debug("%s <!-- Ready(%s) -->" % (__name__, exitcode))
        if exitcode != None:
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
        self._waiting = False
        #appena il processo ha terminato il lavoro controllo la coda con
        self._sync()

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
        errstr = str(self.process.readAllStandardError())
        msg  = ["Err(%s, %s):" % (process_error, exitcode)]
        msg += ["-"*20]
        msg += [errstr]
        debug("\n".join(msg))
        self.emit(SIGNAL("processError"), process_error, exitcode, errstr)
