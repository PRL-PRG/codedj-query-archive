from VisualObject import CVisualObject

class CContainer(CVisualObject):
    def __init__(self):
        CVisualObject.__init__(self)
        self.childs = []

    def AppendChild(self, child):
        self.childs.append(child)
        child.SetParent(self)
    
    def GetResizable(self):
        rx, ry = False, False
        for i in self.childs:
            rcx, rcy = i.GetResizable()
            rx = rx or rcx
            ry = ry or rcy
            if rx == ry == True:
                return rx, ry
        return rx, ry

    def GetChild(self, index):
        return self.childs[index]

    def GetChilds(self):
        return self.childs

    def GetSize(self, canvas, element):
        size = element.GetCachedSize(self)
        if size is not None:
            return size
        w = 0
        h = 0
        for i in self.childs:
            wc, hc = i.GetSize(canvas, element)
            w = max(w, wc)
            h = max(h, hc)
        return element.CacheSize(self,(w, h))
    
    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        for i in self.childs:
            i.PaintShadow(canvas, pos, element, color, size)

    def Paint(self, canvas, pos, element, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        for i in self.childs:
            i.Paint(canvas, pos, element, size)

    def RemoveChild(self, child):
        self.childs.remove(child)
