from SimpleContainer import CSimpleContainer

from lib.lib import ToBool

class CCondition(CSimpleContainer):
    def __init__(self, condition, type = "equal", negate = False, value = None):
        CSimpleContainer.__init__(self)
        self.condition = condition
        self.type = type
        self.negate = ToBool(negate)
        self.value = value
    
    def __IsTrue(self, element):
        ret = True
        condition, value = self.GetVariables(element, 'condition', 'value')
        if self.type == 'empty':
            ret = not condition
        elif self.type == 'equal':
            ret = condition == value
        if self.negate:
            return not ret
        return ret

    def GetSize(self, canvas, element):
        size = element.GetCachedSize(self)
        if size is not None:
            return size
        if self.__IsTrue(element):
            return element.CacheSize(self, CSimpleContainer.GetSize(self, canvas, element))
        return element.CacheSize(self, (0, 0))

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        if self.__IsTrue(element):
            for child in self.childs:
                CSimpleContainer.Paint(self, canvas, pos, element, color, size)

    def Paint(self, canvas, pos, element, size = (None, None)):
        if self.__IsTrue(element):
            for child in self.childs:
                CSimpleContainer.Paint(self, canvas, pos, element, size)
