from Line import CConnectionLine
from Arrow import CConnectionArrow
from math import atan2

class CConnectionType(object):
    def __init__(self, id, line = None, scrArrow = None, destArrow = None, icon = None):
        self.line = line
        self.scrArrow = scrArrow
        self.destArrow = destArrow
        self.id = id
        self.icon = icon
        self.labels = []
        self.attributes = {}
        self.attributeList = []
        self.visAttrs = {}
    
    def SetIcon(self, value):
        self.icon = value
    
    def GetDefValue(self, id):
        type, options = self.attributes[id]
        if len(options) > 0:
            temp = options[0]
        else:
            temp = None
        if type == 'int':
            if temp is None:
                return 0
            else:
                return int(temp)
        elif type == 'float':
            if temp is None:
                return 0.0
            else:
                return float(temp)
        elif type == 'bool':
            if temp is None:
                return False
            else:
                return ToBool(temp)
        elif type == 'str':
            if temp is None:
                return ""
            else:
                return str(temp)
        elif type == 'attrs':
            return []
        elif type == 'opers':
            return []
    
    def AppendAttribute(self, value, type, propid = None, options = []):
        if propid is not None:
            self.visAttrs[propid] = value
        self.attributes[value] = (type, options)
        self.attributeList.append(value)
    
    def AddLabel(self, position, label):
        self.labels.append((position,label))
    
    def RemoveLabel(self, label):
        for id, i in enumerate(self.labels):
            if i[1] is label:
                del self.labels[id]
                return
        else:
            raise UMLException("LabelNotExists")
    
    def GetIcon(self):
        return self.icon
    
    def GetDestArrow(self):
        return self.destArrow
    
    def GetLine(self):
        return self.line

    def GetSrcArrow(self):
        return self.scrArrow
    
    def GetId(self):
        return self.id
    
    def SetDestArrow(self, value):
        self.destArrow = value
    
    def SetSrcArrow(self, value):
        self.scrArrow = value

    def HasVisualAttribute(self, id):
        return id in self.visAttrs.itervalues()

    def Paint(self, canvas, Connection, delta = (0, 0)):
        dx, dy = delta
        tmp = [(x + dx, y + dy) for (x, y) in Connection.GetPoints(canvas)]
        o = tmp[0]
        for i in tmp[1:]:
            self.line.Paint(canvas, o, i, Connection)
            o = i
        
        if self.scrArrow is not None:
            X = tmp[0][0] - tmp[1][0]
            Y = tmp[0][1] - tmp[1][1]
            self.scrArrow.Paint(canvas, tmp[0], atan2(-X, Y), Connection)
        
        if self.destArrow is not None:
            X = tmp[-1][0] - tmp[-2][0]
            Y = tmp[-1][1] - tmp[-2][1]
            self.destArrow.Paint(canvas, tmp[-1], atan2(-X, Y), Connection)
        
        for id, lbl in enumerate(self.labels):
            size = lbl[1].GetSize(canvas, Connection)
            pos = Connection.GetLabelPosition(canvas, id, lbl[0], size)
            lbl[1].Paint(canvas, pos, Connection)
    
    def GetLabels(self):
        for id, label in enumerate(self.labels):
            yield id, label[0]
            
    def GetAttribute(self, key):
        if key in self.attributes:
            return self.attributes[key]
        else:
            raise UMLException("BadKey")
    
    def GetAttributes(self):
        for i in self.attributeList:
            yield i
    
    def GetVisAttr(self, id):
        if id in self.visAttrs:
            return self.visAttrs[id]
        else:
            raise UMLException('VisAttrDontExists')
    
    ID = property(GetId)
    Icon = property(GetIcon, SetIcon)
    DestinationArrow = property(GetDestArrow, SetDestArrow)
    SourceArrow = property(GetSrcArrow, SetSrcArrow)