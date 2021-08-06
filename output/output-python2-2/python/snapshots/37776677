from SimpleContainer import CSimpleContainer
from HBox import CHBox
from VBox import CVBox
from lib.lib import UMLException

class CLoop(CSimpleContainer):
    def __init__(self, collection):
        CSimpleContainer.__init__(self)
        self.collection = collection
    
    def __GetOrientation(self):
        parent = self.GetParent()
        if isinstance(parent, CHBox):
            return "horizontal"
        elif isinstance(parent, CVBox):
            return "vertical"
        else:
            raise UMLException("XMLError")
    
    def GetSize(self, canvas, element):
        size = element.GetCachedSize(self)
        if size is not None:
            return size
        w, h = 0, 0
        o = self.__GetOrientation()
        for line, item in enumerate(element.GetObject().GetVisualProperty(self.collection)):
            for i in self.childs:
                element.__LOOPVARS__ = {'item': item, 'line': line}
                wc, hc = i.GetSize(canvas, element)
                if o == "horizontal":
                    if wc > w:
                        w = wc
                    h += hc
                else:
                    w += wc
                    if hc > h:
                        h = hc
                del element.__LOOPVARS__
        return element.CacheSize(self, (w, h))

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        w, h = size
        x, y = pos
        o = self.__GetOrientation()
        for line, item in enumerate(element.GetObject().GetVisualProperty(self.collection)):
            for i in self.childs:
                element.__LOOPVARS__ = {'item': item, 'line': line}
                wc, hc = i.GetSize(canvas, element)
                if o == "horizontal":
                    h = hc
                else:
                    w = wc
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
        for line, item in enumerate(element.GetObject().GetVisualProperty(self.collection)):
            for i in self.childs:
                element.__LOOPVARS__ = {'item': item, 'line': line}
                wc, hc = i.GetSize(canvas, element)
                if o == "horizontal":
                    h = hc
                else:
                    w = wc
                i.Paint(canvas, (x, y), element, (w, h))
                if o == "horizontal":
                    y += h
                else:
                    x += w
                del element.__LOOPVARS__
