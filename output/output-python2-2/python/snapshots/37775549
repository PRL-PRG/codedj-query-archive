from lib.lib import XMLEncode, IDGenerator
from ProjectNode import CProjectNode
from cStringIO import StringIO
from zipfile import ZipFile, ZIP_STORED, ZIP_DEFLATED
from lib.Exceptions.UserException import *
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
from lib.Drawing import CDiagram
import os.path
from lib.consts import ROOT_PATH, VERSIONS_PATH, DIAGRAMS_PATH, ELEMENTS_PATH, CONNECTIONS_PATH, UMLPROJECT_NAMESPACE
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
    xmlschema_doc = etree.parse(os.path.join(config['/Paths/Schema'], "umlproject.xsd"))
    xmlschema = etree.XMLSchema(xmlschema_doc)


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
        
        #e.g. path = Project:Package/New Class diagram:=Diagram=
        k = path.split('/')[0]
        i,j = k.split(':')
        
        if i == self.root.GetName() and j == self.root.GetType() and len(path.split('/')) == 1:
            return root        
        
        if i == self.root.GetName() and j == self.root.GetType():
            for i in path.split('/')[1:]:
                j, k  = i.split(':')
                if k == "=Diagram=":
                    return node
                else:
                    node = node.GetChild(j, k)
                if node is None:
                    raise ProjectError("BadPath")
            return node
        raise ProjectError("BadPath3")
    
    
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

        id = IDGenerator()
        
        def saveattr(object, element):
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
                    propertyNode = etree.Element(UMLPROJECT_NAMESPACE+'property', name=unicode(attr), value=unicode(value))
                else:
                    propertyNode = etree.Element(UMLPROJECT_NAMESPACE+'property', name=unicode(attr), type="list")
                    for item in value:
                        itemNode = etree.Element(UMLPROJECT_NAMESPACE+'item')
                        saveattr(item, itemNode)
                        propertyNode.append(itemNode)
                element.append(propertyNode)
        
        def savetree(node, element):
            nodeNode = etree.Element(UMLPROJECT_NAMESPACE+'node', id=unicode(id(node.GetObject())))
            if node.HasChild():
                childsNode = etree.Element(UMLPROJECT_NAMESPACE+'childs')
                for chld in node.GetChilds():
                    savetree(chld, childsNode)
                nodeNode.append(childsNode)
                
            diagramsNode = etree.Element(UMLPROJECT_NAMESPACE+'diagrams')
            if node.HasDiagram():
                for area in node.GetDiagrams():
                    diagramNode = etree.Element(UMLPROJECT_NAMESPACE+'diagram', name=area.GetName(), type=unicode(area.GetType().GetId()))
                    for e in area.GetElements():
                        pos = e.GetPosition()
                        dw, dh = e.GetSizeRelative()
                        elementNode = etree.Element(UMLPROJECT_NAMESPACE+'element', id=unicode(id(e.GetObject())), x=unicode(pos[0]), y=unicode(pos[1]), dw=unicode(dw), dh=unicode(dh))
                        diagramNode.append(elementNode)

                    for c in area.GetConnections():
                        connectionNode = etree.Element(UMLPROJECT_NAMESPACE+'connection', id=unicode(id(c.GetObject())))
                        for pos in c.GetMiddlePoints():
                            pointNode = etree.Element(UMLPROJECT_NAMESPACE+'point', x=unicode(pos[0]), y=unicode(pos[1]))
                            connectionNode.append(pointNode)

                        for num, (index, t, dist, angle) in enumerate(c.GetLabelDefinedPositions()):
                            if index is not None:
                                labelNode = etree.Element(UMLPROJECT_NAMESPACE+'label', num=unicode(num), index=unicode(index), section=unicode(t), distance=unicode(dist), angle=unicode(angle))
                                connectionNode.append(labelNode)

                        diagramNode.append(connectionNode)
                    diagramsNode.append(diagramNode)
            nodeNode.append(diagramsNode)
            element.append(nodeNode)
        
        elements, connections = self.searchCE(self.root)
        
        rootNode = etree.XML('<umlproject xmlns="http://umlfri.kst.fri.uniza.sk/xmlschema/umlproject.xsd"></umlproject>')
        
        objectsNode = etree.Element(UMLPROJECT_NAMESPACE+'objects')
        connectionsNode = etree.Element(UMLPROJECT_NAMESPACE+'connections')
        projtreeNode = etree.Element(UMLPROJECT_NAMESPACE+'projecttree')
        
        for object in elements:
            objectNode = etree.Element(UMLPROJECT_NAMESPACE+'object', type=unicode(object.GetType().GetId()), id=unicode(id(object)))
            saveattr(object, objectNode)
            objectsNode.append(objectNode)
            
        rootNode.append(objectsNode)
        
        for connection in connections:
            connectionNode = etree.Element(UMLPROJECT_NAMESPACE+'connection', type=unicode(connection.GetType().GetId()), id=unicode(id(connection)), source=unicode(id(connection.GetSource())), destination=unicode(id(connection.GetDestination())))
            saveattr(connection, connectionNode)
            connectionsNode.append(connectionNode)
            
        rootNode.append(connectionsNode)
        savetree(self.root, projtreeNode)
        rootNode.append(projtreeNode)
        
        #xml tree is validate with xsd schema (recentfile.xsd)
        if HAVE_LXML:
            if not xmlschema.validate(rootNode):
                raise XMLError(xmlschema.error_log.last_error)

        #save Recent File Tree into ZIP file
        out = ZipFile(filename, 'w', ZIP_DEFLATED)
        #out.writestr('content.xml', etree.tostring(rootNode, encoding='utf-8', xml_declaration=True, pretty_print=True))
        out.writestr('content.xml', '<?xml version="1.0" encoding="utf-8"?>\n'+etree.tostring(rootNode, encoding='utf-8'))
        out.close()
    
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
            for elem in root:
                if elem.tag == UMLPROJECT_NAMESPACE+'childs':
                    for node in elem:
                        proNode = CProjectNode(parentNode,ListObj[node.get("id").decode('unicode_escape')],parentNode.GetPath() + "/" + ListObj[node.get("id").decode('unicode_escape')].GetName() + ":" + ListObj[node.get("id").decode('unicode_escape')].GetType().GetId())
                        self.AddNode(proNode,parentNode)
                        CreateTree(node,proNode)

                elif elem.tag == UMLPROJECT_NAMESPACE+'diagrams':
                    for area in elem:
                        if area.tag == UMLPROJECT_NAMESPACE+'diagram':
                            diagram = CDiagram(self.DiagramFactory.GetDiagram(area.get("type").decode('unicode_escape')),area.get("name").decode('unicode_escape'))
                            diagram.SetPath(parentNode.GetPath() + "/" + diagram.GetName() + ":=Diagram=")
                            parentNode.AddDiagram(diagram)
                            for pic in area:
                                if pic.tag == UMLPROJECT_NAMESPACE+"element":
                                    element = CElement(diagram,ListObj[pic.get("id").decode('unicode_escape')],True)
                                    element.SetPosition((int(pic.get("x").decode('unicode_escape')),int(pic.get("y").decode('unicode_escape'))))
                                    dw = int(pic.get("dw").decode('unicode_escape'))
                                    dh = int(pic.get("dh").decode('unicode_escape'))
                                    element.SetSizeRelative((dw, dh))
                                elif pic.tag == UMLPROJECT_NAMESPACE+"connection":
                                    for e in diagram.GetElements():
                                        if e.GetObject() is ListCon[pic.get("id").decode('unicode_escape')].GetSource():
                                            source = e
                                        if e.GetObject() is ListCon[pic.get("id").decode('unicode_escape')].GetDestination():
                                            destination = e
                                    conect = CConnection(diagram,ListCon[pic.get("id").decode('unicode_escape')],source,destination,[])
                                    for propCon in pic:
                                        if propCon.tag == UMLPROJECT_NAMESPACE+"point":
                                            conect.AddPoint((int(propCon.get("x").decode('unicode_escape')),int(propCon.get("y").decode('unicode_escape'))))
                                        elif propCon.tag == UMLPROJECT_NAMESPACE+"label":
                                            conect.SetLabelPosition(int(propCon.get("num").decode('unicode_escape')),
                                                int(propCon.get("index").decode('unicode_escape')),
                                                float(propCon.get("section").decode('unicode_escape')),
                                                int(propCon.get("distance").decode('unicode_escape')),
                                                float(propCon.get("angle").decode('unicode_escape')))

        root = etree.XML(data)

        #xml (version) file is validate with xsd schema (metamodel.xsd)
        if HAVE_LXML:
            if not xmlschema.validate(root):
                raise XMLError(xmlschema.error_log.last_error)

        for element in root:
            if element.tag == UMLPROJECT_NAMESPACE+'objects':
                for subelem in element:
                    if subelem.tag == UMLPROJECT_NAMESPACE+'object':
                        id = subelem.get("id").decode('unicode_escape')
                        object = CElementObject(self.ElementFactory.GetElement(subelem.get("type").decode('unicode_escape')))

                        for property in subelem:
                            if property.get("value") is not None:
                                object.SetAttribute(property.get("name").decode('unicode_escape'),property.get("value").decode('unicode_escape'))
                            elif property.get("type") is not None:
                                attributes = []
                                for item in property:
                                    atrib = {}
                                    for attribute in item:
                                        atrib[attribute.get("name").decode('unicode_escape')] = attribute.get("value").decode('unicode_escape')
                                    if len(atrib) > 0:
                                        attributes.append(atrib)
                                object.SetAttribute(property.get("name").decode('unicode_escape'),attributes)
                        ListObj[id] = object

            elif element.tag == UMLPROJECT_NAMESPACE+'connections':
                for connection in element:
                    if connection.tag == UMLPROJECT_NAMESPACE+'connection':
                        id = connection.get("id").decode('unicode_escape')
                        con = CConnectionObject(self.ConnectionFactory.GetConnection(connection.get("type").decode('unicode_escape')),ListObj[connection.get("source").decode('unicode_escape')],ListObj[connection.get("destination").decode('unicode_escape')])
                        for propCon in connection:
                            con.SetAttribute(propCon.get("name").decode('unicode_escape'),propCon.get("value").decode('unicode_escape'))
                        ListCon[id] = con
            elif element.tag == UMLPROJECT_NAMESPACE+'projecttree':
                for subelem in element:
                    if subelem.tag == UMLPROJECT_NAMESPACE+'node':
                        proNode = CProjectNode(None,ListObj[subelem.get("id").decode('unicode_escape')],ListObj[subelem.get("id").decode('unicode_escape')].GetName() + ":" + ListObj[subelem.get("id").decode('unicode_escape')].GetType().GetId())
                        self.SetRoot(proNode)
                        CreateTree(subelem,proNode)
                        
        
    Root = property(GetRoot, SetRoot)
    