# -*- coding: utf-8 -*-
import lxml
from lxml import etree
import unittest
import sys, os

#I want to be able to use my modules... 
sys.path.append("../")
from flighttracking.flight import *
from service import google

def loadFile(file_in):
    locations={}
    if os.path.exists(file_in):
        xml_doc=etree.parse(file_in)
        root=xml_doc.getroot()
        T=FlightsTracking(root)
        return T
def geoCode(doc):
    for loc in doc["locations"]:
        lat, long,z=getLatLong(str(loc))
        if lat is not None and long is not None and z is not None:
            loc.setCoordinates(lat, long,z)
        else:
            raise Exception, "Can't obtains Coordinates"
    return doc
def getLatLong(place):
    #rep=google.getResponse("Aéroport "+place, proxy=("www-cache","3128"))
    rep=google.getResponse("Aéroport "+place)
    
    if rep is not None:
        try:
            root=etree.XML(rep)
        except:
            #File contains an utf-8 declaration but it's in fact iso-8859-1.....
            root=etree.XML(rep.replace("UTF-8","iso-8859-1"))
    coordinates=root.xpath("//g:coordinates",namespaces={"g":root.nsmap[None]})
    if len(coordinates)>0:
        lat, long, z= coordinates[0].text.split(',')
        return (lat, long, z)
    else:
        return (None, None, None) 
def save(doc, file_out):
    f=open(file_out,"w")
    f.write(doc.toXML())
    f.close()
    
if __name__=="__main__":
    file_in=sys.argv[1]
    file_out=sys.argv[2]
    doc=loadFile(file_in)
    doc=geoCode(doc)
    save(doc, file_out)