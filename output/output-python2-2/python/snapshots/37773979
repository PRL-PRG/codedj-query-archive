from SimpleContainer import CSimpleContainer
from HBox import CHBox
from VBox import CVBox

class CLoop(CSimpleContainer):
    def __init__(self, collection):
        CSimpleContainer.__init__(self)
        self.collection = collection

    def GetCollection(self):
        return self.collection
    
    def __GetOrientation(self):
        parent = self.GetParent()
        if isinstance(parent, CHBox):
            return "horizontal"
        elif isinstance(parent, CVBox):
            return "vertical"
        else:
            raise UMLException("XMLError")
    
    def GetWidth(self, canvas, element):
        w = 0
        o = self.__GetOrientation()
        for item in element.GetObject().GetVisualProperty(self.collection):
            for i in self.childs:
                element.__LOOPVARS__ = item
                v = i.GetWidth(canvas, element)
                if o == "horizontal":
                    if v > w:
                        w = v
                else:
                    w += v
                del element.__LOOPVARS__
        return w
    
    def GetHeight(self, canvas, element):
        h = 0
        o = self.__GetOrientation()
        for item in element.GetObject().GetVisualProperty(self.collection):
            for i in self.childs:
                element.__LOOPVARS__ = item
                v = i.GetHeight(canvas, element)
                if o == "horizontal":
                    h += v
                else:
                    if v > h:
                        h = v
                del element.__LOOPVARS__
        return h

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        w, h = size
        x, y = pos
        o = self.__GetOrientation()
        for item in element.GetObject().GetVisualProperty(self.collection):
            for i in self.childs:
                element.__LOOPVARS__ = item
                if o == "horizontal":
                    h = i.GetHeight(canvas, element)
                else:
                    w = i.GetWidth(canvas, element)
                i.PaintShadow(canvas, (x, y), element, color, (w, h))
                if o == "horizontal":
                    y += h
                else:
                    x += w
                del element.__LOOPVARS__

    def Paint(self, canvas, pos, element, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        w, h = size
        x, y = pos
        o = self.__GetOrientation()
        for item in element.GetObject().GetVisualProperty(self.collection):
            for i in self.childs:
                element.__LOOPVARS__ = item
                if o == "horizontal":
                    h = i.GetHeight(canvas, element)
                else:
                    w = i.GetWidth(canvas, element)
                i.Paint(canvas, (x, y), element, (w, h))
                if o == "horizontal":
                    y += h
                else:
                    x += w
                del element.__LOOPVARS__

    def SetCollection(self, collection):
        self.collection = collection
