#!/usr/bin/env python
# -*- coding: iso-8859-1 -*-
"""Library to access del.icio.us via python.

This module helps you to ...

get()
a = apiNew()
a.tags.get
a.tags.rename
a.posts.get
"""


##
# License: pydelicious is released under the bsd license. 
# See 'license.txt' for more informations.
#

##
# TODO fuer pydelicious.py                                                
#  * die dokumentation aller docs muss noch geschehen                   
#  * dokumentation                                                      
#  * welche lizense benutze ich                                         
#  * lizense einbinden und auch via setup.py verteilen
#  * readme auch schreiben und via setup.py verteilen
#  * mehr tests
#  * auch die funktion von setup.py testen?
#  * auch auf anderen systemen testen (linux -> uni)
#  * automatisch releases bauen lassen, richtig benennen und in das
#    richtige verzeichnis verschieben.
#  * was können die anderen librarys denn noch so? (ruby, java, perl, etc)
#  * was wollen die, die es benutzen?
#  * wofür könnte ich es benutzen?
#  * entschlacken?
#
# realy?
#  * date object von del.icio.us muss ich noch implementieren           
#
# done!
#  * stimmt das so? muss eher noch täg str2utf8 konvertieren
#    >>> pydelicious.getrss(tag="täg")
#    url: http://del.icio.us/rss/tag/täg
#  * requester muss eine sekunde warten                                 
#  * __init__.py gibt die funktionen weiter                             
#  * html parser funktioniert noch nicht, gar nicht                     
#  * alte funktionen fehlen, get_posts_by_url, etc.                     
#  * post funktion erstellen, die auch die fehlenden attribs addiert.   
#  * die api muss ich noch weiter machen                                
#  * requester muss die 503er abfangen                                  
#  * rss parser muss auf viele möglichkeiten angepasst werden           


import re, md5, httplib
import urllib, urllib2, time
# import datetime, 
import StringIO
    
# this is new                                                           #
# this relays on an external library, will probably be kept             #
import sys
import os

# !!! zweiter Versuch funzt nur auf linux rechner in der uni - 
# ersteres auch auf win32, doof.
from pydeliciouslibs.elementtree.ElementTree import parse
import pydeliciouslibs.feedparser.feedparser as feedparser

# Taken from FeedParser.py                                                                      
# timeoutsocket allows feedparser to time out rather than hang forever on ultra-slow servers.   
# Python 2.3 now has this functionality available in the standard socket library, so under      
# 2.3 you don't need to install anything.  But you probably should anyway, because the socket   
# module is buggy and timeoutsocket is better.                                                  
try:
    import timeoutsocket # http://www.timo-tasi.org/python/timeoutsocket.py
    timeoutsocket.setDefaultSocketTimeout(20)
except ImportError:
    import socket
    if hasattr(socket, 'setdefaulttimeout'): socket.setdefaulttimeout(20)

# some basic settings
VERSION = '0.4.0'
AUTHOR = 'Greg Pinero/Frank Timmermann' #Frank Timmermann orginally, Greg Pinero made it work again
AUTHOR_EMAIL = 'regenkind_at_gmx_dot_de' #doesn't respond to emails! GP
PROJECT_URL = 'http://deliciouspython.python-hosting.com/'
# das folgende ist mit python 2.2.3 nicht erlaubt. :(
CONTACT = '%(URL)s or %(email)s'%dict(URL = PROJECT_URL, email = AUTHOR_EMAIL)
DESCRIPTION = '''pydelicious.py allows you to access the web service of del.icio.us via it's API through python.'''
LONG_DESCRIPTION = '''the goal is to design an easy to use and fully functional python interface to
del.icio.us. '''
DWS_HOSTNAME = 'https://api.del.icio.us/' #GP -Change this for API change
DWS_HOSTNAME_RSS = 'http://del.icio.us/rss/'
DWS_REALM = 'del.icio.us API'
DWS_API = 'https://api.del.icio.us/v1/' #GP -Change this for API change
USER_AGENT = 'pydelicious.py/%(version)s %(contact)s' % dict(version = VERSION, contact = CONTACT)

LAST_CALL = 0
DEBUG = 0

delicious_date_pattern  = re.compile("[1,2][0-9]{3}-[0-2][0-9]-[0-3][0-9]T[0-2][0-9]:[0-5][0-9]:[0-5][0-9]Z")


# Helper Funktion
def str2uni(s):
    # type(in) str or unicode
    # type(out) unicode
    return ("".join([unichr(ord(i)) for i in s]))


def str2utf8(s):
    # type(in) str or unicode
    # type(out) str
    return ("".join([unichr(ord(i)).encode("utf-8") for i in s]))


