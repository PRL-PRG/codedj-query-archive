from SimpleContainer import CSimpleContainer

from lib.lib import ToBool

class CCondition(CSimpleContainer):
    def __init__(self, condition, type, negate):
        CSimpleContainer.__init__(self)
        self.condition = condition
        self.type = type
        self.negate = ToBool(negate)
    
    def __IsTrue(self, element):
        ret = True
        if self.type == 'empty':
            if element.GetObject().GetVisualProperty(self.condition):
                ret = False
            else:
                ret = True
        if self.negate:
            return not ret
        return ret

    def GetCondition(self):
        return self.condition

    def GetNegate(self):
        self.negate

    def GetType(self):
        self.type

    def GetHeight(self, element):
        if self.__IsTrue(element):
            return CSimpleContainer.GetHeight(self, element)
        return 0

    def GetWidth(self, element):
        if self.__IsTrue(element):
            return CSimpleContainer.GetWidth(self, element)
        return 0

    def PaintShadow(self, x, y, element, color, w = None, h = None):
        if self.__IsTrue(element):
            for child in self.childs:
                CSimpleContainer.Paint(self, x, y, element, color, w, h)

    def Paint(self, x, y, element, w = None, h = None):
        if self.__IsTrue(element):
            for child in self.childs:
                CSimpleContainer.Paint(self, x, y, element, w, h)
    
    def SetCondition(self, condition):
        self.condition = condition

    def SetNegate(self, negate):
        self.negate = negate

    def SetType(self, type):
        self.type = type