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
    
    def IsTrue(self, context, value):
        ret = True
        condition, = self.GetVariables(context, 'condition')
        
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
    
    def ComputeSize(self, context):
        value, = self.GetVariables(context, 'value')
        
        for i in self.childs:
            if i.IsTrue(context, value):
                return i.GetSize(context)
        
        return (0, 0)

    def Paint(self, context):
        value, = self.GetVariables(context, 'value')
        
        for i in self.childs:
            if i.IsTrue(context, value):
                i.Paint(context)
                break