def str2quote(s):
    return urllib.quote_plus("".join([unichr(ord(i)).encode("utf-8") for i in s]))


def dict0(d):
    # {'a':'a', 'b':'', 'c': 'c'} => {'a': 'a', 'c': 'c'}
    dd = dict()
    for i in d:
            if d[i] != "": dd[i] = d[i]
    return dd


class Waiter:

    def __init__(self, t = 0, sleeptime = 1):
        self.t=t
	self.sleeptime = sleeptime
	self.waitcommand = 0
	self.waited = 0

    def wait(self, t=None):
        self.waitcommand += 1
        if t == None: t = time.time()
        if DEBUG: print "Waiter:",t-self.t
        if t-self.t<self.sleeptime:
	    time.sleep(self.sleeptime-t+self.t)
	    self.waited += 1
	self.t = time.time()


Waiter = Waiter()
        

# Fehlerbehandlung
comment='''Fehlerbehandlungszeug,
kopiert aus delicious025.py, damit der reqester funktioniert. brauche ich das alles so?
'''
class DeliciousException(Exception):
    '''Std. Error Function'''
    pass


class DefaultErrorHandler(urllib2.HTTPDefaultErrorHandler):

    '''Handles HTTP Error, currently only 503
    Behandelt die HTTP Fehler, behandelt nur 503 Fehler'''
    def http_error_503(self, req, fp, code, msg, headers):
        raise urllib2.HTTPError(req, code, throttled_message, headers, fp)


# ! #
class post(dict):

    # a post object contains of this:
    #  href 
    #  description
    #  hash
    #  dt
    #  tags
    #  extended
    #  user
    #  count
    def __init__(self, href = "", description = "", hash = "", time = "", tag = "", extended = "", user = "", count = "",
                 tags = "", url = "", dt = ""): # tags or tag?
        self["href"] = href
        if url != "": self["href"] = url
        self["description"] = description
        self["hash"] = hash
        self["dt"] = dt
        if time != "": self["dt"] = time
        self["tags"] = tags
        if tag != "":  self["tags"] = tag     # tag or tags? # !! tags
        self["extended"] = extended
        self["user"] = user
        self["count"] = count

    def __getattr__(self, name):
        try: return self[name]
        except: object.__getattribute__(self, name)


class posts(list):

    def __init__(self, *args):
        for i in args: self.append(i)

    def __getattr__(self, attr):
        try: return [p[attr] for p in self]
        except: object.__getattribute__(self, attr)

    
# handle all the RSS stuff
comment='''rss sollte nun wieder funktionieren, aber diese try, except scheisse ist so nicht schoen

rss wird unterschiedlich zusammengesetzt. ich kann noch keinen einheitlichen zusammenhang
zwischen daten (url, desc, ext, usw) und dem feed erkennen. warum können die das nicht einheitlich machen? 
'''
def _readRSS(tag = "", popular = 0, user = "", url = ''):
    '''handle a rss request to del.icio.us'''
    tag = str2quote(tag)
    user = str2quote(user)
    if url != '':
        # http://del.icio.us/rss/url/efbfb246d886393d48065551434dab54
        url = DWS_HOSTNAME_RSS + '''url/%s'''%md5.new(url).hexdigest()    
    elif user != '' and tag != '':
        url = DWS_HOSTNAME_RSS + '''%(user)s/%(tag)s'''%dict(user=user, tag=tag)
    elif user != '' and tag == '':
        # http://del.icio.us/rss/delpy
        url = DWS_HOSTNAME_RSS + '''%s'''%user
    elif popular == 0 and tag == '':
        url = DWS_HOSTNAME_RSS
    elif popular == 0 and tag != '':
        # http://del.icio.us/rss/tag/apple
        # http://del.icio.us/rss/tag/web2.0
        url = DWS_HOSTNAME_RSS + "tag/%s"%tag
    elif popular == 1 and tag == '':
        url = DWS_HOSTNAME_RSS + '''popular/'''
    elif popular == 1 and tag != '':
        url = DWS_HOSTNAME_RSS + '''popular/%s'''%tag
    rss = _request(url, useUrlAsIs = 1)
    rss = feedparser.parse(rss)
    # print rss
#     for e in rss.entries: print e;print
    l = posts()
    for e in rss.entries:
        if e.has_key("links") and e["links"]!=[] and e["links"][0].has_key("href"):
            url = e["links"][0]["href"]
        elif e.has_key("link"):
            url = e["link"]
        elif e.has_key("id"):
            url = e["id"]
        else:
            url = ""
        if e.has_key("title"):
            description = e['title']
        elif e.has_key("title_detail") and e["title_detail"].has_key("title"):
            description = e["title_detail"]['value']
        else:
            description = ''
        try: tags = e['categories'][0][1]
        except:
            try: tags = e["category"]
            except: tags = ""
        if e.has_key("modified"):
            dt = e['modified']
        else:
            dt = ""
        if e.has_key("summary"):
            extended = e['summary']
        elif e.has_key("summary_detail"):
            e['summary_detail']["value"]
        else:
            extended = ""
        if e.has_key("author"):
            user = e['author']
        else:
            user = ""
