from lxml import etree
import unittest
import sys
#I want to be able to use my modules... 
sys.path.append("../")
from flighttracking.flight import *


class FlightTest(unittest.TestCase):
    
    def testFlightConstructor1(self):
        """ Test Flight.__init__() with a well formed full  node with no namespace"""
        nodeText='''<flight name="AF5921" status="canceled">
                            <departure datetime="2008-11-03T07:55:00Z" location="ORYW"/>
                            <arrival datetime="2008-11-03T09:15:00Z" location="NCY"/>
                    </flight>
                '''

        node=etree.fromstring(nodeText)
        f= Flight(node)
        #Check if our new object is well instanciated with goods attributes
        for attr,value in node.items(): 
            if getattr(f,attr)!=node.get(attr):
                raise Exception , "Mapping problem : value of attribute '%s' is different than 'Airport.%s'"%(attr,attr)
    
    def testFlightConstructor2(self):
        """ Test Flight.__init__() with a well formed full node with a Namespace."""
        rootText='''<?xml version="1.0" encoding="UTF-8"?>
                    <flightsTracking  xmlns="http://snibbits.net/~gcarrier/ns/tracking">
                        <flight name="AF5921" status="canceled">
                                <departure datetime="2008-11-03T07:55:00Z" location="ORYW"/>
                                <arrival datetime="2008-11-03T09:15:00Z" location="NCY"/>
                        </flight>
                    </flightsTracking>
                '''

        root=etree.fromstring(rootText)        
        f=Flight(root.getchildren()[0])
        #Check if our new object is well instanciated with goods attributes
        for attr,value in root.items(): 
            if getattr(f,attr)!=root.get(attr):
                raise Exception , "Mapping problem : value of attribute '%s' is different than 'Airport.%s'"%(attr,attr)

    def testFlightConstructor3(self):
        """ Test Flight.__init__() with an uncomplete node (name is missing) with no namespace"""
        nodeText='''<flight status="canceled">
                            <departure datetime="2008-11-03T07:55:00Z" location="ORYW"/>
                            <arrival datetime="2008-11-03T09:15:00Z" location="NCY"/>
                    </flight>
                '''

        node=etree.fromstring(nodeText)
        self.assertRaises(Exception, Flight,node)
    
    def testDepartureConstructor1(self):
        """ Test Departure.__init__() with a well formed node with no namespace"""
        nodeText='''
                    <departure datetime="2008-11-03T07:55:00Z" location="ORYW"/>
                '''
        node=etree.fromstring(nodeText)
        d= Departure(node)
        for attr,value in node.items(): 
            if getattr(d,attr)!=node.get(attr):
                raise Exception , "Mapping problem : value of attribute '%s' is different than 'Airport.%s'"%(attr,attr)
    
    def testDepartureConstructor2(self):
        """ Test Departure.__init__() with a well formed node with no namespace"""
        nodeText='''
                        <departure datetime="2008-11-03T07:55:00Z" location="ORYW" xmlns="http://snibbits.net/~gcarrier/ns/tracking"/>
                '''
        node=etree.fromstring(nodeText)
        d= Departure(node)
        
        for attr,value in node.items(): 
            if getattr(d,attr)!=node.get(attr):
                raise Exception , "Mapping problem : value of attribute '%s' is different than 'Airport.%s'"%(attr,attr)
    def testDepartureConstructor3(self):
        """ Test Departure.__init__() with an uncomplete node (datetime is missing) with no namespace"""
        nodeText='''
                        <departure location="ORYW" xmlns="http://snibbits.net/~gcarrier/ns/tracking"/>
                '''
        node=etree.fromstring(nodeText)
        self.assertRaises(Exception, Departure,node)
        
    def testArrivalConstructor1(self):
        """ Test Departure.__init__() with a well formed node with no namespace"""
        nodeText='''
                    <arrival datetime="2008-11-03T09:15:00Z" location="NCY"/>
                '''
        node=etree.fromstring(nodeText)
        a= Arrival(node)
        for attr,value in node.items():
            if getattr(a,attr)!=node.get(attr):
                raise Exception , "Mapping problem : value of attribute '%s' is different than 'Airport.%s'"%(attr,attr)
    
    def testArrivalConstructor2(self):
        """ Test Departure.__init__() with a well formed node with a namespace"""
        nodeText='''
                        <arrival datetime="2008-11-03T09:15:00Z" location="NCY" xmlns="http://snibbits.net/~gcarrier/ns/tracking"/>
                '''
        node=etree.fromstring(nodeText)
        a= Arrival(node)
        
        for attr,value in node.items():
            if getattr(a,attr)!=node.get(attr):
                raise Exception , "Mapping problem : value of attribute '%s' is different than 'Airport.%s'"%(attr,attr)
    
    def testArrivalConstructor3(self):
        """ Test Arrival.__init__() with an uncomplete node (datetime is missing) with no namespace"""
        nodeText='''
                        <departure location="ORYW" xmlns="http://snibbits.net/~gcarrier/ns/tracking"/>
                '''
        node=etree.fromstring(nodeText)
        self.assertRaises(Exception, Arrival,node)
    
    def testAirportConstructor1(self):
        """ Test Departure.__init__() with a well formed node with no namespace"""
        nodeText='''
                        <airport code="ORY" name="Orly" city="Paris" country="France"/>
                '''
        node=etree.fromstring(nodeText)
        a= Airport(node)
        
        for attr,value in node.items(): 
            if getattr(a,attr)!=node.get(attr):
                raise Exception , "Mapping problem : value of attribute '%s' is different than 'Airport.%s'"%(attr,attr)
    
    
    def testAirportConstructor2(self):
        """ Test Departure.__init__() with a well formed node with a namespace"""
        nodeText='''
                        <airport code="ORY" name="Orly" city="Paris" country="France" xmlns="http://snibbits.net/~gcarrier/ns/tracking"/>
                '''
        node=etree.fromstring(nodeText)
        a= Airport(node)
        
        for attr,value in node.items(): 
            if getattr(a,attr)!=node.get(attr):
                raise Exception , "Mapping problem : value of attribute '%s' is different than 'Airport.%s'"%(attr,attr)
    
    def testAirportConstructor3(self):
        """ Test Arrival.__init__() with an uncomplete node (country is missing) with a namespace"""
        nodeText='''
                        <airport code="ORY" name="Orly" city="Paris"  xmlns="http://snibbits.net/~gcarrier/ns/tracking"/>
                '''
        node=etree.fromstring(nodeText)
        self.assertRaises(Exception, Airport,node)


def test_suite():
    tests=[unittest.makeSuite(FlightTest)]
    return unittest.TestSuite(tests)

if __name__=="__main__":
    unittest.main(defaultTest="test_suite")
