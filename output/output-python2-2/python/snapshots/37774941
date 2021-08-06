from lib.Depend.etree import etree, HAVE_LXML

from lib.lib import XMLEncode, IDGenerator, Indent
from ProjectNode import CProjectNode
from cStringIO import StringIO
from zipfile import ZipFile, ZIP_STORED, ZIP_DEFLATED
from lib.Exceptions.UserException import *
from lib.Exceptions.DevException import DomainObjectError
from lib.Storages import open_storage
from lib.Drawing import CElement
from lib.Drawing import CConnection
from lib.Domains import CDomainFactory
from lib.Elements.Type import CElementType
from lib.Elements.Object import CElementObject
from lib.Connections.Object import CConnectionObject
from lib.Elements.Factory import CElementFactory
from lib.Diagrams.Factory import CDiagramFactory
from lib.Connections.Factory import CConnectionFactory
from lib.Versions.Factory import CVersionFactory
from lib.Drawing import CDiagram
import os.path
from lib.consts import ROOT_PATH, VERSIONS_PATH, DIAGRAMS_PATH, ELEMENTS_PATH, CONNECTIONS_PATH, DOMAINS_PATH, UMLPROJECT_NAMESPACE, PROJECT_EXTENSION, PROJECT_CLEARXML_EXTENSION
from lib.config import config

#if lxml.etree is imported successfully, we use xml validation with xsd schema
if HAVE_LXML:
    xmlschema_doc = etree.parse(os.path.join(config['/Paths/Schema'], "umlproject.xsd"))
    xmlschema = etree.XMLSchema(xmlschema_doc)


