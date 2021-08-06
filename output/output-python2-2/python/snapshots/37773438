from Container import CContainer

class CHBox(CContainer):
    def __init__(self, expand=""):
        CContainer.__init__(self)
        self.expand = tuple(int(cell) for cell in expand.split())
    
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
        h = []
        for i in self.childs:
            h.append(i.GetHeight(canvas, element))
        if size[1] is not None and self.expand:
            hs = size[1] - sum(h)
            if hs > 0:
                hx = hs / len(self.expand)
                for i in self.expand:
                    h[i] += hx
                    hs -= hx
                if hs > 0:
                    h[self.expand[-1]] += hs
        x, y = pos
        for id, i in enumerate(self.childs):
            i.PaintShadow(canvas, (x, y), element, color, (w, h[id]))
            y += h[id]

    def Paint(self, canvas, pos, element, size = (None, None)):
        if size[0] is None:
            w = self.GetWidth(canvas, element)
        else:
            w = size[0]
        h = []
        for i in self.childs:
            h.append(i.GetHeight(canvas, element))
        if size[1] is not None and self.expand:
            hs = size[1] - sum(h)
            if hs > 0:
                hx = hs / len(self.expand)
                for i in self.expand:
                    h[i] += hx
                    hs -= hx
                if hs > 0:
                    h[self.expand[-1]] += hs
        x, y = pos
        for id, i in enumerate(self.childs):
            i.Paint(canvas, (x, y), element, (w, h[id]))
            y += h[id]
