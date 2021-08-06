from lib.Exceptions.UserException import *
import weakref
from lib.Domains import CDomainObject

class CElementObject(object):
    """
    Object that represents logical element and its properties
    """
    def __init__(self, type):
        """
        Initialize element object and set it into default state
        
        @param type: Type of the new element
        @type  type: L{CElementType<Type.CElementType>}
        """
        self.revision = 0
        self.type = type
        self.domainobject = CDomainObject(self.type.GetDomain())
        self.path = None
        self.connections = []
        self.node = lambda: None
        self.appears = []
        if self.domainobject.GetType().HasAttribute('name'):
            self.domainobject.SetValue('name',self.type.GenerateName())
    
    def GetRevision(self):
        """
        Get revision of this object. Revision is incremented after each
        object state chage
        
        @return: Object revision
        @rtype:  integer
        """
        return self.revision
    
    def AddRevision(self):
        """
        Increase revision on external change (Like movement in project tree)
        """
        
        self.revision += 1
    
    def GetAppears(self):
        """
        Get list of object appearances on diagrams
        
        @return: list of diagrams
        @rtype:  iterator over L{CDiagram<lib.Drawing.Diagram.CDiagram>}
        """
        for i in self.appears:
            yield i

    def AddAppears(self, diagram):
        """
        Add element appearance
        
        @param diagram: Diagram on which element appears
        @type  diagram: L{CDiagram<lib.Drawing.Diagram.CDiagram>}
        """
        self.appears.append(diagram)

    def RemoveAppears(self, diagram):
        """
        Remove element appearance
        
        @param diagram: Diagram on which element was appearing
        @type  diagram: L{CDiagram<lib.Drawing.Diagram.CDiagram>}
        
        @raise ValueError: if given diagram is not found
        """
        self.appears.remove(diagram)
    
    def GetPath(self):
        """
        Get path of this element object in the project
        
        @return: Element object path
        @rtype:  string
        """
        return self.path
    
    def SetPath(self, path):
        """
        Set path of this element object in the project
        
        @param path: Element object path
        @type  path:  string
        """
        self.path = path 
    
    def GetNode(self):
        """
        Get the project node which is associated with this element object
        
        @return: project node of this element
        @rtype:  L{CProjectNode<lib.Project.Node.CProjectNode>}
        """
        return self.node
        
    def AddConnection(self, connection):
        """
        Add the connection object connected to this element object
        
        @param connection: connected connection
        @type  connection: L{CConnectionObject<lib.Connections.Object.CConnectionObject>}
        """
        self.revision += 1
        if connection not in self.connections:
            self.connections.append(connection)
        else:
            pass
            
    def GetConnections(self):
        """
        Get list of connections connected to this object
        
        @return: List of connected connections
        @rtype:  iterator over L{CConnectionObject<lib.Connections.Object.CConnectionObject>}(s)
        """
        for c in self.connections:
            yield c
        
    def GetType(self):
        return self.type
    
    def GetSize(self, context):
        return self.type.GetSize(context)
    
    def GetDomainName(self, key=''):
        return self.domainobject.GetDomainName(key)
    
    def GetDomainType(self, key=''):
        return self.domainobject.GetType(key)
    
    def GetValue(self, key):
        return self.domainobject.GetValue(key)
    
    def SetValue(self, key, value):
        self.domainobject.SetValue(key, value)
        self.revision += 1
        
    def GetSaveInfo(self):
        return self.domainobject.GetSaveInfo()
    
    def SetSaveInfo(self, value):
        return self.domainobject.SetSaveInfo(value)
        
    def GetName(self):
        return self.domainobject.GetValue('name')
    
    def GetVisualProperty(self, key):
        if key == 'CHILDREN':
            node = self.node()
            if node is None:
                return []
            v = []
            for vi in node.GetChilds():
                o = {}
                o['icon'] = vi.GetObject().GetType().GetIcon()
                o['name'] = vi.GetObject().GetName()
                v.append(o)
            return v
        
        else:
            return self.domainobject.GetValue(key)
    
    def HasVisualAttribute(self, key):
        return self.domainobject.HasVisualAttribute(key)

    def Paint(self, context):
        self.type.Paint(context)

    def Disconnect(self, connection):
        connection.Disconnect()
        
    def RemoveConnection(self, connection):
        self.revision += 1
        if connection in self.connections:
            self.connections.remove(connection)
        else:
            raise ConnectionError("ConnectionNotFound")
    
    def AppendItem(self, key):
        self.domainobject.AppendItem(key)
        self.revision += 1
    
    def RemoveItem(self, key):
        self.domainobject.RemoveItem(key)
        self.revision += 1
