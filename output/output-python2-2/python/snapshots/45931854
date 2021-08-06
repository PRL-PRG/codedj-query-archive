import lxml
from lxml import etree
import re

nspattern="(\{.*\}){0,1}"
class PlaceTime:
    datetime=""
    location=""
    def __init__(self, node):
        if not "datetime" in node.keys():
            raise Exception,"PlaceTime Attribute ERROR : a PlaceTime node may have a 'datetime' attribute."
        if not "location" in node.keys():
            raise Exception,"PlaceTime Attribute ERROR : a PlaceTime node may have a 'location' attribute."
        # List comprehension + instrospection = :)
        [setattr(self, attr,value) for attr,value in node.items()]
    
class Departure (PlaceTime):
    def __init__(self, node):
        
        pattern=nspattern+"departure"
        if not re.match(pattern, node.tag):
            raise Exception,"Departure Node ERROR : %s not allowed here"%node.tag          
        else:
            PlaceTime.__init__(self, node)
        

class Arrival (PlaceTime):
    def __init__(self, node):
        pattern=nspattern+"arrival"
        if not re.match(pattern, node.tag):
            raise Exception,"Arrival Node ERROR : %s not allowed here"%node.tag          
        else:
            PlaceTime.__init__(self, node)

class Airport:
    code=""
    name=""
    city=""
    country=""
    
    def __init__(self, node):
        if node.tag=="airport":
            pass
        else:
            raise Exception,"Node ERROR"
        
    def __unicode__(self):
        return "%s, %s, %s" % (self.name,self.city, self.country)
        
    
class Location:
    name=""
    airport=None
    gate=""
    def __init__(self):
        pass
    def __init__(self, node):
        if node.tag=="location":
            pass
        else:
            raise Exception,"Node ERROR"

class Flight:
    name=""
    status=""
    departure=None
    arrival=None
    ns=None
    def __init__(self, node):
        """If it's not the good node name"""
        #TODO : a better way to find the tag without the namespace
    
        # tag with namespace pattern like {namespace}tag
        pattern=nspattern+"flight"
        
        #the if statement in comment Works just without namespace, so we try to get it by a regexp
        #if not node.tag=="flight":   
        if not re.match(pattern, node.tag):
            raise Exception,"Flight Node ERROR : %s not allowed here"%node.tag          
        else:
            # Trying to find the namespace (uggly way)
            c=re.compile(nspattern)
            m=c.match(node.tag)
            if m:
                if m.group()!='' and m.group()!=None:
                    self.ns=m.group()
                    self.ns=self.ns.strip('{}')
                else:
                    print "Can't find ns"
                    
            """If node don't contains expected attributes"""
            if not "name" in node.keys():
                raise Exception,"Attribute ERROR"
            if not "status" in node.keys():
                raise Exception,"Attribute ERROR"
            # List comprehension + instrospection = :)
            [setattr(self, attr,value) for attr,value in node.items()]
            
            #We have a namespace, modifying XPATH query in consequence
            if self.ns is  not None and self.ns is not '':
                depXp="//f:departure"
                arXp="//f:arrival"
                depNode=node.xpath(depXp,namespaces={"f":self.ns})
                arNode=node.xpath(arXp,namespaces={"f":self.ns})
            #we don't have a namsepace, simple XPATH query
            else:
                depXp="//departure"
                arXp="//arrival"
                depNode=node.xpath(depXp)
                arNode=node.xpath(arXp)
            self.departure=Departure(depNode[0])
            self.arrival=Arrival(arNode[0])
            