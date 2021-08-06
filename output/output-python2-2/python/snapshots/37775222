from SimpleContainer import CSimpleContainer

class CEllipse(CSimpleContainer):
    def __init__(self, fill = None, border = "white", borderwidth = 1):
        CSimpleContainer.__init__(self)
        self.fill = fill
        self.border = border
        
        self.borderwidth = int(borderwidth)
    
    def GetResizable(self):
        return True, True

    def Paint(self, context):
        size = context.ComputeSize(self)
        shadowcolor = context.GetShadowColor()
        if shadowcolor is None:
            border, fill = self.GetVariables(context, 'border', 'fill')
        else:
            border, fill = None, shadowcolor
        
        context.GetCanvas().DrawArc(context.GetPos(), context.GetSize(), (0, 360), border, fill)
        
        if shadowcolor:
            return
        
        for i in self.childs:
            i.Paint(context)
