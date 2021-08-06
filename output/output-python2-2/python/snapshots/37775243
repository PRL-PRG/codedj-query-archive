from SimpleContainer import CSimpleContainer
from HBox import CHBox
from VBox import CVBox
from lib.Exceptions.UserException import *

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
            raise XMLError("Orientation.")
    
    def ComputeSize(self, context):
        w, h = 0, 0
        o = self.__GetOrientation()
        for line, item in enumerate(context.GetAttribute(self.collection)):
            for i in self.childs:
                context.Push()
                context.SetVariables({'item': item, 'line': line})
                wc, hc = i.GetSize(context)
                context.Pop()
                
                if o == "horizontal":
                    if wc > w:
                        w = wc
                    h += hc
                else:
                    w += wc
                    if hc > h:
                        h = hc
        return (w, h)

    def Paint(self, context):
        size = context.ComputeSize(self)
        w, h = context.GetSize()
        x, y = context.GetPos()
        o = self.__GetOrientation()
        for line, item in enumerate(context.GetAttribute(self.collection)):
            for i in self.childs:
                context.Push()
                context.SetVariables({'item': item, 'line': line})
                wc, hc = i.GetSize(context)
                
                if o == "horizontal":
                    h = hc
                else:
                    w = wc
                
                context.Move((x, y))
                context.Resize((w, h))
                i.Paint(context)
                
                if o == "horizontal":
                    y += h
                else:
                    x += w
                
                context.Pop()
