from exceptions import MathException
import math

class CPoint:
    def __init__(self, point):
        self.point = point
        
    def GetPos(self):
        return self.point
        
    def GetIntPos(self):
        return int(self.point[0]), int(self.point[1])
        
    def __hash__(self):
        return hash(self.point)
        
    def __sub__(self, other):
        if isinstance(other, CPoint):
            x1, y1 = self.point
            x2, y2 = other.GetPos()
            return math.sqrt((x1 - x2)**2 + (y1 - y2)**2)
        else:
            return NotImplemented
            
    __rsub__ = __sub__
    
    def __eq__(self, other):
        return self.point == other.GetPos()
        
    def __ne__(self, other):
        return self.point != other.GetPos()
        
    def __lt__(self, other):
        x1, y1 = self.GetPos()
        x2, y2 = other.GetPos()
        return x1 < x2 and y1 < y2
        
    def __le__(self, other):
        x1, y1 = self.GetPos()
        x2, y2 = other.GetPos()
        return x1 <= x2 and y1 <= y2
        
    def __gt__(self, other):
        return other < self

    def __ge__(self, other):
        return other <= self
        
    def __repr__(self):
        return repr(self.point)

class CLine:
    def __init__(self, start, end):
        if not isinstance(start, CPoint):
            start = CPoint(start)
        if not isinstance(end, CPoint):
            end = CPoint(end)
        self.start = start
        self.end = end
        
    def GetStart(self):
        return self.start
        
    def GetEnd(self):
        return self.end
        
    def GetPos(self):
        return self.GetStart().GetPos(), self.GetEnd().GetPos()
        
    def Angle(self):
        (x1, y1), (x2, y2) = self.GetPos()
        return math.atan2(y2 - y1, x2 - x1)
        
    def Scale(self, factor):
        (Ax, Ay), (Bx, By) = self.GetPos()
        return CLine(self.GetStart(), (Ax + (Bx - Ax)*factor, Ay + (By - Ay)*factor))

    def Nearest(self, point):
        (Ax, Ay), (Bx, By) = self.GetPos()
        Cx, Cy = point.GetPos()
        D = float((Bx - Ax)**2 + (By - Ay)**2)
        if D == 0:
            return self.GetStart(), self.GetStart - point, math.atan2(Cy - Ay, Cx - Ax)
        t1 = (Cx - Ax)*(Bx - Ax) - (Cy - Ay)*(Ay - By)
        t2 = (Bx - Ax)*(Cy - Ay) - (Cx - Ax)*(By - Ay)
        if t1 <= 0:
            angle = math.atan2(Cy - Ay, Cx - Ax) - self.Angle()
            return self.GetStart(), self.GetStart() - point, angle
        elif 0 < t1 < D:
            pos = CPoint((Ax + (Bx - Ax)*t1/D, Ay + (By - Ay)*t1/D))
            if t2 >= 0:
                angle = math.atan2(Cy - By, Cx - Bx) - math.pi/2
            else:
                angle = math.atan2(Cy - By, Cx - Bx) + math.pi/2
            return pos, pos - point, angle
        else:
            angle = math.atan2(Cy - By, Cx - Bx) - self.Angle()
            return self.GetEnd(), self.GetEnd() - point, angle
    
    def __repr__(self):
        return '[' + repr(self.start) + ' ' + repr(self.end) + ']'
        
    def __abs__(self):
        return self.GetStart() - self.GetEnd()
        
    def __mul__(self, other):
        if isinstance(other, CPoint):
            if self - other == 0:
                return [other]
            return []
        elif isinstance(other, CLine):
            (ax1, ay1), (ax2, ay2) = self.GetPos()
            (bx1, by1), (bx2, by2) = other.GetPos()
            D = (((ay2 - ay1)*(bx2 - bx1)) + ((by1 - by2)*(ax2 - ax1)))
            if D == 0:
                result = set()
                if self - other.GetStart() == 0:
                    result.add(other.GetStart())
                if self - other.GetEnd() == 0:
                    result.add(other.GetEnd())
                if other - self.GetStart() == 0:
                    result.add(self.GetStart())
                if other - self.GetEnd() == 0:
                    result.add(self.GetEnd())
                return list(result)
            t1D = ((bx2*(by1 - ay1)) + (bx1*(ay1 - by2)) + (ax1*(by2 - by1)))
            t2D = ((ax2*(by1 - ay1)) + (ax1*(ay2 - by1)) + (bx1*(ay1 - ay2)))
            if (0 <= t1D <= D and 0 <= t2D <= D) or \
                (D <= t1D <= 0 and D <= t2D <= 0):
                t1 = float(t1D) / D
                return [CPoint((ax1 + (ax2 - ax1)*t1, ay1 + (ay2 - ay1)*t1))]
            return []
        else:
            return NotImplemented
    
    __rmul__ = __mul__
    
    def __sub__(self, other):
        if isinstance(other, CPoint):
            (x1, y1), (x2, y2) = self.GetPos()
            x, y = other.GetPos()
            A = y2 - y1
            B = x1 - x2
            C = x2*y1 - x1*y2
            if A == 0 and B == 0:
                return math.sqrt((x2 - x)**2 + (y2 - y)**2)
            T = (-B*x2 + A*y2 - A*y + B*x)/(A**2 + B**2)
            if T < 0:
                return math.sqrt((x2 - x)**2 + (y2 - y)**2)
            elif T > 1:
                return math.sqrt((x1 - x)**2 + (y1 - y)**2)
            else:
                return abs(A*x + B*y + C)/math.sqrt(A**2 + B**2)
        elif isinstance(other, CLine):
            if len(self * other):
                    return 0
            return min(self - other.GetStart(), self - other.GetEnd(), other - self.GetStart(), other - self.GetEnd())
        else:
            return NotImplemented
        
    __rsub__ = __sub__
    
    def __eq__(self, other):
        return (self.start == other.GetStart() and self.end == other.GetEnd()) or \
            (self.start == other.GetEnd() and self.end ==other.GetStart())
            
