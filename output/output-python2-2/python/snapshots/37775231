from SimpleContainer import CSimpleContainer

class CPadding(CSimpleContainer):
    def __init__(self, padding):
        CSimpleContainer.__init__(self)
        self.padding = int(padding)

    def ComputeSize(self, context):
        w, h = CSimpleContainer.ComputeSize(self, context)
        return (w + 2*self.padding, h + 2*self.padding)

    def Paint(self, context):
        size = context.ComputeSize(self)
        pos = context.GetPos()
        
        context.Push()
        context.Move((pos[0]+self.padding, pos[1]+self.padding))
        context.Resize((size[0] - 2*self.padding, size[1] - 2*self.padding))
        CSimpleContainer.Paint(self, context)
        context.Pop()
