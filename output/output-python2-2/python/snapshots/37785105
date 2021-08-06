from lib.Elements import CElementObject
from lib.Connections import CConnectionObject
from lib.Project import CProjectNode

class CNode:
    def __init__(self, xmlelem, lexem):
        self.childs = []
        self.values = []
        self.xmlelem = xmlelem
        self.lexem = lexem
        self.projectnode = None
        self.parent = None
    
    def SetProjectNode(self, node):
        self.projectnode = node
        
    def AddChild(self, child):
        child.parent = self
        self.childs.append(child)
        
    def AddValue(self, value):
        self.values.append(value)
            
    def GetParent(self):
        return self.parent
        
    def __repr__(self, depth = 0):
        result = str(depth)+' '*depth + `self.xmlelem` + ' '*(depth+2) +`self.values`+ ' '+`self.lexem` + '\n'
        for child in self.childs:
            result += child.__repr__(depth+1)
        return result
        
    def GetProjectNode(self):
        return self.projectnode
        
    def GetParentElement(self):
        return self.parent.GetParentElement()
        
    def GetParentProperty(self):
        return self.parent.GetParentProperty()
        
    def Execute(self):
        pass
    
    def Create(self, ElementFactory):
        for child in self.childs:
            child.Create(ElementFactory)
            
    def Search(self, Project):
        for child in self.childs:
            child.Search(Project)
            
    def Connect(self, ConnectionFactory):
        for child in self.childs:
            child.Connect(ConnectionFactory)
        

class CElementNode(CNode):
    def __repr__(self, depth = 0):
        result = 'E'+' '*depth + `self.xmlelem` + ' '*(depth+2) +`self.values`+ ' '+`self.lexem` + '\n'
        for child in self.childs:
            result += child.__repr__(depth+1)
        return result
        
    def Create(self, ElementFactory):
        if self.xmlelem.CreateNew():
            parentnode = self.GetParent().GetProjectNode()
            type = ElementFactory.GetElement(self.xmlelem.GetType())
            object = CElementObject(type)
            try:
                object.SetAttribute('Name', self.lexem)
            except:
                pass
            node = CProjectNode(parentnode, object)
            parentnode.AddChild(node)
            self.projectnode = node
            for child in self.childs:
                child.Create(ElementFactory)
    
    def GetParentElement(self):
        return self
        
    def Search(self, Project):
        if not self.xmlelem.CreateNew():
            name = None
            for child in self.childs:
                if isinstance(child, CAttributeNode) and child.xmlelem.id == '#Name':
                    name = child.lexem
            self.projectnode = Project.Find( name )
        for child in self.childs:
            child.Search(Project)
        
class CPropertyNode(CNode):
    def __repr__(self, depth = 0):
        result = 'P'+' '*depth + `self.xmlelem` + ' '*(depth+2) +`self.values`+ ' '+`self.lexem` + '\n'
        for child in self.childs:
            result += child.__repr__(depth+1)
        return result
        
    #~ def Create(self, ElementFactory):
        #~ pass
        
    def GetParentProperty(self):
        return self
        
    def Create(self, ElementFactory):
        if self.xmlelem.CreateNew():
            parentnode = self.parent.GetParentElement().GetProjectNode()
            if parentnode:
                self.projectnode =  parentnode.GetObject().GetType().GetBlankAttribute(self.xmlelem.collection)
                for child in self.childs:
                    child.Create(ElementFactory)
                oldprops = parentnode.GetObject().GetAttribute(self.xmlelem.collection)
                parentnode.GetObject().SetAttribute(self.xmlelem.collection, oldprops + [self.projectnode])
            
    def Search(self, Project):
        if not self.xmlelem.CreateNew():
            try:
                name = None
                for child in self.childs:
                    if isinstance(child, CAttributeNode) and child.xmlelem.id == '@Name':
                        name = child.lexem
                properties = self.parent.GetParentElement().GetProjectNode().GetObject().GetAttribute(self.xmlelem.collection)
                for property in properties:
                    if property['Name'] == name:
                        self.projectnode = property
                for child in self.childs:
                    child.Create(ElementFactory)
            except:
                pass
                
        
            
class CAttributeNode(CNode):
    #~ def __init__(self, action, lexem):
    def __repr__(self, depth = 0):
        result = 'A'+' '*depth + `self.xmlelem` + ' '*(depth+2) +`self.values`+ ' '+`self.lexem` + '\n'
        for child in self.childs:
            result += child.__repr__(depth+1)
        return result

    def Create(self, ElementFactory):
        parentnode = None
        id = self.xmlelem.id
        if id[0] == '#':
            parentnode = self.GetParentElement().GetProjectNode()
            if parentnode:
                if self.values:
                    value = ''.join(self.values)
                else:
                    value = self.lexem
                object = parentnode.GetObject()
                object.SetAttribute(id[1:], value)
        if id[0] == '@':
            parentnode = self.GetParentProperty().GetProjectNode()
            if parentnode:
                if self.values:
                    value = ''.join(self.values)
                else:
                    value = self.lexem
                parentnode[id[1:]] = value
        if parentnode:
            for child in self.childs:
                child.Create(ElementFactory)

class CConnectionNode(CNode):
    def __repr__(self, depth = 0):
        result = 'C'+' '*depth + `self.xmlelem` + ' '*(depth+2) +`self.values`+ ' '+`self.lexem` + '\n'
        for child in self.childs:
            result += child.__repr__(depth+1)
        return result
        
    def Connect(self, ConnectionFactory):
        try:
            src = self.parent.GetProjectNode().GetObject()
            dest = self.childs[0].GetProjectNode().GetObject()
            type = ConnectionFactory.GetConnection(self.xmlelem.value)
            CConnectionObject(type, src, dest)
        except:
            pass
        for child in self.childs:
            child.Connect(ConnectionFactory)