class CProject(object):
    SaveVersion = '1.0' # save file format version
    
    def __init__(self):
        self.root = None
        
        self.Storage = open_storage(os.path.join(ROOT_PATH, 'etc', 'uml'))
        self.DomainFactory = CDomainFactory(self.Storage, DOMAINS_PATH)
        self.ElementFactory = CElementFactory(self.Storage, ELEMENTS_PATH, self.DomainFactory)
        self.DiagramFactory = CDiagramFactory(self.Storage, DIAGRAMS_PATH)
        self.ConnectionFactory = CConnectionFactory(self.Storage, CONNECTIONS_PATH, self.DomainFactory)
        self.VersionFactory = CVersionFactory(self.Storage, VERSIONS_PATH)
        self.version = self.VersionFactory.GetVersion('UML 1.4')
        self.MetamodelVersion = '1.4.0'
        self.MetamodelUri = 'http://umlfri.kst.fri.uniza.sk/metamodel/uml.frim'
        
        self.defaultDiagram = None
        
        self.filename = None
    
    def GetDefaultDiagrams(self):
        if self.defaultDiagram is not None:
            yield self.defaultDiagram
    
    def AddDefaultDiagram(self, diagram):
        self.defaultDiagram = diagram
    
    def DeleteDefaultDiagram(self, diagram):
        if self.defaultDiagram is diagram:
            self.defaultDiagram = None
    
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
            return self.root
        
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
        
        def SaveDomainObjectInfo(data, name=None):
            '''
            '''
            if isinstance(data, dict):
                element = etree.Element(UMLPROJECT_NAMESPACE+'dict')
                for key, value in data.iteritems():
                    element.append(SaveDomainObjectInfo(value, key))
            elif isinstance(data, list):
                element = etree.Element(UMLPROJECT_NAMESPACE+'list')
                for value in data:
                    element.append(SaveDomainObjectInfo(value))
            elif isinstance(data, (str, unicode)):
                element = etree.Element(UMLPROJECT_NAMESPACE+'text')
                element.text = data
            else:
                raise Exception("unknown data format")
            if name:
                element.set('name', name)
            return element
        
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
                    if area is self.defaultDiagram:
                        diagramNode.attrib['default'] = 'true'
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

                        for num, info in enumerate(c.GetAllLabelPositions()):
                            connectionNode.append(etree.Element(UMLPROJECT_NAMESPACE+'label', 
                                dict(map(lambda x: (x[0], unicode(x[1])), info.iteritems())), #transform {key:value, ...} -> {key:unicode(value), ...}
                                num=unicode(num)))

                        diagramNode.append(connectionNode)
                    diagramsNode.append(diagramNode)
            nodeNode.append(diagramsNode)
            element.append(nodeNode)
        
        elements, connections = self.searchCE(self.root)
        
        rootNode = etree.XML('<umlproject saveversion="%s" xmlns="http://umlfri.kst.fri.uniza.sk/xmlschema/umlproject.xsd"></umlproject>'%self.SaveVersion)
        
        metamodelNode = etree.Element(UMLPROJECT_NAMESPACE+'metamodel')
        objectsNode = etree.Element(UMLPROJECT_NAMESPACE+'objects')
        connectionsNode = etree.Element(UMLPROJECT_NAMESPACE+'connections')
        projtreeNode = etree.Element(UMLPROJECT_NAMESPACE+'projecttree')
        counterNode = etree.Element(UMLPROJECT_NAMESPACE+'counters')
        
        # metamodel informations
        metamodelUriNode = etree.Element(UMLPROJECT_NAMESPACE+'uri')
        metamodelUriNode.text = self.MetamodelUri
        metamodelVersionNode = etree.Element(UMLPROJECT_NAMESPACE+'version')
        metamodelVersionNode.text = self.MetamodelVersion
        
        metamodelNode.append(metamodelUriNode)
        metamodelNode.append(metamodelVersionNode)
        rootNode.append(metamodelNode)
        
        for object in elements:
            objectNode = etree.Element(UMLPROJECT_NAMESPACE+'object', type=unicode(object.GetType().GetId()), id=unicode(id(object)))
            objectNode.append(SaveDomainObjectInfo(object.GetSaveInfo()))
            objectsNode.append(objectNode)
            
        rootNode.append(objectsNode)
        
        for connection in connections:
            connectionNode = etree.Element(UMLPROJECT_NAMESPACE+'connection', type=unicode(connection.GetType().GetId()), id=unicode(id(connection)), source=unicode(id(connection.GetSource())), destination=unicode(id(connection.GetDestination())))
            connectionNode.append(SaveDomainObjectInfo(connection.GetSaveInfo()))
            connectionsNode.append(connectionNode)
            
        rootNode.append(connectionsNode)
        savetree(self.root, projtreeNode)
        rootNode.append(projtreeNode)
        
        for type in self.ElementFactory.IterTypes():
            counterNode.append(etree.Element(UMLPROJECT_NAMESPACE+'count', id = type.GetId(), value = unicode(type.GetCounter())))
        for type in self.DiagramFactory:
            counterNode.append(etree.Element(UMLPROJECT_NAMESPACE+'count', id = type.GetId(), value = unicode(type.GetCounter())))
        
        rootNode.append(counterNode)
        
        #xml tree is validate with xsd schema (recentfile.xsd)
        if HAVE_LXML:
            if not xmlschema.validate(rootNode):
                raise XMLError(xmlschema.error_log.last_error)

        #make human-friendly tree
        Indent(rootNode)
        
        #save Recent File Tree into ZIP file if it is .frip
        if (("." + filename.rsplit('.',1)[-1]) != PROJECT_CLEARXML_EXTENSION):     #zipped
            fZip = ZipFile(filename, 'w', ZIP_DEFLATED)
            fZip.writestr('content.xml', '<?xml version="1.0" encoding="utf-8"?>\n'+etree.tostring(rootNode, encoding='utf-8'))
            fZip.close()
        else:                                       #no zipped - clear xml file
            f = open(filename, 'w')
            f.write('<?xml version="1.0" encoding="utf-8"?>\n'+etree.tostring(rootNode, encoding='utf-8'))
            f.close()
    
    def LoadProject(self, filename, copy = False):
        ListObj = {}
        ListCon = {}
        
        ext = filename.split('.')
        ext.reverse()
        if (("."+ext[0]) != PROJECT_CLEARXML_EXTENSION):
            file = ZipFile(filename,'r')
            data = file.read('content.xml')
        else:
            file = open(filename, 'r')
            data = file.read()
        
        if copy:
            self.filename = None
        else:
            self.filename = filename
        
        def LoadDomainObjectInfo(element):
            '''
            Transform element back to the dictionary readable by 
            L{CDomainObject.SetSaveInfo<lib.Domains.Object.CDomainObject.SetSaveInfo>}
            
            @return: structured dictionary
            @rtype: dict
            '''
            if element.tag == UMLPROJECT_NAMESPACE + 'dict':
                return dict([(item.get('name'), LoadDomainObjectInfo(item)) for item in element])
            elif element.tag == UMLPROJECT_NAMESPACE + 'list':
                return [LoadDomainObjectInfo(item) for item in element]
            elif element.tag == UMLPROJECT_NAMESPACE + 'text':
                return element.text
            else:
                raise ProjectError("malformed project file")
        
        def CreateTree(root, parentNode):
            for elem in root:
                if elem.tag == UMLPROJECT_NAMESPACE+'childs':
                    for node in elem:
                        proNode = CProjectNode(parentNode,ListObj[node.get("id")],parentNode.GetPath() + "/" + ListObj[node.get("id")].GetName() + ":" + ListObj[node.get("id")].GetType().GetId())
                        self.AddNode(proNode,parentNode)
                        CreateTree(node,proNode)

                elif elem.tag == UMLPROJECT_NAMESPACE+'diagrams':
                    for area in elem:
                        if area.tag == UMLPROJECT_NAMESPACE+'diagram':
                            diagram = CDiagram(self.DiagramFactory.GetDiagram(area.get("type")),area.get("name"))
                            diagram.SetPath(parentNode.GetPath() + "/" + diagram.GetName() + ":=Diagram=")
                            if 'default' in area.attrib and area.attrib['default'].lower() in ('1', 'true'):
                                self.defaultDiagram = diagram
                            parentNode.AddDiagram(diagram)
                            for pic in area:
                                if pic.tag == UMLPROJECT_NAMESPACE+"element":
                                    element = CElement(diagram,ListObj[pic.get("id")],True)
                                    element.SetPosition((int(pic.get("x")),int(pic.get("y"))))
                                    dw = int(pic.get("dw"))
                                    dh = int(pic.get("dh"))
                                    element.SetSizeRelative((dw, dh))
                                elif pic.tag == UMLPROJECT_NAMESPACE+"connection":
                                    for e in diagram.GetElements():
                                        if e.GetObject() is ListCon[pic.get("id")].GetSource():
                                            source = e
                                        if e.GetObject() is ListCon[pic.get("id")].GetDestination():
                                            destination = e
                                    conect = CConnection(diagram,ListCon[pic.get("id")],source,destination,[])
                                    for propCon in pic:
                                        if propCon.tag == UMLPROJECT_NAMESPACE+"point":
                                            conect.AddPoint((int(propCon.get("x")),int(propCon.get("y"))))
                                        elif propCon.tag == UMLPROJECT_NAMESPACE+"label":
                                            data = dict(propCon.items())
                                            del data["num"]
                                            conect.RestoreLabelPosition(int(propCon.get("num")), data)

        root = etree.XML(data)

        #xml (version) file is validate with xsd schema (metamodel.xsd)
        if HAVE_LXML:
            if not xmlschema.validate(root):
                raise XMLError(xmlschema.error_log.last_error)

        for element in root:
            if element.tag == UMLPROJECT_NAMESPACE+'objects':
                for subelem in element:
                    if subelem.tag == UMLPROJECT_NAMESPACE+'object':
                        id = subelem.get("id")
                        object = CElementObject(self.ElementFactory.GetElement(subelem.get("type")))
                        object.SetSaveInfo(LoadDomainObjectInfo(subelem[0]))
                        ListObj[id] = object

            elif element.tag == UMLPROJECT_NAMESPACE+'connections':
                for connection in element:
                    if connection.tag == UMLPROJECT_NAMESPACE+'connection':
                        id = connection.get("id")
                        con = CConnectionObject(self.ConnectionFactory.GetConnection(connection.get("type")),ListObj[connection.get("source")],ListObj[connection.get("destination")])
                        con.SetSaveInfo(LoadDomainObjectInfo(connection[0]))
                        ListCon[id] = con
            
            elif element.tag == UMLPROJECT_NAMESPACE+'projecttree':
                for subelem in element:
                    if subelem.tag == UMLPROJECT_NAMESPACE+'node':
                        proNode = CProjectNode(None,ListObj[subelem.get("id")],ListObj[subelem.get("id")].GetName() + ":" + ListObj[subelem.get("id")].GetType().GetId())
                        self.SetRoot(proNode)
                        CreateTree(subelem,proNode)
            
            elif element.tag == UMLPROJECT_NAMESPACE + 'counters':
                for item in element:
                    if self.ElementFactory.HasType(item.get('id')):
                        self.ElementFactory.GetElement(item.get('id')).SetCounter(int(item.get('value')))
                    elif self.DiagramFactory.HasType(item.get('id')):
                        self.DiagramFactory.GetDiagram(item.get('id')).SetCounter(int(item.get('value')))
                        
        
    Root = property(GetRoot, SetRoot)
    
