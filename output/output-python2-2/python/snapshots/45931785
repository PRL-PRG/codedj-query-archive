import lxml
from lxml import etree
import re

nspattern = "(\{.*\}){0,1}"
"""
Remarque :
Les attributs de chaque classe dont le nom commence par ___
representent des sous-noeuds dans l'arborescence xml et
doivent avoir une methode toXML() (heritee de Mappable ou surchargee).

Les attributs de chaque classe dont le nom commence par __
ne sont que des attributs prives, en accord avec la notation
python.
"""

class Mappable(object):
    __attList__ = []
    __subnodeattlist__ = []
    __nodename__ = ""
    __ns__ = None
        
    def __init__(self, node):
        if node is not None:
            #Getting all declared attributes
            self.__attList__ = [att for att in dir(self.__class__) if not callable(getattr(self.__class__, att)) and not att.startswith("__")]
            # Checks if attributes are present in XML node
            for att in self.__attList__:
                if not att in node.keys():
                    raise Exception, "%s Attribute ERROR : a %s object may be construct with node who have a '%s' attribute." % (self.__class__, self.__class__, att)
            [setattr(self, attr, value) for attr, value in node.items()]
    def toXML(self):
        # Gets attributes whose name starts with ___
        self.__subnodeattlist__ = \
            [att for att in dir(self.__class__) \
            if not callable(getattr(self.__class__, att)) \
               and att.startswith("___")]
        ret = "<%s " % self.__nodename__

        #if self.__ns__ is not None and self.__ns__ != "":
        #    ret+="xmlns=\"%s\" "%self.__ns__
        for attr in self.__attList__:
            if getattr(self, attr) != None:
                ret += "%s=\"%s\" " % (attr, getattr(self, attr))
        ret += ">"

        for subnode in self.__subnodeattlist__:
            if getattr(self, subnode) != None:
                ret += str(getattr(self, subnode).toXML())
                
        ret += "</%s>" % self.__nodename__
        return ret

    # Hash method
    # Permits to call, for example, Object.___airport___
    # as Object["airport"] 
    def __getitem__(self, attr):
        try:
            return getattr(self, attr)
        except:
            return getattr(self, "___%s___" % attr)
    
class PlaceTime(Mappable):
    datetime = ""
    location = ""
    def __init__(self, node):
        super(PlaceTime, self).__init__(node)

class Departure (PlaceTime):
    def __init__(self, node):
        self.__nodename__ = "departure"
        pattern = nspattern + self.__nodename__
        if not re.match(pattern, node.tag):
            raise Exception, "Departure Node ERROR : %s not allowed here" % node.tag          
        else:
            PlaceTime.__init__(self, node)
        

class Arrival (PlaceTime):
    def __init__(self, node):
        self.__nodename__ = "arrival"
        pattern = nspattern + self.__nodename__
        if not re.match(pattern, node.tag):
            raise Exception, "Arrival Node ERROR : %s not allowed here" % node.tag          
        else:
            PlaceTime.__init__(self, node)

class Airport(Mappable):
    code = ""
    name = ""
    city = ""
    country = ""
    def __init__(self, node):
        self.__nodename__ = "airport"
        pattern = nspattern + self.__nodename__
        
        if not re.match(pattern, node.tag):
            raise Exception, "Node ERROR"
        else:
            super(Airport, self).__init__(node)
    def __unicode__(self):
        return "%s, %s, %s" % (self.name, self.city, self.country)
        
class Gate(Mappable):
    name = None
    def __init__(self, node):
        self.__nodename__ = "gate"
        super(Gate, self).__init__(node)
        pattern = nspattern + self.__nodename__
        if not re.match(pattern, node.tag):
            raise Exception, "Location Node ERROR : %s not allowed here" % node.tag
        else:
            self.name = node.get("name")
class Coordinate(Mappable):
    lat = None
    long = None
    z = None
    def __init__(self, lat, long, z):
        super(Coordinate, self).__init__(None)
        self.__nodename__ = "coordinates"
        self.lat = lat
        self.long = long
        self.z = z
    def toXML(self):
        return "<%s>%s,%s,%s</%s>" % (self.__nodename__, self.lat, self.long, self.z, self.__nodename__)

class Location(Mappable):
    name = ""
    ___airport___ = None
    ___gate___ = None
    ___cordinates___ = None    
    def __init__(self, node):
        self.__nodename__ = "location"
        pattern = nspattern + self.__nodename__
        if not re.match(pattern, node.tag):
            raise Exception, "Location Node ERROR : %s not allowed here" % node.tag
        else:
            super(Location, self).__init__(node)
            # Trying to find the namespace (uggly way)
            c = re.compile(nspattern)
            m = c.match(node.tag)
            if m:
                if m.group() != '' and m.group() != None:
                    self.__ns__ = m.group()
                    self.__ns__ = self.__ns__.strip('{}')
                    
            #We have a namespace, modifying XPATH query in consequence
            if self.__ns__ is  not None and self.__ns__ is not '':
                gateXp = "./f:gate"
                airXp = "./f:airport"
                gateNode = node.xpath(gateXp, namespaces={"f":self.__ns__})
                airNode = node.xpath(airXp, namespaces={"f":self.__ns__})
            #we don't have a namsepace, simple XPATH query
            else:
                gateXp = "./gate"
                airXp = "./airport"
                gateNode = node.xpath(gateXp)
                airNode = node.xpath(airXp)
            if len(gateNode) == 1:
                self.___gate___ = Gate(gateNode[0])
            if not len(airNode) == 1:
                raise Exception, "A location may be instanciated with a node containig an <airport/> child, %s present here" % len(airNode)
            else:
                self.___airport___ = Airport(airNode[0])
    def setCoordinates(self, lat, long, z):
        self.___cordinates___ = Coordinate(lat, long, z)
    def __str__(self):
        a = self.___airport___
        return "%s, %s, %s" % (a.name, a.city, a.country)
