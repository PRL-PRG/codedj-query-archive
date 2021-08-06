from exceptions import MathException

class Point:
    def __init__(self, point):
        self.point = point
        
    def GetPos(self):
        return self.point
        
    def GetIntPos(self):
        return int(self.point[0]), int(self.point[1])

class Line:
    def __init__(self, src, dest):
        self.src = src
        self.dest = dest
        
    def GetStart(self):
        return self.src
        
    def GetEnd(self):
        return self.dest
        
        
    def __mul__(self, other):
        if isinstance(other, Line):
            (ax1, ay1), (ax2, ay2) = self.GetStart().GetPos(), self.GetEnd().GetPos()
            (bx1, by1), (bx2, by2) = other.GetStart().GetPos(), other.GetEnd().GetPos()
            D = (((ay2 - ay1)*(bx2 - bx1)) + ((by1 - by2)*(ax2 - ax1)))
            if D == 0:
                return []
            t1D = ((bx2*(by1 - ay1)) + (bx1*(ay1 - by2)) + (ax1*(by2 - by1)))
            t2D = ((ax2*(by1 - ay1)) + (ax1*(ay2 - by1)) + (bx1*(ay1 - ay2)))
            if 0 <= t1D <= D and 0 <= t2D <= D:
                t1 = float(t1D) / D
                return [Point((ax1 + (ax2 - ax1)*t1, ay1 + (ay2 - ay1)*t1))]
            return []
        else:
            return NotImplemented
    
    __rmul__ = __mul__

class PolyLine:
    def __init__(self, *points):
        if len(points) < 2:
            raise MathException
        self.lines = []
        start = points[0]
        for end in points[1:]:
            self.lines.append(Line(start, end))
            start = end
            
    def __mul__(self, other):
        result = []
        for line in self.lines:
            result.extend(line * other)
        return result
        
    def GetLines(self):
        yield self.lines
        
    __rmul__ = __mul__
    
class Polygon(PolyLine):
    def __init__(self, *points):
        PolyLine.__init__(self, *points)
        self.lines.append(Line(points[-1], points[0]))
        
        
class Square(Polygon):
    def __init__(self, topLeft, bottomRight):
        (x1, y1), (x2, y2) = topLeft.GetPos(), bottomRight.GetPos()
        Polygon.__init__(self, topLeft, Point((x2, y1)), bottomRight, Point((x1, y2)))
        
    def GetTopLeft(self):
        return self.lines[0].GetStart()
        
    def GetBottomRight(self):
        return self.lines[3].GetStart()
        