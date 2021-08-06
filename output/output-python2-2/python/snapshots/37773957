from SimpleContainer import CSimpleContainer

class CAlign(CSimpleContainer):
    def __init__(self, align):
        CSimpleContainer.__init__(self)
        self.align = align
        self.alignx = "left"
        self.aligny = "top"
        align = align.split()
        if len(align) > 2:
            raise UMLException("XMLError", ("Align", "align"))
        if align == ['center', 'center']:
            self.alignx, self.aligny = align
        elif 'center' in align:
            if align[0] == 'center':
                self.alignx = 'center'
                del align[0]
            else:
                self.aligny = 'center'
                del align[1]
        for i in align:
            if i in ('left', 'right'):
                self.alignx = i
            elif i in ('bottom', 'top'):
                self.aligny = i
            elif i == 'middle':
                self.aligny = 'center'

    def GetAlignX(self):
        return self.alignx
    
    def GetAlignY(self):
        return self.aligny
    
    def SetAlignX(self, align):
        self.alignx = align
    
    def SetAlignY(self, align):
        self.aligny = align

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        x, y = pos
        w, h = size
        if w is None:
            w = self.GetWidth(canvas, element)
        else:
            if self.alignx == "center":
                x += (w - self.GetChild().GetWidth(canvas, element))/2
            elif self.alignx == "right":
                x += w - self.GetChild().GetWidth(canvas, element)
        if h is None:
            w = self.GetHeight(canvas, element)
        else:
            if self.aligny == "center":
                y += (h - self.GetChild().GetHeight(canvas, element))/2
            elif self.aligny == "bottom":
                y += h - self.GetChild().GetHeight(canvas, element)
        self.GetChild().PaintShadow(canvas, (x, y), element, color, (w, h))

    def Paint(self, canvas, pos, element, size = (None, None)):
        x, y = pos
        w, h = size
        if w is None:
            w = self.GetWidth(canvas, element)
        else:
            if self.alignx == "center":
                x += (w - self.GetChild().GetWidth(canvas, element))/2
            elif self.alignx == "right":
                x += w - self.GetChild().GetWidth(canvas, element)
        if h is None:
            w = self.GetHeight(canvas, element)
        else:
            if self.aligny == "center":
                y += (h - self.GetChild().GetHeight(canvas, element))/2
            elif self.aligny == "bottom":
                y += h - self.GetChild().GetHeight(canvas, element)
        self.GetChild().Paint(canvas, (x, y), element, (w, h))
