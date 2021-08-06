from lib.lib import UMLException
from lib.consts import SELECT_SQUARES_SIZE, SELECT_SQUARES_COLOR
from lib.Connections.Object import CConnectionObject
from math import sqrt

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
    
    def Select(self):
        self.selected = True
    
    def Deselect(self):
        self.selected = False
        
    def GetSelelected(self):
        return self.selected
    
    def GetPointAtPosition(self, x, y):
        for i, point in enumerate(self.points):
            if max(abs(point[0] - x), abs(point[1]-y)) <= SELECT_SQUARES_SIZE//2:
                return i
        else:
            return None
        
    def GetSource(self):
        return self.source
        
    def GetDestination(self):
        return self.destination
        
    def GetSourceObject(self):
        return self.object.GetSource()
        
    def GetNeighbours(self, index, canvas):
        if index < 0 or index >= len(self.points):
            raise UMLException("PointNotExists")
        if index == 0:
            previous = self.source.GetCenter(canvas)
        else:
            previous = self.points[index-1]
        if index == len(self.points) - 1:
            next = self.destination.GetCenter(canvas)
        else:
            next = self.points[index+1]
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
    
    def SetLabelPosition(self, label, x, y):
        self.labels[label] = (x, y)
        
        
    def AddPoint(self, x, y, index = None):
        if index is None:
            self.points.append((x, y))
        elif index < len(self.points) - 1:
            self.points.insert(index,(x,y))
        else:
            raise UMLException("PointNotExists")

    def AreYouAtPosition(self, canvas, x, y):
        points = [p for p in self.GetPoints(canvas)]
        Xo, Yo = points[0]
        for i in points[1:]:
            A = Yo - i[1]
            B = i[0] - Xo
            if A + B == 0:
                if (A-x)**2 + (B-x)**2 <= 2:
                    return True
                else:
                    Xo, Yo = i
                    continue
            C = Xo*i[1] - i[0] * Yo
            T = (-B*Xo + A*Yo - A*y + B*x)/(A**2 + B**2)
            if T < 0:
                if sqrt((Xo - x)**2 + (Yo - y)**2) <= 2:
                    return True
            elif T > 1:
                if sqrt((i[0] - x)**2 + (i[1] - y)**2) <= 2:
                    return True
            else:
                if abs((abs(A*x + B*y + C))/sqrt(A**2 + B**2)) <= 2:
                    return True
            Xo, Yo = i
        else:
            return False

    def MoveAll(self, deltax, deltay):
        points = []
        for x, y in self.points:
            points.append((x+deltax, y+deltay))
        self.points = points
        
    def MovePoint(self, index, x, y):
        if index < len(self.points):
            self.points[index] = (x,y)
        else:
            raise UMLException("PointNotExists")

    def Paint(self, canvas):
        self.object.Paint(canvas, self)
        if self.selected is True:
            for i in self.GetPoints(canvas):
                canvas.DrawRectangle((i[0] - SELECT_SQUARES_SIZE//2, i[1] - SELECT_SQUARES_SIZE//2), (SELECT_SQUARES_SIZE, SELECT_SQUARES_SIZE), SELECT_SQUARES_COLOR)

    def RemovePoint(self, index):
        if index < len(self.points) - 1:
            self.points.pop(index)
        else:
            raise UMLException("PointNotExists")
    
    def GetPoints(self, canvas):
        if self.source is not None:
            yield self.source.GetCenter(canvas)
        for point in self.points:
            yield point
        if self.destination is not None:
            yield self.destination.GetCenter(canvas)
    
    def GetMiddlePoints(self):
        for point in self.points:
            yield point

    def GetDrawingArea(self):
        return self.drawingArea
    
