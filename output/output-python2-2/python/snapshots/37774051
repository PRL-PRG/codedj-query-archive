from VisualObject import CVisualObject
import gtk.gdk
import lib.consts
import sys
import pango

class CTextBox(CVisualObject):
    def __init__(self, text, align = "center", linestart = "", color = "black"):
        CVisualObject.__init__(self)
        self.text = text
        self.align = align
        self.linestart = linestart
        self.color = color
    
    def __GetValue(self, element):
        if self.text[0] == '#':
            return element.GetObject().GetVisualProperty(self.text[1:])
        if self.text[0] == '@':
            return element.__LOOPVARS__[self.text[1:]]
        return self.text

    def GetAlign(self):
        return self.align

    def GetLineStart(self):
        return self.linestart
    
    def GetColor(self):
        return self.color

    def GetHeight(self, element):
        txt = self.__GetValue(element)
        ctx, layout = element.GetDrawingArea().GetPango()
        layout.set_text(txt)
        return int(layout.get_size()[1]/float(pango.SCALE))

    def GetWidth(self, element):
        txt = self.__GetValue(element)
        ctx, layout = element.GetDrawingArea().GetPango()
        layout.set_text(txt)
        return int(layout.get_size()[0]/float(pango.SCALE))

    def PaintShadow(self, x, y, element, color, w = None, h = None):
        txt = self.__GetValue(element)
        wgt = element.GetDrawingArea().GetDrawable()
        cmap = wgt.get_colormap()
        gc = wgt.new_gc(foreground = cmap.alloc_color(color))
        
        ctx, layout = element.GetDrawingArea().GetPango()
        layout.set_text(txt)
        wgt.draw_layout(gc, x=x, y=y, layout=layout)

    def Paint(self, x, y, element, w = None, h = None):
        txt = self.__GetValue(element)
        wgt = element.GetDrawingArea().GetDrawable()
        cmap = wgt.get_colormap()
        gc = wgt.new_gc(foreground = cmap.alloc_color(self.color))
        
        ctx, layout = element.GetDrawingArea().GetPango()
        layout.set_text(txt)
        wgt.draw_layout(gc, x=x, y=y, layout=layout)

    def SetAlign(self, align = "center"):
        self.align = align

    def SetLineStart(self, linestart = ""):
        self.linestart = linestart
    
    def SetColor(self, color):
        self.color = color
        self.color_obj = gtk.gdk.color_parse(color)
