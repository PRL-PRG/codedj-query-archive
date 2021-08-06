from VisualObject import CVisualObject
import sys

class CIcon(CVisualObject):
    def __init__(self, filename):
        CVisualObject.__init__(self)
        self.filename = filename

    def GetHeight(self, canvas, element):
        filename, = self.GetVariables(element, 'filename')
        return canvas.GetIconSize(filename)[1]

    def GetWidth(self, canvas, element):
        filename, = self.GetVariables(element, 'filename')
        return canvas.GetIconSize(filename)[0]

    def Paint(self, canvas, pos, element, size = (None, None)):
        filename, = self.GetVariables(element, 'filename')
        canvas.DrawIcon(pos, filename)
