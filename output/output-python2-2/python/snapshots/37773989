from Container import CContainer

class CHBox(CContainer):
    def GetHeight(self, canvas, element):
        h = 0
        for i in self.childs:
            h += i.GetHeight(canvas, element)
        return h

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        if size[0] is None:
            w = self.GetWidth(canvas, element)
        else:
            w = size[0]
        x, y = pos
        for i in self.childs:
            h = i.GetHeight(canvas, element)
            i.PaintShadow(canvas, (x, y), element, color, (w, h))
            y += h

    def Paint(self, canvas, pos, element, size = (None, None)):
        if size[0] is None:
            w = self.GetWidth(canvas, element)
        else:
            w = size[0]
        x, y = pos
        for i in self.childs:
            h = i.GetHeight(canvas, element)
            i.Paint(canvas, (x, y), element, (w, h))
            y += h
