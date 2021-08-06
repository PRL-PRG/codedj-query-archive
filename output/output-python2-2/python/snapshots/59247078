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

import urllib, urllib2, re, time, datetime, sys, logging, logging.config, copy
from xml.parsers.expat import ExpatError
from htmlentitydefs import entitydefs

try:
    from xml.etree import ElementTree as ET
except ImportError:
    try:
        from elementtree import ElementTree as ET
    except ImportError:
        raise ImportError, "ElementTree (or py2.5) needed"

logging.config.fileConfig("logger.ini")
log = logging.getLogger("pyuac.lib")
#TODO: risolvere il problema dei log contemporanei del processo
#      principale e di quello in QProcess. Entrambi caricano libRemoteTimereg
#      e quindi entrambi caricano il logger "pyuac.lib"

def timeRound(inTime, stepTime=15):
    """
    Arrotonda la stringa hh:mm alla risoluzione inviata
    e restituisce un oggetto timedelta:

    >>> timeRound("2:44")
    '02:45'
    >>> timeRound("13:10", 20)
    '13:20'
    """
    inTime_tuple = time.strptime(inTime, "%H:%M")
    step = datetime.timedelta(minutes=stepTime)
    if step == datetime.timedelta():
        step = datetime.timedelta(minutes=1)
    pre = datetime.timedelta(hours=inTime_tuple.tm_hour,
                             minutes=inTime_tuple.tm_min)
    res = int(round(pre.seconds/float(step.seconds))*step.seconds)
    return "%02d:%02d" % (res/3600, (res%3600)/60)

def msgParse(msg, encoding="utf-8"):
    return ET.fromstring(msg.encode(encoding))

def emsgDump(emsg, encoding="utf-8"):
    return ET.tostring(emsg, encoding)

class RemoteTimereg:
    """
    RemoteTimereg si interfaccia (in modo sincrono) con il modulo Achievo "remote".
    Sia server che client sono fondamentalmente stateles, l'unico stato è
    l'aver fatto login, condizione obbligatoria per compiere qualsiasi funzione.
    I metodi accettano parametri standard e restituiscono un oggetto ElementTree.
    """
    def __init__(self, achievouri, user, password):
        """
        Classe di interfaccia per il modulo Achievo "remote"
        """
        self.user = user
        self.userid = 0
        self.password = password
        self._achievouri = achievouri
        self._loginurl = urllib.basejoin(self._achievouri, "index.php")
        self._dispatchurl = urllib.basejoin(self._achievouri, "dispatch.php")

        self._smartquery = self.parseSmartQuery("")
        self._projects = msgParse("<response />")
        
        self.whoami()
        #self.search("%")
        
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
                kwargs[key+"[]"] = val
        qstring = urllib.urlencode(params.items() + kwargs.items(), doseq=True)
        if __debug__:
            log.debug("########### Dispatch: %s" % qstring)
        page = urllib2.urlopen(self._dispatchurl, qstring).read().decode("utf-8")
        log.debug("_urlDispatch " + page)
        return page

    def whoami(self):
        """
        Restituisce il nome utente della sessione attiva
        """
        page = self._login()
        elogin = self.parse(page)
        if self.userid == 0:
            self.userid = elogin[0].get("id")
        return elogin

    def parseSmartQuery(self, smartquery):
        getsq = re.compile("""
            (?P<input_project>[^ ]+|)\ *
            (?P<input_phase>[^ ]+|)\ *
            (?P<input_activity>[^ ]+|)\ *
            (?P<input_hours>\d{1,2}:\d{1,2}|)\ *
            (?P<input_remark>.*|)
            """, re.VERBOSE)
        res = getsq.search(smartquery).groupdict()
        log.debug("parseSmartQuery: %s" % res)
        return res
         
    def search(self, smartquery="%"):
        """
        Ottiene la lista dei progetti/fasi/attività coerenti
        con la smart-string inviata,
        restituisce un AchievoTimereg
        """
        #Se vuota converte in "trova tutto"
        # % permette la ricerca anche all'interno del nome del progetto
        #FIXME: nel sever, SQL INJECT?
        if __debug__:
            #simula la latenza di internet
            import time
            time.sleep(1)
        _smartquery = self.parseSmartQuery(smartquery)
        _ppa = " ".join(map(_smartquery.get,
                            ["input_project", "input_phase", "input_activity"]))
        _old_ppa = " ".join(map(self._smartquery.get,
                            ["input_project", "input_phase", "input_activity"]))
        self._smartquery = _smartquery
        # Fa la query al server solo se la parte
        # "project", "phase", "activity" è cambiata
        if _ppa != _old_ppa:
            page = self._urlDispatch("query", input=_ppa)
            self._projects = self.parse(page)
        log.debug("Search results: %s" % emsgDump(self._projects))
        if len(self._projects) == 1:
            self._prepareTimereg()
        return self._projects

    def _prepareTimereg(self):
        """
        Se ho un solo progetto, posso riempire tutti i campi necessari alla
        registrazione delle ore lavorate.
        """
        project = self._projects[0]
        #TODO: non ci sono già in project?
        project.set("input_hours", self._smartquery["input_hours"])
        project.set("input_remark", self._smartquery["input_remark"].decode("utf-8"))
        # il remark potrebbe essere filtrato in futuro, ad esempio permettendo
        # di creare todo o appuntamenenti
        project.set("remark", self._smartquery["input_remark"].decode("utf-8"))
        project.set("hmtime",  timeRound(self._smartquery["input_hours"] or "0:00"))
    
    def timereport(self, date):
        """
        Ottiene la lista delle ore registrate nei giorni
        passati nel parametro date
        """
        page = self._urlDispatch("timereport", date=date)
        return self.parse(page)

    def timereg(self, projectid, activityid, phaseid, hmtime, activitydate, remark, id=0):
        args = {"projectid": projectid,
                "activityid": activityid,
                "phaseid": phaseid,
                "time[hours]": hmtime.split(":")[0],
                "time[minutes]": hmtime.split(":")[1],
                "activitydate": activitydate,
                "entrydate": time.strftime("%Y%m%d", time.gmtime()),
                "remark": remark,
                "userid": "person.id=%s" % self.userid}
        if id == 0: #save new record
            page = self._urlDispatch("timereg", action="save", **args)
        else: #update
            args["id"] = id
            args["atkprimkey"] = "hours.id=%s" % id
            page = self._urlDispatch("timereg", action="edit", **args)
        return self.parse(page)

    def parse(self, message):
        try:
            return msgParse(message)
        except:
            log.error(message)
            raise

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
