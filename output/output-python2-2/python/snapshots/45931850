import lxml
from lxml import etree
import re

nspattern="(\{.*\}){0,1}"

class Mappable(object):
    def __init__(self, node):
        #Getting all declared attributes
        attList = [att for att in dir(self.__class__) if not callable(getattr(self.__class__, att)) and not att.startswith("__")]
        #Cheking ifattributes are present in XML node
        for att in attList:
            if not att in node.keys():
                raise Exception,"%s Attribute ERROR : a %s object may be construct with node who have a '%s' attribute."%(self.__class__,self.__class__,att)
        [setattr(self, attr,value) for attr,value in node.items()]
        
        
class PlaceTime(Mappable):
    datetime=""
    location=""
    def __init__(self, node):
        super(PlaceTime,self).__init__(node)        
        # List comprehension + instrospection = :)
        
    
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

class Airport(Mappable):
    code=""
    name=""
    city=""
    country=""
    
    def __init__(self, node):
        pattern=nspattern+"airport"
        if not re.match(pattern, node.tag):
            raise Exception,"Node ERROR"
        else:
            super(Airport,self).__init__(node)
            
    def __unicode__(self):
        return "%s, %s, %s" % (self.name,self.city, self.country)
        
    
class Location(Mappable):
    name=""
    airport=None
    gate=""
    def __init__(self):
        pass
    def __init__(self, node):
        if node.tag=="location":
            super(Location,self).__init__(node)
        else:
            raise Exception,"Node ERROR"

class Flight(Mappable):
    name=""
    status=""
    __departure__=None
    __arrival__=None
    __ns__=None
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
                    self.__ns__=m.group()
                    self.__ns__=self.__ns__.strip('{}')
                    
            super(Flight,self).__init__(node)
            
            #We have a namespace, modifying XPATH query in consequence
            if self.__ns__ is  not None and self.__ns__ is not '':
                depXp="//f:departure"
                arXp="//f:arrival"
                depNode=node.xpath(depXp,namespaces={"f":self.__ns__})
                arNode=node.xpath(arXp,namespaces={"f":self.__ns__})
            #we don't have a namsepace, simple XPATH query
            else:
                depXp="//departure"
                arXp="//arrival"
                depNode=node.xpath(depXp)
                arNode=node.xpath(arXp)
            
            #Creating the departure and arrival objects
            self.__departure__=Departure(depNode[0])
            self.__arrival__=Arrival(arNode[0])
            