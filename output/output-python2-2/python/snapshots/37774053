from Container import CContainer

class CVBox(CContainer):
    def GetWidth(self, element):
        w = 0
        for i in self.childs:
            w += i.GetWidth(element)
        return w

    def PaintShadow(self, x, y, element, color, w = None, h = None):
        if h is None:
            h = self.GetHeight(element)
        for i in self.childs:
            w = i.GetWidth(element)
            i.PaintShadow(x, y, element, color, w, h)
            x += w

    def Paint(self, x, y, element, w = None, h = None):
        if h is None:
            h = self.GetHeight(element)
        for i in self.childs:
            w = i.GetWidth(element)
            i.Paint(x, y, element, w, h)
            x += w
