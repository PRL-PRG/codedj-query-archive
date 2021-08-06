import xml.dom.minidom
import os
import os.path
from lib.lib import UMLException
from Type import CElementType
from gtk.gdk import pixbuf_new_from_file

from lib.Drawing.Objects import ALL

class CElementFactory:
    def __init__(self, path):
        self.types = {}
        self.path = path
        
        for file in os.listdir(self.path):
            self.__Load(os.path.join(self.path, file))

    def GetElement(self, type):
        return self.types[type]

    def __Load(self, file_path):
        dom = xml.dom.minidom.parse(file_path)
        root = dom.documentElement
        if root.tagName != 'ElementType':
            raise UMLException("XMLError", root.tagName)
        if not root.hasAttribute('id'):
            raise UMLException("XMLError", ('ElementType', 'id'))
        obj = CElementType(root.getAttribute('id'))
        
        for i in root.childNodes:
            if i.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                continue
            en = i.tagName
            if en == 'Icon':
                if not i.hasAttribute('path'):
                    raise UMLException("XMLError", ('Icon', 'path'))
                obj.SetIcon(pixbuf_new_from_file(i.getAttribute('path')))
            elif en == 'Connections':
                for item in i.childNodes:
                    if item.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                        continue
                    if item.tagName != 'Item':
                        raise UMLException("XMLError", item.tagName)
                    if not item.hasAttribute('value'):
                        raise UMLException("XMLError", ('Item', 'value'))
                    value = item.getAttribute('value')
                    with_what = None
                    allow_recursive = False
                    if item.hasAttribute('with'):
                        with_what = item.getAttribute('with')
                    if item.hasAttribute('allowrecursive'):
                        allow_recursive = item.getAttribute('allowrecursive').lower() in ('1', 'true', 'yes')
                    obj.AppendConnection(value, with_what, allow_recursive)
            elif en == 'Attributes':
                for item in i.childNodes:
                    if item.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                        continue
                    if item.tagName != 'Item':
                        raise UMLException("XMLError", item.tagName)
                    if not item.hasAttribute('value'):
                        raise UMLException("XMLError", ('Item', 'value'))
                    value = item.getAttribute('value')
                    if not item.hasAttribute('type'):
                        raise UMLException("XMLError", ('Item', 'type'))
                    type = item.getAttribute('type')
                    propid = None
                    options = []
                    if item.hasAttribute('propid'):
                        propid = item.getAttribute('propid')
                    if item.hasChildNodes():
                        options = []
                        for opt in item.childNodes:
                            if opt.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                                continue
                            if opt.tagName != 'Option':
                                raise UMLException("XMLError", opt.tagName)
                            if not opt.hasAttribute('value'):
                                raise UMLException("XMLError", ('Option', 'value'))
                            options.append(opt.getAttribute('value'))
                    obj.AppendAttribute(value, type, propid, options)
            elif en == 'Appearance':
                tmp = None
                for j in i.childNodes:
                    if j.nodeType == xml.dom.minidom.Node.ELEMENT_NODE:
                        if tmp is not None:
                            raise UMLException("XMLError", 'Appearance')
                        tmp = j
                obj.SetAppearance(self.__LoadAppearance(tmp))
            else:
                raise UMLException('XMLError', en)
        
        self.types[root.getAttribute('id')] = obj
    
    def __LoadAppearance(self, root):
        if root.tagName not in ALL:
            raise UMLException("XMLError", root.tagName)
        cls = ALL[root.tagName]
        params = {}
        for i in root.attributes.values():
            params[str(i.name)] = i.nodeValue
        obj = cls(**params)
        for child in root.childNodes:
            if child.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                continue
            obj.AppendChild(self.__LoadAppearance(child))
        return obj
