import os
import os.path

from lib.lib import UMLException
from Type import CVersionType
from lib.config import config

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
        #print("running with cElementTree on Python 2.5+")
    except ImportError:
        try:
            # Python 2.5
            import xml.etree.ElementTree as etree
            #print("running with ElementTree on Python 2.5+")
        except ImportError:
            try:
                # normal cElementTree install
                import cElementTree as etree
                #print("running with cElementTree")
            except ImportError:
                # normal ElementTree install
                import elementtree.ElementTree as etree
                #print("running with ElementTree")
                    
#if lxml.etree is imported successfully, we use xml validation with xsd schema
if HAVE_LXML:
    xmlschema_doc = etree.parse(os.path.join(config['/Paths/Schema'], "metamodel.xsd"))
    xmlschema = etree.XMLSchema(xmlschema_doc)

class CVersionFactory:
    def __init__(self, storage, path):
        self.types = {}
        self.path = path
        self.storage = storage
        
        for file in storage.listdir(self.path):
            if file.endswith('.xml'):
                self.__Load(os.path.join(self.path, file))
                
    def __iter__(self):
        for i in self.types.values():
            yield i
            
    def GetVersion(self, verName):
        if self.types.has_key(verName):
            return self.types[verName]
        else:
            raise UMLException("Version not found")

    def __Load(self, file_path):
        root = etree.XML(self.storage.read_file(file_path))
        #xml (version) file is validate with xsd schema (metamodel.xsd)
        if HAVE_LXML:
            if not xmlschema.validate(root):
                #print(xmlschema.error_log)
                raise UMLException("XMLError", xmlschema.error_log.last_error)
                
        version = CVersionType(root.get('id'))
        
        #Iterate over the descendants of root element (only element with tag=Item)
        for diagram in root.getchildren():
            for element in diagram.getchildren():
                diagName = element.get('value')
                #version.AddRestrictions(diagName, None)
                version.AddDiagram(diagName)
        
        self.types[root.get('id')] = version 
