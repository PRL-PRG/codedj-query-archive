from VisualObject import CVisualObject
import sys

class CTextBox(CVisualObject):
    def __init__(self, text, linestart = "", color = "black", font = "Arial 10"):
        CVisualObject.__init__(self)
        self.text = text
        self.linestart = linestart
        self.color = color
        self.font = font
    
    def __GetValue(self, element):
        if self.text[0] == '#':
            return element.GetObject().GetVisualProperty(self.text[1:])
        if self.text[0] == '@':
            return element.__LOOPVARS__[self.text[1:]]
        return self.text

    def GetHeight(self, canvas, element):
        txt = self.__GetValue(element)
        return canvas.GetTextSize(txt, self.font)[1]

    def GetWidth(self, canvas, element):
        txt = self.__GetValue(element)
        return canvas.GetTextSize(txt, self.font)[0]

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        txt = self.__GetValue(element)
        canvas.DrawText(pos, txt, self.font, color)

    def Paint(self, canvas, pos, element, size = (None, None)):
        txt = self.__GetValue(element)
        canvas.DrawText(pos, txt, self.font, self.color)
