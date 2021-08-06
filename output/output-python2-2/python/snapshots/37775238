class CDrawingContext(object):
    def __init__(self, canvas, element, pos, size = (None, None)):
        self.canvas = canvas
        self.element = element
        self.pos = pos
        self.size = size
        self.variables = {}
        self.stack = []
        self.shadowcolor = None
    
    def Push(self):
        self.stack.append((self.pos, self.size, self.variables, self.stack, self.shadowcolor))
    
    def Pop(self):
        self.pos, self.size, self.variables, self.stack, self.shadowcolor = self.stack.pop()
    
    def ComputeSize(self, object):
        size = self.size
        if None in size:
            w, h = object.GetSize(self)
            if size[0] is None:
                size = w, size[1]
            if size[1] is None:
                size = size[0], h
        return size
    
    def GetSize(self):
        return self.size
    
    def GetCachedSize(self, object):
        return self.element.GetCachedSize(object)
    
    def GetPos(self):
        return self.pos
    
    def GetCanvas(self):
        return self.canvas
    
    def GetShadowColor(self):
        return self.shadowcolor
    
    def GetVariable(self, varname):
        return self.variables[varname]
    
    def GetAttribute(self, varname):
        return self.element.GetObject().GetVisualProperty(varname)
    
    __getitem__ = GetVariable
    
    def Resize(self, newsize):
        self.size = newsize
    
    def CacheSize(self, object, size):
        return self.element.CacheSize(object, size)
    
    def Move(self, newpos):
        self.pos = newpos
    
    def SetVariables(self, vars):
        self.variables = vars
    
    def SetShadowColor(self, color):
        self.shadowcolor = color
    
    def GetPoints(self):
        return self.element.GetPoints(self.canvas)
