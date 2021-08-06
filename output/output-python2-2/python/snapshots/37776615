class UMLException(Exception):
    """
    Exception used for marking incorrect user operations
    """
    def __init__(self, name, params = ()):
        """
        Initialize this exception and set its values
        
        @param name: ID of an exception
        @type  name: string
        
        @param params: parameters for exception
        @type  params: tuple of anything, or anything
        """
        self.name = name
        self.params = params
    
    def GetName(self):
        """
        Get the ID of this exception
        
        @return: exception ID
        @rtype:  string
        """
        return self.name
    
    def GetParam(self, index):
        """
        Get parameter by given index
        
        @param index: index of parameter
        @type  index: integer
        
        @return: parameter value
        @rtype:  anything
        """
        if isinstance(self.params, (list, tuple)):
            return self.params[index]
        else:
            if index == 0:
                return self.params
            else:
                return None
    
    def __str__(self):
        """
        Create string representation of the exception
        
        @return: string representation
        @rtype:  string
        """
        return self.name+" "+str(self.params)
    
    def __repr__(self):
        """
        Get name of this class
        
        @return: class name
        @rtype:  string
        """
        return self.__class__.__name__


def ToBool(val):
    """
    Convert any value to boolean
    
    @param val: value to convert
    @type  val: string
    
    @return: boolean value of given parameter
    @rtype:  boolean
    """
    if type(val) in (str, unicode):
        return val.lower() in ('1', 'yes', 'true')
    else:
        return val == True

def XMLEncode(val):
    """
    Encode given parameter for usage in XML files
    
    @param val: normal string
    @type  val: string
    
    @return: xml encoded value
    @rtype:  string
    """
    ret = repr(val)
    if isinstance(val, str):
        ret = ret[1:-1]
    elif isinstance(val, unicode):
        ret = ret[2:-1]
    return ret.replace('&', '&amp;').replace('<', '&lt;').replace('>', '&gt;').replace('<', '&gt;').replace('"', '&quot;').encode('utf8')

class IDGenerator:
    """
    ID number generator for group of objects
    """
    def __init__(self):
        """
        Initialize generator to empty
        """
        self.ids = {}
    
    def id(self, obj):
        """
        Calculate ID for given object
        
        @param obj: object
        @type  obj: anything
        
        @return: ID of given object
        @rtype:  integer
        """
        return self.ids.setdefault(id(obj), len(self.ids))
    
    __call__ = id

