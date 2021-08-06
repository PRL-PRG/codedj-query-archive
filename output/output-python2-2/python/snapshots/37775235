from Container import CContainer

class CVBox(CContainer):
    def __init__(self, expand=""):
        CContainer.__init__(self)
        self.expand = tuple(int(cell) for cell in expand.split())
    
    def ComputeSize(self, context):
        w = 0
        h = 0
        for i in self.childs:
            wi, hi = i.GetSize(context)
            w += wi
            h = max(h, hi)
        return (w, h)

    def Paint(self, context):
        x, y = context.GetPos()
        we, he = context.GetSize()
        if he is None:
            h = 0
        else:
            h = he
        
        w = []
        for i in self.childs:
            wi, hi = i.GetSize(context)
            w.append(wi)
            if he is None:
                h = max(h, hi)
        
        if we is not None and self.expand:
            ws = size[0] - sum(w)
            if ws > 0:
                wx = ws / len(self.expand)
                for i in self.expand:
                    w[i] += wx
                    ws -= wx
                if ws > 0:
                    w[self.expand[-1]] += ws
        
        
        for id, i in enumerate(self.childs):
            context.Push()
            context.Move((x, y))
            context.Resize((w[id], h))
            i.Paint(context)
            context.Pop()
            x += w[id]
