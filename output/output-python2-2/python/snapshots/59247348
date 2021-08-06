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
        
    def _login(self, user=None, password=None):
        """
        Restituisce il nome utente e rinfresca la sessione di Achievo
        """
        if user is not None and password is not None:
            self.user = user
            self.password = password
        auth = urllib.urlencode({"auth_user":self.user,"auth_pw":self.password})
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
        urllib2.install_opener(opener)
        
    def urlDispatch(self, node, action="search", **kwargs):
        """
        Invoca il dispatch.php di Achievo
        """
        params = {"atknodetype": "remote.%s" % node,
                  "atkaction": action}
        #This is the way PHP accepts arrays, without [] it gets only the last value.
        for key, val in kwargs.items():
            if type(val) == list:
                del kwargs[key]
                kwargs[key+"[]"] = val
        qstring = urllib.urlencode(params.items() + kwargs.items(), doseq=True)
        if __debug__:
            print "########### Dispatch:", qstring
        try:
            return urllib2.urlopen(self._dispatchurl, qstring).read()
        #TODO: scoprire che eccezione solleva...
        #      oppure no tanto richiamo la stessa funzione subito
        except: 
            self._login() #Achievo session timeout?
            return urllib2.urlopen(self._dispatchurl, qstring).read()
        
    def whoami(self):
        """
        Restituisce il nome utente della sessione attiva
        """
        page = self._login()
        elogin = ET.fromstring(page)
        if elogin.find("record").get('name') == self.user:
            return self.user
        else:
            return False
        
    def search(self, input):
        """
        Ottiene la lista dei progetti/fasi/attività coerenti
        con la smart-string inviata
        """
        page = self.urlDispatch("query", input="%"+input)
        eprojects = ET.fromstring(page)
        res = []
        for project in eprojects:
            res.append(AchievoProject(project))
        if __debug__:
            print res
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
            res.append(AchievoTimereg(timereg))
        if __debug__:
            print res
        return res
    
class AchievoProject:
    """
    Classe di utilità per la smart-search
    """
    def __init__(self, record):
        self.record = record
    def __repr__(self):
        return ET.tostring(self.record)
    def __str__(self):
        return "%(project_name)s (%(phase_name)s) %(activity_name)s" % dict(self.record.items())

class AchievoTimereg:
    """
    Classe di utilità per il timeReport
    """
    #TODO: Valutare se le due classi sono destinate a differenziarsi
    #      o se conviene unificarle
    def __init__(self, record):
        self.record = record
    def __repr__(self):
        return ET.tostring(self.record)
    def __str__(self):
        return "%s" % self.record.get("remark")
    
def parseSmartQuery(squery):
    getsq = re.compile("""
        (?P<project>[^ ]+)\ *
        (?P<phase>[^ ]+|)\ *
        (?P<activity>[^ ]+|)\ *
        (?P<hours>\d{1,2}:\d{1,2}|)\ *
        (?P<comment>.*|)
        """, re.VERBOSE)
    return getsq.search(squery).groupdict()
        

if __name__ == "__main__":
    rl = RemoteTimereg("http://www.develer.com/~naufraghi/achievo/", "matteo", "matteo99")
    rl.whoami() and "Login OK" or "Login Error!"
    rl.search("pr")
    rl.search("pr me")
    rl.search("pr me an")
    rl.timeReport("2006-11-7")
    rl.timeReport(["2006-11-7","2006-11-8"])