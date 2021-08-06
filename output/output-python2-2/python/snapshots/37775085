from lib.Depend.etree import etree, HAVE_LXML

import os
import os.path
from lib.Exceptions.DevException import *
from Type import CElementType
from lib.config import config
from lib.consts import METAMODEL_NAMESPACE
from lib.Drawing.Objects import ALL

#if lxml.etree is imported successfully, we use xml validation with xsd schema
if HAVE_LXML:
    xmlschema_doc = etree.parse(os.path.join(config['/Paths/Schema'], "metamodel.xsd"))
    xmlschema = etree.XMLSchema(xmlschema_doc)


class CElementFactory(object):
    """
    Factory, that creates element type objects
    """
    def __init__(self, storage, path):
        """
        Create the element factory
        
        @param storage: Storage in which is file located
        @type  storage: L{CAbstractStorage<lib.Storages.AbstractStorage.CAbstractStorage>}
        
        @param path: Path to directory with connection metamodel XMLs
        @type path: string
        """
        self.types = {}
        self.path = path
        
        self.storage = storage
        for file in storage.listdir(self.path):
            if file.endswith('.xml'):
                self.__Load(os.path.join(self.path, file))
                
    
    def GetElement(self, type=None):
        """
        Get element type by name. If name is None, that return value is dictionary of all elements
        
        @param type: Element type name
        @type  type: string
        """
        if type == None:
            return self.types
        else:
            return self.types[type]

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

        obj = CElementType(root.get('id'))
        
        for element in root:
            if element.tag == METAMODEL_NAMESPACE+'Icon':
                obj.SetIcon(element.get('path'))
            elif element.tag == METAMODEL_NAMESPACE+'Connections':
                for item in element:
                    value = item.get('value')
                    with_what = None
                    allow_recursive = False
                    if item.get('with') != None:
                        with_what = item.get('with').split(',')
                    if item.get('allowrecursive') != None:
                        allow_recursive = item.get('allowrecursive').lower() in ('1', 'true', 'yes')
                    obj.AppendConnection(value, with_what, allow_recursive)
            elif element.tag == METAMODEL_NAMESPACE+'Attributes':
                for item in element:
                    value = item.get('value')
                    type = item.get('type')
                    propid = item.get('propid')
                    if item.get('notgenerate') != None:
                        obj.SetGenerateName(not item.get('notgenerate'))
                    options = []
                    for opt in item:
                        options.append(opt.get('value'))
                    obj.AppendAttribute(value, type, propid, options)
            elif element.tag == METAMODEL_NAMESPACE+'Appearance':
                tmp = None
                for j in element:
                    tmp = j
                obj.SetAppearance(self.__LoadAppearance(tmp))
            elif element.tag == METAMODEL_NAMESPACE+'Options':
                for item in element:
                    name = item.tag.split('}')[1]
                    value = item.text
                    obj.AppendOptions(name, value)
        
        self.types[root.get('id')] = obj
    
    def __LoadAppearance(self, root):
        """
        Loads an appearance section of an XML file
        
        @param root: Appearance element
        @type  root: L{Element<lxml.etree.Element>}
        
        @return: Visual object representing this section
        @rtype:  L{CVisualObject<lib.Drawing.Objects.VisualObject.CVisualObject>}
        """
        if root.tag.split("}")[1] not in ALL:
            raise FactoryError("XMLError", root.tag)
        cls = ALL[root.tag.split("}")[1]]
        params = {}
        for attr in root.attrib.items():    #return e.g. attr == ('id', '1') => attr[0] == 'id', attr[1] == '1'
            params[attr[0]] = attr[1]
        obj = cls(**params)
        if hasattr(obj, "LoadXml"):
            obj.LoadXml(root)
        else:
            for child in root:
                obj.AppendChild(self.__LoadAppearance(child))
        return obj
