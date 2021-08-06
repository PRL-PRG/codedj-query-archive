import xml.dom.minidom
import os
import os.path

from lib.lib import UMLException
from Type import CLanguageType
from lib.CodeEngineering.XMLElements import CODEALL

class CLanguageFactory:
    def __init__(self, storage, path):
        self.types = {}
        self.path = path
        self.storage = storage
        for file in storage.listdir(path):
            if file.endswith('.xml'): 
                self.__Load(os.path.join(self.path, file))
    
    def __iter__(self):
        for i in self.types.values():
            yield i
        
    def GetType(self, language):
        return self.types[language]        
    
    def __Load(self, filePath):
        dom = xml.dom.minidom.parseString(self.storage.read_file(filePath))
        root = dom.documentElement
        if root.tagName != 'Template':
            raise UMLException("XMLError", root.tagName)
        
        if not root.hasAttribute('diagram'):
            raise UMLException("XMLError", ('ElementType', 'diagram'))
        if not root.hasAttribute('language'):
            raise UMLException("XMLError", ('ElementType', 'language'))
        if not root.hasAttribute('type'):
            raise UMLException("XMLError", ('ElementType', 'type'))
        
        obj = CLanguageType(root.getAttribute('language'),root.getAttribute('type'))
        
        for i in root.childNodes:
            if i.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                continue
            if i.tagName == 'Element':
                if not i.hasAttribute('id'):
                    raise UMLException("XMLError", ('ElementType', 'id'))
                obj.AddElement(i.getAttribute('id'),self.__LoadElement(i))
                
        
        self.types[root.getAttribute('language')] = obj
            
    
    def __LoadElement(self, root):
        if root.tagName in CODEALL:  
            cls = CODEALL[root.tagName]
            params = {}
            for i in root.attributes.values():
                params[str(i.name)] = i.nodeValue
            obj = cls(**params)
        else:
            obj = CODEALL['None']()
                    
        for i in root.childNodes:
            if i.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                if i.nodeType is xml.dom.minidom.Node.TEXT_NODE:
                    for k in i.nodeValue.splitlines():
                        if len(k.expandtabs(1).strip(' ')) > 0:
                            obj.AppendChild(CODEALL['Text'](k.expandtabs(1).strip(' ')))
                continue
            if obj is not None:
                a = self.__LoadElement(i)
                if a is not None:
                    obj.AppendChild(a)
            else:
                self.__LoadElement(i)
            
        return obj
        
        