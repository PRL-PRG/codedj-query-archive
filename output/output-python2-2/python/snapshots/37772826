import xml.dom.minidom
import os
import os.path

from lib.lib import UMLException
from Type import CDataType

class CDataTypeFactory:
    def __init__(self, storage, path):
        self.types = {}
        self.path = path
        
        self.storage = storage
        for file in storage.listdir(self.path):
            if file.endswith('.xml'):
                self.__Load(os.path.join(self.path, file))
        
        self.types["own"] = CDataType("own")
    
    def __Load(self, file_path):
        dom = xml.dom.minidom.parseString(self.storage.read_file(file_path))
        root = dom.documentElement
        if root.tagName != 'Types':
            raise UMLException("XMLError", root.tagName)
        
        for i in root.childNodes:
            if i.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                continue
            if i.tagName == 'Language':
                typ = CDataType(i.getAttribute('id'))
                for item in i.childNodes:
                    if item.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                        continue
                    if item.tagName == 'Item':
                        if not item.hasAttribute('type'):
                            raise UMLException("XMLError", ('Item', 'type'))
                        typ.AddDataType(item.getAttribute('type'))
                self.types[i.getAttribute('id')] = typ
    
    def GetDataType(self, id):
        return self.types[id]
    
    def GetLanguages(self):
        for i in self.types.keys():
            yield i
    
    def GetFirstLanguage(self):
        for i in self.types.keys():
            if i != "own":
                return i
        else:
            return ""
        