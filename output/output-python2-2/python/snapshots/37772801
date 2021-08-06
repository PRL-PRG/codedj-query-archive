from lib.lib import UMLException
import weakref

class CElementObject:
    def __init__(self, type):
        self.type = type
        self.path = None
        self.connections = []
        self.attribs = {}
        for i in self.type.GetAttributes():
            self.SetAttribute(i, self.type.GetDefValue(i))            
        if type.GetGenerateName():
            self.SetAttribute('Name', 'New' + type.GetId())
        else:
            self.SetAttribute('Name', '')
        self.node = lambda: None
        self.appears = []
        
        
    def GetAppears(self):
        for i in self.appears:
            yield i


    def AddAppears(self, drawingArea):
        self.appears.append(drawingArea)

    def RemoveAppears(self, drawingArea):
        self.appears.remove(drawingArea)
    
    def GetPath(self):
        return self.path
    
    def SetPath(self, path):
        self.path = path 
    
    def GetNode(self):
        return self.node()
        
    def AddConnection(self, connection):
        if connection not in self.connections:
            self.connections.append(connection)
        else:
            pass
            
    def GetConnections(self):
        for c in self.connections:
            yield c
    
    def CountConnections(self, type = "All"):
        if type == "All":
            return len(self.connections)
        else:
            return 2
        
    def GetType(self):
        return self.type
    
    def GetSize(self, canvas, element):
        return self.type.GetSize(canvas, element)
        
    def GetName(self):
        if 'Name' in self.attribs:
            return self.attribs['Name']
        else:
            raise UMLException("KeyError")

    def GetAttribute(self, key):
        if key in self.attribs:
            return self.attribs[key]
        else:
            return None
    
    def GetAttributes(self):
        for attr in self.attribs:
            yield attr
    
    
    def __CreateFullName(self):
        name = self.GetName()
        parent = self.GetNode().GetParent()
        while parent is not None:
            if parent.GetObject().GetType().GetId() == self.type.GetId():
                name = parent.GetObject().GetName() + "::" + name
            parent = parent.GetParent()
        return name
    
    def GetProperty(self, key = None):

        if key == "fullname":
            return self.__CreateFullName()

        if self.type.HasKeyVisualAttribute(key):
            attr = self.type.GetVisAttr(key)
        else:
            attr = key
        
        type = self.type.GetAttribute(attr)
        if type is None:
            return None
        val = self.attribs[attr]

        if type[0] == 'attrs':
            v = []
            for vi in val:
                o = {}
                o = vi
                o['default'] = vi['initial']
                if vi['getter'] != "":
                    o['getter'] = vi['getter'].split('(')[0]
                if vi['setter'] != "":
                    o['setter'] = vi['setter'].split('(')[0]
                v.append(o)
            val = v
        elif type[0] == 'opers':
            v = []
            for vi in val:
                v.append(vi)
            val = v
        return val
  
    def ParseParams(self, params):
        if len(params) == 0:
            return None
        listParams = params.split(',')
        ret = {}
        ret['paramsName'] = []
        ret['paramsType'] = []
        ret['paramsDefault'] = []
        for i in listParams:
            if len(i.split(':')) == 1:
                ret['paramsType'].append("")
                if len(i.split('=')) == 1:
                    ret['paramsDefault'].append("")
                    ret['paramsName'].append(i.split(':')[0])
                else:
                    ret['paramsDefault'].append(i.split('=')[1])
                    ret['paramsName'].append(i.split('=')[0])
            else:
                ret['paramsName'].append(i.split(':')[0])
                ret['paramsType'].append(i.split(':')[1].split('=')[0])
                if len(i.split(':')[1].split('=')) == 1:
                    ret['paramsDefault'].append("")
                else:
                    ret['paramsDefault'].append(i.split(':')[1].split('=')[1])        
        return ret
        
    
    def GetVisualProperty(self, key):
        if key == 'CHILDREN':
            node = self.node()
            if node is None:
                return []
            v = []
            for vi in node.GetChilds():
                o = {}
                o['icon'] = vi.GetObject().GetType().GetIcon()
                o['name'] = vi.GetObject().GetName()
                v.append(o)
            return v
        attr = self.type.GetVisAttr(key)
        type = self.type.GetAttribute(attr)
        val = self.attribs[attr]
        if type[0] == 'attrs':
            v = []
            for vi in val:
                s = ''
                o = {}
                if vi['scope'] == 'private':
                    o['scope'] = '-'
                elif vi['scope'] == 'public':
                    o['scope'] = '+'
                elif vi['scope'] == 'protected':
                    o['scope'] = '#'
                l = vi['name']
                o['name'] = vi['name']
                o['type'] = vi['type']
                if 'type' in vi and vi['type']:
                    l += ": "+vi['type']
                if 'initial' in vi and vi['initial']:
                    l += " = "+vi['initial']
                o['line'] = l
                o['static'] = vi['static']
                v.append(o)
            val = v
        elif type[0] == 'opers':
            v = []
            for vi in val:
                s = ''
                o = {}
                if vi['scope'] == 'private':
                    o['scope'] = '-'
                elif vi['scope'] == 'public':
                    o['scope'] = '+'
                elif vi['scope'] == 'protected':
                    o['scope'] = '#'
                l = vi['name']
                l += "("
                if 'params' in vi and vi['params']:
                    l += vi['params']
                l += ")"
                if 'type' in vi and vi['type']:
                    l += ": "+vi['type']
                o['line'] = l
                o['type'] = vi['type']
                o['name'] = vi['name']
                o['params'] = vi['params']
                o['static'] = vi['static']
                o['abstract'] = vi['abstract']
                v.append(o)
            val = v
        return val

    def Paint(self, canvas, element, delta = (0, 0)):
        self.type.Paint(canvas, element, delta)

    def RemoveAttribute(self, key):
        if self.attribs.has_key(key):
            del self.attribs[key]
        else:
            raise UMLException("KeyError")
    
    def HasAttribute(self, key):
        return key in self.attribs
            
    def SetAttribute(self, key, value):
        self.attribs[key] = self.type.TypeCastAttribute(key, value)
        
    def Disconnect(self, connection):
        connection.Disconnect()
        
    def RemoveConnection(self, connection):
        if connection in self.connections:
            self.connections.remove(connection)
        else:
            raise UMLException("ConnectionNotFound")
     
    # Automaticke generovanie mena elementu 
    # pomocou cprojNode zisti mena elementov na rovnakej urovni
    # ak meno uz existuje (a je rovnaky typ), objekt sa premenuje
    def Assign(self, cprojNode):
        if not self.type.GetGenerateName():
            return
        self.node = weakref.ref(cprojNode)
        if cprojNode.parent is not None:
            id = 1
            # zisti nazvy / typy deti, porovnaj a pripadne sa premenuj
            checkNames = True
            while checkNames :
                checkNames = False
                for child in cprojNode.parent.childs:
                    if child.GetName() == self.GetName() and child.GetObject().GetType() is self.GetType():
                        nName = self.GetName()
                        while nName[-1].isdigit(): # useknem cisla
                            nName = nName[:-1]
                        if nName.endswith(' '):
                            nName = nName + str(id)
                        else:
                            nName = nName + str(id)
                        self.SetAttribute('Name', nName)
                        id = id + 1
                        checkNames = True #znovu prekontroluj nazvy
            cprojNode.Change()
