from lib.lib import ToBool
from lib.Exceptions.UserException import *

class CElementType(object):
    '''
    Scheme for a class of elements
    '''
    
    def __init__(self, id):
        '''
        create new instance of element type
        '''
        self.icon = None
        self.id = id
        self.attributes = {}
        self.connections = {}
        self.appearance = None
        self.attributeList = []
        self.generatename = True
        self.domain = None
        self.counter = 0
        self.options = {}
    
    def SetDomain(self, domain):
        '''
        @param domain: domain type that holds info about data
        @type domain: L{CDomainType<lib.Domains.Type.CDomainType>}
        '''
        self.domain = domain
    
    def GetDomain(self):
        '''
        @return: current domain type
        @rtype: L{CDomainType<lib.Domains.Type.CDomainType>}
        '''
        return self.domain
    
    def AppendOptions(self, name, value):
        self.options[name] = value
    
    def GetOptions(self):
        return self.options
    
    def GenerateName(self):
        '''
        @return: new name for object, name
        @rtype: str
        '''
        self.counter += 1
        return self.id + str(self.counter)
        
    def GetCounter(self):
        '''
        @return: current value of counter
        @rtype: int
        '''
        return self.counter
    
    def SetCounter(self, value):
        '''
        set new value to counter
        
        @param value: new value of counter
        @type value: int
        '''
        assert type(value) in (int, long)
        self.counter = value
    
    def AppendConnection(self, value, withobject, allowrecursive):
        '''
        add allowed connection as defined in metamodel
        '''
        self.connections[value] = (withobject, allowrecursive)
    
    def GetAppearance(self):
        '''
        @return: appearance as was defined in metamodel
        '''
        return self.appearance
                
    def GetConnections(self):
        '''
        iterator over allowed connections
        
        @return: tuple of values (withobject, allowrecursive)
        '''
        for item in self.connections.iteritems():
            yield item
    
    def GetIcon(self):
        '''
        @return: relative path to the icon of current type
        @rtype: str
        '''
        return self.icon
    
    def GetId(self):
        '''
        @return: ID or name of the element type as used in metamodel
        @rtype: str
        '''
        return self.id
    
    def GetResizable(self):
        '''
        @return: True if element can be resized - depends on the uppermost
        authoritative visual object.
        @rtype: bool
        '''
        return self.appearance.GetResizable()
    
    def Paint(self, context):
        '''
        Paint element type using context
        '''
        self.appearance.Paint(context)
    
    def SetAppearance(self, appearance):
        '''
        Set appearance as defined in metamodel
        '''
        self.appearance = appearance
    
    def SetIcon(self, pixbuf):
        '''
        set relative path to the icon
        '''
        self.icon = pixbuf
    
    def SetId(self, id):
        '''
        set identifier / name of the current element type
        '''
        self.id = id
    
    def GetSize(self, context):
        '''
        @return: size as tuple {width, height)
        '''
        return self.appearance.GetSize(context)
    
    def HasVisualAttribute(self, name):
        '''
        @note: This is fake function for interface compatibility reasons
        
        @return: True if name points to anything but "text" domain attribute
        @rtype: bool
        '''
        return self.GetDomain().GetAttribute(name)['type'] != 'text'
