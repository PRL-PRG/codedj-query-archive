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
            pass
            
    def GetConnections(self):
        for c in self.connections:
            yield c
        
    def GetType(self):
        return self.type
    
    def GetWidth(self, canvas, element):
        return self.type.GetWidth(canvas, element)
    
    def GetHeight(self, canvas, element):
        return self.type.GetHeight(canvas, element)
        
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

    def Paint(self, canvas, element):
        self.type.Paint(canvas, element)

    def RemoveAttribute(self, key):
        if self.attribs.has_key(key):
            del self.attribs[key]
        else:
            raise UMLException("KeyError")
    
    def HasAttribute(self, key):
        return key in self.attribs
            
    def SetAttribute(self, key, value):
        self.attribs[key] = value
        
    def Disconnect(self, connection):
        connection.Disconnect()
        
    def RemoveConnection(self, connection):
        if connection in self.connections:
            self.connections.remove(connection)
        else:
            raise UMLException("ConnectionNotFound")