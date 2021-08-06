from exceptions import MathException
import math

class CPoint(object):
    '''Interprets one point on plane
    
    Allowed operations for points A, B:
    
        - C{A - B} distance from A to B
        - C{A == B} both at equal position
        - C{A <> B} at different positions
        - C{A < B} A is sharply to the left and up from B
        - C{A <= B} like with horizontaly or verticaly same position
    '''
    
    def __init__(self, point):
        '''
        create new instance of CPoint
        
        param point: (x, y) position
        type point: (float, float) tuple
        '''
        self.point = point
        
    def GetPos(self):
        '''
        get absolute position of point
        
        @return: (x, y) position
        @rtype: tuple
        '''
        return self.point
        
    def GetIntPos(self):
        '''
        get absolute position in integers, fractions are truncated
        
        @return: (x, y) position
        @rtype: tuple
        '''
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

class CLine(object):
    '''
    line segment
    
    @attention: Always keep in mind that it's an segment not an infinite line 
    
    Allowed operations for segments AB, CD and point E:
    
        - C{abs(AB)} lenght of segment
        - C{AB * CD} list of intersections of two segments. if they have some
            common segment, only endpoints of common segment are returned
        - C{AB * E} position of E if E belongs to AB 
        - C{AB - CD} minimal distance from AB to CD
        - C{AB - E} minimal distance from AB to E
        - C{AB == CD} the same lines, endpoints can be swaped
    '''
    def __init__(self, start, end):
        '''
        create new instance of CLine
        
        @param start: (x, y) position of first endpoint of segment
        @type start: tuple (float, float) or CPoint
        
        @param end: (x, y) position of second endpoint of segment
        @type end: tuple (float, float) or CPoint or None
        '''
        if not isinstance(start, CPoint):
            start = CPoint(start)
        if not isinstance(end, CPoint):
            end = CPoint(end)
        self.start = start
        self.end = end
    
    @classmethod
    def CreateAsVector(cls, start, alpha, length):
        '''
        Alternative constructor to create new instance of CLine
        
        @return: new instance of L{CLine<CLine>}
        @rtype: L{CLine<CLine>}
        
        @param start: (x, y) position of first endpoint of segment
        @type start: tuple (float, float) or CPoint
        
        @param alpha: angle of line
        @type alpha: float
        
        @param length: length of segment
        @type length: float
        '''
        if not isinstance(start, CPoint):
            start = CPoint(start)
        x, y = start.GetPos()
        end = CPoint((x + math.cos(alpha)*length, y + math.sin(alpha)*length))
        return cls(start, end)
        
    def GetStart(self):
        '''
        @return: first endpoint of segment
        @rtype: L{CPoint<CPoint>}
        '''
        return self.start
        
    def GetEnd(self):
        '''
        @return: second endpoint of segment
        @rtype: L{CPoint<CPoint>}
        '''
        return self.end
        
    def GetPos(self):
        '''
        @return: ((x1, y1), (x2, y2)) positions of both endpoints
        @rtype: tuple ((float, float), (float, float))
        '''
        return self.GetStart().GetPos(), self.GetEnd().GetPos()
        
    def Angle(self):
        '''
        @return: angle of vector (start, end)-> to ((0,0), (0,1))->  
        in range (-pi, pi]
        @rtype: float
        '''
        (x1, y1), (x2, y2) = self.GetPos()
        return math.atan2(y2 - y1, x2 - x1)
        
    def Scale(self, factor):
        '''
        @return: another line with start at the same position but with end moved
        so that new one is factor-times longer
        @rtype: L{CLine<CLine>}
        '''
        (Ax, Ay), (Bx, By) = self.GetPos()
        return CLine(self.GetStart(), (Ax + (Bx - Ax)*factor, Ay + (By - Ay)*factor))

    def Nearest(self, point):
        '''
        Find the closest point at the line segment to the specified position
        
        @return: (scale, distance, angle) - position of tha closest point at 
        segment, distance to it and angle of (point,(x,y)) to the line
        @rtype:  tuple
        
        @param point: specified position
        @type point:  L{CPoint<CPoint>}
        '''
        
        if isinstance(point, CPoint):
            Cx, Cy = point.GetPos()
        else:
            Cx, Cy = point
        (Ax, Ay), (Bx, By) = self.GetPos()
        
        ux = float(Bx - Ax)
        uy = float(By - Ay)
        
        if ux == 0:
            t = (Cy - Ay) / uy
        elif uy == 0:
            t = (Cx - Ax) / ux
        else:
            t = ((Cx - Ax) * ux + (Cy - Ay) * uy) / (ux**2 + uy**2)
        
        if t < 0:
            t = 0.0
            d = CPoint((Ax, Ay)) - CPoint((Cx, Cy))
        elif t > 1:
            t = 1.0
            d = CPoint((Bx, By)) - CPoint((Cx, Cy))
        else:
            d = self.Scale(t).GetEnd() - CPoint((Cx, Cy))
        
        a = CLine(self.Scale(t).GetEnd(), (Cx, Cy)).Angle() - self.Angle()
        a %= 2 * math.pi
        return t, d, a
    
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
    
    
class CPolyLine(object):
    '''
    sequence of subsequent line segments formed by sequence of points
    
    Allowed operations for polylines ABCD, EFGH, line IJ and point K:
        
        - C{abs(ABCD)} length of C{abs(AB) + abs(BC) + abs(CD)}
        - C{ABCD * EFGH} list of all the intersections, see L{CLine<CLine>}
        - C{ABCD * IJ} list of all the intersections
        - C{ABCD * K} K if it lies at the AB, BC or CD
    '''
    def __init__(self, points):
        '''
        create new instance of polyline
        
        @param points: sequence of points or tuples that can form a point
        @type points: sequence
        
        @raise MathException: if there is less than 2 points
        '''
        if len(points) < 2:
            raise MathException
        self.lines = []
        start = points[0]
        for end in points[1:]:
            self.lines.append(CLine(start, end))
            start = end
            
    def GetLine(self, index):
        '''
        @return: segment of polyline at index
        @rtype: L{CLine<CLine>}
        
        @param index: index of line segment, negative values indexes backwards
        @type index: int
        
        @raise IndexError: out of range
        '''
        return self.lines[index]
        
    def GetLines(self):
        '''
        Iterator over the line segments
        '''
        for line in self.lines:
            yield line
            
    def GetPos(self):
        '''
        @return: list of CPoint instances forming chain of segments
        @rtype: list
        '''
        result = [self.lines[0].GetStart().GetPos()]
        result.extend([line.GetEnd().GetPos() for line in self.lines])
        return result
        
    def Nearest(self, point):
        '''
        Get index of the closest line segment from the polyline to the
        specified point, position of the closest point at the segment, distance
        to it and angle
        
        @note: see L{CLine.Nearest<CLine.Nearest>}
        
        @return: (index, (x, y), distance, angle)
        @rtype: tuple
        
        @param point: specified position
        @type point: L{CPoint<CPoint>}
        '''
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
    '''
    Same functionality as L{CPolyLine<CPolyLine>} but includes segment from
    the last point to the first one.
    '''
    def __init__(self, points):
        CPolyLine.__init__(self, points)
        self.lines.append(CLine(points[-1], points[0]))
        
    def GetPos(self):
        return [line.GetStart().GetPos() for line in self.lines]
        
