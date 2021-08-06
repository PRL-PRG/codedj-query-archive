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
    
    def GetWidth(self, element):
        w = 0
        o = self.__GetOrientation()
        for item in element.GetObject().GetVisualProperty(self.collection):
            for i in self.childs:
                element.__LOOPVARS__ = item
                v = i.GetWidth(element)
                if o == "horizontal":
                    if v > w:
                        w = v
                else:
                    w += v
                del element.__LOOPVARS__
        return w
    
    def GetHeight(self, element):
        h = 0
        o = self.__GetOrientation()
        for item in element.GetObject().GetVisualProperty(self.collection):
            for i in self.childs:
                element.__LOOPVARS__ = item
                v = i.GetHeight(element)
                if o == "horizontal":
                    h += v
                else:
                    if v > h:
                        h = v
                del element.__LOOPVARS__
        return h

    def PaintShadow(self, x, y, element, color, w = None, h = None):
        if w is None:
            w = self.GetWidth(element)
        if h is None:
            h = self.GetHeight(element)
        o = self.__GetOrientation()
        for item in element.GetObject().GetVisualProperty(self.collection):
            for i in self.childs:
                element.__LOOPVARS__ = item
                h = i.GetHeight(element)
                i.PaintShadow(x, y, element, color, w, h)
                if o == "horizontal":
                    y += h
                else:
                    x += w
                del element.__LOOPVARS__

    def Paint(self, x, y, element, w = None, h = None):
        if w is None:
            w = self.GetWidth(element)
        if h is None:
            h = self.GetHeight(element)
        o = self.__GetOrientation()
        for item in element.GetObject().GetVisualProperty(self.collection):
            for i in self.childs:
                element.__LOOPVARS__ = item
                h = i.GetHeight(element)
                i.Paint(x, y, element, w, h)
                if o == "horizontal":
                    y += h
                else:
                    x += w
                del element.__LOOPVARS__

    def SetCollection(self, collection):
        self.collection = collection
