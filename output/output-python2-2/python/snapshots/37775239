import os
import os.path
from lib.Exceptions.DevException import *
from Type import CDiagramType
from lib.config import config
from lib.consts import METAMODEL_NAMESPACE

#try to import necessary lybraries for XML parsing
try:
    from lxml import etree
    HAVE_LXML = True
    #print("running with lxml.etree")
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
               
#if lxml.etree is imported successfully, we use xml validation with xsd schema
if HAVE_LXML:
    xmlschema_doc = etree.parse(os.path.join(config['/Paths/Schema'], "metamodel.xsd"))
    xmlschema = etree.XMLSchema(xmlschema_doc)

class CDiagramFactory(object):
    """
    Creates diagram types from metamodel XMLs
    """
    def __init__(self, storage, path):
        """
        Parse metamodel and create list of diagram types
        
        @param storage: Storage in which is file located
        @type  storage: L{CAbstractStorage<lib.Storages.AbstractStorage.CAbstractStorage>}
        
        @param path: Path to directory with diagram metamodel XMLs
        @type  path: string
        """
        self.types = {}
        self.path = path
        self.storage = storage
        
        self.Reload()
        
    def GetDiagram(self, type):
        """
        Get diagram type by its name
        
        @param type: diagram type name
        @type  type: string
        
        @return: Diagram type of given name
        @rtype: L{CDiagramType<lib.Diagrams.Type.CDiagramType>}
        """
        if self.types.has_key(type):
            return self.types[type]
        else:
            raise FactoryError("KeyError")
    
    def Reload(self):
        """
        Reload diagrams metamodel
        """
        for file in self.storage.listdir(self.path):
            if file.endswith('.xml'):
                self.__Load(os.path.join(self.path, file))
    
    def __iter__(self):
        """
        Iterator over all contained diagram types
        
        @return: diagram types
        @rtype:  iterator over L{CDiagramType<lib.Diagrams.Type.CDiagramType>}(s)
        """
        for i in self.types.values():
            yield i
        
    def __Load(self, file_path):
        """
        Load an XMLs from given path
        
        @param file_path: Path to connections metamodel (within storage)
        @type  file_path: string
        """
        
        root = etree.XML(self.storage.read_file(file_path))
        #xml (version) file is validate with xsd schema (metamodel.xsd)
        if HAVE_LXML:
            if not xmlschema.validate(root):
                raise FactoryError("XMLError", xmlschema.error_log.last_error)

        obj = CDiagramType(root.get('id'))
        
        for element in root:
            if element.tag == METAMODEL_NAMESPACE+'Icon':
                obj.SetIcon(element.get('path'))
                
            elif element.tag == METAMODEL_NAMESPACE+'Special':
                swimlines = element.get('swimlines')
                lifelines = element.get('lifelines')
                obj.SetSpecial(swimlines, lifelines)
                
            elif element.tag == METAMODEL_NAMESPACE+'Elements':
                for item in element:
                    if not isinstance(item, etree._Comment):
                        value = item.get('value')
                        obj.AppendElement(value)
                    
            elif element.tag == METAMODEL_NAMESPACE+'Connections':
                for item in element:
                    value = item.get('value')
                    obj.AppendConnection(value)
        
        self.types[root.get('id')] = obj
    