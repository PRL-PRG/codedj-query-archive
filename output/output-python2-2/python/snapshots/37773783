from lib.lib import UMLException, XMLEncode, IDGenerator
from ProjectNode import CProjectNode
from cStringIO import StringIO
from zipfile import ZipFile, ZIP_STORED, ZIP_DEFLATED

class CProject(object):
    def __init__(self, file = None):
        self.root = None
      
    def SetRoot(self, value):
        self.root = value
    
    def GetRoot(self):
        return self.root
    
    def GetNode(self, path):
        node = self.root
        
        k = path.split('/')[0]
        i,j = k.split(':')
                
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
        node.GetParent(node).RemoveChild(node)
    
    def searchCE(self, node): # search for all connections and elements under given node
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
    
    def SaveProject(self, filename):
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
                    print>>f, '  '*level+'    <drawingarea name="%s">'%XMLEncode(area.GetName())
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
        print>>f, '<?xml version="1.0">'
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

    Root = property(GetRoot, SetRoot)