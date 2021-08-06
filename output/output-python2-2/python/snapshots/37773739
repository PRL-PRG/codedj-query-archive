from lib.lib import UMLException
from lib.consts import SELECT_SQUARES_SIZE, SELECT_POINT_SQUARES_COLOR, SELECT_SQUARES_COLOR
from lib.Connections.Object import CConnectionObject
from lib.Math2D import Point, Line, Square
from math import sqrt, atan2, pi

class CConnection:
    def __init__(self, drawingArea, obj, source, destination, points):
        self.drawingArea = drawingArea
        self.drawingArea.AddConnection(self)
        self.object = obj
        self.points = points
        self.source = source
        self.destination = destination
        self.labels = {}
        self.selected = False
        self.selpoint = None
    
    def Select(self):
        self.selected = True
    
    def Deselect(self):
        self.selected = False
        self.selpoint = None
        
    def GetSelelected(self):
        return self.selected
        
    def SelectPoint(self, index):
        if 0 < index <= len(self.points):
            self.selpoint = index
        else:
            raise UMLException("PointNotExists")
            
    def DeselectPoint(self):
        self.selpoint = None
        
    def GetSelectedPoint(self):
        return self.selpoint
    
    def GetPointAtPosition(self, pos):
        x, y = pos
        for i, point in enumerate(self.points):
            if max(abs(point[0] - x), abs(point[1]-y)) <= SELECT_SQUARES_SIZE//2:
                return i + 1
        else:
            return None
        
    def GetSource(self):
        return self.source
        
    def GetDestination(self):
        return self.destination
        
    def GetSourceObject(self):
        return self.object.GetSource()
        
    def GetNeighbours(self, index, canvas):
        if not (0 < index  <= len(self.points)):
            raise UMLException("PointNotExists")
        if index == 1:
            previous = self.source.GetCenter(canvas)
        else:
            previous = self.points[index - 2]
        if index == len(self.points):
            next = self.destination.GetCenter(canvas)
        else:
            next = self.points[index]
        return previous, next
        
        
    def GetDestinationObject(self):
        return self.object.GetDestination()
        
    def GetObject(self):
        return self.object
    
    def GetLabelPosition(self, canvas, position, id):
        if id in self.labels:
            return self.labels[id]
        else:
            points = [p for p in self.GetPoints(canvas)]
            if position == 'source':
                tmp = self.labels[id] = (points[0][0] , points[0][1])
            elif position == 'destination':
                tmp = self.labels[id] = (points[-1][0] , points[-1][1])
            elif position == 'center':
                L = 0
                tmp = Lo = points[0]
                for i in points[1:]:
                    L += sqrt((Lo[0] - i[0])**2 + (Lo[1] - i[1])**2)
                    Lo = i
                Lo = points[0]
                L1 = L/2
                L = 0
                for i in points[1:]:
                    LX = sqrt((Lo[0] - i[0])**2 + (Lo[1] - i[1])**2)
                    L += LX
                    if L > L1:
                        L -= L1
                        t = L / LX
                        tmp = self.labels[id] = (int(Lo[0] * t  + i[0] * (1 - t)),
                                                 int(Lo[1] * t  + i[1] * (1 - t)))
                        break
                    Lo = i
            else:
                raise UMLException("UndefinedPosition")
            return tmp
    
    def GetLabelDefinedPositions(self):
        for id, lbl in self.GetObject().GetType().GetLabels():
            yield self.labels.get(id, None)
    
    def SetLabelPosition(self, label, pos):
        self.labels[label] = pos
        
        
    def AddPoint(self, canvas, point, index = None):
        if index is None:
            self.points.append(point)
        elif 0 <= index <= len(self.points):
            self.points.insert(index, point)
        else:
            raise UMLException("PointNotExists")
        self.ValidatePoints(canvas)

    def WhatPartOfYouIsAtPosition(self, canvas, point):
        points = [p for p in self.GetPoints(canvas)]
        x, y = point
        Xo, Yo = points[0]
        for index, i in enumerate(points[1:]):
            A = Yo - i[1]
            B = i[0] - Xo
            if A + B == 0:
                if (A-x)**2 + (B-x)**2 <= 2:
                    return index
                else:
                    Xo, Yo = i
                    continue
            C = Xo*i[1] - i[0] * Yo
            T = (-B*Xo + A*Yo - A*y + B*x)/(A**2 + B**2)
            if T < 0:
                if sqrt((Xo - x)**2 + (Yo - y)**2) <= 2:
                    return index
            elif T > 1:
                if sqrt((i[0] - x)**2 + (i[1] - y)**2) <= 2:
                    return index
            else:
                if abs((abs(A*x + B*y + C))/sqrt(A**2 + B**2)) <= 2:
                    return index
            Xo, Yo = i
        else:
            return None
    
    def AreYouAtPosition(self, canvas, point):
        return self.WhatPartOfYouIsAtPosition(canvas, point) is not None

    def MoveAll(self, delta):
        deltax, deltay = delta
        points = []
        for x, y in self.points:
            points.append((x+deltax, y+deltay))
        self.points = points
        
    def MovePoint(self, canvas, point, index):
        if 0 < index <= len(self.points):
            self.points[index - 1] = point
        else:
            raise UMLException("PointNotExists")
        self.ValidatePoints(canvas)

    def Paint(self, canvas):
        self.object.Paint(canvas, self)
        if self.selected is True:
            for index, i in enumerate(self.GetPoints(canvas)):
                canvas.DrawRectangle((i[0] - SELECT_SQUARES_SIZE//2, i[1] - SELECT_SQUARES_SIZE//2), (SELECT_SQUARES_SIZE, SELECT_SQUARES_SIZE), SELECT_SQUARES_COLOR)
                if index  == self.selpoint:
                    canvas.DrawRectangle((i[0] - SELECT_SQUARES_SIZE//2, i[1] - SELECT_SQUARES_SIZE//2), (SELECT_SQUARES_SIZE, SELECT_SQUARES_SIZE), SELECT_POINT_SQUARES_COLOR)

    def RemovePoint(self, canvas, index):
        if 0 < index <= len(self.points):
            self.points.pop(index - 1)
            if index  == self.selpoint:
                self.selpoint = None
        else:
            raise UMLException("PointNotExists")
        self.ValidatePoints(canvas)
    
    def GetPoints(self, canvas):
        if self.source is not None:
            center = self.source.GetCenter(canvas)
            if len(self.points) == 0 and self.destination is None:
                yield center
            else:
                if len(self.points) == 0:
                    point = self.destination.GetCenter(canvas)
                else:
                    point = self.points[0]
                yield self.__ComputeIntersect(canvas, self.source, center, point)
                    
        for point in self.points:
            yield point
            
        if self.destination is not None:
            center = self.destination.GetCenter(canvas)
            if len(self.points) == 0 and self.source is None:
                yield center
            else:
                if len(self.points) == 0:
                    point = self.source.GetCenter(canvas)
                else:
                    point = self.points[-1]
                yield self.__ComputeIntersect(canvas, self.destination, center, point)
    
    def GetMiddlePoints(self):
        for point in self.points:
            yield point

    def GetDrawingArea(self):
        return self.drawingArea
        
    def __ComputeIntersect(self, canvas, element, center, point):
        topLeft, bottomRight = element.GetSquare(canvas)
        square = Square(Point(topLeft), Point(bottomRight))
        line = Line(Point(center), Point(point))
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
        points = list(self.GetPoints(canvas))
        changed = True
        while changed:
            changed = False
            for i in xrange(1, len(points) - 1):
                (x1, y1), (x2, y2), (x3, y3) = points[i-1], points[i], points[i+1]
                angle = atan2(y1 - y2, x1 - x2) - atan2(y2 - y3, x2 - x3)
                if -0.1 < angle < 0.1:
                    del points[i]
                    changed = True
                    break
        result = []
        if self.source is None:
            result.append(points[0])
        result.extend(points[1:-1])
        if self.destination is None:
            result.append(points[-1])
        if len(self.points) != len(result):
            self.points = result
            self.DeselectPoint()
            
        
