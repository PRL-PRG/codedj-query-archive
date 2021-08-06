# -*- coding: utf-8 -*-
import httplib, urllib, urllib2
import sys
import lxml
import os

GSERVER = "maps.google.com"
GAPP = "/maps/geo"
FORMATS = ['xml', ]


def getResponse(place, proxy=None, output='xml'):
    if not place == '':
        if output not in FORMATS:
           raise exception, "%s not implemented." % format
        GPARAMS = {
                'q':place,
                "output":output,
                }
        gparams = urllib.urlencode(GPARAMS)
        if proxy is not None:
            httpserv = httplib.HTTPConnection(proxy[0], proxy[1])
            request = "http://" + GSERVER + GAPP + '?' + gparams
        else:
            httpserv = httplib.HTTPConnection(GSERVER, 80)
            request = GSERVER + GAPP + '?' + gparams
        httpserv.request('GET', request)
        response = httpserv.getresponse()
        if httplib.OK == response.status:
            r = response.read()
            return r
        else:
            print "REQUEST ERROR :"
            print "Status %s" % response.status
            return None
    else:
        return None