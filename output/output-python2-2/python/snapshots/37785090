from Exceptions import EConnectionRestriction
from lib.lib import UMLException

class CConnectionObject(object):
    def __init__(self, type, source, dest):
        if not self.__CheckRecursiveConnection(type, source, dest):
            raise EConnectionRestriction
        checksrc = self.__CheckConnection(type, source, dest)
        checkdest = self.__CheckConnection(type, dest, source)
        if not (checksrc or checkdest or (checksrc is checkdest is None)):
            raise EConnectionRestriction
        self.appears = []
        self.type = type
        self.SetSource(source)
        self.SetDestination(dest)
        self.attributes = {}
        for i in self.type.GetAttributes():
            self.SetAttribute(i, self.type.GetDefValue(i))
            
    def __CheckRecursiveConnection(self, type, source, dest):
        if source is not dest:
            return True
        typeid = type.GetId()
        destid = dest.GetType().GetId()
        allow = dict(source.GetType().GetConnections())
        withelem, allowrecursive = allow.get(typeid, (None, False))
        if allowrecursive and (withelem is None or '*' in withelem  or destid in withelem):
            return True
        return False
        
    def __CheckConnection(self, type, source, dest):
        typeid = type.GetId()
        destid = dest.GetType().GetId()
        allow = dict(source.GetType().GetConnections())
        if typeid in allow:
            withelem, allowrecursive = allow[typeid]
            if withelem is None:
                return None
            elif '*' in withelem or destid in withelem:
                return True
        return False
        
    
    def GetAppears(self):
        for i in self.appears:
            yield i

    def AddAppears(self, drawingArea):
        self.appears.append(drawingArea)

    def RemoveAppears(self, drawingArea):
        self.appears.remove(drawingArea)

    def GetType(self):
        return self.type
    
    def SetType(self, value):
        self.type = value
    
    def GetConnectedObject(self, object):
        if self.source is not object:
            return self.source
        else:
            return self.destination
        
    def GetDestination(self):
        return self.destination

    def GetSource(self):
        return self.source

    def SetDestination(self, dest):
        if dest is not None:
            dest.AddConnection(self)
        self.destination = dest

    def SetSource(self, source):
        if source is not None:
            source.AddConnection(self)
        self.source = source
    
    def Disconnect(self):
        if self.source is self.destination:
            self.GetSource().RemoveConnection(self)
        else:
            self.GetSource().RemoveConnection(self)
            self.GetDestination().RemoveConnection(self)            
    
    def Paint(self, canvas, Connection, delta = (0, 0)):
        self.type.Paint(canvas, Connection, delta)
    
    def GetAttributes(self):
        for attr in self.attribs:
            yield attr
        
    def GetAttribute(self, key):
        if key in self.attributes:
            return self.attributes[key]
        else:
            raise UMLException("BadKey")
    
    def SetAttribute(self, key, value):
        self.attributes[key] = value        
    
    def RemoveAttribute(self, key):
        if key in self.attributes:
            del self.attributes[key]
        else:
            raise UMLException("BadKey")
    
    def GetVisualProperty(self, key):
        return self.attributes[self.type.GetVisAttr(key)]

    def GetProperty(self, key = None):
        o = {}
        #~ o['name'] = self.GetAttribute('Name')
        print "@@@", self.attributes
        if self.attributes.has_key('Name'):
            print "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"
            o['name'] = self.attributes['Name']
        else:
            o['name'] = ""
        o['type'] = self.type.GetId()
        o['dest'] = self.GetDestination()
        o['source'] = self.GetSource()
        if key is not None:
            return o[key]
        return o
        
    Source = property(GetSource, SetSource)
    Destination = property(GetDestination, SetDestination)