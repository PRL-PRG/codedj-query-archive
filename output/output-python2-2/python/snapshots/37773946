from Abstract import AbstractCanvas

import pango
import gtk
import gtk.gdk

LINE_STYLES = {'solid': gtk.gdk.LINE_SOLID,
               'dot': gtk.gdk.LINE_ON_OFF_DASH,
               'doubledot': gtk.gdk.LINE_DOUBLE_DASH}

class GtkCanvas(AbstractCanvas):
    def __init__(self, widget, window = None):
        self.widget = widget
        if window is None:
            self.window = widget.window
        else:
            self.window = window
        self.pango_ctx = self.widget.create_pango_context()
        self.pango_layout = self.widget.create_pango_layout("")
        self.fonts = {}
    
    def DrawArc(self, pos, size, arc = (0, 360), fg = None, bg = None, line_width = None, line_style = None):
        gc = self.window.new_gc()
        if line_width is not None:
            gc.line_width = line_width
        if line_style is not None:
            gc.line_style = LINE_STYLES[line_style]
        cmap = self.window.get_colormap()
        if bg is not None:
            gc.foreground = cmap.alloc_color(bg)
            self.window.draw_arc(gc, True, int(pos[0]), int(pos[1]), int(size[0]), int(size[1]), int(arc[0]*64), int(arc[1]*64))
        if fg is not None:
            gc.foreground = cmap.alloc_color(fg)
            self.window.draw_arc(gc, False, int(pos[0]), int(pos[1]), int(size[0]), int(size[1]), int(arc[0]*64), int(arc[1]*64))
    
    def DrawLine(self, start, end, fg, line_width = None, line_style = None):
        cmap = self.window.get_colormap()
        gc = self.window.new_gc(foreground = cmap.alloc_color(fg))
        if line_width is not None:
            gc.line_width = line_width
        if line_style is not None:
            gc.line_style = LINE_STYLES[line_style]
        self.window.draw_line(gc, int(start[0]), int(start[1]), int(end[0]), int(end[1]))
    
    def DrawLines(self, points, fg, line_width = None, line_style = None):
        cmap = self.window.get_colormap()
        gc = self.window.new_gc(foreground = cmap.alloc_color(fg))
        if line_width is not None:
            gc.line_width = line_width
        if line_style is not None:
            gc.line_style = LINE_STYLES[line_style]
        self.window.draw_lines(gc, [(int(x), int(y)) for x,y in points])
    
    def DrawPolygon(self, points, fg = None, bg = None, line_width = None, line_style = None):
        gc = self.window.new_gc()
        if line_width is not None:
            gc.line_width = line_width
        if line_style is not None:
            gc.line_style = LINE_STYLES[line_style]
        cmap = self.window.get_colormap()
        if bg is not None:
            gc.foreground = cmap.alloc_color(bg)
            self.window.draw_polygon(gc, True, [(int(x), int(y)) for x,y in points])
        if fg is not None:
            gc.foreground = cmap.alloc_color(fg)
            self.window.draw_polygon(gc, False, [(int(x), int(y)) for x,y in points])
    
    def DrawBezier(self, pt1, pt2, pt3, pt4, fg, line_width = None, line_style = None):
        t = 0
        step = 1/16.0
        old = pt1
        while (t-step) < 1:
            new = (1-t)**3*pt1[0]+3*t*(1-t)**2*pt2[0]+3*t**2*(1-t)*pt3[0]+t**3*pt4[0], \
                  (1-t)**3*pt1[1]+3*t*(1-t)**2*pt2[1]+3*t**2*(1-t)*pt3[1]+t**3*pt4[1]
            self.DrawLine(old, new, fg, line_width, line_style)
            old = new
            t += step
        self.DrawLine(old, pt4, fg, line_width, line_style)
    
    def DrawRectangle(self, pos, size, fg = None, bg = None, line_width = None, line_style = None):
        gc = self.window.new_gc()
        if line_width is not None:
            gc.line_width = line_width
        if line_style is not None:
            gc.line_style = LINE_STYLES[line_style]
        cmap = self.window.get_colormap()
        if bg is not None:
            gc.foreground = cmap.alloc_color(bg)
            self.window.draw_rectangle(gc, True, int(pos[0]), int(pos[1]), int(size[0]), int(size[1]))
        if fg is not None:
            gc.foreground = cmap.alloc_color(fg)
            self.window.draw_rectangle(gc, False, int(pos[0]), int(pos[1]), int(size[0]), int(size[1]))
    
    def DrawText(self, pos, text, font, fg):
        if font in self.fonts:
            fontobj = self.fonts[font]
        else:
            self.fonts[font] = fontobj = pango.FontDescription(font)
        self.pango_layout.set_font_description(fontobj)
        
        cmap = self.window.get_colormap()
        gc = self.window.new_gc(foreground = cmap.alloc_color(fg))
        
        self.pango_layout.set_text(text)
        self.window.draw_layout(gc, x=int(pos[0]), y=int(pos[1]), layout=self.pango_layout)
    
    def GetTextSize(self, text, font):
        if font in self.fonts:
            fontobj = self.fonts[font]
        else:
            self.fonts[font] = fontobj = pango.FontDescription(font)
        self.pango_layout.set_font_description(fontobj)
        
        self.pango_layout.set_text(text)
        return int(self.pango_layout.get_size()[0]/float(pango.SCALE)), int(self.pango_layout.get_size()[1]/float(pango.SCALE))
    
    def Clear(self):
        gc = self.widget.get_style().white_gc
        self.window.draw_rectangle(gc, True, 0, 0, *self.window.get_size())
