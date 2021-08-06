from lib.Exceptions.UserException import *
from lib.config import config
#from lib.Connections.Object import CConnectionObject
from lib.Math2D import CPoint, CLine, CLineVector, CPolyLine, CRectangle
from math import sqrt, atan2, pi
from CacheableObject import CCacheableObject
from SelectableObject import CSelectableObject
from ConLabelInfo import CConLabelInfo
from lib.consts import LABELS_CLICKABLE

class CConnection(CCacheableObject, CSelectableObject):
    '''Graphical representation of connection
    
    In the program you have to distinguish between logical connection and its
    graphical representation.
    Logical connection between source and destination is only one in a project,
    but it can have several graphical representations, in each diagram one.
    
    @note: in text below, if not written otherwise, "connection" means its
    graphical representation
    
    @ivar diagram: owner of connection
    @type diagram: L{CDiagram<Diagram.CDiagram>}
    
    @ivar object: reference to logical connection
    @type object: L{CConnectionObject<CConnectionObject>}
    
    @ivar points: list of (x, y) positions of points forming poly line
    @type points: list
    
    @ivar selpoints: index of selected point (to be moved), None if any is not 
    selected
    @type selpoints: int / NoneType
    
    @ivar labels: dictionary of pairs {id: L{label<CConLabelInfo>}}
    @type labels: dict
    
    @ivar source: Element at the beginning of the connection
    @type source: L{CElement<CElement>}
    
    @ivar destination: Element at the end of the connection
    @type destination: L{CElement<CElement>}
    '''
    
    def __init__(self, diagram, obj, source, destination, points = None):
        '''Create new instance of connection
        
        @param diagram: owner of the connection
        @type  diagram: L{CDiagram<Diagram.CDiagram>}
        
        @param obj: logical connection between source and destination
        @type  obj:
        
        @param source: Element at the beginning of the connection
        @type  source: L{CElement<CElement>}
        
        @param destination: Element at the end of the connection
        @type  destination: L{CElement<CElement>}
        
        @param points: list of points [(x1, y1), (x2, y2), ... ] forming the
        connection line
        @type  points: list
        '''
        
        self.diagram = diagram
        self.diagram.AddConnection(self)
        self.object = obj
        if points is None:
            self.points = []
            if source is destination:
                self.points = []
        else:
            self.points = points
        self.source = source
        self.destination = destination
        self.labels = {}
        self.selpoint = None
        self.object.AddAppears(diagram)
        super(CConnection, self).__init__()
   
    def Deselect(self):
        '''Execute L{CSelectableObject.Deselect<CSelectableObject.Deselect>} 
        and L{self.DeselectPoint<self.DeselectPoint>}
        '''
        super(CConnection, self).Deselect()
        self.DeselectPoint()
        
    def SelectPoint(self, index):
        '''set self.selpoint to index if index within range
        
        @param index: index of point to be selected
        @type  index: int
        '''
        if 0 < index <= len(self.points):
            self.selpoint = index
        else:
            raise ConnectionError("PointNotExists")
            
    def DeselectPoint(self):
        '''set self.selpoint to None'''
        self.selpoint = None
        
    def GetSelectedPoint(self):
        '''
        Get index of selected point. None if no one is selected.
        
        @return: self.selpoint
        @rtype: int / NoneType
        '''
        return self.selpoint
    
    def GetPointAtPosition(self, pos):
        '''
        Get index of point from connection, if there is one close enough to 
        point defined by pos. None if there is no close point.
        
        @param pos: (x, y) position
        @type  pos: tuple
        
        @return: index of point or None
        @rtype: int / NoneType
        '''
        x, y = pos
        size = config['/Styles/Selection/PointsSize']
        for i, point in enumerate(self.points):
            if max(abs(point[0] - x), abs(point[1]-y)) <= size //2:
                return i + 1
        else:
            return None
            
    def GetSquare(self, canvas):
        '''get absolute positoin of minimal rectangle to which line fits,
        excluding labels.
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}
        
        @return: ((left, top), (right, bottom))
        @rtype: tuple
        '''
        left, top, right, bottom = 1000000, 1000000, -1000000, -1000000
        for x, y in self.GetPoints(canvas):
            left, top, right, bottom = min(left, x), min(top, y), max(right, x), max(bottom, x)
        return ((left, top), (right, bottom))
        
    def GetSource(self):
        '''
        Get element at the beginning of connection
        
        @return: self.source
        @rtype: L{CElement<CElement>}
        '''
        return self.source
        
    def GetDestination(self):
        '''
        Get element at the end of connection
        
        @return: self.destination
        @rtype: L{CElement<CElement>}
        '''
        return self.destination
        
    def GetSourceObject(self):
        """
        Get source object of logical connection
        
        @return: connection source
        @rtype:  L{CElementObject<CElementObject>}
        """
        return self.object.GetSource()
        
    def GetDestinationObject(self):
        """
        Get destination object of logical connection
        
        @return: connection destination
        @rtype:  L{CElementObject<CElementObject>}
        """
        return self.object.GetDestination()
        
    def GetNeighbours(self, index, canvas):
        '''get positions of neighbouring points to point 
        selected by index.
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}

        @return: ((x1,y1),(x2,y2)) 
        '''
        if not (0 < index  <= len(self.points)):
            raise ConnectionError("PointNotExists")
        if index == 1:
            previous = self.source.GetCenter(canvas)
        else:
            previous = self.points[index - 2]
        if index == len(self.points):
            next = self.destination.GetCenter(canvas)
        else:
            next = self.points[index]
        return previous, next
        
    def GetObject(self):
        '''Get object of logical connection
        
        @return: logical connection
        @rtype: L{CConnectionObject<CConnectionObject>}
        '''
        return self.object
    
    def GetLabelPosition(self, canvas, id, logicalLabelInfo):
        '''
        Get absolute (x,y) position of label defined by id
        
        If connection doesn't have id in cache, it saves it and writes
        it's position. Position is calculated from position and size so that
        center of label is at the default position.
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}
        
        @param id: identifier of label
        @type  id: whatever hasheable
        
        @param logicalLabelInfo: (defaultPosition, logicalLabel)
        @type logicalLabelInfo: tuple
        
        @param defaultPosition: default position of label - valid value
        @type  position: str
        
        @param logicalLabel: reference to logical representation of label.
        @type logicalLabel: L{CVisualObject
        <lib.Drawing.Objects.VisualObject.CVisualObject>}
        '''
        if id in self.labels:
            self.labels[id].SetLogicalLabel(logicalLabelInfo[1])
            
        else:
            self.labels[id] = CConLabelInfo(self, canvas, 
                logicalLabel = logicalLabelInfo[1])
            self.labels[id].SetToDefaultPosition(canvas, logicalLabelInfo[0])
        
        return self.labels[id].GetPosition(canvas)
    
    def GetAllLabelPositions(self):
        '''Yield information about positions of all labels, generator
        
        Used to gather information to be saved to .frip file
        
        @return: yielding information stored in dictionary - responsibility for
        contents is on L{CConLabelInfo.GetSaveInfo<CConLabelInfo.GetSaveInfo>}
        @rtype: dict
        '''
        for label in  self.labels.values():
            yield label.GetSaveInfo()
    
    def RestoreLabelPosition(self, id, info):
        '''Reset position of label, add new respecitvely using info
        
        @param id: identification of label - how to recognize it
        @type  id: int
        
        @param info: dictionary with parameters to restore label position
        @type  info: dict
        '''
        self.labels[id] = CConLabelInfo(self, None, **info)
        
    def InsertPoint(self, canvas, point, index = None):
        '''
        Add new point forming polyline of connection
        
        Label can be moved, if new point appears at the same segment of 
        polyline to which is this label bound. At first, relative position
        is adjusted to new situation - label.pos, then absolute position is 
        recalculated and again relative position is recalculated to make sure,
        that label is bound to closest segment of polyline.
        
        Creation of new point is ignored if new point is too close to 
        neighbouring point or angle the two new segments form is too close to
        pi. 
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}
        
        @param point: (x, y) position of point to be appended
        @type  point: tuple
        
        @param index: position at polyline to which to put new point. 
        @type  index: int
        
        @raise IndexError: if 0 > index or len(self.points) < index
        '''
        if index < 0 or index > len(self.points):
            raise IndexError('index out of range') 
        
        prevPoint = self.GetPoint(canvas, index)
        nextPoint = self.GetPoint(canvas, index + 1)
        
        if not self.ValidPoint([prevPoint, point, nextPoint]):
            return
        
        line1 = CLine(prevPoint, point)
        line2 = CLine(point, nextPoint)
        len1 = abs(line1)
        len2 = abs(line2)
        changed = []
            
        for label in self.labels.values():
            if label.idx == index:
                if len1 >= (len1 + len2) * label.pos:
                    label.pos = (len1 + len2) * label.pos / len1
                else:
                    label.pos = ((len1 + len2) * label.pos - len1) / len2
                    label.idx += 1
                changed.append(label)
            elif label.idx > index:
                label.idx += 1
        
        self.points.insert(index, point)
        self.ValidatePoints(canvas)
        
        for label in changed:
            label.RecalculateAbsolutePosition(canvas) # adjust (x, y) to new position
            label.RecalculateRelativePosition(canvas) # make sure, that label is bound to closest segment
    
    def AddPoint(self, point):
        '''
        Append next point forming polyline as last
        
        @attention: use only during loading project from file, as no 
        calculations are performed 
        
        @param point: point to be appended (x, y)
        @type  point: tuple
        '''
        self.points.append(point)
        
    def WhatPartOfYouIsAtPosition(self, canvas, point):
        '''
        What is on the position defined by point
        
            - L{CConLabelInfo<CConLabelInfo>} instance
            - index of line, forming connection
            - None, if not hit
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}

        @rtype: L{CConLabelInfo<CConLabelInfo>} / int / NoneType
        '''
        if LABELS_CLICKABLE:
            for label in self.labels.values():
                if label.AreYouAtPosition(canvas, point):
                    return label
        points = list(self.GetPoints(canvas))
        point = CPoint(point)
        point1 = points[0]
        for index, point2 in enumerate(points[1:]):
            line = CLine(CPoint(point1), CPoint(point2))
            if line - point < 2:
                return index
            point1 = point2
        else:
            return None
    
    def AreYouAtPosition(self, canvas, point):
        '''
        Get state whether point hits a part of connection, labels including
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}

        @return: True if L{WhatPartOfYouIsAtPosition
        <self.WhatPartOfYouIsAtPosition>} returns something
        @rtype: bool
        '''
        return self.WhatPartOfYouIsAtPosition(canvas, point) is not None

    def MoveAll(self, delta):
        '''Move all points and labels of connection
        
        @param delta: (dx, dy) distance to move
        @type  delta: tuple
        '''
        self.points = map(
            lambda x: (x[0] + delta[0], x[1] + delta[1]), 
            self.points)
        for label in self.labels.values():
            label.MoveWithOthers(delta)
        
    def MovePoint(self, canvas, pos, index):
        '''
        Change position of point defined by index to to new position pos
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}
        
        @param pos: (x, y) new position of point
        @type  pos: tuple
        
        @param index: index of point in self.points
        @type  index: int
        
        @raise IndexError: if index <= 0 or index > len(self.points)
        '''
        
        if index <= 0 or index > len(self.points):
            raise IndexError('Out of range')
        
        prevPoint = self.GetPoint(canvas, index - 1)
        nextPoint = self.GetPoint(canvas, index + 1)

        if self.ValidPoint([prevPoint, pos, nextPoint]):
            for label in self.labels.values():
                if label.idx in (index - 1, index):
                    label.RecalculateAbsolutePosition(canvas)
                    label.RecalculateRelativePosition(canvas)
            self.ValidatePoints(canvas)
        else:
            self.RemovePoint(canvas, index)

    def Paint(self, canvas, delta = (0, 0)):
        '''
        Paint connection including labels at canvas
        
        In fact L{CConnectionObject.Paint<CConnectionObject.Paint>} is used to 
        paint polyline itself. This method is afterwards responsible for
        drawing rectangles if the connection is selected.
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}
        
        @param delta: (x, y) offset by which is drawing area moved by 
        scrollbars on the screen
        @type  delta: tuple
        '''
        size = config['/Styles/Selection/PointsSize']
        self.object.Paint(canvas, self, delta)
        if self.selected is True:
            dx, dy = delta
            for index, i in enumerate(self.GetPoints(canvas)):
                canvas.DrawRectangle((i[0] + dx - size//2, i[1] + dy - size//2), (size, size), config['/Styles/Selection/PointsColor'])

    def RemovePoint(self, canvas, index, runValidation = True):
        '''
        Delete point from polyline and colapse two neighbouring segments of 
        polyline
        
        @param index: index of point to be deleted
        @type  index: int
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}
        
        @param runValidation: if True then at the end executes 
        L{self.ValidatePoints<self.ValidatePoints>}
        @type  runValidation: bool

        @raise IndexError: if index <= 0 or index > len(self.points)
        '''
        if index <= 0 or index > len(self.points):
            raise IndexError('Out of range')
        
        prevpoint = self.GetPoint(canvas, index - 1)
        point = self.GetPoint(canvas, index)
        nextpoint = self.GetPoint(canvas, index + 1)
        len1 = abs(CLine(prevpoint, point))
        len2 = abs(CLine(point, nextpoint))
        changed = []
        
        for label in self.labels.values():
            if label.idx == index - 1:
                label.pos = (len1 * label.pos) / (len1 + len2)
                changed.append(label)
            elif label.idx == index:
                label.pos = (len1 + len2 * label.pos) / (len1 + len2)
                label.idx -= 1
                changed.append(label)
            elif label.idx > index:
                label.idx -= 1
        del self.points[index - 1]
        
        if index  == self.selpoint:
            self.selpoint = None
        elif self.selpoint > index:
            self.selpoint -= 1

        for label in changed:
            label.RecalculateAbsolutePosition(canvas)
            label.RecalculateRelativePosition(canvas)
        
        if runValidation:
            self.ValidatePoints(canvas)
    
    def GetPoints(self, canvas):
        '''
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}
        '''
        yield self.GetPoint(canvas, 0)
            
        for point in self.points:
            yield point
            
        yield self.GetPoint(canvas, len(self.points) + 1)
    
    def GetPoint(self, canvas, index):
        '''
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}
        '''
        if self.source is self.destination and len(self.points) == 0:
            topleft, bottomright = self.source.GetSquare(canvas)
            y = bottomright[1] + 30
            xc = (topleft[0] + bottomright[0])/2
            self.points = [(xc - 10, y),( xc + 10, y)]
        if index == 0:
            center = self.source.GetCenter(canvas)
            if len(self.points) == 0:
                point = self.destination.GetCenter(canvas)
            else:
                point = self.points[0]
            result = self.__ComputeIntersect(canvas, self.source, center, point)
            return int(result[0]), int(result[1])
        elif index - 1 < len(self.points):
            return int(self.points[index - 1][0]), int(self.points[index - 1][1])
        elif index - 1 == len(self.points) :
            center = self.destination.GetCenter(canvas)
            if len(self.points) == 0:
                point = self.source.GetCenter(canvas)
            else:
                point = self.points[-1]
            result = self.__ComputeIntersect(canvas, self.destination, center, point)
            return int(result[0]), int(result[1])
        else:
            raise ConnectionError("PointNotExists")
        
    def GetMiddlePoints(self):
        for point in self.points:
            yield point

    def GetDiagram(self):
        return self.diagram
        
    def __ComputeIntersect(self, canvas, element, center, point):
        '''
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}
        '''
        topLeft, bottomRight = element.GetSquare(canvas)
        square = CRectangle(CPoint(topLeft), CPoint(bottomRight))
        line = CLine(CPoint(center), CPoint(point))
        intersects = square * line
        if len(intersects) > 0:
            return intersects[0].GetPos()
        else:
            dx1, dx2 = point[0] - topLeft[0], bottomRight[0] - point[0]
            dy1, dy2 = point[1] - topLeft[1], bottomRight[1] - point[1]
            if dx1 < min(dx2, dy1, dy2):
                return topLeft[0], point[1]
            elif dx2 < min(dy1, dy2):
                return bottomRight[0], point[1]
            elif  dy1 <  dy2:
                return point[0], topLeft[1]
            else:
                return point[0], bottomRight[1]
    
    def ValidatePoints(self, canvas):
        '''
        Remove unnecessary points from polyline forming connection and colapse
        segments
        
        If two points are too close to each other, first of them is discarted.
        If two segments contain angle too close to pi => create almost straight
        line, middle point is discarted
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}
        '''
        
        i = 1
        points = list(self.GetPoints(canvas))
        while i + 1 < len(points):
            if self.ValidPoint(points[i - 1 : i + 2]):
                i += 1
            else:
                self.RemovePoint(canvas, i, False)
                del points[i]

    def ValidPoint(self, points):
        '''
        Check whether is middle point of the three at a valid position.
        
        Conditions for the middle point to be valid:
        
            - Middle point mustn't be too close to any of side points, closer
            than /Styles/Selection/PointsSize  in config.xml
            - Lines from 1st to 2nd and from 2nd to 3rd point must form angle 
            sharper than (pi - /Styles/Connection/MinimalAngle)
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}

        
        @param points: list of three points [(x1, y1), (x2, y2), (x3, y3)]. 
        The middle point defined by (x2, y2) is to be examined
        @type  points: list
            
        @return: True if both conditions stand
        @rtype:  bool
        '''
        
        pointSize = config['/Styles/Selection/PointsSize']
        minAngle = config['/Styles/Connection/MinimalAngle']
        
        line1 = CLine(points[0], points[1])
        line2 = CLine(points[1], points[2])
        
        return ( abs(line1) > pointSize and abs(line2) > pointSize and
            minAngle < (line1.Angle() - line2.Angle()) % (2 * pi) < \
            2 * pi - minAngle )

if __name__ == '__main__':
    pass
