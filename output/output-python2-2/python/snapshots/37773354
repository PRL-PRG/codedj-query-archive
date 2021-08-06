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
        txt, font = self.GetVariables(element, 'text', 'font')
        return canvas.GetTextSize(txt, font)

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        txt, font = self.GetVariables(element, 'text', 'font')
        canvas.DrawText(pos, txt, font, color)

    def Paint(self, canvas, pos, element, size = (None, None)):
        txt, color, font = self.GetVariables(element, 'text', 'color', 'font')
        canvas.DrawText(pos, txt, font, color)
