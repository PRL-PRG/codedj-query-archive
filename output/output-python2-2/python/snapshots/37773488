from VisualObject import CVisualObject
import sys

class CIcon(CVisualObject):
    def __init__(self, filename):
        CVisualObject.__init__(self)
        self.filename = filename
    
    def __GetValue(self, element):
        if self.filename[0] == '#':
            return element.GetObject().GetVisualProperty(self.filename[1:])
        if self.filename[0] == '@':
            return element.__LOOPVARS__[self.filename[1:]]
        return self.filename

    def GetHeight(self, canvas, element):
        filename = self.__GetValue(element)
        return canvas.GetIconSize(filename)[1]

    def GetWidth(self, canvas, element):
        filename = self.__GetValue(element)
        return canvas.GetIconSize(filename)[0]

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        filename = self.__GetValue(element)
        canvas.DrawIcon(pos, filename)

    def Paint(self, canvas, pos, element, size = (None, None)):
        filename = self.__GetValue(element)
        canvas.DrawIcon(pos, filename)
