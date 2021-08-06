from lib.Math2D import CPoint, CLineVector, CPolyLine, CLine
from math import pi, sqrt
#from lib.lib import UMLException
from CacheableObject import CCacheableObject
from SelectableObject import CSelectableObject

#imported for epydoc to be able to interconnect documentation links
#from lib.Drawing.Canvas import CCairoCanvas
#from VisibleObject import CVisibleObject


class EConLabelInfo(Exception): pass

class CConLabelInfo(CCacheableObject, CSelectableObject):
    '''
    Carries information about graphical representation of label 
    
    @Note: Older style of storing position of label was to keep it in list of
    form [position, index, t, dist, angle] with meaning:
        - position  --> (self.x, self.y)
        - index     --> self.idx
        - t         --> self.pos
        - dist      --> self.dist
        - angle     --> self.angle
    '''
    
    def __init__(self, connection, canvas, x = None, y = None, idx = 0, 
                       pos = 0.5, dist = 0, angle = pi/2, logicalLabel = None,
                       **kwds):
        '''create new instance of CLabelInfo
        
        @param connection: owner
        @type  connection: L{CConection<CConnection>}
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}
        
        @param x: absolute horizontal position of middle of label
        @type x: float 
        
        @param y: absolute vertical position of middle of label
        @type y: float 
        
        @param idx: index of connection part which is bound to
        @type idx: int >= 0 
        
        @param pos: relative position of point at lineidx which is bound to
        @type pos: float 
        
        @param dist: distance from label to point which is bound to
        @type dist: float
        
        @param logicalLabel: reference to logical representation of label.
        As long as not set, size of label defaults to (0, 0).
        @type logicalLabel: L{CVisualObject
        <lib.Drawing.Objects.VisualObject.CVisualObject>}
        
        @kwparam kwds: anything, will be ignored anyway. used so that won't 
        raise "Unexpected parameter" when passing parameters from XML
        
        @note: parameters idx, pos, dist can be also passed in
        string or unicode as long as they can be transformed to their required
        types by int() and float() functions respectively
        
        @attention: Either canvas or absolute position must be defined so that
        absolute position is either given or can be calculated
        
        @raise EConLabelInfo: if both absolute position and canvas is None
        '''
        
        super(CConLabelInfo, self).__init__()
        self.connection = connection
        self.x = float(x) if x is not None else None
        self.y = float(y) if y is not None else None
        self.idx = int(idx)
        self.dist = float(dist)
        self.pos = float(pos)
        self.angle = float(angle)
        if self.x is None or self.y is None:
            if canvas is None:
                raise EConLabelInfo('Insufficient data to create instance')
            self.RecalculateAbsolutePosition(canvas)
        self.logicalLabel = logicalLabel
    
    def GetSaveInfo(self):
        '''Get information about self to be saved to .frip file
        
        This information can be used to restore values of attributes
        
        @return: dictionary containing essential information
        @rtype: dict
        '''
        return {\
            'x': self.x,
            'y': self.y,
            'idx': self.idx,
            'pos': self.pos,
            'dist': self.dist,
            'angle': self.angle}
    
    def GetDiagram(self):
        '''
        @return: diagram to which belongs connection (owner)
        @rtype:  L{CDiagram<Diagram.CDiagram>}
        '''
        return self.connection.GetDiagram()
    
    def GetPosition(self, canvas):
        '''
        @return: absolute position of top-left corner (x, y)
        @rtype: tuple
        '''
        width, height = self.GetSize(canvas)
        return int(self.x - width / 2.0), int(self.y - height / 2.0)
    
    def SetLogicalLabel(self, logicalLabel):
        '''
        Set reference to logical representation of label
        
        Real size of label cannot be known before this is set. Defaults to 
        (0, 0)
        
        @param logicalLabel: reference to logical representation of label.
        @type logicalLabel: L{CVisualObject
        <lib.Drawing.Objects.VisualObject.CVisualObject>}
        '''
        self.logicalLabel = logicalLabel
    
    def SetPosition(self, pos, canvas):
        '''
        Set absolute position of top-left corner of label
        
        @param pos: (x, y)
        @type  pos: tuple
        '''
        width, height = self.GetSize(canvas)
        self.x, self.y = pos[0] + width / 2.0, pos[1] + height / 2.0
    
    def GetSize(self, canvas):
        '''
        @return: measures of label (width, height)
        @rtype:  tuple
        '''
        
        return (self.logicalLabel.GetSize(canvas, self.connection) 
            if self.logicalLabel is not None else (0, 0))
    
    def GetMinimalSize(self, canvas):
        '''
        executes L{self.GetSize<self.GetSize>}
        '''
        return self.GetSize(canvas)
        
    def GetSquare(self, canvas):
        '''
        Get absolute position of rectangle to wich label fits
        
        @return: ((left, top), (right, bottom)) positions of corners
        @rtype:  tuple
        '''
        
        width, height = self.GetSize(canvas)
        
        return ( (int(self.x - width / 2.), int(self.y - height / 2.) ),
                 (int(self.x + width / 2.), int(self.y + height / 2.) ) )
    
    def GetCentered(self):
        '''
        Get absolute position of middle of rectangle to which label fits
        
        @return: (x, y) position of the middle
        @rtype:  tuple
        '''
        return self.x, self.y
    
    def RecalculateAbsolutePosition(self, canvas):
        '''
        Reset absolute position of label according to its relative position
        to the connection.
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}
        '''
        
        points = list(self.connection.GetPoints(canvas))
        scaled = CLine(points[self.idx], points[self.idx + 1]).Scale(self.pos)
        self.x, self.y = CLineVector(scaled.GetEnd(),
            scaled.Angle() + self.angle, self.pos).GetEnd().GetPos()
    
    def RecalculateRelativePosition(self, canvas):
        '''
        Reset relative position of label to the connection according to the
        absolute position of label and polyline of the connection.
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}
        '''
        points = list(self.connection.GetPoints(canvas))
        self.idx, point, self.dist, self.angle = \
            CPolyLine(tuple(points)).Nearest(CPoint((self.x, self.y)))
        try:
            self.pos = (CPoint(points[self.idx]) - point) / \
                (CPoint(points[self.idx + 1]) - CPoint(points[self.idx]))
        except ZeroDivisionError:
            self.pos = 0.0
    
    def MoveWithOthers(self, delta):
        '''Move position of label by delta.
        
        @attention: Called only if whole connection moved by delta. Thus called
        only from L{CConnection.MoveAll<CConnection.MoveAll>} and only absolute
        position is changed.
        
        @param delta: (dx, dy) distance
        @type  delta: tuple
        '''
        self.x += delta[0]
        self.y += delta[1]

    def AreYouAtPosition(self, canvas, point):
        '''@return: True if (x, y) hits label
        @rtype: bool
        
        @param point: (x, y) position
        @type  point: tuple
        '''
        x, y = point
        ((x1, y1), (x2, y2)) = self.GetSquare(canvas)
        return x1 <= x <= x2 and y1 <= y <= y2
            
    def AreYouInRange(self, canvas, topleft, bottomright, all = False):
        '''Check wheter label is withinin rectangular area
        
        Can use two policy decision, depending on value of parameter all:
        
            - Whole label must be inside the rectangular area (all == True)
            - Label and rectangular area must have some intersection
        
        @return: True if label is in area
        @rtype: bool
        
        @param canvas: Canvas on which its being drawn.
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}
        
        @param topleft: (x, y) position of top-left corner
        @type  topleft: tuple
        
        @param bottomright: (x, y) position of bottom-right corner
        @type  bottomright: tuple
        
        @param all: policy switch
        @type  all: bool
        '''
        
        class Test(object):
            def __init__(self, square):
                (self.x1, self.y1), (self.x2, self.y2) = square
            def __call__(self, pos):
                return self.x1 <= pos[0] <= self.x2 \
                    and self.y1 <= pos[1] <= self.y2
        
        t, l = topleft
        b, r = bottomright
        ((x1, y1), (x2, y2)) = self.GetSquare(canvas)
        if all:
            return l <= x1 <= x2 <= r and t <= y1 <= y2 <= b
        else:
            return (
                any( map( Test(((x1, y1), (x2, y2))), 
                    ((t,l),(t,r),(b,l),(b,r)))) or
                (x1 <= l <= r <= x2 and t <= y1 <= y2 <= b ) or
                (l <= x1 <= x2 <= r and y1 <= t <= b <= y2 ) )
    
    def SetToDefaultPosition(self, canvas, position):
        '''Set absolute and relative position according to default position
        defined by parameter position. Can be moved by offset by appending sign
        "+" or "-" and float number to recognized names of position.
        
        @param canvas: Canvas on which its being drawn
        @type  canvas: L{CCairoCanvas<CCairoCanvas>}
        
        @param position: one of "center", "source", "destination"
        @type  position: str
        '''
        
        points = list(self.connection.GetPoints(canvas))
        if position.count('+'): # if there is offset specified
            position, offset = position.split('+', 1) # separate them
            try:
                offset = float(offset)
            except ValueError:
                raise EConLabelInfo('UndefinedOffset')
        elif position.count('-'): # offset as negative number
            position, offset = position.split('-', 1) # separate them
            try:
                offset = -float(offset)
            except ValueError:
                raise EConLabelInfo('UndefinedOffset')
        else:
            offset = None
        
        if position == 'source': # exactly at the very first point of the line
            self.idx = 0
            self.pos = 0.0
        elif position == 'destination': #exactly at the very last point of 
            self.idx = len(points) - 2
            self.pos = 1.0
        elif position == 'center':
            # find lenght of all parts of line as sum of lengths of distinct
            # parts
            lengths = [ sqrt( (points[i][0] - points[i+1][0])**2 +  
                                 (points[i][1] - points[i+1][1])**2 )
                           for i in xrange(len(points)-1) ]
            length = sum(lengths)
            if length == 0.0:
                self.idx = 0
                self.pos = 0.5
            else:
                i = 1 # index of part, where is the middle of the polyline
                half = length / 2.0
                while sum(lengths[:i]) < half:
                    i += 1
                # now update attributes
                self.idx = i-1
                if self.idx == 0:
                    self.pos = half / lengths[0]
                else:
                    self.pos = (half - sum(lengths[:i-1])) / lengths[i-1]
        else:
            raise EConLabelInfo("UndefinedPosition")
        self.dist = 0.0
        self.angle = 0.0
        self.RecalculateAbsolutePosition(canvas)
        if offset is not None:
            multi = -1 if (points[self.idx][1] - points[self.idx + 1][1]) * \
                (.5 - self.pos) < 0 else 0
            self.y += multi * offset * self.height
            self.RecalculateRelativePosition(canvas)
    
