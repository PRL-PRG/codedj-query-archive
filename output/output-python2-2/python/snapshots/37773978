import gtk.gdk

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

    def Paint(self, canvas, start, end, Connection):
        canvas.DrawLine(start, end, line_width = self.width, line_style = self.style)

    def SetColor(self, value):
        self.color = value

    def SetStyle(self, value):
        self.style = value

    def SetWidth(self, value):
        self.width = value 
    
    Color = property(GetColor, SetColor)
    Style = property(GetStyle, SetStyle)
    Width = property(GetWidth, SetWidth)