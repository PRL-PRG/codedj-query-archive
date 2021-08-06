from Abstract import CAbstractCanvas
from lib.Math2D import Path, PathSingle, PathPartArc, PathPartBezier, PathPartLine, PathPartMove, PathPart
import math
import Gtk
from lib.lib import XMLEncode

LINE_STYLES = {'solid': 'none',
               'dot': '1,1',
               'doubledot': '1,4'}

class CSvgCanvas(CAbstractCanvas):
    def __init__(self, width, height, gtkcanvas, storage = None):
        self.gtkcanvas = gtkcanvas
        self.storage = storage
        self.width = width
        self.height = height
        
        self.paths = []
    
    def WriteOut(self, f):
        print>>f, '<?xml version="1.0" encoding="UTF-8" standalone="no"?>'
        print>>f, '<svg width="%d" height="%d" xmlns:svg="http://www.w3.org/2000/svg" xmlns="http://www.w3.org/2000/svg">'%(self.width, self.height)
        print>>f, '    <g id="Layer1">'
        for type, params, style in self.paths:
            style = ' '.join(["%s: %s;"%s for s in style.iteritems()])
            if type == 'path':
                print>>f, '        <path d="%s" style="%s" />'%(params, style)
            elif type == 'text':
                print>>f, '        <text x="%s" y="%s" style="%s">%s</text>'%(params[0][0], params[0][1], style, XMLEncode(params[1]))
            elif type == 'img':
                print>>f, '        <image x="%s" y="%s" style="%s" xlink:href="data:application/octetstream;base64,%s" />'%(params[0][0], params[0][1], style, params[1].encode('base64'))
        print>>f, '    </g>'
        print>>f, '</svg>'
    
    def __createpart(self, parts, styles = {}):
        if isinstance(parts, (Path, PathSingle, PathPart)):
            parts = [parts]
        tmp = Path([PathSingle([PathPartMove(parts[0].GetFirstPos())]+parts)])
        self.paths.append(('path', tmp, styles))
        return tmp
    
    def __createstyle(self, fg = None, bg = None, line_width = None, line_style = None, font = None):
        style = {}
        if fg is not None:
            style['stroke'] = fg
        if bg is not None:
            style['fill'] = bg
        if line_width is not None:
            style['stroke-width'] = line_width
        if line_style is not None:
            style['stroke-dashoffset'] = LINE_STYLES[line_style]
        if font is not None:
            family, size, fstyle = (font.split()+['normal'])[:3]
            style['font-family'] = family
            style['font-size'] = size+'px'
        return style
    
    def DrawArc(self, pos, size, arc = (0, 360), fg = None, bg = None, line_width = None, line_style = None):
        x1 = size[0]/2.0*cos(arc[0]*math.pi/180)+pos[0]+size[0]/2.0
        y1 = size[1]/2.0*cos(arc[0]*math.pi/180)+pos[1]+size[1]/2.0
        x2 = size[0]/2.0*cos(arc[1]*math.pi/180)+pos[0]+size[0]/2.0
        y2 = size[1]/2.0*cos(arc[1]*math.pi/180)+pos[1]+size[1]/2.0
        roz = arc[1] - arc[0]
        self.__createpart(PathPartArc((x1, y1), (size[0]/2.0, size[1]/2.0), 0, (roz > 0, abs(roz) >= 180)),
                self.__createstyle(fg, bg, line_width, line_style))
    
    def DrawLine(self, start, end, fg, line_width = None, line_style = None):
        self.__createpart(PathPartLine(start, end),
                self.__createstyle(fg, None, line_width, line_style))
    
    def DrawLines(self, points, fg, line_width = None, line_style = None):
        old = points[0]
        out = []
        for i in points[1:]:
            out.append(PathPartLine(old, i))
            old = i
        self.__createpart(out,
                self.__createstyle(fg, None, line_width, line_style))
    
    def DrawPolygon(self, points, fg = None, bg = None, line_width = None, line_style = None):
        old = points[0]
        out = []
        for i in points[1:]:
            out.append(PathPartLine(old, i))
            old = i
        self.__createpart(out,
                self.__createstyle(fg, bg, line_width, line_style)).Close()
    
    def DrawPath(self, path, fg = None, bg = None, line_width = None, line_style = None):
        self.paths.append(('path', path, self.__createstyle(fg, bg, line_width, line_style)))
    
    def DrawRectangle(self, pos, size, fg = None, bg = None, line_width = None, line_style = None):
        self.DrawPolygon([pos, (pos[0]+size[0], pos[1]), (pos[0]+size[0], pos[1]+size[1]), (pos[0], pos[1]+size[1])], fg, bg, line_width, line_style)
    
    def DrawText(self, pos, text, font, fg):
        self.paths.append(('text', (pos, text), self.__createstyle(bg=fg, font=font)))
    
    def GetTextSize(self, text, font):
        return self.gtkcanvas.GetTextSize(text, font)
    
    def DrawIcon(self, pos, filename):
        self.paths.append(('img', (pos, self.storage.read_file(filename)), {}))
    
    def GetIconSize(self, filename):
        return self.gtkcanvas.GetIconSize(filename)
    
    def Clear(self):
        self.paths = []
