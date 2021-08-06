from VisualObject import CVisualObject
from VBox import CVBox
import gtk.gdk

class CLine(CVisualObject):
    def __init__(self, type = "auto", color = "black"):
        CVisualObject.__init__(self)
        self.type = type
        self.color = color

    def GetType(self):
        return self.type

    def ComputeType(self):
        tp = self.type
        if tp == 'auto':
            if isinstance(self.parent, CVBox):
                tp = 'vertical'
            else:
                tp = 'horizontal'
        return tp
    
    def GetColor(self):
        return self.color

    def GetHeight(self, element):
        tp = self.ComputeType()
        if tp == 'horizontal':
            return 1
        else:
            return 0

    def GetWidth(self, element):
        tp = self.ComputeType()
        if tp == 'vertical':
            return 1
        else:
            return 0

    def PaintShadow(self, x, y, element, color, w = None, h = None):
        tp = self.ComputeType()
        wgt = element.GetDrawingArea().GetDrawable()
        cmap = wgt.get_colormap()
        gc = wgt.new_gc(foreground = cmap.alloc_color(color))
        if tp == 'horizontal' and w is not None:
            wgt.draw_line(gc, x, y, x+w, y)
        elif tp == 'vertical' and h is not None:
            wgt.draw_line(gc, x, y, x, y+h)

    def Paint(self, x, y, element, w = None, h = None):
        tp = self.ComputeType()
        wgt = element.GetDrawingArea().GetDrawable()
        cmap = wgt.get_colormap()
        gc = wgt.new_gc(foreground = cmap.alloc_color(self.color))
        if tp == 'horizontal' and w is not None:
            wgt.draw_line(gc, x, y, x+w, y)
        elif tp == 'vertical' and h is not None:
            wgt.draw_line(gc, x, y, x, y+h)

    def SetType(self, type = "auto"):
        self.type = type
    
    def SetColor(self, color):
        self.color = color
