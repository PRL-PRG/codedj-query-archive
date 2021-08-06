from Container import CContainer

class CVBox(CContainer):
    def __init__(self, expand=""):
        CContainer.__init__(self)
        self.expand = tuple(int(cell) for cell in expand.split())
    
    def GetSize(self, canvas, element):
        size = element.GetCachedSize(self)
        if size is not None:
            return size
        w = 0
        h = 0
        for i in self.childs:
            wi, hi = i.GetSize(canvas, element)
            w += wi
            h = max(h, hi)
        return element.CacheSize(self, (w, h))

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        if size[1] is None:
            h = 0
        else:
            h = size[1]
        w = []
        for i in self.childs:
            wi, hi = i.GetSize(canvas, element)
            w.append(wi)
            if size[1] is None:
                h = max(h, hi)
        if size[0] is not None and self.expand:
            ws = size[0] - sum(w)
            if ws > 0:
                wx = ws / len(self.expand)
                for i in self.expand:
                    w[i] += wx
                    ws -= wx
                if ws > 0:
                    w[self.expand[-1]] += ws
        x, y = pos
        for id, i in enumerate(self.childs):
            i.PaintShadow(canvas, (x, y), element, color, (w[id], h))
            x += w[id]

    def Paint(self, canvas, pos, element, size = (None, None)):
        if size[1] is None:
            h = 0
        else:
            h = size[1]
        w = []
        for i in self.childs:
            wi, hi = i.GetSize(canvas, element)
            w.append(wi)
            if size[1] is None:
                h = max(h, hi)
        if size[0] is not None and self.expand:
            ws = size[0] - sum(w)
            if ws > 0:
                wx = ws / len(self.expand)
                for i in self.expand:
                    w[i] += wx
                    ws -= wx
                if ws > 0:
                    w[self.expand[-1]] += ws
        x, y = pos
        for id, i in enumerate(self.childs):
            i.Paint(canvas, (x, y), element, (w[id], h))
            x += w[id]
