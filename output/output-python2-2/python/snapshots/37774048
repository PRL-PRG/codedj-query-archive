from SimpleContainer import CSimpleContainer

import gtk.gdk

class CEllipse(CSimpleContainer):
    def __init__(self, fill = None, border = "white", borderwidth = 1):
        CSimpleContainer.__init__(self)
        self.fill = fill
        self.border = border
        
        self.fill_obj = gtk.gdk.color_parse(fill)
        self.border_obj = gtk.gdk.color_parse(border)
        
        self.borderwidth = int(borderwidth)

    def GetBorder(self):
        return self.border

    def GetBorderWidth(self):
        return self.borderwidth

    def GetFill(self):
        return self.fill

    def PaintShadow(self, x, y, element, color, w = None, h = None):
        wgt = element.GetDrawingArea().GetDrawable()
        if w is None:
            w = self.GetWidth(element)
        if h is None:
            h = self.GetHeight(element)
        gc = wgt.new_gc()
        cmap = wgt.get_colormap()
        gc.foreground = cmap.alloc_color(color)
        wgt.draw_arc(gc, True, x, y, w, h, 0, 360*64)

    def Paint(self, x, y, element, w = None, h = None):
        wgt = element.GetDrawingArea().GetDrawable()
        if w is None:
            w = self.GetWidth(element)
        if h is None:
            h = self.GetHeight(element)
        gc = wgt.new_gc()
        cmap = wgt.get_colormap()
        if self.fill is not None:
            gc.foreground = cmap.alloc_color(self.fill)
            wgt.draw_arc(gc, True, x, y, w, h, 0, 360*64)
        gc.foreground = cmap.alloc_color(self.border)
        wgt.draw_arc(gc, False, x, y, w, h, 0, 360*64)
        for i in self.childs:
            i.Paint(x, y, element, w, h)

    def SetBorder(self, border):
        self.border = border
        self.border_obj = gtk.gdk.color_parse(border)

    def SetBorderWidth(self, width = 1):
        self.borderwidth = width

    def SetFill(self, fill = None):
        self.fill = fill
        self.fill_obj = gtk.gdk.color_parse(fill)
