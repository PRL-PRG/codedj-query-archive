from lxml import etree
import unittest
import sys
#I want to be able to use my modules... 
sys.path.append("../../")
from flighttracking.flight import Flight


class FlightTest(unittest.TestCase):
    
    def testFlightConstructor1(self):
        """ Test Flight.__init__() with a well formed node with no namespace"""
        nodeText='''<flight name="AF5921" status="canceled">
                            <departure datetime="2008-11-03T07:55:00Z" location="ORYW"/>
                            <arrival datetime="2008-11-03T09:15:00Z" location="NCY"/>
                    </flight>
                '''

        node=etree.fromstring(nodeText)
        f= Flight(node)
        if f.name != node.get("name"):
             raise Exception, "Mapping problem : value of attribute 'name' is different than 'Flight.name' "
        if f.status != node.get("status"):
            raise Exception, "Mapping problem : value of attribute 'status' is different than 'Flight.status'"
    
    def testFlightConstructor2(self):
        """ Test Flight.__init__() with a well formed node with a Namespace."""
        rootText='''<?xml version="1.0" encoding="UTF-8"?>
                    <flightsTracking ns="flight" xmlns="http://snibbits.net/~gcarrier/ns/tracking">
                        <flight name="AF5921" status="canceled">
                                <departure datetime="2008-11-03T07:55:00Z" location="ORYW"/>
                                <arrival datetime="2008-11-03T09:15:00Z" location="NCY"/>
                        </flight>
                    </flightsTracking>
                '''

        root=etree.fromstring(rootText)        
        f=Flight(root.getchildren()[0])
    
    
def test_suite():
    tests=[unittest.makeSuite(FlightTest)]
    return unittest.TestSuite(tests)

if __name__=="__main__":
    unittest.main(defaultTest="test_suite")
