from SimpleContainer import CSimpleContainer
from lib.lib import UMLException

class CShadow(CSimpleContainer):
    def __init__(self, padding, color):
        CSimpleContainer.__init__(self)
        self.padding = int(padding)
        self.color = color

    def GetSize(self, canvas, element):
        size = element.GetCachedSize(self)
        if size is not None:
            return size
        return element.CacheSize(self, self.GetChild().GetSize(canvas, element))

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        raise UMLException("ShadowInShadow")

    def Paint(self, canvas, pos, element, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        color, = self.GetVariables(element, 'color')
        self.GetChild().PaintShadow(canvas, (pos[0] + self.padding, pos[1] + self.padding),
                                    element, color, size)
        self.GetChild().Paint(canvas, pos, element, size)
