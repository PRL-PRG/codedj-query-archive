from lib.config import config

class CVisualObject:
    def __init__(self):
        self.parent = None
    
    def ParseVariables(self, element, *vals):
        for val in vals:
            if not isinstance(val, (str, unicode)):
                yield val
            elif val[0] == '#':
                yield element.GetObject().GetVisualProperty(val[1:])
            elif val[0] == '@':
                yield element.__LOOPVARS__['item'][val[1:]]
            elif val[0] == '/':
                yield config[val]
            else:
                yield val
    
    def GetVariables(self, element, *names):
        return self.ParseVariables(element, *(getattr(self, name) for name in names))
    
    def GetResizable(self):
        return False, False
    
    def GetSize(self, canvas, element):
        return 0, 0

    def GetParent(self):
        return self.parent

    def Paint(self, canvas, pos, element, size = (None, None)):
        pass
    
    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        pass

    def SetParent(self, parent):
        self.parent = parent
    
    def ComputeSize(self, canvas, element, size = (None, None)):
        if None in size:
            w, h = self.GetSize(canvas, element)
        if size[0] is None:
            size = w, size[1]
        if size[1] is None:
            size = size[0], h
        return size
