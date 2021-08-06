import xml.dom.minidom
import os
import os.path

from lib.lib import UMLException
from Type import CVersionType

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
        dom = xml.dom.minidom.parseString(self.storage.read_file(file_path))
        root = dom.documentElement
        if root.tagName != 'Version':
            raise UMLException("XMLError", root.tagName)
        if not root.hasAttribute('id'):
            raise UMLException("XMLError", ('Version', 'id'))
        
        version = CVersionType(root.getAttribute('id')) 

        for rootChild in root.childNodes:
            if rootChild.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                continue
            elemName = rootChild.tagName
            if elemName == 'Diagrams':  
                for item in rootChild.childNodes: 
                    if item.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                        continue
                    if item.tagName != 'Item':
                        raise UMLException("XMLError", item.tagName)
                    if not item.hasAttribute('value'):
                        raise UMLException("XMLError", ('Item', 'value'))
                    diagName = item.getAttribute('value') 
                        
                    if item.hasChildNodes(): 
                        diagRestrictions = []
                        for restr in item.childNodes:
                            if restr.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                                continue
                            if restr.tagName != 'Restriction':
                                raise UMLException("XMLError", restr.tagName)
                            if not restr.hasAttribute('value'):
                                raise UMLException("XMLError", ('Restriction', 'value'))
                            if not restr.hasAttribute('type'):
                                raise UMLException("XMLError", ('Restriction', 'type'))

                            diagRestrictions.append((restr.getAttribute('type'), restr.getAttribute('value')))

                    version.AddRestrictions(diagName, diagRestrictions)
            else:
                raise UMLException('XMLError', elemName)
        
        self.types[root.getAttribute('id')] = version 
