from VisualObject import CVisualObject
import lib.consts
import sys

class CTextBox(CVisualObject):
    def __init__(self, text, linestart = "", color = "black"):
        CVisualObject.__init__(self)
        self.text = text
        self.linestart = linestart
        self.color = color
    
    def __GetValue(self, element):
        if self.text[0] == '#':
            return element.GetObject().GetVisualProperty(self.text[1:])
        if self.text[0] == '@':
            return element.__LOOPVARS__[self.text[1:]]
        return self.text

    def GetLineStart(self):
        return self.linestart
    
    def GetColor(self):
        return self.color

    def GetHeight(self, canvas, element):
        txt = self.__GetValue(element)
        return canvas.GetTextSize(txt, lib.consts.FONT_TYPE)[1]

    def GetWidth(self, canvas, element):
        txt = self.__GetValue(element)
        return canvas.GetTextSize(txt, lib.consts.FONT_TYPE)[0]

    def PaintShadow(self, pos, canvas, element, color, size = (None, None)):
        txt = self.__GetValue(element)
        canvas.DrawText(pos, txt, lib.consts.FONT_TYPE, color)

    def Paint(self, canvas, pos, element, size = (None, None)):
        txt = self.__GetValue(element)
        canvas.DrawText(pos, txt, lib.consts.FONT_TYPE, self.color)

    def SetLineStart(self, linestart = ""):
        self.linestart = linestart
    
    def SetColor(self, color):
        self.color = color
        self.color_obj = gtk.gdk.color_parse(color)
