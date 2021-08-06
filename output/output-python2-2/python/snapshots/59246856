#!/usr/bin/python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>
#

import urllib, urllib2
from pyuac_utils import *

ACHIEVO_ENCODING = "ISO-8859-1"

class RemoteTimereg:
    """
    RemoteTimereg si interfaccia (in modo sincrono) con il modulo Achievo "remote".
    Sia server che client sono fondamentalmente stateles, l'unico stato è
    l'aver fatto login, condizione obbligatoria per compiere qualsiasi funzione.
    I metodi accettano parametri standard e restituiscono un oggetto ElementTree.
    """

    actions = {"query": "Search the project matching the smartquery",
               "whoami": "Returns login info",
               "timereg": "Register worked time",
               "delete": "Delete the timered by id",
               "timereport": "Report time registered in the provided date"}

    def __init__(self, achievouri, user, password):
        """
        Classe di interfaccia per il modulo Achievo "remote"
        Fornire la path di achievo, username e password
        """
        self.user = user
        self.userid = 0
        self.password = password
        self._achievouri = achievouri
        self._loginurl = urllib.basejoin(self._achievouri, "index.php")
        self._dispatchurl = urllib.basejoin(self._achievouri, "dispatch.php")

        self._smartquery_dict = parseSmartQuery("")
        self._projects = ET.fromstring("<response />")

        self.whoami()

    def _login(self, user=None, password=None):
        """
        Restituisce il nome utente e rinfresca la sessione di Achievo
        """
        if user is not None and password is not None:
            self.user = user
            self.password = password
        auth = urllib.urlencode({"auth_user": self.user,
                                 "auth_pw": self.password})
        self._setupAuth()
        #refresh Achievo session
        urllib2.urlopen(self._loginurl, auth).read()
        return self._urlDispatch("whoami")

    def _setupAuth(self):
        """
        Imposta l'autenticazione http e la gestione dei cookies
        """
        passman = urllib2.HTTPPasswordMgrWithDefaultRealm()
        passman.add_password(None, self._loginurl, self.user, self.password)
        passman.add_password(None, self._dispatchurl, self.user, self.password)
        auth_handler = urllib2.HTTPBasicAuthHandler(passman)
        cookie_handler = urllib2.HTTPCookieProcessor()
        opener = urllib2.build_opener(auth_handler, cookie_handler)
        #installa come opener di default per urllib2
        #FIXME: e se fosse multithread con due auth diverse?
        urllib2.install_opener(opener)

    def _urlDispatch(self, node, action="search", **kwargs):
        """
        Invoca il dispatch.php di Achievo
        """
        params = {"atknodetype": "remote.%s" % node,
                  "atkaction": action}
        #This is the way PHP accepts arrays,
        #without [] it gets only the last value.
        for key, val in kwargs.items():
            if type(val) == list:
                del kwargs[key]
                kwargs[key+"[]"] = [v.encode(ACHIEVO_ENCODING) for v in val]
            else:
                kwargs[key] = val.encode(ACHIEVO_ENCODING)
        qstring = urllib.urlencode(params.items() + kwargs.items(), doseq=True)
        #if __debug__:
            #debug("##### Dispatch: %s" % qstring)
        page = urllib2.urlopen(self._dispatchurl, qstring).read()
        #debug("_urlDispatch " + page)
        return ET.fromstring(page)

    def whoami(self):
        """
        Restituisce il nome utente della sessione attiva
        """
        elogin = self._login()
        if self.userid == 0:
            self.userid = elogin[0].get("id")
        return elogin

    def query(self, smartquery):
        """
        Ottiene la lista dei progetti/fasi/attività coerenti
        con la smart-string inviata, restituisce un ElementTree
        """
        #Se vuota converte in "trova tutto"
        # % permette la ricerca anche all'interno del nome del progetto
        #FIXME: nel sever, SQL INJECT?
        _smartquery_dict = parseSmartQuery(smartquery)
        _ppa = " ".join([_smartquery_dict[k] for k in
                        ["in_prj", "in_pha", "in_act"]])
        _old_ppa = " ".join([self._smartquery_dict[k] for k in
                            ["in_prj", "in_pha", "in_act"]])
        self._smartquery_dict = _smartquery_dict
        # Fa la query al server solo se la parte
        # "project", "phase", "activity" è cambiata
        if _ppa != _old_ppa or (_ppa.strip() == _old_ppa.strip() == ""):
            self._projects = self._urlDispatch("query", input=_ppa)
        #debug("Search results: %s" % ET.tostring(self._projects))
        for p in self._projects[:1]:
            # riempio tutti i campi necessari alla registrazione
            # delle ore lavorate solo nel primo progetto.
            p.text = self._smartquery_dict["in_remark"]
            p.set("in_hmtime",  self._smartquery_dict["in_hmtime"])
            try:
                p.set("hmtime", timeRound(self._smartquery_dict["in_hmtime"]))
            except ValueError:
                p.set("hmtime", "00:00")
        return self._projects

    def timereport(self, date):
        """
        Ottiene la lista delle ore registrate nei giorni
        passati nel parametro date
        """
        return self._urlDispatch("timereport", date=date)

    def timereg(self, projectid, activityid, phaseid, hmtime, activitydate, remark, id=None):
        """
        Registra un blocco di ore lavorate
        """
        kwargs = {"projectid": projectid,
                  "activityid": activityid,
                  "phaseid": phaseid,
                  "time[hours]": hmtime.split(":")[0],
                  "time[minutes]": hmtime.split(":")[1],
                  "activitydate": activitydate,
                  "entrydate": time.strftime("%Y%m%d", time.gmtime()),
                  "remark": remark,
                  "userid": "person.id=%s" % self.userid}
        #TODO: fare in modo che il server prenda userid dalla sessione corrente
        if id == None: #save new record
            epage = self._urlDispatch("timereg", action="save", **kwargs)
        else: #update
            kwargs["id"] = id
            kwargs["atkprimkey"] = "hours.id=%s" % id
            #TODO: scoprire quale dei due viene usato da achievo per l'update
            epage = self._urlDispatch("timereg", action="edit", **kwargs)
        return epage

    def delete(self, id):
        kwargs = {"atkselector": "hours.id=%s" % id,
                  "confirm": "Yes"}
        epage = self._urlDispatch("timereg", action="delete", **kwargs)
        return epage

example_save = """
curl -v -b cookie -c cookie \
        -d auth_user=matteo -d auth_pw=matteo99 \
        http://www.develer.com/~naufraghi/achievo/index.php

curl -v -b cookie -c cookie \
-d atknodetype=remote.timereg \
-d atkaction=save \
-d activityid=2 \
-d projectid=6 \
-d phaseid=6 \
-d time[hours]=5 \
-d time[minutes]=15 \
-d activitydate=20061117 \
-d entrydate=20061117 \
-d remark=remarkfoo%20popo%20popop%20opopop \
-d userid=person.id=1 \
http://www.develer.com/~naufraghi/achievo/dispatch.php

curl -v -b cookie -c cookie \
        http://www.develer.com/~naufraghi/achievo/index.php?atklogout=1
"""

if __name__ == "__main__":
    rl = RemoteTimereg("http://www.develer.com/~naufraghi/achievo/",
                       "matteo", "matteo99")
    rl.whoami()
    rl.search("pr")
    rl.search("pr me")
    rl.search("pr me an")
    rl.timeReport("2006-11-7")
    rl.timeReport(["2006-11-7","2006-11-8"])
