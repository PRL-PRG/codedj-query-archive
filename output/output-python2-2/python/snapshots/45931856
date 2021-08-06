import lxml
import re

nspattern="(\{.*\}){0,1}"
class PlaceTime:
    timestamp=""
    location=""
    def __init__(self, node):
        if not "timestamp" in node.keys():
            raise Exception,"PlaceTime Attribute ERROR : a PlaceTime node may have a 'timestamp' attribute."
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
    def __init__(self, node):
        """If it's not the good node name"""
        #TODO : a better way to find the tag without the namespace
    
        # tag with namespace pattern like {namespace}tag
        pattern=nspattern+"flight"
        
        #the if statement in comment Works just without namespace
        #if not node.tag=="flight":
        if not re.match(pattern, node.tag):
            raise Exception,"Flight Node ERROR : %s not allowed here"%node.tag          
        else:
            """If node don't contains expected attributes"""
            if not "name" in node.keys():
                raise Exception,"Attribute ERROR"
            if not "status" in node.keys():
                raise Exception,"Attribute ERROR"
            # List comprehension + instrospection = :)
            [setattr(self, attr,value) for attr,value in node.items()] 
            #self.departure=Departure()
            #self.arrival=Arrival()