from VisualObject import CVisualObject
import sys

class CTextBox(CVisualObject):
    def __init__(self, text, linestart = "", color = "black", font = "Arial 10"):
        CVisualObject.__init__(self)
        self.text = text
        self.linestart = linestart
        self.color = color
        self.font = font

    def GetSize(self, canvas, element):
        size = element.GetCachedSize(self)
        if size is not None:
            return size
        txt, font = self.GetVariables(element, 'text', 'font')
        font = font.split()
        font = font[0], font[1:-1], int(font[-1])
        return element.CacheSize(self, canvas.GetTextSize(txt, font))

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        txt, font = self.GetVariables(element, 'text', 'font')
        font = font.split()
        font = font[0], font[1:-1], int(font[-1])
        canvas.DrawText(pos, txt, font, color)

    def Paint(self, canvas, pos, element, size = (None, None)):
        txt, color, font = self.GetVariables(element, 'text', 'color', 'font')
        font = font.split()
        font = font[0], font[1:-1], int(font[-1])
        canvas.DrawText(pos, txt, font, color)
