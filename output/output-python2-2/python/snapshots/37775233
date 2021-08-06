from VisualObject import CVisualObject
import sys

class CTextBox(CVisualObject):
    def __init__(self, text, linestart = "", color = "black", font = "Arial 10"):
        CVisualObject.__init__(self)
        self.text = text
        self.linestart = linestart
        self.color = color
        self.font = font

    def ComputeSize(self, context):
        txt, font = self.GetVariables(context, 'text', 'font')
        font = font.split()
        font = font[0], font[1:-1], int(font[-1])
        return context.GetCanvas().GetTextSize(txt, font)

    def Paint(self, context):
        txt, color, font = self.GetVariables(context, 'text', 'color', 'font')
        shadowcolor = context.GetShadowColor()
        if shadowcolor is not None:
            color = shadowcolor
        
        font = font.split()
        font = font[0], font[1:-1], int(font[-1])
        context.GetCanvas().DrawText(context.GetPos(), txt, font, color)