# time = dt ist weist auf ein problem hin
# die benennung der variablen ist nicht einheitlich
#  api senden und
#  xml bekommen sind zwei verschiedene schuhe :(
        l.append(post(url = url, description = description, tags = tags, dt = dt, extended = extended, user = user))
    return l


# HTML Parser, deprecated
comment='''paring html gibt mehr infos als die parsing von html, aber
 * eine aenderung des html's macht die funktion kaput 
 * fuer rss gibt es einen guten funktionierenden parser, muss ich denn trotzdem html wirklich parsen
 * get_posts_by_url funktioniert nur mit dem parsen von html ==> stimmt das noch?
'''
def _readHTML(tar = "", popular = 0, user = ""):
    pass
    # construct url 
    # get data      
    # data 2 posts  
    # return posts  


# Requester
comment='''stimmt der requester schon mit den vorgaben von del.icio.us ueberein, nein ...
 * sollte ich die daten von del.icio.us auf nur text untersuchen?

Done
* eine sekunde pause zwischen jedem request klappt.
* ich fange noch nicht den 503 code ab, klappt aber in der alten version
   muss ich auch nicht, denn das läuft über die Exception DefaultErrorHandler.
 '''
def _request(url, params = '', useUrlAsIs = 0, user = '', passwd = '', ):
    if DEBUG: httplib.HTTPConnection.debuglevel = 1
    # Please wait AT LEAST ONE SECOND between queries, or you are likely to get automatically throttled. 
    # If you are releasing a library to access the API, you MUST do this.
    # Everything is in the Caller, don't use this at home!
    Waiter.wait()    
    password_manager = urllib2.HTTPPasswordMgrWithDefaultRealm()
    password_manager.add_password(None, DWS_HOSTNAME, user, passwd)
    #GP - change above to DWS_API?
    auth_handler = urllib2.HTTPBasicAuthHandler(password_manager)
    opener = urllib2.build_opener(auth_handler)
    urllib2.install_opener(opener)
    # params come as a dict => dict0 => urlencode 
    params = urllib.urlencode(dict0(params))
    request = urllib2.Request(DWS_API + url + params)
    if useUrlAsIs: request = urllib2.Request(url)
    request.add_header('User-Agent', USER_AGENT)
    if DEBUG: print "url:", request.get_full_url()
    try:
        o = urllib2.urlopen(request)            
        return o.read()
    except DefaultErrorHandler:
        if DEBUG: return opener.open(request).read()
        return ""

# XML Parser
comment='''ist vollständig, 
 * test fehlt nocht
'''
def _handleXML(data):
    if DEBUG: print data
    x = parse(StringIO.StringIO(data))
    mode = x.getroot().tag
    if mode == 'tags':
        l = [dict(count = t.attrib["count"], tag = t.attrib["tag"]) for t in x.findall("tag")]
    elif mode == "result":
        if (x.getroot().attrib.has_key("code") and x.getroot().attrib["code"] == 'done') or x.getroot().text in ['done', 'ok']:
            l = True
        else :
            l = False
    elif mode == 'update':
        l = x.getroot().attrib['time']
    elif mode == 'dates':
        l = [dict(count = t.attrib["count"], date = t.attrib["date"]) for t in x.findall("date")]
    elif mode == 'bundles':
        l = [dict(name = t.attrib["name"], tags = t.attrib["tags"]) for t in x.findall("bundle")]
    elif mode == 'posts':
        l = posts()
        for t in x.findall("post"):
            href, description, hash = '', '', ''
            tag,time, extended      = '', '', ''
            count = ''
            if t.attrib.has_key("href"): href = t.attrib["href"]
            if t.attrib.has_key("description"): description = t.attrib["description"]
            if t.attrib.has_key("hash"): hash = t.attrib["hash"]
            if t.attrib.has_key("tag"): tag = t.attrib["tag"]
            if t.attrib.has_key("time"): time = t.attrib["time"]
            if t.attrib.has_key("extended"): extended = t.attrib["extended"]
            if t.attrib.has_key("count"): count = t.attrib["count"]
            p = post(href=href, description=description,hash=hash,
                     tag=tag, time=time, extended=extended,
                     count=count)
            l.append(p)
    return l

'''brauche ich das?'''
def _validatePost(post): pass


