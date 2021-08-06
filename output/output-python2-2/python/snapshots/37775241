from Container import CContainer

class CHBox(CContainer):
    def __init__(self, expand=""):
        CContainer.__init__(self)
        self.expand = tuple(int(cell) for cell in expand.split())
    
    def ComputeSize(self, context):
        w = 0
        h = 0
        for i in self.childs:
            wi, hi = i.GetSize(context)
            w = max(w, wi)
            h += hi
        return (w, h)

    def Paint(self, context):
        x, y = context.GetPos()
        we, he = context.GetSize()
        if we is None:
            w = 0
        else:
            w = we
        
        h = []
        for i in self.childs:
            wi, hi = i.GetSize(context)
            if we is None:
                w = max(w, wi)
            h.append(hi)
        
        if he is not None and self.expand:
            hs = he - sum(h)
            if hs > 0:
                hx = hs / len(self.expand)
                for i in self.expand:
                    h[i] += hx
                    hs -= hx
                if hs > 0:
                    h[self.expand[-1]] += hs
        
        for id, i in enumerate(self.childs):
            context.Push()
            context.Move((x, y))
            context.Resize((w, h[id]))
            i.Paint(context)
            context.Pop()
            y += h[id]
