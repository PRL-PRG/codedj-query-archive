from lib.lib import ToBool
from lib.Exceptions.UserException import *
from lib.config import config
from math import pi

from lib.Math2D import TransformMatrix, PointMatrix

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
    """
    Class that represents ending arrow of an connection
    """
    def __init__(self, default = False, possible = True, style = 'simple', color = 'black', fill = '#A5B6C7', size = 10):
        """
        Initialize arrow to values
        
        @param default: Default state of arrow (shown or hidden)
        @type  default: boolean or string
        
        @param possible: True, if arrow is possible to be shown
        @type  possible: boolean or string
        
        @param style: style of arrow (one of ARROW_TYPES)
        @type  style: string
        
        @param color: line color of the arrow in HTML format
        @type  color: string
        
        @param fill: fill color of the arrow in HTML format
        @type  fill: string
        
        @param size: size of the arrow in pixels
        @type  size: integer
        """
        self.possible = ToBool(possible)
        self.default = ToBool(default)
        self.style = style
        self.fill = fill
        self.size = int(size)
        self.color = color
    
    def Paint(self, canvas, pos, angle):
        """
        Paint arrow on the canvas
        
        @param canvas: Arrow will be painted on this canvas
        @type  canvas: L{CAbstractCanvas<lib.Drawing.Canvas.Abstract.CAbstractCanvas>}
        
        @param pos: Position of the center of the arrow
        @type  pos: (integer, integer)
        
        @param angle: Rotation angle of arrow in radians
        @type  angle: float
        """
        if self.default is False:
            return
        
        steps = config['/Styles/Connection/ArrowAngleSteps']
        step = 2 * pi / steps
        angle = step * ( (angle // step + (1 if angle % step / step > .5 
                          else 0)) % steps )
        
        transMatrix = TransformMatrix.mk_translation(pos)*TransformMatrix.mk_rotation(angle)* \
                        TransformMatrix.mk_scale(self.size)
        x, y = pos
        if self.fill[0] == '/':
            fill = config[self.fill]
        else:
            fill = self.fill
        if self.color[0] == '/':
            color = config[self.color]
        else:
            color = self.color
        points = []
        if self.style in ARROW_TYPES.keys():
            for i in ARROW_TYPES[self.style][1]:
                points.append((transMatrix*i).GetIntPos())
            
            if ARROW_TYPES[self.style][0] == 'polyline':
                canvas.DrawLines(points, color)
            elif ARROW_TYPES[self.style][0] == 'polygon':
                canvas.DrawPolygon(points, bg = fill, fg = color)
            elif ARROW_TYPES[self.style][0] == 'fillPolygon':
                canvas.DrawPolygon(points, bg = color)
            elif ARROW_TYPES[self.style][0] == 'line':
                if self.style == 'crosscircle':
                    canvas.DrawArc((x - self.size/2, y - self.size/2), (self.size, self.size), fg = color, bg = fill)
                for i in xrange(0,len(points) - 1, 2):
                    canvas.DrawLine(points[i], points[i+1], color)
        else:
            raise ConnectionError("UndefinedStyleArrow")