# del.icio.us api
comment='''herzstueck

Done
 * was passiert mit nicht ascii daten in der verschickung als parameter?
 * noch sehr unvollstaendig
 * aufbau der api ist so, glaube ich, nicht mehr sinnvoll, vielleicht doch lieber nach dem alten schema, also
        api.tags_get(...) anstatt api.tags.get(...)
'''
class _DeliciousAPI:
# def _request(url, params = '', useUrlAsIs = 0, user = '', passwd = '', ):

    def __init__(self, user, passwd):
        self.user = user
        self.passwd = passwd

    def _main(self, url, params = ''):
        x = _request(url = url, params = params, user = self.user, passwd = self.passwd)
	self.xml = x
        return _handleXML(x)
    
    def tags_get(self):
        return self._main(url = "tags/get?")
        
    def tags_rename(self, old, new):
        return self._main("tags/rename?", (dict(old = str2utf8(old),
                                                new = str2utf8(new))))

    def posts_update(self):
        return self._main("posts/update")

    def posts_dates(self, tag = ""):
        return self._main("posts/dates?", (dict(tag = str2utf8(tag))))

    def posts_get(self, tag="", dt="", url=""):
        return self._main("posts/get?", (dict(tag = str2utf8(tag),
                                              dt = str2utf8(dt),
                                              url = str2utf8(url))))

    def posts_recent(self, tag="", count=""):
        return self._main("posts/recent?", (dict(tag = str2utf8(tag),
                                                 count = str2utf8(count))))

    def posts_all(self, tag=""):
        return self._main("posts/all?", (dict(tag = str2utf8(tag))))

    def posts_add(self, url, description="", extended="", tags="", dt="", replace="no"): 
        '''add an post to del.icio.us
        
        url - the url of the page you like to add
        description - a description of the page, often the title of the page
        extended (opt) - an extended description, could be some kind of comment
        tags - tags to sort your posts
        dt (opt) - current date in format ...., if no date is given, the current
                   date will be used
        '''
        return self._main("posts/add?", (dict(url = str2utf8(url),
                                              description = str2utf8(description),
                                              extended = str2utf8(extended),
                                              tags = str2utf8(tags),
                                              dt = str2utf8(dt),
                                              replace = str2utf8(replace))))

    def posts_delete(self, url): 
        return self._main("posts/delete?", (dict(url = str2utf8(url))))

    def bundles_all(self):
        return self._main(url = "tags/bundles/all")
    
    def bundles_set(self, bundle, tags):
        return self._main(url = "tags/bundles/set?",
                          params = (dict(bundle = str2utf8(bundle),
                                         tags = str2utf8(tags))))
    
    def bundles_delete(self, bundle):
        return self._main(url = "tags/bundles/delete?",
                          params = (dict(bundle = str2utf8(bundle))))

comment='''kick this, brauche das doch nicht, s.o,'''
def apiNew(user, passwd):
    return _DeliciousAPI(user=user, passwd= passwd)

comment=''' holt die Daten via rss, entspricht _readRSS
Done
* basiert auch auf rss, html und api 
* braucht deshalb noch, bis es voll funzt. '''
def getrss(tag = "", popular = 0, url = '', user = ""):
    '''get posts from del.icio.us via parsing Rss or Html
    
    tag (opt) sort by tag
    popular (opt) look for the popular stuff
    user (opt) get the posts by a user, this striks popular
    url (opt) get the posts by url '''
    return _readRSS(tag=tag, popular=popular, user=user, url=url)


comment = '''api funktionen, damit die funktionen aus 0.2.5 bestehen bleiben'''
def add(user, passwd, url, description, tags = "", extended = "", dt = "", replace="no"):
    return apiNew(user=user, passwd=passwd).posts_add(url=url, description=description, extended=extended, tags=tags, dt=dt, replace=replace)


def get(user, passwd, tag="", dt="",  count = 0):
    posts = apiNew(user=user, passwd=passwd).posts_get(tag=tag,dt=dt)
    if count != 0: posts = posts[0:count]
    return posts


def get_all(user, passwd, tag = ""):
    return apiNew(user=user, passwd=passwd).posts_all(tag=tag)


def delete(user, passwd, url):
    return apiNew(user=user, passwd=passwd).posts_delete(url=url)


def rename_tag(user, passwd, oldtag, newtag):
    return apiNew(user=user, passwd=passwd).tags_rename(old=oldtag, new=newtag)


def get_tags(user, passwd):
    return apiNew(user=user, passwd=passwd).tags_get()


def get_userposts(user):
    return getrss(user = user)


def get_tagposts(tag): 
    return getrss(tag = tag)


def get_urlposts(url): 
    return getrss(url = url)


def get_popular(tag = ""):
    return getrss(tag = tag, popular = 1)
