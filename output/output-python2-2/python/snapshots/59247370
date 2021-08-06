#!/usr/bin/python
#-*- coding: utf-8 -*-
#
# Copyright 2006 Develer S.r.l. (http://www.develer.com/)
# All rights reserved.
#
# $Id:$
#
# Author: Matteo Bertini <naufraghi@develer.com>

import urllib, urllib2
try:
    from xml.etree import ElementTree as ET
except:
    try:
        from elementtree import ElementTree as ET
    except:
        raise ImportError, "ElementTree (or py2.5) needed"

class RemoteTimereg:
    def __init__(self, achievouri, user, password):
        self.user = user
        self.password = password
        self._achievouri = achievouri
        self._loginurl = urllib.basejoin(self._achievouri, "index.php")
        self._dispatchurl = urllib.basejoin(self._achievouri, "dispatch.php")
        self._login()
    def _login(self, user=None, password=None):
        if user != None and password != None:
            self.user = user
            self.password = password
        auth = urllib.urlencode({"auth_user":self.user,"auth_pw":self.password})
        self._setup_auth()
        #refresh Achievo session
        urllib2.urlopen(self._loginurl, auth).read()
        return self.urldispatch("whoami")
    def _setup_auth(self):
        passman = urllib2.HTTPPasswordMgrWithDefaultRealm()
        passman.add_password(None, self._loginurl, self.user, self.password)
        passman.add_password(None, self._dispatchurl, self.user, self.password)
        auth_handler = urllib2.HTTPBasicAuthHandler(passman)
        cookie_handler = urllib2.HTTPCookieProcessor()
        opener = urllib2.build_opener(auth_handler, cookie_handler)
        urllib2.install_opener(opener)
    def urldispatch(self, node, action="search", **kwargs):
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
        except:
            self._login() #Achievo session timeout?
            return urllib2.urlopen(self._dispatchurl, qstring).read()
    def whoami(self):
        page = self._login()
        elogin = ET.fromstring(page)
        if elogin.find("record").get('name') == self.user:
            return self.user
        else:
            return False
    def search(self, input):
        page = self.urldispatch("query", input=input)
        eprojects = ET.fromstring(page)
        res = []
        for project in eprojects:
            res.append(AchievoProject(project))
        return res
    def timereport(self, date):
        page = self.urldispatch("timereport", date=date)
        etimeregs = ET.fromstring(page)
        res = []
        for timereg in etimeregs:
            res.append(AchievoTimereg(timereg))
        return res
    
class AchievoProject:
    def __init__(self, record):
        self.record = record
    def __repr__(self):
        return ET.tostring(self.record)
    def __str__(self):
        return "%(project_name)s (%(phase_name)s) %(activity_name)s" % dict(self.record.items())

class AchievoTimereg:
    def __init__(self, record):
        self.record = record
    def __repr__(self):
        return ET.tostring(self.record)
    def __str__(self):
        return "%s" % self.record.get("remark")
    
if __name__ == "__main__":
    rl = RemoteTimereg("http://www.develer.com/~naufraghi/achievo/", "matteo", "matteo99")
    print rl.whoami() and "Login OK" or "Login Error!"
    print rl.search("pr")
    print rl.search("pr me")
    print rl.search("pr me an")
    print rl.timereport("2006-11-7")
    print rl.timereport(["2006-11-7","2006-11-8"])