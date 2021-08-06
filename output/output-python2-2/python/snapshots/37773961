class CConnectionObject(object):
    def __init__(self, type, source = None, dest = None):
        self.type = type
        self.SetSource(source)
        self.SetDestination(dest)
        self.attributes = {}
        for i in self.type.GetAttributes():
            self.SetAttribute(i, self.type.GetDefValue(i))            

    def GetType(self):
        return self.type
    
    def SetType(self, value):
        self.type = value

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
    
    def Paint(self, canvas, Connection):
        self.type.Paint(canvas, Connection)
    
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