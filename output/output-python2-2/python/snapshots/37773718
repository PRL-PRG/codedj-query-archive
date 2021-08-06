import math

class Matrix(object):
    def __init__(self, size = None, matrix = None):
        if size is not None:
            self.matrix = [[0]*size[0] for i in xrange(size[1])]
            self.size = size
        elif matrix is not None:
            if isinstance(matrix, Matrix):
                self.matrix = matrix.matrix
                self.size = matrix.size
            else:
                self.matrix = matrix
                self.size = len(matrix[0]), len(matrix)
    
    def __mul__(self, other):
        if isinstance(other, (int, float, long)):
            return Matrix(matrix = [
                [scol*other for scol in srow]
                for srow in self.matrix
            ])
        if not isinstance(other, Matrix):
            return NotImplemented
        if self.size[0] != other.size[1]:
            return NotImplemented
        omat = other.matrix
        return Matrix(matrix = [
            [
                sum((srow[scol]*omat[scol][ocol] for scol in xrange(self.size[0])))
                for ocol in xrange(other.size[0])
            ]
            for srow in self.matrix
        ])
    
    def __div__(self, other):
        if not isinstance(other, (int, float, long)):
            return NotImplemented
        return Matrix(matrix = [
            [scol/other for scol in srow]
            for srow in self.matrix
        ])
    
    def __getitem__(self, (x, y)):
        return self.matrix[y-1][x-1]
    
    def __repr__(self):
        return "<%s(%dx%d) [%s]>"%(self.__class__.__name__, self.size[0], self.size[1], self.matrix)
    
    def __str__(self):
        return str(self.matrix)

class TransformMatrix(Matrix):
    def __init__(self, matrix = None):
        if matrix is None:
            Matrix.__init__(self, size = (3, 3))
        else:
            Matrix.__init__(self, matrix = matrix)
    
    def __mul__(self, other):
        if not isinstance(other, (TransformMatrix, PointMatrix)):
            return NotImplemented
        xxx = Matrix.__mul__(self, other)
        if isinstance(other, PointMatrix):
            ctl = xxx[(1, 3)]
            if ctl != 1:
                xxx = xxx/float(ctl)
            return PointMatrix(xxx)
        else:
            ctl = xxx[(3, 3)]
            if ctl != 1:
                xxx = xxx/float(ctl)
            return TransformMatrix(xxx)
    
    @classmethod
    def mk_rotation(cls, angle):
        return cls([
            [math.cos(angle),   -math.sin(angle),   0],
            [math.sin(angle),   math.cos(angle),    0],
            [0,                 0,                  1]
        ])
    
    @classmethod
    def mk_scale(cls, scale):
        return cls([
            [scale, 0,      0],
            [0,     scale,  0],
            [0,     0,      1]
        ])
    
    @classmethod
    def mk_scale2(cls, (scaleX, scaleY)):
        return cls([
            [scaleX,    0,      0],
            [0,         scaleY, 0],
            [0,         0,      1]
        ])
    
    @classmethod
    def mk_translation(cls, (dx, dy)):
        return cls([
            [1, 0,  dx],
            [0, 1,  dy],
            [0, 0,  1]
        ])
    
    @classmethod
    def mk_skrewX(cls, angle):
        return cls([
            [1, math.tan(angle),    0],
            [0, 1,                  0],
            [0, 0,                  1]
        ])
    
    @classmethod
    def mk_skrewY(cls, angle):
        return cls([
            [1,                 0,  0],
            [math.tan(angle),   1,  0],
            [0,                 0,  1]
        ])
    
    @classmethod
    def mk_matrix(cls, matrix):
        return cls([
            [matrix[0], matrix[2],  matrix[4]],
            [matrix[1], matrix[3],  matrix[5]],
            [0,         0,          1]
        ])
    
    @classmethod
    def mk_unit(cls):
        return cls([
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1]
        ])

class PointMatrix(Matrix):
    def __init__(self, matrix = None):
        if matrix is None:
            Matrix.__init__(self, size = (1, 3))
        else:
            Matrix.__init__(self, matrix = matrix)
    
    @classmethod
    def mk_xy(cls, (x, y)):
        return cls([
            [x],
            [y],
            [1]
        ])
    
    def GetIntPos(self):
        return int(self.matrix[0][0]), int(self.matrix[1][0])
    
    def GetPos(self):
        return self.matrix[0][0], self.matrix[1][0]
    
    def __mul__(self, other):
        return NotImplemented
