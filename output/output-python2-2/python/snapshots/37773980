class CVisualObject:
    def __init__(self):
        self.parent = None
    
    def GetHeight(self, element):
        return 0

    def GetWidth(self, element):
        return 0

    def GetParent(self):
        return self.parent

    def Paint(self, canvas, pos, element, size = (None, None)):
        pass
    
    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        pass

    def SetParent(self, parent):
        self.parent = parent
    
    def ComputeSize(self, canvas, element, size = (None, None)):
        if size[0] is None:
            size = self.GetWidth(canvas, element), size[1]
        if size[1] is None:
            size = size[0], self.GetHeight(canvas, element)
        return size
