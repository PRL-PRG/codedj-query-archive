from SimpleContainer import CSimpleContainer

class CShadow(CSimpleContainer):
    def __init__(self, padding, color):
        CSimpleContainer.__init__(self)
        self.padding = int(padding)
        self.color = color

    def GetPadding(self):
        return self.padding

    def GetHeight(self, canvas, element):
        return self.GetChilds()[0].GetHeight(canvas, element)

    def GetWidth(self, canvas, element):
        return self.GetChilds()[0].GetWidth(canvas, element)

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        raise UMLException("ShadowInShadow")

    def Paint(self, canvas, pos, element, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        self.GetChilds()[0].PaintShadow(canvas, (pos[0] + self.padding, pos[1] + self.padding),
                                    element, self.color, size)
        self.GetChilds()[0].Paint(canvas, pos, element, size)

    def SetPadding(self, padding):
        self.padding = padding
