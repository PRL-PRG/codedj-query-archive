from SimpleContainer import CSimpleContainer
from lib.lib import UMLException

class CAlign(CSimpleContainer):
    def __init__(self, align):
        CSimpleContainer.__init__(self)
        self.alignx = "left"
        self.aligny = "top"
        align = align.split()
        if len(align) > 2:
            raise UMLException("XMLError", ("Align", "align"))
        if align == ['center', 'center']:
            self.alignx, self.aligny = align
        elif 'center' in align:
            if align[0] == 'center':
                self.alignx = 'center'
                del align[0]
            else:
                self.aligny = 'center'
                del align[1]
        for i in align:
            if i in ('left', 'right'):
                self.alignx = i
            elif i in ('bottom', 'top'):
                self.aligny = i
            elif i == 'middle':
                self.aligny = 'center'

    def PaintShadow(self, canvas, pos, element, color, size = (None, None)):
        x, y = pos
        w, h = size
        alignx, aligny = self.GetVariables(element, 'alignx', 'aligny')
        if w is None:
            w = self.GetWidth(canvas, element)
        else:
            if alignx == "center":
                x += (w - self.GetChild().GetWidth(canvas, element))/2
            elif alignx == "right":
                x += w - self.GetChild().GetWidth(canvas, element)
        if h is None:
            w = self.GetHeight(canvas, element)
        else:
            if aligny == "center":
                y += (h - self.GetChild().GetHeight(canvas, element))/2
            elif aligny == "bottom":
                y += h - self.GetChild().GetHeight(canvas, element)
        self.GetChild().PaintShadow(canvas, (x, y), element, color, (w, h))

    def Paint(self, canvas, pos, element, size = (None, None)):
        x, y = pos
        w, h = size
        alignx, aligny = self.GetVariables(element, 'alignx', 'aligny')
        if w is None:
            w = self.GetWidth(canvas, element)
        else:
            if alignx == "center":
                x += (w - self.GetChild().GetWidth(canvas, element))/2
            elif alignx == "right":
                x += w - self.GetChild().GetWidth(canvas, element)
        if h is None:
            w = self.GetHeight(canvas, element)
        else:
            if aligny == "center":
                y += (h - self.GetChild().GetHeight(canvas, element))/2
            elif aligny == "bottom":
                y += h - self.GetChild().GetHeight(canvas, element)
        self.GetChild().Paint(canvas, (x, y), element, (w, h))
