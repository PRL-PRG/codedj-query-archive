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

    def GetHeight(self, element):
        h = 0
        for i in self.childs:
            v = i.GetHeight(element)
            if h < v:
                h = v
        return h

    def GetWidth(self, element):
        w = 0
        for i in self.childs:
            v = i.GetWidth(element)
            if w < v:
                w = v
        return w
    
    def PaintShadow(self, x, y, element, color, w = None, h = None):
        if w is None:
            w = self.GetWidth(element)
        if h is None:
            h = self.GetHeight(element)
        for i in self.childs:
            i.PaintShadow(x, y, element, color, w, h)

    def Paint(self, x, y, element, w = None, h = None):
        if w is None:
            w = self.GetWidth(element)
        if h is None:
            h = self.GetHeight(element)
        for i in self.childs:
            i.Paint(x, y, element, w, h)

    def RemoveChild(self, child):
        self.childs.remove(child)
