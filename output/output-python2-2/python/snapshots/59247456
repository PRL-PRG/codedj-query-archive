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
#print __name__, "from PyQt4 import uic"
from PyQt4.QtCore import *
#print __name__, "from PyQt4.QtCore import *"
from PyQt4.QtGui import *
#print __name__, "from PyQt4.QtGui import *"


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
        #print "Closing..."
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
        self.auth = auth
        #dizionario con tipo di richiesta come chiave, e lista di azioni come valore
        self._pending_requests = {}
        #None se non ci sono azioni in esecuzione, altrimenti contiene il tipo
        #della richiesta e l'indice dell'azione a cui il processo è arrivato
        self._current_action = None
        #contiene una lista delle risposte inviate dal server
        self._response = []
        #contiene una stringa che è parte di una risposta ottenuta da un processo.
        self._resp = ""
        #contiene un booleano che indica se QRemoteTimereg è in attesa di risposta
        #dal processo
        self._waiting = False
        self.connect(self.process, SIGNAL("finished(int)"), self._ready)
        self.connect(self.process, SIGNAL("readyReadStandardOutput()"), self._ready)
        self.connect(self.process, SIGNAL("error(QProcess::ProcessError)"),
                     self._error)
        self.login([{"achievouri": auth[0], "user": auth[1], "password": auth[2]}])

    def __getattr__(self, request):
        """
        Imposta per l'esecuzione le azioni definite in RemoteTimereg
        ed avvia sync()
        """
        if request in RemoteTimereg.actions.keys() + ["q"]:
            def _request(request_pack=[]):
                #controlla se è presente una richiesta dello stesso tipo tra le
                #richieste pendenti
                if request in self._pending_requests.keys():
                    #controlla se quella in esecuzione è dello stesso tipo
                    if self._current_action != None and self._current_action[0] == request:
                        #resetta la risposta e blocca la richiesta
                        self._response = []
                        self._current_action = None
                        print "QRemoteTimereg " + request + " abortita"
                #in qualsiasi caso alla fine aggiunge la nuova richiesta al dizionario
                print "QRemoteTimereg " + request + " accodata..."
                self._pending_requests[request] = request_pack
                #se il processo non ha richieste in esecuzione o non aspetta risposte
                #viene iniziata una nuova scansione delle richieste pendenti
                if not self._current_action and not self._waiting:
                    self._sync()
            return _request
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
        return action + "?" + qstring

    def _execute(self):
        """
        Avvia il processo e invia la stringa di richiesta.
        Viene invocato da sync() e da ready().
        """
        #controlla se sono presenti azioni da eseguire
        if self._current_action:
            if len(self._pending_requests[self._current_action[0]]) > self._current_action[1]:
                #avvia il processo e scrive il comando
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
                #costruisce la stringa da inviare al processo a partire dalle
                #informazioni presenti in pending_requests e in current_action
                qstring = self._encode(self._current_action[0],
                                       **self._pending_requests[self._current_action[0]]
                                       [self._current_action[1]])
                self._current_action = (self._current_action[0], self._current_action[1] + 1)
                print "QRemoteTimereg " + self._current_action[0] + " mandata in esecuzione al processo"
                print "QRemoteTimereg._waiting = " + str(self._waiting)
                self.process.write(qstring+"\n")
                #setta waiting a true per indicare che stiamo aspettando un
                #messaggio dal processo
                self._waiting = True
            else:
                #nel caso siano terminate le azioni viene emesso il segnale di
                #terminazione e viene restituita la risposta, dopodiché ripulisce
                #tutto e scansiona per altre richieste chiamando la sync()
                print "QRemoteTimereg " + self._current_action[0] + " terminata..."
                print self._response
                self.emit(SIGNAL(self._current_action[0] + "OK"), self._response)
                del self._pending_requests[self._current_action[0]]
                self._current_action = None
                self._response = []
                self._sync()
        else:
            #nel caso current_action sia settato a None vuol dire che l'azione corrente
            #è stata abortita, perché è giunta una richiesta più nuova dello stesso tipo,
            #quindi viene richiamato il metodo sync() per cercare ulteriori altre richieste
            self._sync()

    def _sync(self):
        """
        Provvede ad eseguire le query in attesa
        ed emette i segnali adatti alla query avviata:
            whoamiStarted
            queryStarted
            timeregStarted
            timereportStarted
        """
        if self._pending_requests.keys() != []:
            action = self._pending_requests.keys().pop(0)
            self._current_action = (action, 0)
            self.emit(SIGNAL(action+"Started"))
            self._execute()    
    
    def _ready(self, exitcode=None):
        """
        Slot collegato al QProcess, viene attivato quando il QProcess finisce la
        sua esecuzione, per leggere la risposta e archiviarla sotto forma di
        albero o sotto forma di stringa vuota nel caso di risposta vuota. Finita
        l'archiviazione del messaggio di risposta questo metodo va a richiamare
        _execute() per proseguire con la lista di richieste.
        """
        if exitcode != None:
            self._error(5, exitcode)
        #accoda la risposta parziale appena letta nella variabile _resp
        self._resp += str(self.process.readAllStandardOutput())
        if self._current_action != None:
            #se non trova il tag di chiusura della response non fa niente mentre
            #se lo trova crea un albero dall'xml e lo appende alla lista _response
            if self._resp.find("</response>") == -1:
                print "QRemoteTimereg._ready() risposta incompleta"
                return
            #QRemoteTimereg non è più in attesa di una risposta dal processo
            self._waiting = False
            try:
                eresp = ET.fromstring(self._resp)
            except ExpatError:
                self._resp = ""
                raise
            node = eresp.get("node")
            msg = eresp.get("msg")
            self._response.append(eresp)
            #cancella la variabile contenente la risposta
            self._resp = ""
            self._execute()
        #nel caso la richiesta sia stata interrotta setta a False _waiting, azzera
        #la risposta parziale e richiama la sync()
        else:
            #se la richiesta è stata abortita elimina la risposta parziale e
            #setta _waiting a False poiché QRemoteTimereg non aspetta più la risposta.
            print "QRemoteTimereg pulizia self._resp"
            self._resp = ""
            self._waiting = False
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
        #debug("\n".join(msg))
        self.emit(SIGNAL("processError"), process_error, exitcode, errstr)
