from lib.Exceptions.UserException import *

class CConnectionObject(object):
    """
    Object that represents logical connection and its properties
    """
    def __init__(self, type, source, dest):
        """
        Initialize connection object
        
        @param type: Type of connection
        @type  type: L{CConnectionType<Type.CConnectionType>}
        
        @param source: Source element of connection
        @type  source: L{CElementObject<lib.Elements.Object.CElementObject>}
        
        @param dest: Destination element of connection
        @type  dest: L{CElementObject<lib.Elements.Object.CElementObject>}
        """
        self.source = None
        self.destination = None
        self.revision = 0
        self.appears = []
        self.type = type
        self.SetSource(source)
        try:
            self.SetDestination(dest)
        except:
            if self.source is not None:
                self.source.RemoveConnection(self)
            self.source = None
            raise
        self.attributes = {}
        for i in self.type.GetAttributes():
            self.SetAttribute(i, self.type.GetDefValue(i))
    
    def __CheckRecursiveConnection(self):
        """
        Validate connection for recursion
        """
        source = self.source
        dest = self.destination
        type = self.type
        
        if source is None or dest is None:
            return True
        
        if source is not dest:
            return True
        typeid = type.GetId()
        destid = dest.GetType().GetId()
        allow = dict(source.GetType().GetConnections())
        withelem, allowrecursive = allow.get(typeid, (None, False))
        if allowrecursive and (withelem is None or '*' in withelem  or destid in withelem):
            return True
        return False
        
    def __CheckConnection(self, reversed):
        """
        Validate connection
        """
        if reversed:
            source = self.source
            dest = self.destination
        else:
            source = self.destination
            dest = self.source
        type = self.type
        
        if source is None or dest is None:
            return True
        
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
    
    def __DoCheck(self):
        """
        Do all validations
        
        @raise ConnectionRestrictionError: if there is something wrong
        """
        if not self.__CheckRecursiveConnection():
            raise ConnectionRestrictionError
        checksrc = self.__CheckConnection(False)
        checkdest = self.__CheckConnection(True)
        if not (checksrc or checkdest or (checksrc is checkdest is None)):
            raise ConnectionRestrictionError
    
    def GetRevision(self):
        """
        Get the revision number for this connection.
        Revision number increses after each change in connection
        object.
        
        @return: Revision number
        @rtype:  integer
        """
        return self.revision
    
    def GetAppears(self):
        """
        Gets all diagrams, this connection appers on
        
        @rtype:  iterator over L{CDiagram<lib.Drawing.Diagram.CDiagram>}
        """
        for i in self.appears:
            yield i

    def AddAppears(self, diagram):
        """
        Add diagram, connection is appeared on
        
        @param diagram: Diagram
        @type  diagram: L{CDiagram<lib.Drawing.Diagram.CDiagram>}
        """
        self.appears.append(diagram)

    def RemoveAppears(self, diagram):
        """
        Remove diagram, connection was appeared on, from the list
        
        @param diagram: Diagram
        @type  diagram: L{CDiagram<lib.Drawing.Diagram.CDiagram>}
        
        @raise ValueError: if given diagram is not found
        """
        self.appears.remove(diagram)

    def GetType(self):
        """
        Return type of connection
        
        @return: Type of this connection
        @rtype:  L{CConnectionType<Type.CConnectionType>}
        """
        return self.type
    
    def SetType(self, value):
        """
        Set type for this connection
        
        @param value: New type for this connection
        @type  value: L{CConnectionType<Type.CConnectionType>}
        """
        self.type = value
    
    def GetConnectedObject(self, object):
        """
        Get object that is connected through this connection to another object
        
        @param object: known object
        @type  object: L{CElementObject<lib.Elements.Object.CElementObject>}
        
        @return: other object
        @rtype:  L{CElementObject<lib.Elements.Object.CElementObject>}
        """
        if self.source is object:
            return self.destination
        elif self.destination is object:
            return self.source
        else:
            return None
        
    def GetDestination(self):
        """
        Get destination of this connection
        
        @return: connection destination
        @rtype:  L{CElementObject<lib.Elements.Object.CElementObject>}
        """
        return self.destination

    def GetSource(self):
        """
        Get source of this connection
        
        @return: connection source
        @rtype:  L{CElementObject<lib.Elements.Object.CElementObject>}
        """
        return self.source

    def SetDestination(self, dest):
        """
        Set destination object of this connection
        
        @param dest: object which has to be set as destination
        @type  dest: L{CElementObject<lib.Elements.Object.CElementObject>}
        """
        if self.destination and self.destination is not self.source:
            self.destination.RemoveConnection(self)
        old = self.destination
        self.destination = dest
        try:
            self.__DoCheck()
        except:
            self.destination = old
            raise
        if dest is not None:
            dest.AddConnection(self)
        self.revision += 1

    def SetSource(self, source):
        """
        Set source object of this connection
        
        @param source: object which has to be set as source
        @type  source: L{CElementObject<lib.Elements.Object.CElementObject>}
        """
        if self.source and self.destination is not self.source:
            self.source.RemoveConnection(self)
        old = self.source
        self.source = source
        try:
            self.__DoCheck()
        except:
            self.source = old
            raise
        if source is not None:
            source.AddConnection(self)
        self.revision += 1
    
    def Disconnect(self):
        """
        Disconnect self from other objects
        """
        if self.source is self.destination:
            self.source.RemoveConnection(self)
        else:
            self.source.RemoveConnection(self)
            self.destination.RemoveConnection(self)            
    
    def Paint(self, canvas, connection, delta = (0, 0)):
        """
        Paint self on canvas
        
        @param canvas: connection will be painted on this canvas
        @type  canvas: L{CAbstractCanvas<lib.Drawing.Canvas.Abstract.CAbstractCanvas>}
        
        @param connection: connection object which has to be painted on canvas
        @type  connection: L{CAbstractCanvas<lib.Drawing.Connection.CConnection>}
        
        @param delta: translation of point (0, 0)
        @type  delta: (integer, integer)
        """
        self.type.Paint(canvas, connection, delta)
    
    def GetAttributes(self):
        """
        Iterator over attribute names
        
        @return: name of attributes
        @rtype:  iterator over strings
        """
        for attr in self.attribs:
            yield attr
        
    def GetAttribute(self, key):
        """
        Get the attribute value
        
        @param key: name of the attribute
        @type  key: string
        
        @return: attribute value
        @rtype:  anything
        """
        if key in self.attributes:
            return self.attributes[key]
        else:
            raise UMLException("BadKey")
    
    def SetAttribute(self, key, value):
        """
        Set the attribute value
        
        @param key: name of the attribute
        @type  key: string
        
        @param value: new value for the attribute
        @type  value: anything
        """
        self.revision += 1
        self.attributes[key] = value        
    
    def RemoveAttribute(self, key):
        """
        Remove the attribute by given name
        
        @param key: name of the attribute
        @type  key: string
        """
        self.revision += 1
        if key in self.attributes:
            del self.attributes[key]
        else:
            raise UMLException("BadKey")
    
    def GetVisualProperty(self, key):
        """
        Get property value
        
        @param key: property ID
        @type  key: string
        
        
        @return: property value
        @rtype:  anything
        """
        return self.attributes[self.type.GetVisAttr(key)]
        
    Source = property(GetSource, SetSource)
    Destination = property(GetDestination, SetDestination)