import xml.dom.minidom
import os
import os.path
from lib.lib import UMLException
from Type import CDiagramType

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
            raise UMLException("KeyError")
    
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
        dom = xml.dom.minidom.parseString(self.storage.read_file(file_path))
        root = dom.documentElement
        if root.tagName != 'DiagramType':
            raise UMLException("XMLError")
        if not root.hasAttribute('id'):
            raise UMLException("XMLError")
        
        obj = CDiagramType(root.getAttribute('id'))
        
        for i in root.childNodes:
            if i.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                continue
            en = i.tagName
            if en == 'Icon':
                if not i.hasAttribute('path'):
                    raise UMLException("XMLError", ('Icon', 'path'))
                obj.SetIcon(i.getAttribute('path'))
            elif en == 'Special':
                swimlines = False
                lifelines = False
                if root.hasAttribute('swimlines'):
                    swimlines = i.getAttribute('swimlines')
                if root.hasAttribute('lifelines'):
                    lifelines = i.getAttribute('lifelines')
                obj.SetSpecial(swimlines, lifelines)
            elif en == 'Elements':
                for item in i.childNodes:
                    if item.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                        continue
                    if item.tagName != 'Item':
                        raise UMLException("XMLError")
                    if not item.hasAttribute('value'):
                        raise UMLException("XMLError")
                    
                    value = item.getAttribute('value')
                    obj.AppendElement(value)
                    
            elif en == 'Connections':
                for item in i.childNodes:
                    if item.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                        continue
                    if item.tagName != 'Item':
                        raise UMLException("XMLError")
                    if not item.hasAttribute('value'):
                        raise UMLException("XMLError")
                    
                    value = item.getAttribute('value')
                    obj.AppendConnection(value)
        
        self.types[root.getAttribute('id')] = obj
    