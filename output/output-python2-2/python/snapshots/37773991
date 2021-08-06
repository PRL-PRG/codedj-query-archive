from SimpleContainer import CSimpleContainer

class CRectangle(CSimpleContainer):
    def __init__(self, fill = None, border = "white", borderwidth = 1):
        CSimpleContainer.__init__(self)
        self.fill = fill
        self.border = border
        
        self.borderwidth = int(borderwidth)

    def GetBorder(self):
        return self.border

    def GetBorderWidth(self):
        return self.borderwidth

    def GetFill(self):
        return self.fill

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        canvas.DrawRectangle(pos, size, None, color)

    def Paint(self, canvas, pos, element, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        canvas.DrawRectangle(pos, size, self.border, self.fill)
        
        for i in self.childs:
            i.Paint(canvas, pos, element, size)

    def SetBorder(self, border):
        self.border = border
        self.border_obj = gtk.gdk.color_parse(border)

    def SetBorderWidth(self, width = 1):
        self.borderwidth = width

    def SetFill(self, fill = None):
        self.fill = fill
        self.fill_obj = gtk.gdk.color_parse(fill)