class CLineVector(CLine):
    def __init__(self, start, alpha, length):
        if not isinstance(start, CPoint):
            start = CPoint(start)
        x, y = start.GetPos()
        end = CPoint((x + math.cos(alpha)*length, y + math.sin(alpha)*length))
        CLine.__init__(self, start, end)
    
class CPolyLine:
    def __init__(self, points):
        if len(points) < 2:
            raise MathException
        self.lines = []
        start = points[0]
        for end in points[1:]:
            self.lines.append(CLine(start, end))
            start = end
            
    def GetLine(self, index):
        return self.lines[index]
        
    def GetLines(self):
        for line in self.lines:
            yield line
            
    def GetPos(self):
        result = [self.lines[0].GetStart().GetPos()]
        result.extend([line.GetEnd().GetPos() for line in self.lines])
        return result
        
    def Nearest(self, point):
        result = None
        for index, line in enumerate(self.lines):
            nearest, dist, angle = line.Nearest(point)
            if result is None or dist < result[2]:
                result = (index, nearest, dist, angle)
        return result
        
    def __repr__(self):
        return repr(self.lines)
        
    def __abs__(self):
        return sum([abs(line) for line in self.lines])
            
    def __mul__(self, other):
        result = set()
        for line in self.lines:
            result |= set(line * other)
        return list(result)
        
    __rmul__ = __mul__
    
class CPolygon(CPolyLine):
    def __init__(self, points):
        CPolyLine.__init__(self, points)
        self.lines.append(CLine(points[-1], points[0]))
        
    def GetPos(self):
        return [line.GetStart().GetPos() for line in self.lines]
        
class CRectangle(CPolygon):
    def __init__(self, topLeft, bottomRight):
        if isinstance(topLeft, CPoint):
            (x1, y1) = topLeft.GetPos()
        else:
            x1, y1 = topLeft
        if isinstance(bottomRight, CPoint):
            (x2, y2) = bottomRight.GetPos()
        else:
            x2, y2 = bottomRight
        if x1 > x2:
            x1, x2 = x2, x1
        if y1 > y2:
            y1, y2 = y2, y1
        CPolygon.__init__(self, (CPoint((x1, y1)), CPoint((x2, y1)), CPoint((x2, y2)), CPoint((x1, y2))))
        
    def GetTopLeft(self):
        return self.lines[0].GetStart()
        
    def GetBottomRight(self):
        return self.lines[2].GetStart()
        
    def __mul__(self, other):
        if isinstance(other, CRectangle):
            if self.GetTopLeft() <= other.GetTopLeft() and self.GetBottomRight() >= other.GetBottomRight():
                return [other.GetTopLeft(), other.GetBottomRight()]
            if self.GetTopLeft() >= other.GetTopLeft() and self.GetBottomRight() <= other.GetBottomRight():
                return [self.GetTopLeft(), self.GetBottomRight()]
        return CPolygon.__mul__(self, other)