class Flight(Mappable):
    name = ""
    status = ""
    
    # Attributes beginning with ___ are built from subnodes
    ___departure___ = None
    ___arrival___ = None
    
    
    def __init__(self, node):
        self.__nodename__ = "flight"
        """If it's not the good node name"""
        # TODO: a better way to find the tag without the namespace
    
        # tag with namespace pattern like {namespace}tag
        pattern = nspattern + self.__nodename__
        
        #the if statement in comment Works just without namespace, so we try to get it by a regexp
        #if not node.tag=="flight":   
        if not re.match(pattern, node.tag):
            raise Exception, "Flight Node ERROR : %s not allowed here" % node.tag          
        else:
            super(Flight, self).__init__(node)
            # Trying to find the namespace (uggly way)
            c = re.compile(nspattern)
            m = c.match(node.tag)
            if m:
                if m.group() != '' and m.group() != None:
                    self.__ns__ = m.group()
                    self.__ns__ = self.__ns__.strip('{}')
                    
            
            #We have a namespace, modifying XPATH query in consequence
            if self.__ns__ is  not None and self.__ns__ is not '':
                depXp = "./f:departure"
                arXp = "./f:arrival"
                depNode = node.xpath(depXp, namespaces={"f":self.__ns__})
                arNode = node.xpath(arXp, namespaces={"f":self.__ns__})
            #we don't have a namsepace, simple XPATH query
            else:
                depXp = "./departure"
                arXp = "./arrival"
                depNode = node.xpath(depXp)
                arNode = node.xpath(arXp)
            
            #Creating the departure and arrival objects
            self.___departure___ = Departure(depNode[0])
            self.___arrival___ = Arrival(arNode[0])

class FlightsTracking(Mappable):
    ___flights___ = []
    ___locations___ = []
    __nodename__ = "flightsTracking"
    def __init__(self, node):
        """If it's not the good node name"""
        # TODO : a better way to find the tag without the namespace
        
        # tag with namespace pattern like {namespace}tag
        pattern = nspattern + self.__nodename__
        if not re.match(pattern, node.tag):
            raise Exception, "flightsTracking Node ERROR: %s not allowed here" % node.tag          
        else:
            if not re.match(pattern, node.tag):
                raise Exception, "Flight Node ERROR: %s not allowed here" % node.tag          
            else:
                super(FlightsTracking, self).__init__(node)
                # Trying to find the namespace (uggly way)
                c = re.compile(nspattern)
                m = c.match(node.tag)
                if m:
                    if m.group() != '' and m.group() != None:
                        self.__ns__ = m.group()
                        self.__ns__ = self.__ns__.strip('{}')
            
            
            if self.__ns__ is  not None and self.__ns__ is not '':
                flightsXp = "//f:flights/f:flight"
                locationsXp = "//f:locations/f:location"
                
                flightsNode = node.xpath(flightsXp, namespaces={"f":self.__ns__})
                locationsNode = node.xpath(locationsXp, namespaces={"f":self.__ns__})
            else:
                flightsXp = "//flights/flight"
                locationsXp = "//locations/location"
                
                flightsNode = node.xpath(flightsXp)
                locationsNode = node.xpath(locationsXp)
            for fnode in flightsNode:
                f = Flight(fnode)
                self.___flights___.insert(- 1, f)            
            for lnode in locationsNode:
                l = Location(lnode)
                self.___locations___.insert(- 1, l)
    def toXML(self):
        # TODO
        self.__subnodeattlist__ = [att for att in dir(self.__class__) if not callable(getattr(self.__class__, att)) and att.startswith("___")]
        
        ret = "<%s " % self.__nodename__
        
        if self.__ns__ is not None and self.__ns__ != "":
            ret += "xmlns=\"%s\" " % self.__ns__
        for attr in self.__attList__:
            if getattr(self, attr) != None:
                ret += "%s=\"%s\" " % (attr, getattr(self, attr))
        ret += ">"
        
        ret += "<flights>"
        for x in self.___flights___:
            ret += str(x.toXML())
        ret += "</flights>"
        ret += "<locations>"
        for y in self.___locations___:
            ret += str(y.toXML())
        ret += "</locations>"    
            
        ret += "</%s>" % self.__nodename__
        return ret