import os
import os.path
from lib.lib import UMLException
from Type import CConnectionType
from Line import CConnectionLine
from Arrow import CConnectionArrow
from lib.consts import METAMODEL_NAMESPACE
from lib.Drawing.Objects import ALL
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


class CConnectionFactory(object):
    """
    Creates connection types from metamodel XMLs
    """
    def __init__(self, storage, path):
        """
        Parse metamodel XMLs and creates connection types
        
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

    def GetConnection(self, type):
        """
        Gets connection type by its name
        
        @param type: Name of connection type
        @type  type: string
        
        @return: Connection type of given name
        @rtype:  L{CConnectionType<Type.CConnectionType>}
        """
        return self.types[type]

    def __Load(self, file_path):
        """
        Load an XMLs from given path
        
        @param file_path: Path to connections metamodel (within storage)
        @type  file_path: string
        """
        #dom = xml.dom.minidom.parseString(self.storage.read_file(file_path))
        #root = dom.documentElement
        
        root = etree.XML(self.storage.read_file(file_path))

        #xml (version) file is validate with xsd schema (metamodel.xsd)
        if HAVE_LXML:
            if not xmlschema.validate(root):
                #print(xmlschema.error_log)
                raise UMLException("XMLError", xmlschema.error_log.last_error)

        id = root.get('id')
        
        sarr = {}
        darr = {}
        ls = {}
        icon = None
        labels = []
        attrs = []
        for element in root.getchildren():
            if element.tag == METAMODEL_NAMESPACE+'Icon':
                icon = element.get('path')
            elif element.tag == METAMODEL_NAMESPACE+'SrcArrow':
                sarr['possible'] = element.get('possible')
                sarr['default'] = element.get('default')
            elif element.tag == METAMODEL_NAMESPACE+'DestArrow':
                darr['possible'] = element.get('possible')
                darr['default'] = element.get('default')
            elif element.tag == METAMODEL_NAMESPACE+'Attributes':
                for item in element.getchildren():
                    value = item.get('value')
                    type = item.get('type')
                    propid = item.get('propid')
                    options = []
                    for opt in item.getchildren():
                        options.append(opt.get('value'))
                    attrs.append((value, type, propid, options))
            elif element.tag == METAMODEL_NAMESPACE+'Appearance':
                for subelem in element.getchildren():
                    if subelem.tag == METAMODEL_NAMESPACE+'LineStyle':
                        ls['color'] = subelem.get('color')
                        ls['style'] = subelem.get('style')
                        if subelem.get('width') is not None:
                            ls['width'] = subelem.get('width')
                    elif subelem.tag == METAMODEL_NAMESPACE+'ArrowStyle':
                        darr['fill'] = sarr['fill'] = subelem.get('fill')
                        darr['color'] = sarr['color'] = subelem.get('color')
                        darr['style'] = sarr['style'] = subelem.get('style')
                        if subelem.get('size') is not None:
                            darr['size'] = sarr['size'] = subelem.get('size')
                    elif subelem.tag == METAMODEL_NAMESPACE+'Label':
                        tmp = None
                        for k in subelem.getchildren():
                            tmp = k
                        labels.append((subelem.get('position'), self.__LoadAppearance(tmp)))

        tmp = self.types[id] = CConnectionType(id, CConnectionLine(**ls),
                                    CConnectionArrow(**sarr), CConnectionArrow(**darr), icon)
        for pos, lbl in labels:
            tmp.AddLabel(pos, lbl)
        
        for attr in attrs:
            tmp.AppendAttribute(*attr)
    
    def __LoadAppearance(self, root):
        """
        Loads an appearance section of an XML file
        
        @param root: Appearance element
        @type  root: L{Element<xml.dom.minidom.Element>}
        
        @return: Visual object representing this section
        @rtype:  L{CVisualObject<lib.Drawing.Objects.VisualObject.CVisualObject>}
        """
        #this condition is not necessary, I thing
        if root.tag.split("}")[1] not in ALL:
            raise UMLException("XMLError", root.tag)
        
        cls = ALL[root.tag.split("}")[1]]
        params = {}
        for attr in root.attrib.items():
            params[attr[0]] = attr[1]
        obj = cls(**params)
        if hasattr(obj, "LoadXml"):
            obj.LoadXml(root)
        else:
            for child in root.getchildren():
                obj.AppendChild(self.__LoadAppearance(child))
        return obj
