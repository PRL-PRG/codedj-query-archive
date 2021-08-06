#!/usr/bin/python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import urllib, urllib2, re, time, datetime
try:
    from xml.etree import ElementTree as ET
except ImportError:
    try:
        from elementtree import ElementTree as ET
    except ImportError:
        raise ImportError, "ElementTree (or py2.5) needed"

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

class RemoteTimereg:
    def __init__(self, achievouri, user, password):
        """
        Classe di interfaccia per il modulo Achievo "remote"
        """
        self.user = user
        self.password = password
        self._achievouri = achievouri
        self._loginurl = urllib.basejoin(self._achievouri, "index.php")
        self._dispatchurl = urllib.basejoin(self._achievouri, "dispatch.php")
        self._login()

        self._smartquery = self.parseSmartQuery("")
        self._projects = []
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
        return self.urlDispatch("whoami")
    
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
        
    def urlDispatch(self, node, action="search", **kwargs):
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
            print "########### Dispatch:", qstring
        return urllib2.urlopen(self._dispatchurl, qstring).read()
        
    def whoami(self):
        """
        Restituisce il nome utente della sessione attiva
        """
        page = self._login()
        elogin = ET.fromstring(page)
        if elogin.find("record").get("name") == self.user:
            return elogin
        else:
            return False

    def parseSmartQuery(self, smartquery):
        getsq = re.compile("""
            (?P<input_project>[^ ]+|)\ *
            (?P<input_phase>[^ ]+|)\ *
            (?P<input_activity>[^ ]+|)\ *
            (?P<input_hours>\d{1,2}:\d{1,2}|)\ *
            (?P<input_remark>.*|)
            """, re.VERBOSE)
        return getsq.search(smartquery).groupdict()
         
    def search(self, smartquery):
        """
        Ottiene la lista dei progetti/fasi/attività coerenti
        con la smart-string inviata
        """
        #Se vuota converte in "trova tutto"
        # % permette la ricerca anche all'interno del nome del progetto
        #FIXME: nel sever, SQL INJECT?
        if __debug__:
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
            page = self.urlDispatch("query", input=_ppa)
            self._projects = ET.fromstring(page)
        for project in self._projects:
            project.set("input_hours", _smartquery["input_hours"])
            project.set("input_remark", _smartquery["input_remark"])
        if __debug__:
            print ET.tostring(self._projects)
        return self._projects
    
    def timeReport(self, date):
        """
        Ottiene la lista delle ore registrate nei giorni
        passati nel parametro date
        """
        page = self.urlDispatch("timereport", date=date)
        etimeregs = ET.fromstring(page)
        if __debug__:
            print ET.tostring(etimeregs)
        return etimeregs

    def timeReg(self, aTimereg):
        pass
    

example_save = """
curl -v -b cookie -c cookie \
-d atknodetype=timereg.hours \
-d atkaction=save \
-d activityid=0 \
-d projectid=1 \
-d phaseid=3 \
-d time[hours]=4 \
-d time[minutes]=0 \
-d activitydate=20050515 \
-d entrydate=20050515 \
-d remark=remarkfoo \
-d userid=person.id=0 \
http://localhost/dispatch.php
"""

class AchievoTimereg:
    def __init__(self):
        self.activityid = None
        self.projectid = None
        self.phaseid = None
        self.time_hours = None
        self.time_minutes = None
        self.activitydate = None
        self.entrydate = None
        self.remark = None
        self.userid = None

if __name__ == "__main__":
    rl = RemoteTimereg("http://www.develer.com/~naufraghi/achievo/",
                       "matteo", "matteo99")
    print rl.whoami() and "Login OK" or "Login Error!"
    rl.search("pr")
    rl.search("pr me")
    rl.search("pr me an")
    rl.timeReport("2006-11-7")
    rl.timeReport(["2006-11-7","2006-11-8"])
