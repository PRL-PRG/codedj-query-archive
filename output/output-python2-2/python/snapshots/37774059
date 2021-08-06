from SimpleContainer import CSimpleContainer

class CPadding(CSimpleContainer):
    def __init__(self, padding):
        CSimpleContainer.__init__(self)
        self.padding = int(padding)

    def GetPadding(self):
        return self.padding

    def GetHeight(self, element):
        return CSimpleContainer.GetHeight(self, element) + 2*self.padding

    def GetWidth(self, element):
        return CSimpleContainer.GetWidth(self, element) + 2*self.padding

    def PaintShadow(self, x, y, element, color, w = None, h = None):
        if w is None:
            w = self.GetWidth(element)
        if h is None:
            h = self.GetHeight(element)
        CSimpleContainer.PaintShadow(self, x+self.padding, y+self.padding, element, color,
                                    w - 2*self.padding, h - 2*self.padding)

    def Paint(self, x, y, element, w = None, h = None):
        if w is None:
            w = self.GetWidth(element)
        if h is None:
            h = self.GetHeight(element)
        CSimpleContainer.Paint(self, x+self.padding, y+self.padding, element, w - 2*self.padding,
                                    h - 2*self.padding)

    def SetPadding(self, padding):
        self.padding = padding
