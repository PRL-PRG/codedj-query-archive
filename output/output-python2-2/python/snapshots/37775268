from Container import CContainer
from SimpleContainer import CSimpleContainer
from lib.Exceptions.UserException import *
from lib.lib import ToBool

class CCase(CSimpleContainer):
    def __init__(self, condition = None, negate = False, type = "equal"):
        CSimpleContainer.__init__(self)
        self.condition = condition
        self.type = type
        self.negate = ToBool(negate)

    def SetParent(self, parent):
        if not isinstance(parent, CSwitch):
            raise XMLError("Switch as parent needed")
        CSimpleContainer.SetParent(self, parent)
    
    def IsTrue(self, element, value):
        ret = True
        condition, = self.GetVariables(element, 'condition')
        
        if condition is None:
            return True
        
        if self.type == 'equal':
            ret = condition == value
        if self.negate:
            return not ret
        return ret

class CSwitch(CContainer):
    def __init__(self, value):
        CContainer.__init__(self)
        self.value = value
    
    def AppendChild(self, child):
        if not isinstance(child, CCase):
            raise XMLError("Case needed")
        CContainer.AppendChild(self, child)
    
    def GetSize(self, canvas, element):
        value, = self.GetVariables(element, 'value')
        size = element.GetCachedSize(self)
        if size is not None:
            return size
        w = 0
        h = 0
        for i in self.childs:
            if i.IsTrue(element, value):
                w, h = i.GetSize(canvas, element)
                break
        return element.CacheSize(self, (w, h))

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        value, = self.GetVariables(element, 'value')
        for i in self.childs:
            if i.IsTrue(element, value):
                i.PaintShadow(canvas, pos, element, color, size)
                break

    def Paint(self, canvas, pos, element, size = (None, None)):
        value, = self.GetVariables(element, 'value')
        for i in self.childs:
            if i.IsTrue(element, value):
                i.Paint(canvas, pos, element, size)
                break