class CRectangle(CPolygon):
    '''
    Rectangular L{CPolygon<CPolygon>} with sides parallel to the sides of the 
    screen
    
    @note: only sides of the object are taken into account, not the area inside
    
    Allowed operations:
    
        - the same as for L{CPolygon<CPolygon>}
        - C{CRectangle() * CRectangle()} return list with top-left and 
        bottom-right L{CPoint<CPoint>}s of intersecting area or empty, if there
        is no intersection
    '''
    def __init__(self, topLeft, bottomRight):
        '''
        Create a rectangular polygon
        
        @param topLeft: (x, y) of top-left corner
        @type topLeft: (float, float) or L{CPoint<CPoint>}
        
        @param bottomRight: (x, y) of bottom-right corner
        @type bottomRight: (float, float) or L{CPoint<CPoint>}
        '''
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
        '''
        @return: top-left corner of rectangle
        @rtype: L{CPoint<CPoint>}
        '''
        return self.lines[0].GetStart()
        
    def GetBottomRight(self):
        '''
        @return: bottom-right corner of rectangle
        @rtype: L{CPoint<CPoint>}
        '''
        return self.lines[2].GetStart()
        
    def __mul__(self, other):
        if isinstance(other, CRectangle):
            if self.GetTopLeft() <= other.GetTopLeft() and self.GetBottomRight() >= other.GetBottomRight():
                return [other.GetTopLeft(), other.GetBottomRight()]
            elif self.GetTopLeft() >= other.GetTopLeft() and self.GetBottomRight() <= other.GetBottomRight():
                return [self.GetTopLeft(), self.GetBottomRight()]
        return CPolygon.__mul__(self, other)

