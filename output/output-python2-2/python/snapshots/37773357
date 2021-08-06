from VisualObject import CVisualObject
import sys

class CIcon(CVisualObject):
    def __init__(self, filename):
        CVisualObject.__init__(self)
        self.filename = filename

    def GetSize(self, canvas, element):
        filename, = self.GetVariables(element, 'filename')
        return canvas.GetIconSize(filename)

    def Paint(self, canvas, pos, element, size = (None, None)):
        filename, = self.GetVariables(element, 'filename')
        canvas.DrawIcon(pos, filename)
