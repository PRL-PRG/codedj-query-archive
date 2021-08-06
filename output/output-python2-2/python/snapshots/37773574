from SimpleContainer import CSimpleContainer
from lib.lib import UMLException

class CProportional(CSimpleContainer):
    def __init__(self, ratio):
        CSimpleContainer.__init__(self)
        self.strratio = ratio
        ratio = ratio.split(':')
        if len(ratio) != 2:
            raise UMLException("XMLError", ("Proportional", "ratio"))
        self.ratio = float(int(ratio[0]))/int(ratio[1])

    def GetHeight(self, canvas, element):
        h = CSimpleContainer.GetHeight(self, canvas, element)
        w = CSimpleContainer.GetWidth(self, canvas, element)
        ch = int(w / self.ratio)
        if ch > h:
            return ch
        else:
            return h

    def GetWidth(self, canvas, element):
        h = CSimpleContainer.GetHeight(self, canvas, element)
        w = CSimpleContainer.GetWidth(self, canvas, element)
        cw = int(h * self.ratio)
        if cw > w:
            return cw
        else:
            return w
