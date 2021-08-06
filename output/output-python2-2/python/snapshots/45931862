import libxml2

class PlaceTime:
    timestamp=""
    location=""
    def __init__(self):
        pass
    def __init__(self, node):
        pass
    
class Departure (PlaceTime):
    def __init__(self, node):
        if node.name=="departure":
            PlaceTime.__init__(self, node)
        else:
            raise Exception,"Node ERROR"

class Arrival (PlaceTime):
    def __init__(self, node):
        if node.name=="arrival":
            PlaceTime.__init__(self, node)
        else:
            raise Exception,"Node ERROR"

class Airport:
    code=""
    name=""
    city=""
    country=""
    
    def __init__(self):
        pass
    def __init__(self, node):
        if node.name=="airport":
            pass
        else:
            raise Exception,"Node ERROR"
        
    def getFormatedAdress(self):
        return "%s, %s, %s" % (self.name,self.city, self.country)
        
    
class Location:
    name=""
    airport=None
    gate=""
    def __init__(self):
        pass
    def __init__(self, node):
        if node.name=="location":
            pass
        else:
            raise Exception,"Node ERROR"

    
class Flight:
    name=""
    status=""
    departure=None
    arrival=None
    def __init__(self):
        pass
    def __init__(self, node):
        if node.name=="flight":
            pass
        else:
            raise Exception,"Node ERROR"
    def toXML(self):
        pass