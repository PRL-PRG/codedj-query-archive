from SimpleContainer import CSimpleContainer
from lib.Math2D import Path, PathPartLine, PathPartMove, TransformMatrix
import math
from lib.Exceptions.UserException import *

corners = {
    'rounded': 'M 0,1 C 0,0.446 0.446,0 1,0',
    'note': 'M 0,1 L 1,1 L 1,0 L 0,1 z  M 0,1 L 1,0',
}

sides = {
    'rounded': 'M 0,0 C -0.554,0 -1,-0.223 -1,-0.5 C -1,-0.777 -0.554,-1 0,-1',
}

class CRectangle(CSimpleContainer):
    def __init__(self, fill = None, border = "white", lefttop = None, righttop = None, leftbottom = None, rightbottom = None, left = None, right = None, top = None, bottom = None):
        CSimpleContainer.__init__(self)
        self.fill = fill
        self.border = border
        
        self.corners = []
        for i, c in enumerate((lefttop, righttop, rightbottom, leftbottom)):
            if isinstance(c, (str, unicode)):
                c = c.split(None, 2)
                if len(c) == 2:
                    c = c[0], None, c[1]
                trans = TransformMatrix.mk_scale(int(c[0]))*TransformMatrix.mk_rotation(i*math.pi/2)
                c = str(c[1]), trans*Path(corners.get(c[2], c[2]))
            self.corners.append(c)
        self.corners = tuple(self.corners)
        
        self.sides = []
        for i, s in enumerate((top, right, bottom, left)):
            if isinstance(s, (str, unicode)):
                s = s.split(None, 2)
                if len(s) == 2:
                    s = s[0], None, s[1]
                trans = TransformMatrix.mk_rotation((i+1)*math.pi/2)
                s = str(s[1]), trans*Path(sides.get(s[2], s[2])), int(s[0])
            self.sides.append(s)
        self.sides = tuple(self.sides)
        if top is not None:
            if lefttop is not None or righttop is not None:
                raise XMLError("Rectangle", "top")
        if bottom is not None:
            if leftbottom is not None or rightbottom is not None:
                raise XMLError("Rectangle", "bottom")
        if left is not None:
            if lefttop is not None or leftbottom is not None:
                raise XMLError("Rectangle", "left")
        if right is not None:
            if righttop is not None or rightbottom is not None:
                raise XMLError("Rectangle", "right")
    
    def GetResizable(self):
        return True, True

    def Paint(self, context):
        size = context.ComputeSize(self)
        shadowcolor = context.GetShadowColor()
        if shadowcolor is None:
            border, fill = self.GetVariables(context, 'border', 'fill')
        else:
            border, fill = None, shadowcolor
        
        canvas = context.GetCanvas()
        pos = context.GetPos()
        size = context.ComputeSize(self)
        
        if self.sides == self.corners == (None, None, None, None):
            canvas.DrawRectangle(pos, size, border, fill)
        else:
            corners = []
            (x, y), (w, h) = pos, size
            if self.sides[0] is not None:
                y += self.sides[0][2]
                h -= self.sides[0][2]
            if self.sides[1] is not None:
                w -= self.sides[1][2]
            if self.sides[2] is not None:
                h -= self.sides[2][2]
            if self.sides[3] is not None:
                x += self.sides[3][2]
                w -= self.sides[3][2]
            positions = (x, y), (x + w, y), (x + w, y+h), (x, y+h)
            oldpos = None
            lastside = None
            for i, c in enumerate(self.corners):
                if c is None:
                    if self.sides[i] is not None:
                        scale = ((w, self.sides[i][2]), (self.sides[i][2], h), (w, self.sides[i][2]), (self.sides[i][2], h))
                        if i == 3 and lastside is not None:
                            tmp = lastside
                        else:
                            tmp = TransformMatrix.mk_translation(positions[i])*TransformMatrix.mk_scale2(scale[i])*self.sides[i][1][-1]
                        if self.sides[i-1] is None:
                            if i:
                                corners.append(PathPartLine(oldpos, tmp.GetFirstPos()))
                            else:
                                corners.append(PathPartMove(tmp.GetFirstPos()))
                        corners.append(tmp)
                        oldpos = tmp.GetLastPos()
                    elif self.sides[i-1] is not None:
                        if not i:
                            scale = ((w, self.sides[i-1][2]), (self.sides[i-1][2], h), (w, self.sides[i-1][2]), (self.sides[i-1][2], h))
                            lastside = TransformMatrix.mk_translation(positions[i-1])*TransformMatrix.mk_scale2(scale[i-1])*self.sides[i-1][1][-1]
                            oldpos = lastside.GetLastPos()
                    else:
                        if i:
                            corners.append(PathPartLine(oldpos, positions[i]))
                        else:
                            corners.append(PathPartMove(positions[i]))
                        oldpos = positions[i]
                else:
                    tmp = TransformMatrix.mk_translation(positions[i])*c[1][-1]
                    corners.append(tmp)
                    oldpos = tmp.GetLastPos()
            corners = Path.Join(corners).Flattern()
            corners.Close()
            canvas.DrawPath(corners, border, fill)
        
        if shadowcolor is not None:
            return
        
        CSimpleContainer.Paint(self, context)
        
        for i, c in enumerate(self.corners):
            if c is not None and len(c[1]) > 0:
                tmp = TransformMatrix.mk_translation(positions[i])*c[1][:-1]
                color, = self.ParseVariables(context, c[0])
                canvas.DrawPath(tmp, border, color)
        
        for i, s in enumerate(self.sides):
            if s is not None and len(s[1]) > 0:
                tmp = TransformMatrix.mk_translation(positions[i])*s[1][:-1]
                color, = self.ParseVariables(context, s[0])
                canvas.DrawPath(tmp, border, color)
