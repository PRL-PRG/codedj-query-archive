from VisualObject import CVisualObject
import sys

class CIcon(CVisualObject):
    def __init__(self, filename):
        CVisualObject.__init__(self)
        self.filename = filename

    def GetSize(self, canvas, element):
        size = element.GetCachedSize(self)
        if size is not None:
            return size
        filename, = self.GetVariables(element, 'filename')
        return element.CacheSize(self, canvas.GetIconSize(filename))

    def Paint(self, canvas, pos, element, size = (None, None)):
        filename, = self.GetVariables(element, 'filename')
        canvas.DrawIcon(pos, filename)
