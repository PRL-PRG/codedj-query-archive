from Exceptions import EConnectionRestriction

class CConnectionObject(object):
    def __init__(self, type, source, dest):
        if not self.__CheckConnection(type, source, dest):
            raise EConnectionRestriction
        self.appears = []
        self.type = type
        self.SetSource(source)
        self.SetDestination(dest)
        self.attributes = {}
        for i in self.type.GetAttributes():
            self.SetAttribute(i, self.type.GetDefValue(i))
            
    def __CheckConnection(self, type, source, dest):
        allowsrc = dict(source.GetType().GetConnections())
        allowdest = dict(dest.GetType().GetConnections())
        typeid = type.GetId()
        srcid = source.GetType().GetId()
        destid = dest.GetType().GetId()
        return True
        
    
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
        
    Source = property(GetSource, SetSource)
    Destination = property(GetDestination, SetDestination)