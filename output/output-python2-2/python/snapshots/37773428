from lib.config import config

class CConnectionLine(object):
    def __init__(self, color = 'black', style = 'solid', width = 1):
        self.color = color
        self.style = style
        self.width = int(width)

    def Paint(self, canvas, start, end, Connection):
        if self.color[0] == '/':
            color = config[self.color]
        else:
            color = self.color
        canvas.DrawLine(start, end, color, line_width = self.width, line_style = self.style)
