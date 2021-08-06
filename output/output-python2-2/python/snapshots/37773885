import xml.dom.minidom
import os
import os.path
from lib.lib import UMLException
from Type import CConnectionType
from Line import CConnectionLine
from Arrow import CConnectionArrow

from lib.Drawing.Objects import ALL

class CConnectionFactory:
    def __init__(self, path):
        self.types = {}
        self.path = path
        for file in os.listdir(self.path):
            if file.endswith('.xml'):
                self.__Load(os.path.join(self.path, file))

    def GetConnection(self, type):
        return self.types[type]

    def __Load(self, file_path):
        dom = xml.dom.minidom.parse(file_path)
        root = dom.documentElement
        if root.tagName != 'ConnectionType':
            raise UMLException("XMLError", root.tagName)
        if not root.hasAttribute('id'):
            raise UMLException("XMLError", ('ConnectionType', 'id'))
        id = root.getAttribute('id')
        sarr = {}
        darr = {}
        ls = {}
        icon = None
        labels = []
        attrs = []
        for i in root.childNodes:
            if i.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                continue
            en = i.tagName
            if en == 'Icon':
                if not i.hasAttribute('path'):
                    raise UMLException("XMLError", ('Icon', 'path'))
                icon = i.getAttribute('path')
            elif en == 'SrcArrow':
                if i.hasAttribute('possible'):
                    sarr['possible'] = i.getAttribute('possible')
                if i.hasAttribute('default'):
                    sarr['default'] = i.getAttribute('default')
            elif en == 'DestArrow':
                if i.hasAttribute('possible'):
                    darr['possible'] = i.getAttribute('possible')
                if i.hasAttribute('default'):
                    darr['default'] = i.getAttribute('default')
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
                    attrs.append((value, type, propid, options))
            elif en == 'Appearance':
                for j in i.childNodes:
                    if j.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                        continue
                    en = j.tagName
                    if en == 'LineStyle':
                        if j.hasAttribute('color'):
                            ls['color'] = j.getAttribute('color')
                        if j.hasAttribute('style'):
                            ls['style'] = j.getAttribute('style')
                        if j.hasAttribute('width'):
                            ls['width'] = j.getAttribute('width')
                    elif en == 'ArrowStyle':
                        if j.hasAttribute('fill'):
                            darr['fill'] = sarr['fill'] = j.getAttribute('fill')
                        if j.hasAttribute('color'):
                            darr['color'] = sarr['color'] = j.getAttribute('color')
                        if j.hasAttribute('style'):
                            darr['style'] = sarr['style'] = j.getAttribute('style')
                        if j.hasAttribute('size'):
                            darr['size'] = sarr['size'] = j.getAttribute('size')
                    elif en == 'Label':
                        if not j.hasAttribute('position'):
                            raise UMLException("XMLError", ('Label', 'position'))
                        tmp = None
                        for k in j.childNodes:
                            if k.nodeType == xml.dom.minidom.Node.ELEMENT_NODE:
                                if tmp is not None:
                                    raise UMLException("XMLError", 'Label')
                                tmp = k
                        labels.append((j.getAttribute('position'), self.__LoadAppearance(tmp)))
                    else:
                        raise UMLException('XMLError', en)
            else:
                raise UMLException('XMLError', en)
        tmp = self.types[id] = CConnectionType(id, CConnectionLine(**ls),
                                    CConnectionArrow(**sarr), CConnectionArrow(**darr), icon)
        for pos, lbl in labels:
            tmp.AddLabel(pos, lbl)
        
        for attr in attrs:
            tmp.AppendAttribute(*attr)
    
    def __LoadAppearance(self, root):
        if root.tagName not in ALL:
            raise UMLException("XMLError", root.tagName)
        cls = ALL[root.tagName]
        params = {}
        for i in root.attributes.values():
            params[str(i.name)] = i.nodeValue
        obj = cls(**params)
        if hasattr(obj, "LoadXml"):
            obj.LoadXml(root)
        else:
            for child in root.childNodes:
                if child.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                    continue
                obj.AppendChild(self.__LoadAppearance(child))
        return obj
