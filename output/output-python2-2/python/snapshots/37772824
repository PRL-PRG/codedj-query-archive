import xml.dom.minidom
import os
import os.path

from lib.lib import UMLException
from Type import CLanguageType

class CLanguageFactory:
    def __init__(self, storage, path):
        self.types = {}
        self.path = path
        self.storage = storage
        for file in storage.listdir(path):
            if file.endswith('.xml'):
                self.__Load(os.path.join(self.path, file))
    
    def GetLanguage(self, type):
        return self.types[type]
    
    def GetLanguages(self):
        for language in self.types.keys():
            yield language
            
    
    def __Load(self, filePath):
        dom = xml.dom.minidom.parseString(self.storage.read_file(filePath))
        root = dom.documentElement
        if root.tagName != 'LanguageType':
            raise UMLException("XMLError", root.tagName)
        if not root.hasAttribute('id'):
            raise UMLException("XMLError", ('ElementType', 'id'))
        obj = CLanguageType(root.getAttribute('id'))
        
        for i in root.childNodes:
            if i.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                continue
            en = i.tagName
            if en == 'Icon':
                if not i.hasAttribute('path'):
                    raise UMLException("XMLError", ('Icon', 'path'))
                obj.SetIcon(i.getAttribute('path'))
            elif en == 'Namespace':
                pass
            elif en == 'Class':
                pass
        
        self.types[root.getAttribute('id')] = obj
        
        
        