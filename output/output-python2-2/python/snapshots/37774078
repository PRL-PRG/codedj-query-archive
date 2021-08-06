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
    
    def SetId(self, value):
        self.id = value
    
    def SetDestArrow(self, value):
        self.destArrow = value
    
    def SetSrcArrow(self, value):
        self.scrArrow = value

    def Paint(self, Connection):
        tmp = Connection.GetPoints()
        Xo,Yo = tmp[0]
        for i in tmp[1:]:
            self.line.Paint(Xo,Yo,i[0],i[1],Connection)
            Xo,Yo = i
            
        if self.scrArrow is not None:
            X = tmp[0][0] - tmp[1][0]
            Y = tmp[0][1] - tmp[1][1]
            self.scrArrow.Paint(tmp[0][0],tmp[0][1],atan2(-X,Y),Connection)
        
        if self.destArrow is not None:
            X = tmp[-1][0] - tmp[-2][0]
            Y = tmp[-1][1] - tmp[-2][1]
            self.destArrow.Paint(tmp[-1][0],tmp[-1][1],atan2(-X,Y),Connection)
            
        for id, lbl in enumerate(self.labels):
            x, y = Connection.GetLabelPosition(lbl[0], id)
            lbl[1].Paint(x, y, Connection)
    
    def GetLabels(self):
        for id, label in enumerate(self.labels):
            yield id, label[0]
    
    def GetAttributes(self):
        for i in self.attributes:
            yield i
    
    def GetVisAttr(self, id):
        if id in self.visAttrs:
            return self.visAttrs[id]
        else:
            raise UMLException('VisAttrDontExists')
    
    ID = property(GetId, SetId)
    Icon = property(GetIcon, SetIcon)
    DestinationArrow = property(GetDestArrow, SetDestArrow)
    SourceArrow = property(GetSrcArrow, SetSrcArrow)