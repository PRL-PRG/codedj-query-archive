from lib.config import config

class CConnectionLine(object):
    """
    Line that visualy represents the connection
    """
    def __init__(self, color = 'black', style = 'solid', width = 1):
        """
        Initialize line and set its style
        
        @param color: Color of the line (in HTML format)
        @type  color: string
        
        @param style: line style
        @type  style: string
        
        @param width: width of the line
        @type  width: integer or string
        """
        self.color = color
        self.style = style
        self.width = int(width)

    def Paint(self, canvas, start, end):
        """
        Paint the line on given canvas
        
        @param canvas: Arrow will be painted on this canvas
        @type  canvas: L{CAbstractCanvas<lib.Drawing.Canvas.Abstract.CAbstractCanvas>}
        
        @param start: starting coordinates of the line
        @type  start: (int, int)
        
        @param end: ending coordinates of the line
        @type  end: (int, int)
        """
        if self.color[0] == '/':
            color = config[self.color]
        else:
            color = self.color
        canvas.DrawLine(start, end, color, line_width = self.width, line_style = self.style)
