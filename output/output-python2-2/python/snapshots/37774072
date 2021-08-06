import gtk.gdk
from lib.lib import ToBool,UMLException
from math import sin, cos, radians

ARROW_TYPES = {'simple': ('polyline',
                          [
                           [[-0.5],[-1],[1]],
                           [[0],[0],[1]],
                           [[0.5],[-1],[1]],
                          ]),
                'triangle': ('polygon',
                              [
                               [[-0.5],[-1],[1]],
                               [[0],[0],[1]],
                               [[0.5],[-1],[1]],
                              ]),
                'ftriangle': ('fillPolygon',
                              [
                               [[-0.5],[-1],[1]],
                               [[0],[0],[1]],
                               [[0.5],[-1],[1]],
                              ]),
                'diamond': ('polygon',
                            [
                               [[-0.5],[-1],[1]],
                               [[0],[0],[1]],
                               [[0.5],[-1],[1]],
                               [[0],[-2],[1]]
                            ]),
                'fdiamond': ('fillPolygon',
                            [
                               [[-0.5],[-1],[1]],
                               [[0],[0],[1]],
                               [[0.5],[-1],[1]],
                               [[0],[-2],[1]]
                            ]),
                'crosscircle': ('line',
                                [
                                    [[-0.5],[0],[1]],
                                    [[0.5],[0],[1]],
                                    [[0],[0.5],[1]],
                                    [[0],[-0.5],[1]],
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
    
    def __RotationMatrix(self, angle):
        return   [
                    [cos(angle),-sin(angle),0],
                    [sin(angle),cos(angle),0],
                    [0,0,1]
                 ]
    
    def __MultiplyMatrix(self, rotationMatrix, columnMatrix):
        resultMatrix = []
        for i in rotationMatrix:
            number = 0
            for j in xrange(len(i)):
                number += i[j] * columnMatrix[j][0]
            resultMatrix.append([number])
        return resultMatrix
    
    
    def __ResizeMatrix(self, matrix, size):
        for i in matrix:
            for j in i:
                j[0] = j[0] * size
        return matrix
    
    def Paint(self, x, y, angle, Connection):
        if self.possible is False:
            return
        rotationMatrix = self.__RotationMatrix(angle)
        points = []
        if self.style in ARROW_TYPES.keys():
            for i in ARROW_TYPES[self.style][1]:
                matrix = self.__MultiplyMatrix(rotationMatrix, i)
                points.append((int(matrix[0][0]*self.size+x),int(matrix[1][0]*self.size+y)))
            
            wgt = Connection.GetDrawingArea().GetDrawable()
            cmap = wgt.get_colormap()
            gc = wgt.new_gc()
            if ARROW_TYPES[self.style][0] == 'polyline':
                gc.foreground = cmap.alloc_color(self.color)
                wgt.draw_lines(gc,points)
            elif ARROW_TYPES[self.style][0] == 'polygon':
                gc.foreground = cmap.alloc_color(self.fill)
                wgt.draw_polygon(gc, True, points)
                gc.foreground = cmap.alloc_color(self.color)
                wgt.draw_polygon(gc, False, points)
            elif ARROW_TYPES[self.style][0] == 'fillPolygon':
                gc.foreground = cmap.alloc_color(self.color)
                wgt.draw_polygon(gc, True, points)
            elif ARROW_TYPES[self.style][0] == 'line':
                if self.style == 'crosscircle':
                    gc.foreground = cmap.alloc_color(self.fill)
                    wgt.draw_arc(gc, True, x - self.size/2, y - self.size/2, self.size, self.size ,0, 360*64)
                    gc.foreground = cmap.alloc_color(self.color)
                    wgt.draw_arc(gc, False, x - self.size/2, y -self.size/2, self.size, self.size ,0, 360*64)
                for i in xrange(0,len(points) - 1, 2):
                    gc.foreground = cmap.alloc_color(self.color)
                    wgt.draw_line(gc, points[i][0], points[i][1], points[i+1][0], points[i+1][1])                 
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