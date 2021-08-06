import xml.dom.minidom
import os
import os.path
from lib.lib import UMLException
from Type import CDiagramType

class CDiagramFactory:
    def __init__(self, path):
        self.types = {}
        self.path = path
        
        self.Reload()
        
    def GetDiagram(self, type):
        if self.types.has_key(type):
            return self.types[type]
        else:
            raise UMLException("KeyError")
    
    def Reload(self):
        for file in os.listdir(self.path):
            if file.endswith('.xml'):
                self.__Load(os.path.join(self.path, file))
    
    def __iter__(self):
        for i in self.types.values():
            yield i
        
    def __Load(self, file_path):
        dom = xml.dom.minidom.parse(file_path)
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
            if en == 'Special':
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
    