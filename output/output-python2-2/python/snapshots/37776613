from Line import CConnectionLine
from Arrow import CConnectionArrow
from math import atan2
from lib.lib import UMLException

class CConnectionType(object):
    """
    Contains part of metamodel that represents connection type
    """
    def __init__(self, id, line = None, scrArrow = None, destArrow = None, icon = None):
        """
        Initialize connection type and fill its properties
        
        @param id: name of this connection type
        @type  id: string
        
        @param line: line style
        @type  line: L{CConnectionLine<Line.CConnectionLine>}
        
        @param scrArrow: source arrow style
        @type  scrArrow: L{CConnectionArrow<Arrow.CConnectionArrow>}
        
        @param destArrow: destination arrow style
        @type  destArrow: L{CConnectionArrow<Arrow.CConnectionArrow>}
        
        @param icon: path to connection icon within metamodel storage
        @type  icon: string
        """
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
        """
        Set icon path to new value
        
        @param value: icon path
        @type  value: string
        """
        self.icon = value
    
    def GetDefValue(self, id):
        """
        Get default value for given attribute
        
        @param id: name of the attribute
        @type  id: string
        
        @return: default value
        @rtype:  anything
        """
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
        """
        Append new attribute to the connection type
        
        @param value: name of the attribute
        @type  value: string
        
        @param type: attribute domain
        @type  type: string
        
        @param propid: property ID for this attribute
        @type  propid: string
        
        @param options: enumeration of correct values
        @type  options: list of strings
        """
        if propid is not None:
            self.visAttrs[propid] = value
        self.attributes[value] = (type, options)
        self.attributeList.append(value)
    
    def AddLabel(self, position, label):
        """
        Add label to connection type
        
        @param position: initial position of label.
            (one of source, center, destination)
        @type  position: string
        
        @param label: visual object representing the label
        @type  label: L{CVisualObject<lib.Drawing.Objects.VisualObject.CVisualObject>}
        """
        self.labels.append((position,label))
    
    def RemoveLabel(self, label):
        """
        Remove label from connection
        
        @param label: visual object representing the label
        @type  label: L{CVisualObject<lib.Drawing.Objects.VisualObject.CVisualObject>}
        """
        for id, i in enumerate(self.labels):
            if i[1] is label:
                del self.labels[id]
                return
        else:
            raise UMLException("LabelNotExists")
    
    def GetIcon(self):
        """
        Get the icon of this connection
        
        @return: icon path within metamodel storage
        @rtype:  string
        """
        return self.icon
    
    def GetDestArrow(self):
        """
        Get the destination arrow
        
        @return: destination arrow object
        @rtype:  L{CConnectionArrow<Arrow.CConnectionArrow>}
        """
        return self.destArrow
    
    def GetLine(self):
        """
        Get the connection line style
        
        @return: line object
        @rtype:  L{CConnectionLine<Line.CConnectionLine>}
        """
        return self.line

    def GetSrcArrow(self):
        """
        Get the source arrow
        
        @return: source arrow object
        @rtype:  L{CConnectionArrow<Arrow.CConnectionArrow>}
        """
        return self.scrArrow
    
    def GetId(self):
        """
        Return name (Id) of this connection type
        
        @return: name of connection
        @rtype:  string
        """
        return self.id
    
    def SetDestArrow(self, value):
        """
        Set destination arrow style
        
        @param value: arrow object to set as destination arrow
        @type  value: L{CConnectionArrow<Arrow.CConnectionArrow>}
        """
        self.destArrow = value
    
    def SetSrcArrow(self, value):
        """
        Set source arrow style
        
        @param value: arrow object to set as source arrow
        @type  value: L{CConnectionArrow<Arrow.CConnectionArrow>}
        """
        self.scrArrow = value

    def HasVisualAttribute(self, id):
        """
        Determine, if object has visual attribute
        
        @param id: visual attribute name
        @type  id: string
        
        @return: True, id attribute exists
        @rtype: boolean
        """
        return id in self.visAttrs.itervalues()

    def Paint(self, canvas, connection, delta = (0, 0)):
        """
        Paint connection of given type on canvas
        
        @param canvas: Connection will be painted on this canvas
        @type  canvas: L{CAbstractCanvas<lib.Drawing.Canvas.Abstract.CAbstractCanvas>}
        
        @param connection: connection, which has to be painted
        @type  connection: L{CConnection<lib.Drawing.Connection.CConnection>}
        
        @param delta: translation of point (0, 0)
        @type  delta: (integer, integer)
        """
        dx, dy = delta
        tmp = [(x + dx, y + dy) for (x, y) in connection.GetPoints(canvas)]
        o = tmp[0]
        for i in tmp[1:]:
            self.line.Paint(canvas, o, i)
            o = i
        
        if self.scrArrow is not None:
            X = tmp[0][0] - tmp[1][0]
            Y = tmp[0][1] - tmp[1][1]
            self.scrArrow.Paint(canvas, tmp[0], atan2(-X, Y))
        
        if self.destArrow is not None:
            X = tmp[-1][0] - tmp[-2][0]
            Y = tmp[-1][1] - tmp[-2][1]
            self.destArrow.Paint(canvas, tmp[-1], atan2(-X, Y))
        
        for id, lbl in enumerate(self.labels):
            size = lbl[1].GetSize(canvas, connection)
            pos = connection.GetLabelPosition(canvas, id, lbl[0], size)
            lbl[1].Paint(canvas, pos, connection)
    
    def GetLabels(self):
        """
        Get list of all labels on this connection type
        
        @return: all labels
        @rtype:  iterator over (string, L{CVisualObject<lib.Drawing.Objects.VisualObject.CVisualObject>}) pairs
        """
        for id, label in enumerate(self.labels):
            yield id, label[0]
            
    def GetAttribute(self, key):
        """
        Get type of given attribute
        
        @param key: name of attribute
        @type  key: string
        
        @return: type with options enumeration included
        @rtype: (string, list of strings)
        """
        if key in self.attributes:
            return self.attributes[key]
        else:
            raise UMLException("BadKey")
    
    def GetAttributes(self):
        """
        Get list of all attribute names
        
        @return: attribute enumeration
        @rtype: iterator over strings
        """
        for i in self.attributeList:
            yield i
    
    def GetVisAttr(self, id):
        """
        Get name of attribute with given property id
        
        @param id: attribute name
        @type  id: string
        
        @return: property id
        @rtype:  string
        """
        if id in self.visAttrs:
            return self.visAttrs[id]
        else:
            raise UMLException('VisAttrDontExists')
    
    ID = property(GetId)
    Icon = property(GetIcon, SetIcon)
    DestinationArrow = property(GetDestArrow, SetDestArrow)
    SourceArrow = property(GetSrcArrow, SetSrcArrow)