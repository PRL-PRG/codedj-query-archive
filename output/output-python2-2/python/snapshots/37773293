from lib.lib import UMLException
from lib.config import config
from lib.Connections.Object import CConnectionObject
from lib.Math2D import CPoint, CLine, CLineVector, CPolyLine, CRectangle
from math import sqrt, atan2, pi

class CConnection:
    def __init__(self, drawingArea, obj, source, destination, points = None):
        self.drawingArea = drawingArea
        self.drawingArea.AddConnection(self)
        self.object = obj
        if points is None:
            self.points = []
        else:
            self.points = points
        self.source = source
        self.destination = destination
        self.labels = {}
        self.selected = False
        self.selpoint = None
        self.object.AddAppears(drawingArea)
    
   
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
        size = config['/Styles/Selection/PointsSize']
        for i, point in enumerate(self.points):
            if max(abs(point[0] - x), abs(point[1]-y)) <= size //2:
                return i + 1
        else:
            return None
            
    def GetSquare(self, canvas):
        left, top, right, bottom = 1000000, 1000000, -1000000, -1000000
        for x, y in self.GetPoints(canvas):
            left, top, right, bottom = min(left, x), min(top, y), max(right, x), max(bottom, x)
        return ((left, top), (right, bottom))
        
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
        
    def __CalculateLabelPos(self, canvas, index, t, dist, angle):
        line = CLine(self.GetPoint(canvas, index), self.GetPoint(canvas, index + 1))
        lineang = line.Angle()
        scaled = line.Scale(t)
        newline = CLineVector(scaled.GetEnd(), angle + lineang,  dist)
        return newline.GetEnd().GetPos()
        
    def GetDestinationObject(self):
        return self.object.GetDestination()
        
    def GetObject(self):
        return self.object
    
    def GetLabelPosition(self, canvas, id, position, size):
        width, height = size
        if id in self.labels:
            pnt, idx, t, dist, angle = self.labels[id]
            if pnt is None:
                self.labels[id][0] = pnt = self.__CalculateLabelPos(canvas, idx, t, dist, angle)
            x, y = pnt
            return x - width/2, y - height/2
        else:
            points = list(self.GetPoints(canvas))
            if position == 'source':
                tmp = self.labels[id] = [points[0], 0, 0.0, 0, 0]
            elif position == 'destination':
                tmp = self.labels[id] = [points[-1], len(points) - 1, 1.0, 0, 0]
            elif position == 'center':
                L = 0
                Lo = points[0]
                for point in points[1:]:
                    L += sqrt((Lo[0] - point[0])**2 + (Lo[1] - point[1])**2)
                    Lo = point
                if L == 0:
                    tmp = self.labels[id] = [points[0], 0, 0.6, height/2, pi/2]
                else:
                    Lo = points[0]
                    L1 = L/2
                    L = 0
                    for index, point in enumerate(points[1:]):
                        LX = sqrt((Lo[0] - point[0])**2 + (Lo[1] - point[1])**2)
                        L += LX
                        if L > L1:
                            L -= L1
                            if LX == 0:
                                t = 0.5
                            else:
                                t = L / LX
                            tmp = self.labels[id] = [self.__CalculateLabelPos(canvas, index, t, height/2, pi/2), 
                                index, t, height/2, pi/2]
                            break
                        Lo = point
            else:
                raise UMLException("UndefinedPosition")
            return tmp[0]
    
    def GetLabelDefinedPositions(self):
        for id, lbl in self.GetObject().GetType().GetLabels():
            yield self.labels.get(id, (None, None, None, None, None))[1:]
    
    def SetLabelPosition(self, label, index, t, dist, angle):
        self.labels[label] = [None, index, t, dist, angle]
        
    def SetLabelPositionXY(self, label, pos, canvas):
        polyline = CPolyLine(tuple(self.GetPoints()))
        point = CPoint(pos)
        index, point, dist, angle = polyline.Nearest(point)
        line = polyline.GetLine(index)
        (Sx, Sy), (Ex, Ey) = line.GetPos()
        Px, Py = point.GetPos()
        if Sx != Ex:
            t = (Px - Sx) / (Ex - Sx)
        elif Sy != Ey:
            t = (Py - Sy) / (Ey - Sy)
        else:
            t = 0
        self.labels[label] = [pos, index, t, dist, angle]
        
    def InsertPoint(self, canvas, point, index = None):
        if index is None:
            self.points.append(point)
            return
        if 0 <= index  <= len(self.points):
            prevpoint = self.GetPoint(canvas, index)
            nextpoint = self.GetPoint(canvas, index + 1)
            len1 = abs(CLine(prevpoint, point))
            len2 = abs(CLine(point, nextpoint))
            for id in self.labels:
                pnt, idx, t, dist, angle = self.labels[id]
                if idx < index:
                    continue
                elif idx == index:
                    if len1 >= (len1 + len2)*t:
                        t = (len1 + len2)*t / len1
                    else:
                        t = ((len1 + len2)*t - len1)/len2
                        idx += 1
                else:
                    idx += 1
                self.labels[id] = [None, idx, t, dist, angle]
            self.points.insert(index, point)
        else:
            raise UMLException("PointNotExists")
        self.ValidatePoints(canvas)
    
    def AddPoint(self, point):
        self.points.append(point)
        
    def WhatPartOfYouIsAtPosition(self, canvas, point):
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
        return self.WhatPartOfYouIsAtPosition(canvas, point) is not None

    def MoveAll(self, delta):
        deltax, deltay = delta
        points = []
        for x, y in self.points:
            points.append((x+deltax, y+deltay))
        for id in self.labels:
            if self.labels[id][0] is not None:
                x, y = self.labels[id][0]
                self.labels[id][0] = (x+deltax, y+deltay)
        self.points = points
        
    def MovePoint(self, canvas, point, index):
        if 0 < index <= len(self.points):
            self.points[index - 1] = point
            for id in self.labels:
                pnt, idx, t, dist, angle = self.labels[id]
                if index == idx or index  == idx + 1:
                    self.labels[id][0] = None
        else:
            raise UMLException("PointNotExists")
        self.ValidatePoints(canvas)

    def Paint(self, canvas, delta = (0, 0)):
        size = config['/Styles/Selection/PointsSize']
        self.object.Paint(canvas, self, delta)
        if self.selected is True:
            dx, dy = delta
            for index, i in enumerate(self.GetPoints(canvas)):
                canvas.DrawRectangle((i[0] + dx - size//2, i[1] + dy - size//2), (size, size), config['/Styles/Selection/PointsColor'])

    def RemovePoint(self, canvas, index):
        if 0 < index <= len(self.points):
            prevpoint = self.GetPoint(canvas, index - 1)
            point = self.GetPoint(canvas, index )
            nextpoint = self.GetPoint(canvas, index + 1)
            len1 = abs(CLine(prevpoint, point))
            len2 = abs(CLine(point, nextpoint))
            for id in self.labels:
                pnt, idx, t, dist, angle = self.labels[id]
                if idx < index - 1:
                    continue
                elif idx == index - 1:
                    t = (len1*t) / (len1 + len2)
                elif idx == index:
                    t = (len1 + len2*t) / (len1 + len2)
                    idx -= 1
                else:
                    idx -= 1
                self.labels[id] = [None, idx, t, dist, angle]
            self.points.pop(index - 1)
            if index  == self.selpoint:
                self.selpoint = None
        else:
            raise UMLException("PointNotExists")
        self.ValidatePoints(canvas)
    
    def GetPoints(self, canvas):
        yield self.GetPoint(canvas, 0)
            
        for point in self.points:
            yield point
            
        yield self.GetPoint(canvas, len(self.points) + 1)
    
    def GetPoint(self, canvas, index):
        if index == 0:
            center = self.source.GetCenter(canvas)
            if len(self.points) == 0:
                point = self.destination.GetCenter(canvas)
            else:
                point = self.points[0]
            return self.__ComputeIntersect(canvas, self.source, center, point)
        elif index - 1 < len(self.points):
            return self.points[index - 1]
        elif index - 1 == len(self.points) :
            center = self.destination.GetCenter(canvas)
            if len(self.points) == 0:
                point = self.source.GetCenter(canvas)
            else:
                point = self.points[-1]
            return self.__ComputeIntersect(canvas, self.destination, center, point)
        else:
            raise UMLException("PointNotExists")
        
    def GetMiddlePoints(self):
        for point in self.points:
            yield point

    def GetDrawingArea(self):
        return self.drawingArea
        
    def __ComputeIntersect(self, canvas, element, center, point):
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
        points = list(self.GetPoints(canvas))
        lenold = len(points)
        changed = True
        for i in xrange(1, len(points) - 1):
            pnts = [CPoint(pnt) for pnt in points[i-1:i+2]]
            if pnts[0] - pnts[1] <= config['/Styles/Selection/PointsSize'] or \
                pnts[1] - pnts[2] <= config['/Styles/Selection/PointsSize']:
                self.RemovePoint(canvas, i)
                return
            line1 = CLine(pnts[0], pnts[1])
            line2 = CLine(pnts[1], pnts[2])
            if -0.1 < line1.Angle() - line2.Angle() < 0.1:
                self.RemovePoint(canvas, i)
                return
                
    def RecalculateLabels(self):
        for id in self.labels:
            self.labels[id][0] = None
