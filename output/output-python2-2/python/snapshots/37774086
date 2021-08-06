import gtk.gdk

LINE_STYLES = {'solid': gtk.gdk.LINE_SOLID,
               'dot': gtk.gdk.LINE_ON_OFF_DASH,
               'doubledot': gtk.gdk.LINE_DOUBLE_DASH}


class CConnectionLine(object):
    def __init__(self, color = 'black', style = 'solid', width = 1):
        self.color = gtk.gdk.color_parse(color)
        self.style = style
        self.width = int(width)

    def GetColor(self):
        return self.color

    def GetStyle(self):
        return self.style

    def GetWidth(self):
        return self.width

    def Paint(self, x1, y1, x2, y2, Connection):
        wgt = Connection.GetDrawingArea().GetDrawable()
        cmap = wgt.get_colormap()
        gc = wgt.new_gc(foreground = cmap.alloc_color(self.color),  line_width = self.width, line_style = LINE_STYLES[self.style])
        wgt.draw_line(gc, x1, y1, x2, y2)
        

    def SetColor(self, value):
        self.color = value

    def SetStyle(self, value):
        self.style = value

    def SetWidth(self, value):
        self.width = value 
    
    Color = property(GetColor, SetColor)
    Style = property(GetStyle, SetStyle)
    Width = property(GetWidth, SetWidth)