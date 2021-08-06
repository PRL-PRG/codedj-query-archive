from SimpleContainer import CSimpleContainer

class CEllipse(CSimpleContainer):
    def __init__(self, fill = None, border = "white", borderwidth = 1):
        CSimpleContainer.__init__(self)
        self.fill = fill
        self.border = border
        
        self.borderwidth = int(borderwidth)

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        canvas.DrawArc(pos, size, (0, 360), None, color)

    def Paint(self, canvas, pos, element, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        canvas.DrawArc(pos, size, (0, 360), self.border, self.fill)
        
        for i in self.childs:
            i.Paint(canvas, pos, element, size)
