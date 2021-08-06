from SimpleContainer import CSimpleContainer

from lib.lib import ToBool

class CCondition(CSimpleContainer):
    def __init__(self, condition, type = "equal", negate = False, value = None):
        CSimpleContainer.__init__(self)
        self.condition = condition
        self.type = type
        self.negate = ToBool(negate)
        self.value = value
    
    def IsTrue(self, context):
        ret = True
        condition, value = self.GetVariables(context, 'condition', 'value')
        if self.type == 'empty':
            ret = not condition
        elif self.type == 'equal':
            ret = unicode(condition) == unicode(value)
        if self.negate:
            return not ret
        return ret

    def ComputeSize(self, context):
        if self.IsTrue(context):
            return CSimpleContainer.ComputeSize(self, context)
        return (0, 0)

    def Paint(self, context):
        if self.IsTrue(context):
            CSimpleContainer.Paint(self, context)
