#try to import necessary lybraries for XML parsing
try:
    from lxml import etree
    HAVE_LXML = True
except ImportError:
    HAVE_LXML = False
    try:
        # Python 2.5
        import xml.etree.cElementTree as etree
    except ImportError:
        try:
            # Python 2.5
            import xml.etree.ElementTree as etree
        except ImportError:
            try:
                # normal cElementTree install
                import cElementTree as etree
            except ImportError:
                # normal ElementTree install
                import elementtree.ElementTree as etree

def Indent(elem, level=0):
    """
    The indent function is a variant of the one in Fredrik Lundh's effbotlib.
    This function make XML Tree more human friendly.
    
    @param  elem: XML element to parse
    @type   elem: L{Element<xml.etree.ElementTree.Element>}
    
    @param  level: level of element
    @type   level: integer
    """
    i = "\n" + level*"  "
    if len(elem):
        if not elem.text or not elem.text.strip():
            elem.text = i + "  "
        for e in elem:
            Indent(e, level+1)
            if not e.tail or not e.tail.strip():
                e.tail = i + "  "
        if not e.tail or not e.tail.strip():
            e.tail = i
    else:
        if level and (not elem.tail or not elem.tail.strip()):
            elem.tail = i


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

