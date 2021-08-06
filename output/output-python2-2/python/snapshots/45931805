from lxml import etree
import unittest
import sys, os
#I want to be able to use my modules... 
sys.path.append("../")
from flighttracking.flight import *

def loadFile(file_in):
    locations={}
    if os.path.exists(file_in):
        xml_doc=etree.parse(file_in)
        root=xml_doc.getroot()
        T=FlightsTracking(root)
        return T
def geoCode(doc):
    for loc in doc["locations"]:
        lat, long=getLatLong(str(loc))
        loc.setCoordinates(lat, long)
    return doc
def getLatLong(place):
    #TODO CALL Google webservice
    #print place
    return 5.7250776, 45.1760751
def save(location, file_out):
    pass
if __name__=="__main__":
    file_in=sys.argv[1]
    file_out=None
    doc=loadFile(file_in)
    doc=geoCode(doc)
    print doc.toXML()
    #save(locations, file_out)