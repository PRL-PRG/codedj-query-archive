from lib.lib import UMLException
from Abstract import CAbstractCanvas
import math
import pango
import pangocairo
import gtk
import gtk.gdk
import cairo
from lib import colors


#  dash sequence for line styles used in self.cr.set_dash(dash_sequence, offset), where
#  dash_sequence - an array specifying alternate lengths of on and off stroke portions
#  offset - an offset into the dash pattern at which the stroke should start
LINE_STYLES = {'solid': [],
               'dot': [2,2],
               'doubledot': [8,2,2,2]}

pixmaps = {}

def PixmapFromPath(storage, path):
    if (storage, path) in pixmaps:
        tmp = pixmaps[(storage, path)]
    else:
        if storage is None:
            pathx = path
        else:
            pathx = storage.get_file_path(path)

        tmp = cairo.ImageSurface.create_from_png(str(pathx))
        pixmaps[(storage, path)] = tmp

    return tmp


def HexToRGB(hexcolor):
    #converts Hex colors(HTML format) to RGB, used to set fg, bg, font color

    if not hexcolor[0] == '#': # if hexcolor is a word 
        hexcolor = colors.colors[hexcolor]
    hexcolor = hexcolor.strip()
    hexcolor = hexcolor[1:]
    if len(hexcolor) != 6:
        raise UMLException('invalid hex color, use #RRGGBB format or color name')
    r, g, b = hexcolor[:2], hexcolor[2:4], hexcolor[4:]
    r, g, b = [int(n, 16) for n in (r, g, b)]
    return (float(r)/255, float(g)/255, float(b)/255)



