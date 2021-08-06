from Container import CContainer

class CVBox(CContainer):
    def GetWidth(self, canvas, element):
        w = 0
        for i in self.childs:
            w += i.GetWidth(canvas, element)
        return w

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        if size[1] is None:
            h = self.GetHeight(canvas, element)
        else:
            h = size[1]
        x, y = pos
        for i in self.childs:
            w = i.GetWidth(canvas, element)
            i.PaintShadow(canvas, (x, y), element, color, (w, h))
            x += w

    def Paint(self, canvas, pos, element, size = (None, None)):
        if size[1] is None:
            h = self.GetHeight(canvas, element)
        else:
            h = size[1]
        x, y = pos
        for i in self.childs:
            w = i.GetWidth(canvas, element)
            i.Paint(canvas, (x, y), element, (w, h))
            x += w
