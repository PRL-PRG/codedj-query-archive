from SimpleContainer import CSimpleContainer

class CPadding(CSimpleContainer):
    def __init__(self, padding):
        CSimpleContainer.__init__(self)
        self.padding = int(padding)

    def GetSize(self, canvas, element):
        size = element.GetCachedSize(self)
        if size is not None:
            return size
        w, h = CSimpleContainer.GetSize(self, canvas, element)
        return element.CacheSize(self, (w + 2*self.padding, h + 2*self.padding))

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        CSimpleContainer.PaintShadow(self, canvas, (pos[0]+self.padding, pos[1]+self.padding),
                    element, color, (size[0] - 2*self.padding, size[1] - 2*self.padding))

    def Paint(self, canvas, pos, element, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        CSimpleContainer.Paint(self, canvas, (pos[0]+self.padding, pos[1]+self.padding), element,
                    (size[0] - 2*self.padding, size[1] - 2*self.padding))
