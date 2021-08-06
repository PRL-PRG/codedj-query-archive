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
    def __init__(self, src, dest):
        self.src = src
        self.dest = dest
        
    def GetStart(self):
        return self.src
        
    def GetEnd(self):
        return self.dest
        
    def __repr__(self):
        return '[' + repr(self.src) + ' ' + repr(self.dest) + ']'
        
    def __mul__(self, other):
        if isinstance(other, CPoint):
            if self - other == 0:
                return [other]
            return []
        elif isinstance(other, CLine):
            (ax1, ay1), (ax2, ay2) = self.GetStart().GetPos(), self.GetEnd().GetPos()
            (bx1, by1), (bx2, by2) = other.GetStart().GetPos(), other.GetEnd().GetPos()
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
            x1, y1 = self.src.GetPos()
            x2, y2 = self.dest.GetPos()
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
        return (self.src == other.GetStart() and self.dest == other.GetEnd()) or \
            (self.src == other.GetEnd() and self.dest ==other.GetStart())
    
class CPolyLine:
    def __init__(self, *points):
        if len(points) < 2:
            raise MathException
        self.lines = []
        start = points[0]
        for end in points[1:]:
            self.lines.append(CLine(start, end))
            start = end
        
    def GetLines(self):
        yield self.lines
        
    def __repr__(self):
        return repr(self.lines)
            
    def __mul__(self, other):
        result = set()
        for line in self.lines:
            result |= set(line * other)
        return list(result)
        
    __rmul__ = __mul__
    
class CPolygon(CPolyLine):
    def __init__(self, *points):
        CPolyLine.__init__(self, *points)
        self.lines.append(CLine(points[-1], points[0]))
        
class CRectangle(CPolygon):
    def __init__(self, topLeft, bottomRight):
        (x1, y1), (x2, y2) = topLeft.GetPos(), bottomRight.GetPos()
        if x1 > x2:
            x1, x2 = x2, x1
        if y1 > y2:
            y1, y2 = y2, y1
        CPolygon.__init__(self, CPoint((x1, y1)), CPoint((x2, y1)), CPoint((x2, y2)), CPoint((x1, y2)))
        
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
