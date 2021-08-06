from lib.lib import ToBool,UMLException
# from math import sin, cos, radians

from lib.transform_matrix import TransformMatrix, PointMatrix

ARROW_TYPES = {'simple': ('polyline',
                          [
                           PointMatrix.mk_xy((-0.5, -1)),
                           PointMatrix.mk_xy((0, 0)),
                           PointMatrix.mk_xy((0.5, -1)),
                          ]),
                'triangle': ('polygon',
                              [
                               PointMatrix.mk_xy((-0.5, -1)),
                               PointMatrix.mk_xy((0, 0)),
                               PointMatrix.mk_xy((0.5, -1)),
                              ]),
                'ftriangle': ('fillPolygon',
                              [
                               PointMatrix.mk_xy((-0.5, -1)),
                               PointMatrix.mk_xy((0, 0)),
                               PointMatrix.mk_xy((0.5, -1)),
                              ]),
                'diamond': ('polygon',
                            [
                               PointMatrix.mk_xy((-0.5, -1)),
                               PointMatrix.mk_xy((0, 0)),
                               PointMatrix.mk_xy((0.5, -1)),
                               PointMatrix.mk_xy((0, -2)),
                            ]),
                'fdiamond': ('fillPolygon',
                            [
                               PointMatrix.mk_xy((-0.5, -1)),
                               PointMatrix.mk_xy((0, 0)),
                               PointMatrix.mk_xy((0.5, -1)),
                               PointMatrix.mk_xy((0, -2)),
                            ]),
                'crosscircle': ('line',
                                [
                                    PointMatrix.mk_xy((-0.5, 0)),
                                    PointMatrix.mk_xy((0.5, 0)),
                                    PointMatrix.mk_xy((0, 0.5)),
                                    PointMatrix.mk_xy((0, -0.5)),
                                ])
              }

class CConnectionArrow(object):
    def __init__(self, default = False, possible = True, style = 'simple', color = 'black', fill = '#A5B6C7', size = 10):
        self.possible = ToBool(possible)
        self.default = ToBool(default)
        self.style = style
        self.fill = fill
        self.size = int(size)
        self.color = color
    
    def Paint(self, canvas, pos, angle, Connection):
        if self.possible is False:
            return
        transMatrix = TransformMatrix.mk_translation(pos)*TransformMatrix.mk_rotation(angle)* \
                        TransformMatrix.mk_scale(self.size)
        x, y = pos
        points = []
        if self.style in ARROW_TYPES.keys():
            for i in ARROW_TYPES[self.style][1]:
                points.append((transMatrix*i).GetIntPos())
            
            if ARROW_TYPES[self.style][0] == 'polyline':
                canvas.DrawLines(points, self.color)
            elif ARROW_TYPES[self.style][0] == 'polygon':
                canvas.DrawPolygon(points, bg = self.fill, fg = self.color)
            elif ARROW_TYPES[self.style][0] == 'fillPolygon':
                canvas.DrawPolygon(points, bg = self.color)
            elif ARROW_TYPES[self.style][0] == 'line':
                if self.style == 'crosscircle':
                    canvas.DrawArc((x - self.size/2, y - self.size/2), (self.size, self.size), fg = self.color, bg = self.fill)
                for i in xrange(0,len(points) - 1, 2):
                    canvas.DrawLine(points[i], points[i+1], self.color)
        else:
            raise UMLException("UndefinedStyleArrow")

    def GetDefault(self):
        self.default

    def GetPossible(self):
        return self.possible

    def GetStyle(self):
        return self.style

    def SetDefault(self, value):
        self.default = value

    def SetPossible(self, value):
        self.possible = value

    def SetStyle(self, value):
        self.style = value
        
    Style = property(GetStyle, SetStyle)
    Possible = property(GetPossible, SetPossible)
    Default = property(GetDefault, SetDefault)