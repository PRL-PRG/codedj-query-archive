from VisualObject import CVisualObject

class CContainer(CVisualObject):
    def __init__(self):
        CVisualObject.__init__(self)
        self.childs = []

    def AppendChild(self, child):
        self.childs.append(child)
        child.SetParent(self)

    def GetChild(self, index):
        return self.childs[index]

    def GetChilds(self):
        return self.childs

    def GetHeight(self, canvas, element):
        h = 0
        for i in self.childs:
            v = i.GetHeight(canvas, element)
            if h < v:
                h = v
        return h

    def GetWidth(self, canvas, element):
        w = 0
        for i in self.childs:
            v = i.GetWidth(canvas, element)
            if w < v:
                w = v
        return w
    
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
