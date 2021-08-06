from transform_matrix import TransformMatrix, PointMatrix

class PathPart:
    operation = None
    
    def GetOp(self):
        return self.operation
    
    def SetOp(self, value):
        self.operation = value

class PathPartMove(PathPart):
    operation = 'start'
    
    def __init__(self, point):
        if isinstance(point, PointMatrix):
            self.point = point
        else:
            self.point = PointMatrix.mk_xy(point)
    
    def __iter__(self):
        yield self.point.GetIntPos()
    
    def __rmul__(self, other):
        if not isinstance(other, TransformMatrix):
            return NotImplemented
        ret = self.__class__(other*self.point)
        ret.SetOp(self.operation)
        
        return ret
    
    def __str__(self):
        return 'M %d,%d'%self.point.GetIntPos()

class PathPartLine(PathPart):
    operation = 'line'
    
    def __init__(self, point1, point2):
        if isinstance(point1, PointMatrix):
            self.point1 = point1
        else:
            self.point1 = PointMatrix.mk_xy(point1)
        if isinstance(point2, PointMatrix):
            self.point2 = point2
        else:
            self.point2 = PointMatrix.mk_xy(point2)
    
    def __iter__(self):
        yield self.point2.GetIntPos()
    
    def __rmul__(self, other):
        if not isinstance(other, TransformMatrix):
            return NotImplemented
        ret = self.__class__(other*self.point1, other*self.point2)
        ret.SetOp(self.operation)
        
        return ret
    
    def __str__(self):
        return 'L %d,%d'%self.point2.GetIntPos()

class PathPartBezier(PathPart):
    operation = 'bezier'
    
    def __init__(self, point1, point2, point3, point4):
        if isinstance(point1, PointMatrix):
            self.point1 = point1
        else:
            self.point1 = PointMatrix.mk_xy(point1)
        if isinstance(point2, PointMatrix):
            self.point2 = point2
        else:
            self.point2 = PointMatrix.mk_xy(point2)
        if isinstance(point3, PointMatrix):
            self.point3 = point3
        else:
            self.point3 = PointMatrix.mk_xy(point3)
        if isinstance(point4, PointMatrix):
            self.point4 = point4
        else:
            self.point4 = PointMatrix.mk_xy(point4)
    
    def __iter__(self):
        pt1 = self.point1.GetIntPos()
        pt2 = self.point2.GetIntPos()
        pt3 = self.point3.GetIntPos()
        pt4 = self.point4.GetIntPos()
        
        t = 0
        step = 1/16.0
        while (t-step) < 1:
            new = (1-t)**3*pt1[0]+3*t*(1-t)**2*pt2[0]+3*t**2*(1-t)*pt3[0]+t**3*pt4[0], \
                  (1-t)**3*pt1[1]+3*t*(1-t)**2*pt2[1]+3*t**2*(1-t)*pt3[1]+t**3*pt4[1]
            yield new
            t += step
        yield pt4
    
    def __rmul__(self, other):
        if not isinstance(other, TransformMatrix):
            return NotImplemented
        ret = self.__class__(other*self.point1, other*self.point2, other*self.point3, other*self.point4)
        ret.SetOp(self.operation)
        
        return ret
    
    def __str__(self):
        return 'C %d,%d %d,%d %d,%d'%(self.point2.GetIntPos()+self.point3.GetIntPos()+self.point4.GetIntPos())

class PathSingle:
    def __init__(self, path = None):
        if path is None:
            self.path = []
        else:
            self.path = path
    
    def append(self, part):
        self.path.append(part)
    
    def __iter__(self):
        return (point for part in self.path for point in part)
        #for part in self.path:
        #    for point in part:
        #        yield point
    
    def __getitem__(self, item):
        return self.path[item]
    
    def __rmul__(self, other):
        if not isinstance(other, TransformMatrix):
            return NotImplemented
        ret = []
        for part in self.path:
            ret.append(other*part)
        return self.__class__(ret)
    
    def GetType(self):
        if self.path[0].GetOp() == 'startstop':
            return 'polygon'
        else:
            return 'polyline'
    
    def __str__(self):
        return ' '.join([str(i) for i in self.path])

class Path:
    def __init__(self, path):
        if isinstance(path, list):
            self.path = path
        else:
            self.path = self.__parsepath(path)
    
    def __popsur(self, arr):
        tmp = arr.pop(0).split(',')
        return (float(tmp[0]), float(tmp[1]))
    
    def __parsepath(self, path):
        ret = []
        path = path.split()
        ret2 = None
        while len(path) > 0:
            cmd = path.pop(0)
            if cmd == 'M':
                ret2 = PathSingle()
                ret.append(ret2)
                pt1 = self.__popsur(path)
                ret2.append(PathPartMove(pt1))
            else:
                if ret2 is None:
                    raise Exception, "First command of path in svg must be move (M)"
                if cmd in ('C', 'c'):
                    pt2 = self.__popsur(path)
                    pt3 = self.__popsur(path)
                    pt4 = self.__popsur(path)
                    if cmd == 'c':
                        pt2 = pt2[0]+pt1[0], pt2[1]+pt1[1]
                        pt3 = pt3[0]+pt1[0], pt3[1]+pt1[1]
                        pt4 = pt4[0]+pt1[0], pt4[1]+pt1[1]
                    ret2.append(PathPartBezier(pt1, pt2, pt3, pt4))
                    pt1 = pt4
                elif cmd in ('L', 'l'):
                    pt2 = self.__popsur(path)
                    if cmd == 'l':
                        pt2 = pt2[0]+pt1[0], pt2[1]+pt1[1]
                    ret2.append(PathPartLine(pt1, pt2))
                    pt1 = pt2
                elif cmd == 'z':
                    ret2[0].SetOp('startstop')
                    ret2 = None
        return ret
    
    def __rmul__(self, other):
        if not isinstance(other, TransformMatrix):
            return NotImplemented
        ret = []
        for path in self.path:
            ret.append(other*path)
        return self.__class__(ret)
    
    def __len__(self):
        return len(self.path)
    
    def __getitem__(self, index):
        if type(index) is slice:
            return Path(self.path[index])
        else:
            return self.path[index]
    
    def __iter__(self):
        return (single for single in self.path)
        #for single in self.path:
        #    yield single
    
    def __repr__(self):
        return ' '.join([str(i) for i in self.path])
