from SimpleContainer import CSimpleContainer
from lib.Math2D import Path, PathPartLine, PathPartMove, TransformMatrix
import math

corners = {
    'rounded': 'M 0,1 C 0,0.446 0.446,0 1,0',
    'note': 'M 0,1 L 1,1 L 1,0 L 0,1 z  M 0,1 L 1,0',
}

class CRectangle(CSimpleContainer):
    def __init__(self, fill = None, border = "white", borderwidth = 1, lefttop = None, righttop = None, leftbottom = None, rightbottom = None):
        CSimpleContainer.__init__(self)
        self.fill = fill
        self.border = border
        
        self.corners = []
        for i, c in enumerate((lefttop, righttop, rightbottom, leftbottom)):
            if isinstance(c, (str, unicode)):
                c = c.split(None, 2)
                trans = TransformMatrix.mk_scale(int(c[0]))*TransformMatrix.mk_rotation(i*math.pi/2)
                c = str(c[1]), trans*Path(corners.get(c[2], c[2]))
            self.corners.append(c)
        self.corners = tuple(self.corners)
        
        self.borderwidth = int(borderwidth)

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        if self.corners == (None, None, None, None):
            canvas.DrawRectangle(pos, size, None, color)
        else:
            corners = []
            positions = pos, (pos[0] + size[0], pos[1]), (pos[0] + size[0], pos[1]+size[1]), (pos[0], pos[1]+size[1])
            oldpos = None
            for i, c in enumerate(self.corners):
                if c is None:
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
            canvas.DrawPath(corners, None, color)

    def Paint(self, canvas, pos, element, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        if self.corners == (None, None, None, None):
            canvas.DrawRectangle(pos, size, self.border, self.fill)
        else:
            corners = []
            positions = pos, (pos[0] + size[0], pos[1]), (pos[0] + size[0], pos[1]+size[1]), (pos[0], pos[1]+size[1])
            oldpos = None
            for i, c in enumerate(self.corners):
                if c is None:
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
            canvas.DrawPath(corners, self.border, self.fill)
        
        for i in self.childs:
            i.Paint(canvas, pos, element, size)
        
        for i, c in enumerate(self.corners):
            if c is not None and len(c[1]) > 0:
                tmp = TransformMatrix.mk_translation(positions[i])*c[1][:-1]
                canvas.DrawPath(tmp, self.border, c[0])
