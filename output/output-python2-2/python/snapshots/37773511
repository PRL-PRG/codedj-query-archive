from lib.lib import UMLException, XMLEncode, IDGenerator
from ProjectNode import CProjectNode
from cStringIO import StringIO
from zipfile import ZipFile, ZIP_STORED, ZIP_DEFLATED
import xml.dom.minidom
from lib.lib import UMLException
from lib.Storages import open_storage

from lib.Drawing import CElement
from lib.Drawing import CConnection
from lib.Elements.Type import CElementType
from lib.Elements.Object import CElementObject
from lib.Connections.Object import CConnectionObject
from lib.Elements.Factory import CElementFactory
from lib.Diagrams.Factory import CDiagramFactory
from lib.Connections.Factory import CConnectionFactory
from lib.Versions.Factory import CVersionFactory
from lib.Drawing import CDrawingArea

import os.path
from lib.consts import ROOT_PATH, VERSIONS_PATH, DIAGRAMS_PATH, ELEMENTS_PATH, CONNECTIONS_PATH


class CProject(object):
    def __init__(self):
        self.root = None
        
        self.Storage = open_storage(os.path.join(ROOT_PATH, 'etc', 'uml'))
        self.ElementFactory = CElementFactory(self.Storage, ELEMENTS_PATH)
        self.DiagramFactory = CDiagramFactory(self.Storage, DIAGRAMS_PATH)
        self.ConnectionFactory = CConnectionFactory(self.Storage, CONNECTIONS_PATH)
        self.VersionFactory = CVersionFactory(self.Storage, VERSIONS_PATH)
        self.version = self.VersionFactory.GetVersion('UML 1.4')
        
        self.filename = None
    
    def GetStorage(self):
        return self.Storage
    
    def GetElementFactory(self):
        return self.ElementFactory
    
    def GetDiagramFactory(self):
        return self.DiagramFactory
    
    def GetConnectionFactory(self):
        return self.ConnectionFactory
    
    def GetVersionFactory(self):
        return self.VersionFactory
    
    def GetStorage(self):
        return self.Storage
    
    def GetVersion(self):
        return self.version
    
    def SetRoot(self, value):
        self.root = value
    
    def GetRoot(self):
        return self.root
    
    def GetFileName(self):
        return self.filename
    
    def GetNode(self, path):
        node = self.root
        
        k = path.split('/')[0]
        i,j = k.split(':')
        
        if i == self.root.GetName() and j == self.root.GetType() and len(path.split('/')) == 1:
            return root        
        
        if i == self.root.GetName() and j == self.root.GetType():
            for i in path.split('/')[1:]:
                j, k  = i.split(':')
                if k == "=DrawingArea=":
                    return node
                else:
                    node = node.GetChild(j, k)
                if node is None:
                    raise UMLException("BadPath")
            return node
        raise UMLException("BadPath3")
    
    
    def Find(self, name):
        stack = [self.root]
        while len(stack) > 0:
            node = stack.pop(0)
            if node.GetName() == name:
                return node
            stack += node.GetChilds()
        return None

    def AddNode(self, node, parent):
        if parent is None:
            self.root = node
        else:
            parent.AddChild(node)
            

    def MoveNode(self, node, newParent):
        node.GetParent(node).RemoveChild(node)
        node.SetParent(newParent)
        newParent.AddChild(node)
              

    def RemoveNode(self, node):
        node.GetParent().RemoveChild(node)
   
    # search for all connections and elements under given node
    def searchCE(self, node): 
        elements = set()
        connections = set()
        def _search(node):
            obj = node.GetObject()
            elements.add(obj)
            for con in obj.GetConnections():
                connections.add(con)
            for chld in node.GetChilds():
                _search(chld)
        
        _search(node)
        return elements, connections
    
    def SaveProject(self, filename = None):
        if filename is None:
            filename = self.filename
        else:
            self.filename = filename
        f = StringIO()
        id = IDGenerator()
        
        def saveattr(object, level):
            if isinstance(object, dict):
                attrs = object.iteritems()
            else:
                attrs = object.GetType().GetAttributes()
            for attr in attrs:
                if isinstance(object, dict):
                    attr, value = attr
                else:
                    value = object.GetAttribute(attr)
                if not isinstance(value, list):
                    print>>f, '  '*level+'<property name="%s" value="%s" />'%(XMLEncode(attr), XMLEncode(value))
                else:
                    print>>f, '  '*level+'<property name="%s" type="list">'%XMLEncode(attr)
                    for item in value:
                        print>>f, '  '*level+'  <item>'
                        saveattr(item, level+2)
                        print>>f, '  '*level+'  </item>'
                    print>>f, '  '*level+'</property>'
        
        def savetree(node, level):
            print>>f, '  '*level+'<node id="%d">'%id(node.GetObject())
            if node.HasChild():
                print>>f, '  '*level+'  <childs>'
                for chld in node.GetChilds():
                    savetree(chld, level+4)
                print>>f, '  '*level+'  </childs>'
            print>>f, '  '*level+'  <drawingareas>'
            if node.HasDrawingArea():
                for area in node.GetDrawingAreas():
                    print>>f, '  '*level+'    <drawingarea name="%s" type="%s">'%(XMLEncode(area.GetName()), XMLEncode(area.GetType().GetId()))
                    for e in area.GetElements():
                        pos = e.GetPosition()
                        print>>f, '  '*level+'    <element id="%d" x="%d" y="%d" />'%(id(e.GetObject()), pos[0], pos[1])
                    for c in area.GetConnections():
                        print>>f, '  '*level+'    <connection id="%d">'%(id(c.GetObject()))
                        for pos in c.GetMiddlePoints():
                            print>>f, '  '*level+'      <point x="%d" y="%d" />'%pos
                        for num, pos in enumerate(c.GetLabelDefinedPositions()):
                            if pos is not None:
                                print>>f, '  '*level+'      <label num="%d" x="%d" y="%d" />'%(num, pos[0], pos[1])
                        print>>f, '  '*level+'    </connection>'
                    print>>f, '  '*level+'    </drawingarea>'
            print>>f, '  '*level+'  </drawingareas>'
            print>>f, '  '*level+'</node>'
        
        elements, connections = self.searchCE(self.root)
        print>>f, '<?xml version="1.0"?>'
        print>>f, '<umlproject>'
        print>>f, '  <objects>'
        for object in elements:
            print>>f, '    <object type="%s" id="%d">'%(XMLEncode(object.GetType().GetId()), id(object))
            saveattr(object, 3)
            print>>f, '    </object>'
        print>>f, '  </objects>'
        print>>f, '  <connections>'
        for connection in connections:
            print>>f, '    <connection type="%s" id="%d" source="%d" destination="%d">'%(XMLEncode(connection.GetType().GetId()), id(connection), id(connection.GetSource()), id(connection.GetDestination()))
            saveattr(connection, 3)
            print>>f, '    </connection>'
        print>>f, '  </connections>'
        print>>f, '  <projecttree>'
        savetree(self.root, 2)
        print>>f, '  </projecttree>'
        print>>f, '</umlproject>'
        
        out = ZipFile(filename, 'w', ZIP_DEFLATED)
        out.writestr('content.xml', f.getvalue())
        out.close()
        
        # file(filename, 'w').write(f.getvalue())
    
    def LoadProject(self, filename, copy = False):
        ListObj = {}
        ListCon = {}
        file = ZipFile(filename,"r")
        
        if copy:
            self.filename = None
        else:
            self.filename = filename
        
        data = file.read('content.xml')
        
        def CreateTree(root, parentNode):
            for i in root.childNodes:
                if i.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                    continue
                if i.tagName == 'childs':
                    for node in i.childNodes:
                        if node.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                            continue
                        proNode = CProjectNode(parentNode,ListObj[node.getAttribute("id").decode('unicode_escape')],parentNode.GetPath() + "/" + ListObj[node.getAttribute("id").decode('unicode_escape')].GetName() + ":" + ListObj[node.getAttribute("id").decode('unicode_escape')].GetType().GetId())
                        self.AddNode(proNode,parentNode)
                        CreateTree(node,proNode)
                        
                elif i.tagName == 'drawingareas':
                    for area in i.childNodes:
                        if area.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                            continue
                        if area.tagName == 'drawingarea':
                            drawingarea = CDrawingArea(self.DiagramFactory.GetDiagram(area.getAttribute("type").decode('unicode_escape')),area.getAttribute("name").decode('unicode_escape'))
                            drawingarea.SetPath(parentNode.GetPath() + "/" + drawingarea.GetName() + ":=DrawingArea=")
                            parentNode.AddDrawingArea(drawingarea)
                            for pic in area.childNodes:
                                if pic.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                                    continue
                                if pic.tagName == "element":
                                    element = CElement(drawingarea,ListObj[pic.getAttribute("id").decode('unicode_escape')])
                                    element.SetPosition((int(pic.getAttribute("x").decode('unicode_escape')),int(pic.getAttribute("y").decode('unicode_escape'))))
                                    proNode.AddAppears(drawingarea)
                                elif pic.tagName == "connection":
                                    for e in drawingarea.GetElements():
                                        if e.GetObject() is ListCon[pic.getAttribute("id").decode('unicode_escape')].GetSource():
                                            source = e
                                        if e.GetObject() is ListCon[pic.getAttribute("id").decode('unicode_escape')].GetDestination():
                                            destination = e
                                    conect = CConnection(drawingarea,ListCon[pic.getAttribute("id").decode('unicode_escape')],source,destination,[])
                                    for propCon in pic.childNodes:
                                        if propCon.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                                            continue
                                        if propCon.tagName == "point":
                                            conect.AddPoint(None,(int(propCon.getAttribute("x").decode('unicode_escape')),int(propCon.getAttribute("y").decode('unicode_escape'))))
                                        elif propCon.tagName == "label":
                                            conect.SetLabelPosition(int(propCon.getAttribute("num").decode('unicode_escape')),(int(propCon.getAttribute("x").decode('unicode_escape')),int(propCon.getAttribute("y").decode('unicode_escape'))))
            
        dom = xml.dom.minidom.parseString(data)
        root = dom.documentElement
        if root.tagName != 'umlproject':
            raise UMLException("XMLError", root.tagName)
        
        for i in root.childNodes:
            if i.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                continue
            en = i.tagName
            if en == 'objects':
                for j in i.childNodes:
                    if j.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                        continue
                    if j.tagName == 'object':
                        id = j.getAttribute("id").decode('unicode_escape')
                        object = CElementObject(self.ElementFactory.GetElement(j.getAttribute("type").decode('unicode_escape')))
                        
                        for property in j.childNodes:
                            if property.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                                continue
                            if property.hasAttribute("value"):
                                object.SetAttribute(property.getAttribute("name").decode('unicode_escape'),property.getAttribute("value").decode('unicode_escape'))
                            elif property.hasAttribute("type"):
                                attributes = []
                                for item in property.childNodes:
                                    atrib = {}
                                    for attribute in item.childNodes:
                                        if attribute.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                                            continue
                                        atrib[attribute.getAttribute("name").decode('unicode_escape')] = attribute.getAttribute("value").decode('unicode_escape')
                                    if len(atrib) > 0:
                                        attributes.append(atrib)
                                object.SetAttribute(property.getAttribute("name").decode('unicode_escape'),attributes)
                        ListObj[id] = object
                        
            elif en == 'connections':
                for connection in i.childNodes:
                    if connection.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                        continue
                    if connection.tagName == 'connection':
                        id = connection.getAttribute("id").decode('unicode_escape')
                        con = CConnectionObject(self.ConnectionFactory.GetConnection(connection.getAttribute("type").decode('unicode_escape')),ListObj[connection.getAttribute("source").decode('unicode_escape')],ListObj[connection.getAttribute("destination").decode('unicode_escape')])
                        for propCon in connection.childNodes:
                            if propCon.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                                continue
                            con.SetAttribute(propCon.getAttribute("name").decode('unicode_escape'),propCon.getAttribute("value").decode('unicode_escape'))
                        ListCon[id] = con
            elif en == 'projecttree':
                for j in i.childNodes:
                    if j.nodeType not in (xml.dom.minidom.Node.ELEMENT_NODE, xml.dom.minidom.Node.DOCUMENT_NODE):
                        continue
                    if j.tagName == 'node':
                        proNode = CProjectNode(None,ListObj[j.getAttribute("id").decode('unicode_escape')],ListObj[j.getAttribute("id").decode('unicode_escape')].GetName() + ":" + ListObj[j.getAttribute("id").decode('unicode_escape')].GetType().GetId())
                        self.SetRoot(proNode)
                        CreateTree(j,proNode)
                        
        
    Root = property(GetRoot, SetRoot)
    