from lib.lib import UMLException
from lib.consts import SELECT_SQUARES_SIZE, SELECT_SQUARES_COLOR
from lib.Connections.Object import CConnectionObject
from math import sqrt

class CConnection:
    def __init__(self, screen, obj, points):
        self.screen = screen
        self.screen.AddConnection(self)
        self.conObject = obj
        self.points = points
        self.labels = {}
        self.selected = False        
    
    def Select(self):
        self.selected = True
    
    def Deselect(self):
        self.selected = False
        
    def Getselelected(self):
        return self.selected
    
    def IsSquareSize(self, x, y):
        for i in self.points:
            if i[0] <= x <= i[0] + SELECT_SQUARES_SIZE and i[1] <= y <= i[1] + SELECT_SQUARES_SIZE:
                return True
        else:
            return False
    
    def GetObject(self):
        return self.conObject
    
    def GetLabelPosition(self, position, id):
        if id in self.labels:
            return self.labels[id]
        else:
            if position == 'source':
                tmp = self.labels[id] = (self.points[0][0] , self.points[0][1])
            elif position == 'destination':
                tmp = self.labels[id] = (self.points[-1][0] , self.points[-1][1])
            elif position == 'center':
                L = 0
                Lo = self.points[0]
                for i in self.points[1:]:
                    L += sqrt((Lo[0] - i[0])**2 + (Lo[1] - i[1])**2)
                    Lo = i
                Lo = self.points[0]
                L1 = L/2
                L = 0
                for i in self.points[1:]:
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
    
    
    def SetLabelPosition(self, label, x, y):
        self.labels[label] = (x, y)
        
        
    def AddPoint(self, index, x, y):
        if index < len(self.points) - 1:
            self.insert(index,(x,y))
        else:
            raise UMLException("PointNotExists")

    def AreYouAtPosition(self, canvas, x, y):
        Xo, Yo = self.points[0]
        for i in self.points[1:]:
            A = Yo - i[1]
            B = i[0] - Xo
            C = Xo*i[1] - i[0] * Yo
            T = (-B*Xo + A*Yo - A*y + B*x)/(A**2 + B**2)
            if T < 0:
                if sqrt((Xo - x)**2 + (Yo - y)**2) <=2:
                    return True
            elif T > 1:
                if sqrt((i[0] - x)**2 + (i[1] - y)**2) <=2:
                    return True
            else:
                if abs((abs(A*x + B*y + C))/sqrt(A**2 + B**2)) <=2:
                    return True
            Xo, Yo = i
        else:
            return False

    def MoveAll(self, deltax, deltay):
        points = []
        for x, y in self.points:
            points.append((x+deltax, y+deltay))
        self.points = points
        
    def MoveEndPoint(self, element, deltax, deltay):
        if element.GetObject() is self.conObject.GetSource():
            x, y = self.points[0]
            self.points[0] = (x+deltax, y+deltay)
        elif element.GetObject() is self.conObject.GetDestination():
            x, y = self.points[-1]
            self.points[-1] = (x+deltax, y+deltay)
        else:
            raise UMLException("InvalidElement")
            
    def MovePoint(self, index, x, y):
        if index < len(self.points) - 1:
            self.points[index] = (x,y)
        else:
            raise UMLException("PointNotExists")

    def Paint(self, canvas):
        self.conObject.Paint(canvas, self)
        if self.selected is True:
            for i in self.points:
                canvas.DrawRectangle((i[0] - SELECT_SQUARES_SIZE//2, i[1] - SELECT_SQUARES_SIZE//2), (SELECT_SQUARES_SIZE, SELECT_SQUARES_SIZE), SELECT_SQUARES_COLOR)

    def RemovePoint(self, index):
        if index < len(self.points) - 1:
            self.points.pop(index)
        else:
            raise UMLException("PointNotExists")
    
    def GetPoints(self):
        return self.points

    def GetDrawingArea(self):
        return self.screen
