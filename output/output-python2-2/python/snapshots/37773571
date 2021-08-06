from SimpleContainer import CSimpleContainer
from lib.lib import UMLException

class CSizer(CSimpleContainer):
    def __init__(self, minwidth = None, minheight = None, maxwidth = None, maxheight = None, width = None, height = None):
        CSimpleContainer.__init__(self)
        
        if (minwidth is not None or maxwidth is not None) and width is not None:
            raise UMLException("XMLError", ("sizer", "width"))
        if (minheight is not None or maxheight is not None) and height is not None:
            raise UMLException("XMLError", ("sizer", "height"))
        
        if minwidth is None:
            self.minwidth = None
        else:
            self.minwidth = int(minwidth)
        if maxwidth is None:
            self.maxwidth = None
        else:
            self.maxwidth = int(maxwidth)
        if minheight is None:
            self.minheight = None
        else:
            self.minheight = int(minheight)
        if maxheight is None:
            self.maxheight = None
        else:
            self.maxheight = int(maxheight)
        if width is None:
            self.width = None
        else:
            self.width = int(width)
        if height is None:
            self.height = None
        else:
            self.height = int(height)

    def GetHeight(self, canvas, element):
        if self.height is not None:
            return None
        h = CSimpleContainer.GetHeight(self, canvas, element)
        if self.minheight is not None and self.minheight > h:
            h = self.minheight
        if self.maxheight is not None and self.maxheight < h:
            h = self.maxheight
        return h

    def GetWidth(self, canvas, element):
        if self.width is not None:
            return None
        w = CSimpleContainer.GetWidth(self, canvas, element)
        if self.minwidth is not None and self.minwidth > w:
            w = self.minwidth
        if self.maxwidth is not None and self.maxwidth < w:
            w = self.maxwidth
        return w

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        CSimpleContainer.PaintShadow(self, canvas, (pos[0], pos[1]), element, color, (size[0], size[1]))

    def Paint(self, canvas, pos, element, size = (None, None)):
        size = self.ComputeSize(canvas, element, size)
        CSimpleContainer.Paint(self, canvas, (pos[0], pos[1]), element, (size[0], size[1]))
