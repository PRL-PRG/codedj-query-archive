from lib.lib import UMLException

class CElementObject:
    def __init__(self, type):
        self.type = type
        self.connections = []
        self.attribs = {}
        for i in self.type.GetAttributes():
            self.SetAttribute(i, self.type.GetDefValue(i))            
        self.SetAttribute('Name', 'New ' + type.GetId())        

    def AddConnection(self, connection):
        if connection not in self.connections:
            self.connections.append(connection)
        else:
            raise UMLException("ConnectionAlreadyExists")
            
    def GetConnections(self):
        return self.connections
        
    def GetType(self):
        return self.type
    
    def GetWidth(self, element):
        return self.type.GetWidth(element)
    
    def GetHeight(self, element):
        return self.type.GetHeight(element)
        
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
        return self.attribs
        
    def GetVisualProperty(self, key):
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
                if 'type' in vi and vi['type']:
                    l += ": "+vi['type']
                if 'initial' in vi and vi['initial']:
                    l += " = "+vi['initial']
                o['line'] = l
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
                if 'parameters' in vi and vi['parameters']:
                    l += vi['parameters']
                l += ")"
                if 'type' in vi and vi['type']:
                    l += ": "+vi['type']
                o['line'] = l
                v.append(o)
            val = v
        return val

    def Paint(self, element):
        self.type.Paint(element)

    def RemoveAttribute(self, key):
        if self.attribs.has_key(key):
            del self.attribs[key]
        else:
            raise UMLException("KeyError")
    
    def HasAttribute(self, key):
        return key in self.attribs
            
    def SetAttribute(self, key, value):
        self.attribs[key] = value
        
    def RemoveConnection(self, connection):
        if connection in self.connections:
            self.connections.remove
        else:
            raise UMLException("ConnectionNotFound")