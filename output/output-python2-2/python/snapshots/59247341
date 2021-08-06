#!/usr/bin/python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import urllib, urllib2, re
try:
    from xml.etree import ElementTree as ET
except ImportError:
    try:
        from elementtree import ElementTree as ET
    except ImportError:
        raise ImportError, "ElementTree (or py2.5) needed"

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
        self.search("%")
        
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
        try:
            return urllib2.urlopen(self._dispatchurl, qstring).read()
        #FIXME: scoprire che eccezione solleva...
        #       oppure no tanto richiamo la stessa funzione subito
        except: 
            self._login() #Achievo session timeout?
            return urllib2.urlopen(self._dispatchurl, qstring).read()
        
    def whoami(self):
        """
        Restituisce il nome utente della sessione attiva
        """
        page = self._login()
        elogin = ET.fromstring(page)
        if elogin.find("record").get("name") == self.user:
            return self.user
        else:
            return False

    def parseSmartQuery(self, smartquery):
        getsq = re.compile("""
            (?P<project>[^ ]+|)\ *
            (?P<phase>[^ ]+|)\ *
            (?P<activity>[^ ]+|)\ *
            (?P<hours>\d{1,2}:\d{1,2}|)\ *
            (?P<comment>.*|)
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
                            ["project", "phase", "activity"]))
        _old_ppa = " ".join(map(self._smartquery.get,
                            ["project", "phase", "activity"]))
        self._smartquery = _smartquery
        # Fa la query al server solo se la parte
        # "project", "phase", "activity" è cambiata
        if _ppa != _old_ppa:
            page = self.urlDispatch("query", input="%"+_ppa)
            self._projects = ET.fromstring(page)
        res = []
        for project in self._projects:
            project.set("hours", _smartquery["hours"])
            project.set("comment", _smartquery["comment"])
            res.append(project)
        if __debug__:
            print map(ET.tostring, res)
        return res
    
    def timeReport(self, date):
        """
        Ottiene la lista delle ore registrate nei giorni
        passati nel parametro date
        """
        page = self.urlDispatch("timereport", date=date)
        etimeregs = ET.fromstring(page)
        res = []
        for timereg in etimeregs:
            res.append(timereg)
        if __debug__:
            print map(ET.tostring, res)
        return res
    
if __name__ == "__main__":
    rl = RemoteTimereg("http://www.develer.com/~naufraghi/achievo/",
                       "matteo", "matteo99")
    print rl.whoami() and "Login OK" or "Login Error!"
    rl.search("pr")
    rl.search("pr me")
    rl.search("pr me an")
    rl.timeReport("2006-11-7")
    rl.timeReport(["2006-11-7","2006-11-8"])