class CCairoCanvas(CAbstractCanvas):


    def __init__(self, widget, window = None, storage = None, surface_type = None, filename = None):

        self.widget = widget
        self.surface_type = surface_type
        self.filename = filename

        if window is None:
            self.window = widget.window
        else:
            self.window = window

        if self.surface_type == 'pdf':
            if not cairo.HAS_PDF_SURFACE:
                raise UMLException ('cairo: no PDF support')
            self.surface = cairo.PDFSurface (self.filename, *self.window.get_size())

        elif self.surface_type == 'svg':
            if not cairo.HAS_SVG_SURFACE:
                raise UMLException ('cairo: no SVG support')
            self.surface = cairo.SVGSurface (self.filename, *self.window.get_size())

        elif self.surface_type == 'png':
            if not cairo.HAS_PNG_FUNCTIONS:
                raise UMLException ('cairo: no PNG support')
            self.surface = cairo.ImageSurface (cairo.FORMAT_ARGB32,*self.window.get_size())

        elif self.surface_type == 'ps':
            if not cairo.HAS_PS_SURFACE:
                raise UMLException ('cairo: no PS support')
            self.surface = cairo.PSSurface (self.filename, *self.window.get_size())
            cairo.PSSurface.dsc_comment(self.surface,"%%Title: uml.FRI diagram export");

        elif self.surface_type == None:
            # drawing on gtk canvas
            pass

        else :
            raise UMLException('unknown export surface or format')


        if self.surface_type is not None:
            self.cairo_context= cairo.Context (self.surface)

        else:
            self.cairo_context = self.window.cairo_create()

        self.alpha = 1.0
        self.storage = storage
        self.cr = pangocairo.CairoContext(self.cairo_context)
        self.pango_layout = self.cr.create_layout()
        self.fonts = {}
        self.cr.save()


    def __SetFont(self, (family, style, size), returndesc = False):

        underline = 'underline' in style
        strikeout = 'strike' in style
        font = [family]
        # some (supported) font styles, append order is important
        if 'Pitch' in style:
            font.append('Pitch')

        if 'Sans' in style:
            font.append('Sans')

        if 'Mono' in style:
            font.append('Mono')

        if 'Serif' in style:
            font.append('Serif')

        if 'Bold' in style:
            font.append('Bold')

        if 'Italic' in style:
            font.append('Italic')

        if 'Oblique' in style:
            font.append('Oblique')

        if 'Semi-Condensed' in style:
            font.append('Semi-Condensed')

        font.append(str(size)+'px')
        font = ' '.join(font)

        if font in self.fonts:
            fontobj = self.fonts[font]
        else:
            self.fonts[font] = fontobj = pango.FontDescription(font)

        if returndesc:
            return fontobj
        self.pango_layout.set_font_description(fontobj)

        atlist = pango.AttrList()
        if underline:
            atlist.insert(pango.AttrUnderline(pango.UNDERLINE_SINGLE, 0, 10000))
        if strikeout:
            atlist.insert(pango.AttrStrikethrough(True, 0, 10000))

        self.pango_layout.set_attributes(atlist)


    def DrawArc(self, pos, size, arc = (0, 360), fg = None, bg = None, line_width = None, line_style = None):
        self.cr.save()

        if int(size[0]) < int(size[1]):
            size1=1.0
            size0=float(size[0])/float(size[1])
            radius = int(size[1])/2

        elif int(size[0]) == int(size[1]):
            size0=1.0
            size1=1.0
            radius = int(size[1])/2
        else:
            size0=1.0
            size1=float(size[1])/float(size[0])
            radius = int(size[0])/2

        self.cr.translate (int(pos[0])+(int(size[0])/2), int(pos[1])+(int(size[1])/2))
        self.cr.scale(size0, size1)
        self.cr.arc(0,0,radius,arc[0],arc[1])

        if bg is not None:
            temp_color = HexToRGB(bg)
            self.cr.set_source_rgba(temp_color[0], temp_color[1], temp_color[2], self.alpha)
            self.cr.fill_preserve()

        if fg is not None:
            temp_color = HexToRGB(fg)
            self.cr.set_source_rgba(temp_color[0], temp_color[1], temp_color[2], self.alpha)

        if line_width is not None:
            self.cr.set_line_width(line_width)

        if line_style is not None:
            self.cr.set_dash(LINE_STYLES[line_style], 0)       

        self.cr.stroke()
        self.cr.restore()


    def DrawLine(self, start, end, fg, line_width = None, line_style = None):
        self.cr.save()
        self.cr.move_to(int(start[0]),int(start[1]))
        self.cr.line_to(int(end[0]),int(end[1]))

        if fg is not None: 
            temp_color = HexToRGB(fg)
            self.cr.set_source_rgba(temp_color[0], temp_color[1], temp_color[2], self.alpha)

        if line_width is not None:
            self.cr.set_line_width(line_width)

        if line_style is not None:
            self.cr.set_dash(LINE_STYLES[line_style], 0)

        self.cr.stroke()
        self.cr.restore()


    def DrawLines(self, points, fg, line_width = None, line_style = None):
        self.cr.save()
        move_pen = True
        for x,y in points :
            if move_pen:
                self.cr.move_to(x,y)
                move_pen = False
            self.cr.line_to(x,y) 

        if fg is not None:
            temp_color = HexToRGB(fg)
            self.cr.set_source_rgba(temp_color[0], temp_color[1], temp_color[2], self.alpha)

        if line_width is not None:
            self.cr.set_line_width(line_width)

        if line_style is not None:
            self.cr.set_dash(LINE_STYLES[line_style], 0)

        self.cr.stroke()
        self.cr.restore()


    def DrawPolygon(self, points, fg = None, bg = None, line_width = None, line_style = None):
        self.cr.save()
        move_pen = True

        for x,y in points :
            if move_pen:
                self.cr.move_to(x,y)
                move_pen = False
            self.cr.line_to(x,y)

        self.cr.close_path()

        if bg is not None:
            temp_color = HexToRGB(bg)
            self.cr.set_source_rgba(temp_color[0], temp_color[1], temp_color[2], self.alpha)
            self.cr.fill_preserve()

        if fg is not None:
            temp_color = HexToRGB(fg)
            self.cr.set_source_rgba(temp_color[0], temp_color[1], temp_color[2], self.alpha)

        if line_width is not None:
            self.cr.set_line_width(line_width)

        if line_style is not None:
            self.cr.set_dash(LINE_STYLES[line_style], 0)

        self.cr.stroke()
        self.cr.restore()

    def DrawPath(self, path, fg = None, bg = None, line_width = None, line_style = None):
        for single in path:
            t = single.GetType()
            if t == 'polygon':
                self.DrawPolygon(single, fg, bg, line_width, line_style)
            elif t == 'polyline':
                self.DrawLines(single, fg, line_width, line_style)


    def DrawRectangle(self, pos, size, fg = None, bg = None, line_width = None, line_style = None):
        self.cr.save()
        self.cr.rectangle(int(pos[0]), int(pos[1]), int(size[0]), int(size[1]))

        if bg is not None:
            temp_color = HexToRGB(bg)
            self.cr.set_source_rgba(temp_color[0], temp_color[1], temp_color[2], self.alpha)
            self.cr.fill_preserve()

        if fg is not None:
            temp_color = HexToRGB(fg)
            self.cr.set_source_rgba(temp_color[0], temp_color[1], temp_color[2], self.alpha)

        if line_width is not None:
            self.cr.set_line_width(line_width)

        if line_style is not None:
            self.cr.set_dash(LINE_STYLES[line_style], 0)

        self.cr.stroke()
        self.cr.restore()


    def DrawText(self, pos, text, font, fg):
        self.cr.save()
        self.__SetFont(font)
        self.cr.move_to (int(pos[0]), int(pos[1]))
        #self.pango_layout.set_markup(text)
        self.pango_layout.set_text(text)
        font_color = HexToRGB(fg)
        self.cr.set_source_rgb(font_color[0], font_color[1], font_color[2])
        self.cr.show_layout(self.pango_layout)
        self.cr.stroke ()
        self.cr.restore()


    def GetTextSize(self, text, font):
        self.__SetFont(font)
        self.pango_layout.set_text(text)
        return int(self.pango_layout.get_size()[0]/float(pango.SCALE)), int(self.pango_layout.get_size()[1]/float(pango.SCALE))


    #obsolete, gets font base line, used by CSvgCanvas class to export to svg
    def GetFontBaseLine(self, font):
        lines = self.pango_layout.get_iter()
        baseline = lines.get_baseline()/pango.SCALE
        return baseline


    def DrawIcon(self, pos, filename):
        if self.storage is None:
            raise UMLException('storage')
        pixmap = PixmapFromPath(self.storage, filename)
        self.cr.save()
        self.cr.set_source_surface (pixmap, pos[0], pos[1])
        self.cr.paint()
        self.cr.restore()


    def GetIconSize(self, filename):
        if self.storage is None:
            raise UMLException('storage')
        pixmap = PixmapFromPath(self.storage, filename)
        return pixmap.get_width(), pixmap.get_height()


    def Clear(self):
        gc = self.widget.get_style().white_gc
        self.window.draw_rectangle(gc, True, 0, 0, *self.window.get_size())

    #finish operations, if writing to a file (exporting diagram)
    def Finish(self):
        if self.surface_type == 'png':
            self.surface.write_to_png (self.filename)
        self.surface.finish()


if __name__ == "__main__":
    pass