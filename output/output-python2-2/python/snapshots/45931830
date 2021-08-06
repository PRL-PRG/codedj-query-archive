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
        
    def testFlightConstructor4(self):
        """ Test Flight.__init__() with a well formed full node with a Namespace. Check if ___departure___ and ___arrival___ are well instancied"""
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
        if not f.___departure___.__class__==Departure:
            raise Exception,"Departure is not well instanciated"
        if not f.___arrival___.__class__==Arrival:
            raise Exception,"Arrival is not well instanciated"
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
        
    def testLocationConstructor1(self):
        """ Test Location.__init__() with a complete node, with no namespace and no sub node 'gate'"""
        nodeText='''
                    <location name="NCY">
                        <airport code="NCY" name="Annecy Meythet" city="Annecy" country="France" />
                    </location>
                '''
        node=etree.fromstring(nodeText)
        l=Location(node)
        for attr,value in node.items():
            if getattr(l,attr)!=node.get(attr):
                raise Exception , "Mapping problem : value of attribute '%s' is different than 'Airport.%s'"%(attr,attr)

    def testLocationConstructor2(self):
        """ Test Location.__init__() with a complete node, with no namespace and no sub node 'gate'
            Check if Place.___airport___ is well instanciated
        """
        nodeText='''
                    <location name="ORYW">
                        <airport code="ORY" name="Orly" city="Paris" country="France" />
                    </location>
                '''
        node=etree.fromstring(nodeText)
        airportNode=node.xpath("//airport")
        l=Location(node)
        
        if l.___airport___ is not None:
            if l.___airport___.__class__==Airport:
                for attr, value in node.items():
                    if not getattr(l,attr)==value:
                        raise Exception,"___airport___ is not well instanciated  : (___airport___.%s != node.%s)"%(attr,attr)
            else:
                raise Exception,"___airport___ is not well instanciated (%s instead of %s)"(l.___airport___.__class__, Airport)
        else:
            raise Exception,"___airport___ is not well instanciated (None)"
    
    def testLocationConstructor3(self):
        """ Test Location.__init__() with a complete node, with no namespace and a sub node 'gate'
            Check if Place.__gate__ is well instanciated
        """
        nodeText='''
                    <location name="ORYW">
                        <airport code="ORY" name="Orly" city="Paris" country="France" />
                        <gate name="W"/>
                    </location>
                '''
        node=etree.fromstring(nodeText)
        l=Location(node)
        gate=node.xpath("//gate")[0]
        if l.___gate___ is not None:
            if not l.___gate___.name==gate.get("name"):
                raise Exception,"___gate___.name is not different than node.get('name')"
        else:
            raise Exception,"___gate___ is not well instanciated (None)"
    def testLocationConstructor4(self):
        """ Test Location.__init__() with an uncomplete node (airport is missing), with no namespace and a sub node 'gate'
            Should fail.
        """
        nodeText='''
                    <location name="ORYW">
                        <gate name="W"/>
                    </location>
                '''
        node=etree.fromstring(nodeText)
        self.assertRaises(Exception,Location, node)

    
    def testLocationConstructor5(self):
        """ Test Location.__init__() with a complete node, with a namespace and no sub node 'gate'"""
        """ Same test as testLocationConstructor1() but with a namespace"""
        
        nodeText='''
                    <location name="NCY" xmlns="http://snibbits.net/~gcarrier/ns/tracking">
                        <airport code="NCY" name="Annecy Meythet" city="Annecy" country="France" />
                    </location>
                '''
        node=etree.fromstring(nodeText)
        l=Location(node)
        for attr,value in node.items():  
            if getattr(l,attr)!=node.get(attr):
                raise Exception , "Mapping problem : value of attribute '%s' is different than 'Airport.%s'"%(attr,attr)
            
    def testLocationConstructor6(self):
        """ Test Location.__init__() with a complete node, with a namespace and a sub node 'gate'
            Check if Place.___airport___ is well instanciated
            Same test as testLocationConstructor2() but with a namespace
        """
        nodeText='''
                    <location name="ORYW" xmlns="http://snibbits.net/~gcarrier/ns/tracking">
                        <airport code="ORY" name="Orly" city="Paris" country="France"/>
                    </location>
                '''
        node=etree.fromstring(nodeText)
        airportNode=node.xpath("//f:airport", namespaces={"f":"http://snibbits.net/~gcarrier/ns/tracking"})
        l=Location(node)
        
        if l.___airport___ is not None:
            if l.___airport___.__class__==Airport:
                for attr, value in node.items():
                    if not getattr(l,attr)==value:
                        raise Exception,"___airport___ is not well instanciated  : (___airport___.%s != node.%s)"%(attr,attr)
            else:
                raise Exception,"___airport___ is not well instanciated (%s instead of %s)"(l.___airport___.__class__, Airport)
        else:
            raise Exception,"___airport___ is not well instanciated (None)"
    def testLocationConstructor7(self):
        """ Test Location.__init__() with a complete node, with a namespace and a sub node 'gate'
            Check if Place.__gate__ is well instanciated
            Same test as testLocationConstructor3() but with a namespace
        """
        nodeText='''
                    <location name="ORYW" xmlns="http://snibbits.net/~gcarrier/ns/tracking">
                        <airport code="ORY" name="Orly" city="Paris" country="France" />
                        <gate name="W"/>
                    </location>
                '''
        node=etree.fromstring(nodeText)
        l=Location(node)
        gate=node.xpath("//f:gate",namespaces={"f":"http://snibbits.net/~gcarrier/ns/tracking"})[0]
        if l.___gate___ is not None and l.___gate___ is not "":
            if l.___gate___.__class__==Gate:
                if not l.___gate___.name==gate.get("name"):
                    raise Exception,"__gate__.name is not the same as node.get('name')"
            else:
                raise Exception,"__gate__ is not well instanciated (%s)"%l.___gate___.__class__
        else:
            raise Exception,"__gate__ is not well instanciated (None)"
    def testLocationConstructor8(self):
        """ Test Location.__init__() with an uncomplete node (airport is missing), with a namespace and a sub node 'gate'
            Same test as testLocationConstructor3() but with a namespace
            Should fail.
        """
        nodeText='''
                    <location name="ORYW" xmlns="http://snibbits.net/~gcarrier/ns/tracking">
                        <gate name="W"/>
                    </location>
                '''
        node=etree.fromstring(nodeText)
        self.assertRaises(Exception,Location, node)

        

    def testDepartureToXML(self):
        """ Test Departure.toXML() with a well formed full  node with no namespace"""
        nodeText='''<departure datetime="2008-11-03T07:55:00Z" location="ORYW"/>'''

        node=etree.fromstring(nodeText)
        f= Departure(node)        
        node2=etree.fromstring(f.toXML())
        if not etree.tostring(node, pretty_print=True) == etree.tostring(node2,pretty_print=True):
            raise Exception, "Erreur parsage XML : la chaine produite par la methode toXML() est differente de celle produite par etree a partir du meme noeud"
    def testArrivalToXML(self):
        """ Test Flight.__init__() with a well formed full  node with no namespace"""
        nodeText='''<arrival datetime="2008-11-03T09:15:00Z" location="NCY"/>'''

        node=etree.fromstring(nodeText)
        f= Arrival(node)
        node2=etree.fromstring(f.toXML())
        
        if not etree.tostring(node, pretty_print=True) == etree.tostring(node2,pretty_print=True):
           raise Exception, "Erreur parsage XML : la chaine produite par la methode toXML() est differente de celle produite par etree a partir du meme noeud"
    def testGateToXML(self):
        """ Test Flight.__init__() with a well formed full  node with no namespace"""
        nodeText='''<gate name="G"/>'''

        node=etree.fromstring(nodeText)
        g= Gate(node)
        print g["name"]
        node2=etree.fromstring(g.toXML())
        if not etree.tostring(node, pretty_print=True) == etree.tostring(node2,pretty_print=True):
            raise Exception, "Erreur parsage XML : la chaine produite par la methode toXML() est differente de celle produite par etree a partir du meme noeud"
    
    """ Due to python instropection attributes and subnode may not appear in the same order. Both tests cases are always raising exceptions 
    def testLocationToXML(self):
        "" Test Flight.__init__() with a well formed full  node with no namespace""
        nodeText='''<location name="ORYW" xmlns="http://snibbits.net/~gcarrier/ns/tracking"><airport code="ORY" name="Orly" city="Paris" country="France"/><gate name="W"/></location>'''

        node=etree.fromstring(nodeText)
        g= Location(node)
        node2=etree.fromstring(g.toXML())
        #print "'"+etree.tostring(node, pretty_print=True)+"'" 
        #print "'"+etree.tostring(node2,pretty_print=True)+"'"
        if not etree.tostring(node, pretty_print=True) == etree.tostring(node2,pretty_print=True):
               raise Exception, "Erreur parsage XML : la chaine produite par la methode toXML() est differente de celle produite par etree a partir du meme noeud"
    def testFlightToXML(self):
        "" Test Flight.toXML() with a well formed full  node with no namespace. I've suppressed the \r and \t because etree.tostring don't react in the same way.""
        nodeText='''<flight name="AF5921" status="canceled"><departure datetime="2008-11-03T07:55:00Z" location="ORYW"/><arrival datetime="2008-11-03T09:15:00Z" location="NCY"/></flight>
                '''
        node=etree.fromstring(nodeText)
        f= Flight(node)
        
        node2=etree.fromstring(f.toXML())
        #print ""
        #print "'"+ etree.tostring(node, pretty_print=True)+"'"
        #print "'"+etree.tostring(node2, pretty_print=True)+"'"
        if not etree.tostring(node, pretty_print=True) == etree.tostring(node2,pretty_print=True):
            raise Exception, "Erreur parsage XML : la chaine produite par la methode toXML() est differente de celle produite par etree a partir du meme noeud"
    """
def test_suite():
    tests=[unittest.makeSuite(FlightTest)]
    return unittest.TestSuite(tests)

if __name__=="__main__":
    unittest.main(defaultTest="test_suite")
