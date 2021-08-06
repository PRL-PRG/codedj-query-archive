# -*- coding: utf-8 -*- 
import httplib, urllib
import sys
from xml.dom import minidom
import time
GSERVER="maps.google.com"
GAPP="/maps/geo"
FORMATS=['xml',]

def getResponse(place,output='xml'):
    if not place=='':
        #if output not in FORMATS:
        #   raise exception,"%s not implemented."%format
        GPARAMS={
                'q':place,
                "output":output,
                }
        
        httpserv=httplib.HTTPConnection(GSERVER,80)
        gparams=urllib.urlencode(GPARAMS)
        request=GSERVER+GAPP+'?'+gparams
        
        
        httpserv.request('GET',request)
        response=httpserv.getresponse()
        if httplib.OK==response.status:
            r=response.read()
            return r
        else:
            print "REQUEST ERROR :"
            return "Error"
    else:
        return "Error"
def getAirportAdress(filein):
    #TODO: 
    # Parser le doc xml, recupere l'adresse 
    return "49 rue General FERRIE, Grenoble"

def parseResponse(response):
    #TODO: 
    # Parser le doc xml, recupere les coordonnées
    print response

def printUsage():
    print sys.argv[0],"file_in file_out"
#Ant seems to don't like that :
if __name__=="__main__":
    if len(sys.argv)==3:
        print sys.argv[1]
        print sys.argv[2]
        addresse=getAirportAdress(sys.argv[1])
        a=getResponse(addresse)
        parseResponse(a)
    else:
        printUsage()
    
