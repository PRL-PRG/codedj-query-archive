from Abstract import CAbstractCanvas
from lib.Math2D import Path, PathSingle, PathPartArc, PathPartBezier, PathPartLine, PathPartMove, PathPart
import math
import Gtk
from lib.lib import XMLEncode
from lib.colors import colors

LINE_STYLES = {'solid': 'none',
               'dot': '3,3',
               'doubledot': '3,2,1,2'}

class CSvgCanvas(CAbstractCanvas):
    def __init__(self, width, height, othercanvas, storage = None):
        self.othercanvas = othercanvas
        self.storage = storage
        self.width = width
        self.height = height
        
        self.paths = []
    
    def WriteOut(self, f):
        print>>f, '<?xml version="1.0" encoding="UTF-8" standalone="no"?>'
        print>>f, '<svg width="%d" height="%d" xmlns:svg="http://www.w3.org/2000/svg" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">'%(self.width, self.height)
        level = 1
        for type, params, style in self.paths:
            style = ' '.join(["%s: %s;"%s for s in style.iteritems()])
            if type == 'path':
                print>>f, '    '*level+'<path d="%s" style="%s" />'%(params, style)
            elif type == 'text':
                print>>f, '    '*level+'<text x="%s" y="%s" style="%s">%s</text>'%(params[0][0], params[0][1], style, XMLEncode(params[1]))
            elif type == 'img':
                print>>f, '    '*level+'<image x="%s" y="%s" width="%s" height="%s" style="%s" xlink:href="data:application/octetstream;base64,%s" />'%(params[0][0], params[0][1], params[1][0], params[1][1], style, params[2].encode('base64').replace('\n', '').replace('\r', ''))
            elif type == 'group':
                if params == 'end':
                    level -= 1
                    print>>f, '    '*level+'</g>'
                else:
                    print>>f, '    '*level+'<g>'
                    level += 1
        print>>f, '</svg>'
    
    def __createpart(self, parts, styles = {}):
        if isinstance(parts, PathPart):
            parts = [parts]
        tmp = Path([PathSingle([PathPartMove(parts[0].GetFirstPos())]+parts)])
        self.paths.append(('path', tmp, styles))
        return tmp
    
    def __createstyle(self, fg = None, bg = None, line_width = None, line_style = None, font = None):
        style = {}
        if fg is not None:
            style['stroke'] = colors.get(fg, fg)
        else:
            style['stroke'] = 'none'
        if bg is not None:
            style['fill'] = colors.get(bg, bg)
        else:
            style['fill'] = 'none'
        if line_width is not None:
            style['stroke-width'] = line_width
        if line_style is not None:
            style['stroke-dashoffset'] = '0'
            style['stroke-miterlimit'] = '4'
            style['stroke-dasharray'] = LINE_STYLES[line_style]
        if font is not None:
            family, fstyle, size = font
            style['font-family'] = family
            if 'bold' in fstyle:
                style['font-weight'] = 'bold'
            if 'italic' in fstyle:
                style['font-style'] = 'italic'
            style['font-size'] = '%dpt'%size
        return style
    
    def DrawArc(self, pos, size, arc = (0, 360), fg = None, bg = None, line_width = None, line_style = None):
        def tmp(arc):
            x1 = size[0]/2.0*math.cos(arc[0]*math.pi/180)+pos[0]+size[0]/2.0
            y1 = size[1]/2.0*math.sin(arc[0]*math.pi/180)+pos[1]+size[1]/2.0
            x2 = size[0]/2.0*math.cos(arc[1]*math.pi/180)+pos[0]+size[0]/2.0
            y2 = size[1]/2.0*math.sin(arc[1]*math.pi/180)+pos[1]+size[1]/2.0
            roz = arc[1] - arc[0]
            return PathPartArc((x1, y1), (size[0]/2.0, size[1]/2.0), 0, (roz > 0, abs(roz) >= 180), (x2, y2))
        
        if arc[1] - arc[0] > 180:
            self.__createpart([tmp((arc[0], arc[0]+180)), tmp((arc[0]+180, arc[1]))], self.__createstyle(fg, bg, line_width, line_style))
        elif arc[1] - arc[0] < -180:
            self.__createpart([tmp((arc[0], arc[0]-180)), tmp((arc[0]-180, arc[1]))], self.__createstyle(fg, bg, line_width, line_style))
        else:
            self.__createpart(tmp(arc), self.__createstyle(fg, bg, line_width, line_style))
    
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
        if '\n' in text:
            text = text.split('\n')
            x, y = pos
            for line in text[:-1]:
                self.DrawText((x, y), line, font, fg)
                y += self.GetTextSize(line, font)[1]
            self.DrawText((x, y), text[-1], font, fg)
            
            return
        
        if 'underline' in font[1] or 'strike' in font[1]:
            size = self.GetTextSize(text, font)
            self.paths.append(('group','',{}))
        bline = self.othercanvas.GetFontBaseLine(font)
        self.paths.append(('text', ((pos[0], pos[1]+bline), text), self.__createstyle(bg=fg, font=font)))
        if 'underline' in font[1]:
            self.__createpart(PathPartLine((pos[0], pos[1]+bline+2), (pos[0]+size[0], pos[1]+bline+2)),
                    self.__createstyle(fg))
        if 'strike' in font[1]:
            self.__createpart(PathPartLine((pos[0], pos[1]+bline//2), (pos[0]+size[0], pos[1]+bline//2)),
                    self.__createstyle(fg))
        if 'underline' in font[1] or 'strike' in font[1]:
            self.paths.append(('group','end',{}))
    
    def GetTextSize(self, text, font):
        return self.othercanvas.GetTextSize(text, font)
    
    def DrawIcon(self, pos, filename):
        self.paths.append(('img', (pos, self.GetIconSize(filename), self.storage.read_file(filename)), {}))
    
    def GetIconSize(self, filename):
        return self.othercanvas.GetIconSize(filename)
    
    def Clear(self):
        self.paths = []
