from SimpleContainer import CSimpleContainer

class CShadow(CSimpleContainer):
    def __init__(self, padding, color):
        CSimpleContainer.__init__(self)
        self.padding = int(padding)
        self.color = color

    def GetPadding(self):
        return self.padding

    def GetHeight(self, element):
        return self.GetChilds()[0].GetHeight(element)

    def GetWidth(self, element):
        return self.GetChilds()[0].GetWidth(element)

    def PaintShadow(self, x, y, element, color, w = None, h = None):
        raise UMLException("ShadowInShadow")

    def Paint(self, x, y, element, w = None, h = None):
        if w is None:
            w = self.GetWidth(element)
        if h is None:
            h = self.GetHeight(element)
        self.GetChilds()[0].PaintShadow(x + self.padding, y + self.padding,
                                    element, self.color, w, h)
        self.GetChilds()[0].Paint(x, y, element, w, h)

    def SetPadding(self, padding):
        self.padding = padding
