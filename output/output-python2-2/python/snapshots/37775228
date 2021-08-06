from SimpleContainer import CSimpleContainer
from lib.Exceptions.UserException import *

class CShadow(CSimpleContainer):
    def __init__(self, padding, color):
        CSimpleContainer.__init__(self)
        self.padding = int(padding)
        self.color = color

    def Paint(self, context):
        if context.GetShadowColor() is not None:
            raise DrawingError("ShadowInShadow")
        size = context.ComputeSize(self)
        pos = context.GetPos()
        color, = self.GetVariables(context, 'color')
        
        context.Push()
        context.SetShadowColor(color)
        context.Move((pos[0] + self.padding, pos[1] + self.padding))
        CSimpleContainer.Paint(self, context)
        context.Pop()
        
        CSimpleContainer.Paint(self, context)
