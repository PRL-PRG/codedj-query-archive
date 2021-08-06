from gettext import gettext as _

import gtk

from sugar.activity.activity import ActivityToolbox, EditToolbar
from sugar.graphics import color
from sugar.graphics.toolcombobox import ToolComboBox
from sugar.graphics.toolbutton import ToolButton
from sugar.graphics.toggletoolbutton import ToggleToolButton
from sugar.graphics.combobox import ComboBox
from sugar.graphics.palette import Palette

from Cursors import Cursors

class Toolbox(ActivityToolbox):
    def __init__(self, activity):
        ActivityToolbox.__init__(self, activity)
        
        # creating toolbars for Draw activity
        
        self._edit_toolbar = DrawEditToolbar(activity)
        self.add_toolbar(_('Edit'), self._edit_toolbar)
        self._edit_toolbar.show()
        
        self._tools_toolbar = ToolsToolbar(activity)
        self.add_toolbar(_('Tools'), self._tools_toolbar)
        self._tools_toolbar.show()

        self._shapes_toolbar = ShapesToolbar(activity)
        self.add_toolbar(_('Shapes'), self._shapes_toolbar)
        self._shapes_toolbar.show()

        self._text_toolbar = TextToolbar(activity)
        self.add_toolbar(_('Text'), self._text_toolbar)
        self._text_toolbar.show()   

        self._image_toolbar = ImageToolbar(activity)
        self.add_toolbar(_('Image'), self._image_toolbar)
        self._image_toolbar.show()

        self._effects_toolbar = EffectsToolbar(activity)
        self.add_toolbar(_('Effects'), self._effects_toolbar)
        self._effects_toolbar.show()

        #self._view_toolbar = ViewToolbar(activity)
        #self.add_toolbar(_('View'), self._view_toolbar)
        #self._view_toolbar.show()
        
    
class DrawEditToolbar(EditToolbar):
    def __init__(self, activity):
        EditToolbar.__init__(self)

        self.undo.connect('clicked', undo, activity)
        self.redo.connect('clicked', redo, activity)

        #FIXME: buttons are not connected to the right callback
        self.copy.connect('clicked', test_connect, activity, 'copy')
        self.paste.connect('clicked', test_connect, activity, 'paste')

        self.copy.hide()
        self.paste.hide()


class ToolsToolbar(gtk.Toolbar):

    _TOOL_PENCIL = 2
    _TOOL_BRUSH = 29
    _TOOL_ERASER = 3
    _TOOL_POLYGON = 27
    _TOOL_BUCKET = 28
    _TOOL_MARQUEE_ELLIPTICAL = 5
    _TOOL_MARQUEE_FREEFORM = 2
    _TOOL_MARQUEE_RECTANGULAR = 26
    _TOOL_MARQUEE_SMART = 2

    
    def __init__(self, activity):
        gtk.Toolbar.__init__(self)

         # FIXME: This should be a file picker instead of a combobox
         
        self._activity = activity

	"""
        self._icon_fill = ToolButton('icon-fill')
        self.insert(self._icon_fill, -1)
        self._icon_fill.show()

        tool_item = ComboFillColors(activity)
        self.insert(tool_item, -1)
        tool_item.show()
	"""

        self._icon_stroke = ToolButton('icon-stroke')
        self.insert(self._icon_stroke, -1)
        self._icon_stroke.show()

        tool_item = ComboStrokeColors(activity)
        self.insert(tool_item, -1)
        tool_item.show()

        tool_item = ComboStrokeSize(activity)
        self.insert(tool_item, -1)
        tool_item.show()
        
        separator = gtk.SeparatorToolItem()
        self.insert(separator, -1)
        separator.show()

        self._tool_pencil = ToolButton('tool-pencil')
        self.insert(self._tool_pencil, -1)
        self._tool_pencil.show()
        self._tool_pencil.set_tooltip(_('Pencil'))

        self._tool_brush = ToolButton('tool-brush')
        self.insert(self._tool_brush, -1)
        self._tool_brush.show()
        self._tool_brush.set_tooltip(_('Brush'))
        self._brush_palette = self.create_palette('Brush')
        self._tool_brush.set_palette(self._brush_palette)

        self._tool_eraser = ToolButton('tool-eraser')
        self.insert(self._tool_eraser, -1)
        self._tool_eraser.show()
        self._tool_eraser.set_tooltip(_('Eraser'))

        self._tool_polygon = ToolButton('tool-polygon')
        self.insert(self._tool_polygon, -1)
        self._tool_polygon.show()
        self._tool_polygon.set_tooltip(_('Polygon'))
        
        self._tool_bucket = ToolButton('tool-bucket')
        self.insert(self._tool_bucket, -1)
        self._tool_bucket.show()
        self._tool_bucket.set_tooltip(_('Bucket'))
        
        """


        self._tool_marquee_elliptical = ToolButton('tool-marquee-elliptical')
        self.insert(self._tool_marquee_elliptical, -1)
        self._tool_marquee_elliptical.show()
        self._tool_marquee_elliptical.set_tooltip(_('Elliptical Marquee'))

        self._tool_marquee_freeform = ToolButton('tool-marquee-freeform')
        self.insert(self._tool_marquee_freeform, -1)
        self._tool_marquee_freeform.show()
        self._tool_marquee_freeform.set_tooltip(_('Freeform Marquee'))

        self._tool_marquee_smart = ToolButton('tool-marquee-smart')
        self.insert(self._tool_marquee_smart, -1)
        self._tool_marquee_smart.show()
        self._tool_marquee_smart.set_tooltip(_('Smart Marquee'))
        """
        self._tool_marquee_rectangular = ToolButton('tool-marquee-rectangular')
        self.insert(self._tool_marquee_rectangular, -1)
        self._tool_marquee_rectangular.show()
        self._tool_marquee_rectangular.set_tooltip(_('Rectangular Marquee'))

        self._tool_polygon.connect('clicked', set_tool, activity, 'tool-polygon', self._TOOL_POLYGON)
        self._tool_pencil.connect('clicked', set_tool, activity, 'tool-pencil', self._TOOL_PENCIL)
        self._tool_brush.connect('clicked', set_tool, activity, 'tool-brush', self._TOOL_BRUSH)
        self._tool_eraser.connect('clicked', set_tool, activity, 'tool-eraser', self._TOOL_ERASER)
        self._tool_bucket.connect('clicked', set_tool, activity, 'tool-bucket', self._TOOL_BUCKET)
        #self._tool_marquee_elliptical.connect('clicked', set_tool, activity, 'tool-marquee-elliptical', self._TOOL_MARQUEE_ELLIPTICAL)
        #self._tool_marquee_freeform.connect('clicked', set_tool, activity, 'tool-marquee-freeform', self._TOOL_MARQUEE_FREEFORM)
        self._tool_marquee_rectangular.connect('clicked', set_tool, activity, 'tool-marquee-rectangular', self._TOOL_MARQUEE_RECTANGULAR)
        #self._tool_marquee_smart.connect('clicked', set_tool, activity, 'tool-marquee-smart', self._TOOL_MARQUEE_SMART)

    def create_palette(self, data=None):
        if data == None:
            return None
        elif data == 'Brush':
            palette = Palette(_(data))
            item_1 = gtk.MenuItem(_('Square'))
            item_2 = gtk.MenuItem(_('Circle'))

            palette.append_menu_item(item_1)
            palette.append_menu_item(item_2)
            item_1.show()
            item_2.show()
            item_1.connect('activate', self.test, 'square')
            item_2.connect('activate', self.test, 'circle')
            
            return palette

    def test(self, button, data=None):
        print button, data
        self._activity._area.brush_shape = data

class ComboFillColors(ToolComboBox):

    _ACTION_BLACK = 0
    _ACTION_PURPLE = 1
    _ACTION_YELLOW = 2
    _ACTION_BLUE = 3
    _ACTION_GREEN = 4
    _ACTION_RED = 5
    _ACTION_ORANGE = 6
    _ACTION_WHITE = 7
 
    def __init__(self, activity):
        ToolComboBox.__init__(self)
        self._activity = activity

        self._fill_color = self.combo
        self._fill_color.append_item(self._ACTION_BLACK, _('Black'))
        self._fill_color.append_item(self._ACTION_PURPLE, _('Purple'))
        self._fill_color.append_item(self._ACTION_YELLOW, _('Yellow'))        
        self._fill_color.append_item(self._ACTION_BLUE, _('Blue'))
        self._fill_color.append_item(self._ACTION_GREEN, _('Green')) 
        self._fill_color.append_item(self._ACTION_RED, _('Red'))
        self._fill_color.append_item(self._ACTION_ORANGE, _('Orange'))
        self._fill_color.append_item(self._ACTION_WHITE, _('White'))

        self._fill_color.set_active(0)
        self._fill_color.connect('changed', self.set_fill_color)


    def set_fill_color(self, combo):
        color = combo.get_active()
        self._activity._area._set_fill_color(color)



class ComboStrokeColors(ToolComboBox):

    _ACTION_BLACK = 0
    _ACTION_PURPLE = 1
    _ACTION_YELLOW = 2
    _ACTION_BLUE = 3
    _ACTION_GREEN = 4
    _ACTION_RED = 5
    _ACTION_ORANGE = 6
    _ACTION_WHITE = 7

    def __init__(self, activity):
        ToolComboBox.__init__(self)
        self._stroke_color = self.combo
        self._activity = activity
        
        self._stroke_color.append_item(self._ACTION_BLACK, _('Black'))
        self._stroke_color.append_item(self._ACTION_PURPLE, _('Purple'))
        self._stroke_color.append_item(self._ACTION_YELLOW, _('Yellow'))
        self._stroke_color.append_item(self._ACTION_BLUE, _('Blue'))
        self._stroke_color.append_item(self._ACTION_GREEN, _('Green')) 
        self._stroke_color.append_item(self._ACTION_RED, _('Red'))
        self._stroke_color.append_item(self._ACTION_ORANGE, _('Orange'))
        self._stroke_color.append_item(self._ACTION_WHITE, _('White'))

        self._stroke_color.set_active(0)
        #self._stroke_color.connect('changed', self._combo_changed_cb)
        self._stroke_color.connect('changed', self.set_stroke_color)
        self.connect("focus", self.event_focus)
       
    def event_focus(self, combo):
        print 'combostroke gained focus' 	

    def set_stroke_color(self, combo):
        color = combo.get_active()
        self._activity._area._set_stroke_color(color)


class ComboStrokeSize(ToolComboBox):

    _ACTION_1 = 1
    _ACTION_2 = 2
    _ACTION_3 = 3
    _ACTION_5 = 5
    _ACTION_10 = 10
    _ACTION_20 = 20
    _ACTION_50 = 50
    _ACTION_100 = 100
    _ACTION_500 = 500
    _ACTION_1000 = 1000
    _ACTION_5000 = 5000
    _ACTION_10000 = 10000
    _ACTION_100000 = 100000

    def __init__(self, activity):
        ToolComboBox.__init__(self)
        self._activity = activity

        self._stroke_size = self.combo
        self._stroke_size.append_item(self._ACTION_1, _('1'))
	self._stroke_size.append_item(self._ACTION_2, _('2'))
        self._stroke_size.append_item(self._ACTION_3, _('3'))        
        self._stroke_size.append_item(self._ACTION_5, _('5'))
        self._stroke_size.append_item(self._ACTION_10, _('10')) 
        self._stroke_size.append_item(self._ACTION_20, _('20'))
        self._stroke_size.append_item(self._ACTION_50, _('50'))
        self._stroke_size.append_item(self._ACTION_100, _('100'))
        self._stroke_size.append_item(self._ACTION_500, _('500'))
        self._stroke_size.append_item(self._ACTION_1000, _('1000'))
        self._stroke_size.append_item(self._ACTION_5000, _('5000'))
        self._stroke_size.append_item(self._ACTION_10000, _('10000'))
        self._stroke_size.append_item(self._ACTION_100000, _('100000'))

        self._stroke_size.set_active(0)
        self._stroke_size.connect('changed', self._combo_changed_cb)

    def _combo_changed_cb(self, combo):
        set_stroke_size(self._activity, combo.get_active())   


class ShapesToolbar(gtk.Toolbar):

    _TOOL_SHAPE_ARROW = 0
    _TOOL_SHAPE_CURVE = 0
    _TOOL_SHAPE_ELLIPSE = 5
    _TOOL_SHAPE_FREEFORM = 0
    _TOOL_SHAPE_HEART = 0
    _TOOL_SHAPE_LINE = 1
    _TOOL_SHAPE_PARALLELOGRAM = 0
    _TOOL_SHAPE_POLYGON = 27
    _TOOL_SHAPE_RECTANGLE = 6
    _TOOL_SHAPE_STAR = 0
    _TOOL_SHAPE_TRAPEZOID = 31
    _TOOL_SHAPE_TRIANGLE = 30

    def __init__(self, activity):
        gtk.Toolbar.__init__(self)

        self._icon_fill = ToolButton('icon-fill')
        self.insert(self._icon_fill, -1)
        self._icon_fill.show()

        # FIXME: This should be a file picker instead of a combobox

        tool_item = ComboFillColors(activity)
        self.insert(tool_item, -1)
        tool_item.show()

        self._icon_stroke = ToolButton('icon-stroke')
        self.insert(self._icon_stroke, -1)
        self._icon_stroke.show()

        tool_item = ComboStrokeColors(activity)
        self.insert(tool_item, -1)
        tool_item.show()

        tool_item = ComboStrokeSize(activity)
        self.insert(tool_item, -1)
        tool_item.show()
        
        separator = gtk.SeparatorToolItem()
        self.insert(separator, -1)
        separator.show()

        self._tool_shape_ellipse = ToolButton('tool-shape-ellipse')
        self.insert(self._tool_shape_ellipse, -1)
        self._tool_shape_ellipse.show()
        self._tool_shape_ellipse.set_tooltip(_('Ellipse'))

        self._tool_shape_rectangle = ToolButton('tool-shape-rectangle')
        self.insert(self._tool_shape_rectangle, -1)
        self._tool_shape_rectangle.show()
        self._tool_shape_rectangle.set_tooltip(_('Rectangle'))

        self._tool_shape_line = ToolButton('tool-shape-line')
        self.insert(self._tool_shape_line, -1)
        self._tool_shape_line.show()
        self._tool_shape_line.set_tooltip(_('Line'))
        """
        self._tool_shape_polygon = ToolButton('tool-shape-polygon')
        self.insert(self._tool_shape_polygon, -1)
        self._tool_shape_polygon.show()
        self._tool_shape_polygon.set_tooltip(_('Polygon'))

        self._tool_shape_freeform = ToolButton('tool-shape-freeform')
        self.insert(self._tool_shape_freeform, -1)
        self._tool_shape_freeform.show()
        self._tool_shape_freeform.set_tooltip(_('Freeform'))

        self._tool_shape_heart = ToolButton('tool-shape-heart')
        self.insert(self._tool_shape_heart, -1)
        self._tool_shape_heart.show()
        self._tool_shape_heart.set_tooltip(_('Heart'))

        self._tool_shape_parallelogram = ToolButton('tool-shape-parallelogram')
        self.insert(self._tool_shape_parallelogram, -1)
        self._tool_shape_parallelogram.show()
        self._tool_shape_parallelogram.set_tooltip(_('Parallelogram'))

        self._tool_shape_arrow = ToolButton('tool-shape-arrow')
        self.insert(self._tool_shape_arrow, -1)
        self._tool_shape_arrow.show()
        self._tool_shape_arrow.set_tooltip(_('Arrow'))

        self._tool_shape_star = ToolButton('tool-shape-star')
        self.insert(self._tool_shape_star, -1)
        self._tool_shape_star.show()
        self._tool_shape_star.set_tooltip(_('Star'))

        """

        self._tool_shape_trapezoid = ToolButton('tool-shape-trapezoid')
        self.insert(self._tool_shape_trapezoid, -1)
        self._tool_shape_trapezoid.show()
        self._tool_shape_trapezoid.set_tooltip(_('Trapezoid'))

        self._tool_shape_triangle = ToolButton('tool-shape-triangle')
        self.insert(self._tool_shape_triangle, -1)
        self._tool_shape_triangle.show()
        self._tool_shape_triangle.set_tooltip(_('Triangle'))

        #self._tool_shape_arrow.connect('clicked', set_tool, activity, 'tool-shape-arrow', self._TOOL_SHAPE_ARROW)
        self._tool_shape_ellipse.connect('clicked', set_tool, activity, 'tool-shape-ellipse', self._TOOL_SHAPE_ELLIPSE)
        #self._tool_shape_freeform.connect('clicked', set_tool, activity, 'tool-shape-freeform', self._TOOL_SHAPE_FREEFORM)
        #self._tool_shape_heart.connect('clicked', set_tool, activity, 'tool-shape-heart', self._TOOL_SHAPE_HEART)
        self._tool_shape_line.connect('clicked', set_tool, activity, 'tool-shape-line', self._TOOL_SHAPE_LINE)
        #self._tool_shape_parallelogram.connect('clicked', set_tool, activity, 'tool-shape-parallelogram', self._TOOL_SHAPE_PARALLELOGRAM)
        #self._tool_shape_polygon.connect('clicked', set_tool, activity, 'tool-shape-polygon', self._TOOL_SHAPE_POLYGON)
        self._tool_shape_rectangle.connect('clicked', set_tool, activity, 'tool-shape-rectangle', self._TOOL_SHAPE_RECTANGLE)
        #self._tool_shape_star.connect('clicked', set_tool, activity, 'tool-shape-star', self._TOOL_SHAPE_STAR)
        self._tool_shape_trapezoid.connect('clicked', set_tool, activity, 'tool-shape-trapezoid', self._TOOL_SHAPE_TRAPEZOID)
        self._tool_shape_triangle.connect('clicked', set_tool, activity, 'tool-shape-triangle', self._TOOL_SHAPE_TRIANGLE)


class TextToolbar(gtk.Toolbar):

    _ACTION_TEXT = 4

    def __init__(self, activity):
        gtk.Toolbar.__init__(self)

        self._text = ToggleToolButton('text')
        self.insert(self._text, -1)
        self._text.show()
        self._text.set_tooltip(_('Type'))
        self._text.connect('clicked', set_tool, activity, 'text', self._ACTION_TEXT)

        """
        #FIXME: this button is not connected to the right callback
        self._bold = ToggleToolButton('format-text-bold')
        self.insert(self._bold, -1)
        self._bold.show()
        self._bold.connect('clicked', test_connect, activity, 'bold')
        
        #FIXME: this button is not connected to the right callback
        self._italic = ToggleToolButton('format-text-italic')
        self.insert(self._italic, -1)
        self._italic.show()
        self._italic.connect('clicked', test_connect, activity, 'italic')
        
        #FIXME: this button is not connected to the right callback
        self._underline = ToggleToolButton('format-text-underline')
        self.insert(self._underline, -1)
        self._underline.show()
        self._underline.connect('clicked', test_connect, activity, 'underline')
        
        # Displays a few colors in a ComboBox 
        # TODO: User's choice is done when clicking at the first Combo item
        # TODO: Keep previous choices at the list
        
        self._text_color = ComboBox()
        self._text_color.append_text('red')        
      
        
        #FIXME: must use a gtk.ToolItem to use 'insert' method
        #self.insert(self._text_color, -1)
        self._text_color.show()
        """
	def type_text(self, activity):
        	set_tool(self._ACTION_TEXT, activity, 'text')
        	activity._textview.show()


class ImageToolbar(gtk.Toolbar):

    _OBJECT_HEIGHT = 30
    _OBJECT_INSERT = 31
    _OBJECT_ROTATE_LEFT = 32
    _OBJECT_ROTATE_RIGHT = 33
    _OBJECT_WIDTH = 34

    def __init__(self, activity):
        gtk.Toolbar.__init__(self)
        
        self._object_insert = ToolButton('object-insert')
        self.insert(self._object_insert, -1)
        self._object_insert.show()
        self._object_insert.set_tooltip(_('object-insert'))
        
        separator = gtk.SeparatorToolItem()
        separator.set_draw(True)
        self.insert(separator, -1)
        separator.show()
        
        """        
	self._object_rotate_left = ToolButton('object-rotate-left')
        self.insert(self._object_rotate_left, -1)
        self._object_rotate_left.show()
        self._object_rotate_left.set_tooltip(_('Rotate Left'))

        self._object_rotate_right = ToolButton('object-rotate-right')
        self.insert(self._object_rotate_right, -1)
        self._object_rotate_right.show()
        self._object_rotate_right.set_tooltip(_('Rotate Right'))
        
        self._object_height = ToolButton('object-height')
        self.insert(self._object_height, -1)
        self._object_height.show()
        self._object_height.connect('clicked', test_connect, activity, 'object-height')
        self._object_height.set_tooltip(_('Height'))           

        self._object_width = ToolButton('object-width')
        self.insert(self._object_width, -1)
        self._object_width.show()
        self._object_width.set_tooltip(_('Width'))


        self._object_height.connect('clicked', set_tool, activity, 'object-height', self._OBJECT_HEIGHT)
        """
        self._object_insert.connect('clicked', self.insertImage, activity)
        #self._object_rotate_left.connect('clicked', self.rotate_left, activity)
        #self._object_rotate_right.connect('clicked', set_tool, activity, 'object-rotate-right', self._OBJECT_ROTATE_RIGHT)
        #self._object_width.connect('clicked', set_tool, activity, 'object-width', self._OBJECT_WIDTH)
	
    def rotate_left(self, widget, activity):    
        #activity._area._rotate_left()
	pass


    def insertImage(self, widget, activity):
        dialog = gtk.FileChooserDialog(title=(_('Open File...')),   
                     action=gtk.FILE_CHOOSER_ACTION_OPEN,   
                     buttons=(gtk.STOCK_CANCEL, gtk.RESPONSE_CANCEL,   
                     gtk.STOCK_OK, gtk.RESPONSE_OK)) 
        dialog.show_all()			
        response = dialog.run()
        if response == gtk.RESPONSE_OK:
            print dialog.get_filename(), 'selected'
            #gtk28 = False 
            file_path = dialog.get_filename()
            #file_path = decode_path((file_path,))[0]
            #open(activity, file_path)
            activity._area.d.loadImage(file_path)
        elif response == gtk.RESPONSE_CANCEL:
            print 'Closed, no files selected'
        dialog.destroy()



class EffectsToolbar(gtk.Toolbar):

    _ACTION_GRAYSCALE = 0

    def __init__(self, activity):
        gtk.Toolbar.__init__(self)
	
        separator = gtk.SeparatorToolItem()
        self.insert(separator, -1)
        separator.show()

        self._effect_grayscale = ToolButton('effect-grayscale')
        self.insert(self._effect_grayscale, -1)
        self._effect_grayscale.show()
        self._effect_grayscale.connect('clicked', test_connect, activity, 'effect-grayscale')
        self._effect_grayscale.set_tooltip(_('Grayscale'))
	
	"""
	#FIXME: Must be implemented
        self._black_and_white = ToolButton('black_and_white')
        self.insert(self._black_and_white, -1)
        self._black_and_white.show()
        self._black_and_white.connect('clicked', test_connect, activity, 'effect-black-and-white')
        self._black_and_white.set_tooltip(_('Black and White'))

        self._invert_colors = ToolButton('invert_colors')
        self.insert(self._invert_colors, -1)
        self._invert_colors.show()
        self._invert_colors.connect('clicked', test_connect, activity, 'invert-colors')
        self._invert_colors.set_tooltip(_('Invert Colors'))

        """
        self._effect_grayscale.connect('clicked', self.grayscale, activity)

    def grayscale(self, widget, activity):	
        activity._area._set_grayscale(widget)


class ViewToolbar(gtk.Toolbar):

    _ACTION_1000 = 0
    _ACTION_500 = 1
    _ACTION_200 = 2
    _ACTION_150 = 3
    _ACTION_100 = 4
    _ACTION_50 = 5
    _ACTION_25 = 6
    _ACTION_10 = 7

    
    def __init__(self, activity):
        gtk.Toolbar.__init__(self)

        tool_item = ToolComboBox()
        self._view_percent = tool_item.combo
        self._view_percent.append_item(self._ACTION_1000, _('1000'))
        self._view_percent.append_item(self._ACTION_500, _('500'))
        self._view_percent.append_item(self._ACTION_200, _('200'))
        self._view_percent.append_item(self._ACTION_150, _('150'))
        self._view_percent.append_item(self._ACTION_100, _('100'))
        self._view_percent.append_item(self._ACTION_50, _('50'))
        self._view_percent.append_item(self._ACTION_25, _('25'))
        self._view_percent.append_item(self._ACTION_10, _('10'))
        self._view_percent.set_active(0)
        self._view_percent.connect('changed', self._combo_changed_cb)
        self.insert(tool_item, -1)
        tool_item.show()
        
        separator = gtk.SeparatorToolItem()
        self.insert(separator, -1)
        separator.show()

        self._zoom_plus = ToolButton('zoom-plus')
        self.insert(self._zoom_plus, -1)
        self._zoom_plus.show()
        self._zoom_plus.set_tooltip(_('ZOOM +'))

        self._zoom_minus = ToolButton('zoom-minus')
        self.insert(self._zoom_minus, -1)
        self._zoom_minus.show()
        self._zoom_minus.set_tooltip(_('ZOOM -'))

        self._zoom_plus.connect('clicked', test_connect, activity, 'zoom_plus')
        self._zoom_minus.connect('clicked', test_connect, activity, 'zoom_minus')

    def _combo_changed_cb(self, combo):
        if combo == self._view_percent:
            print 'treeeter'


def set_tool(widget, activity, data=None, tool=None):
    activity._area.tool = tool
    #setting cursor
    print data
    if data == 'tool-pencil':
        pix = gtk.gdk.pixbuf_new_from_file("./images/lapis_cursor.png")
        cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pix, 6, 21)
        
    elif data == 'tool-eraser':
        pix = gtk.gdk.pixbuf_new_from_file("./images/borracha_cursor.png")
        cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pix, 6, 21)
        
    elif data == 'tool-shape-ellipse':
        pix = gtk.gdk.pixbuf_new_from_file("./images/circulo_cursor.png")
        cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pix, 6, 21)
        
    elif data == 'tool-shape-rectangle':
        pix = gtk.gdk.pixbuf_new_from_file("./images/quadrado_cursor.png")
        cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pix, 6, 21)
        
    elif data == 'tool-marquee-rectangular':    
        cursor = gtk.gdk.Cursor(gtk.gdk.CROSSHAIR)
	activity._area.move = False

    elif data == 'text':
        pix = gtk.gdk.pixbuf_new_from_file("./images/letra_cursor.png")
        cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pix, 6, 21)
        
    elif data == 'tool-shape-line':
        pix = gtk.gdk.pixbuf_new_from_file("./images/linha_cursor.png")
        cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pix, 6, 21)

    elif data == 'tool-brush':
        pix = gtk.gdk.pixbuf_new_from_file("./icons/brush_cursor.svg")
        cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pix, 6, 21)

    elif data == 'tool-bucket':
        pix = gtk.gdk.pixbuf_new_from_file("./icons/bucket_cursor.svg")
        cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pix, 6, 21)

    elif data == 'tool-polygon':
        pix = gtk.gdk.pixbuf_new_from_file("./images/poligono_cursor.png")
        cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pix, 6, 21)

    elif data == 'tool-shape-triangle':
        pix = gtk.gdk.pixbuf_new_from_file("./images/triangle_cursor.png")
        cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pix, 6, 21)

    elif data == 'tool-shape-trapezoid':
        pix = gtk.gdk.pixbuf_new_from_file("./images/trapezoid_cursor.png")
        cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pix, 6, 21)

    else:
        # Uses toolbar icon as cursor
        #FIXME: invert cursor color. Toolbar icons are white
        try:
            archive = './icons/' + data + '.svg'
            pix = gtk.gdk.pixbuf_new_from_file(archive)
            print archive, pix
            cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pix, 6, 21)
        except:
            cursor = None

       
    activity._area.window.set_cursor(cursor)
    #print cursor
    


def set_stroke_size(activity, size):
     activity._area.configure_line(size)
            
def undo(widget, activity):
    activity._area.undo()		

def redo(widget, activity):
    activity._area.redo()

def test_connect(widget, activity, data=None):
    ''' Dummy callback for testing'''
    string = data + ' button clicked\n'
    #activity.textview.get_buffer().insert_at_cursor(string)
    

def insertImage(widget, activity):
    dialog = gtk.FileChooserDialog(title=(_('Open File...')),   
                 action=gtk.FILE_CHOOSER_ACTION_OPEN,   
                 buttons=(gtk.STOCK_CANCEL, gtk.RESPONSE_CANCEL,   
                 gtk.STOCK_OK, gtk.RESPONSE_OK)) 
    dialog.show_all()			
    response = dialog.run()
    if response == gtk.RESPONSE_OK:
        print dialog.get_filename(), 'selected'
        #gtk28 = False 
        file_path = dialog.get_filename()
        #file_path = decode_path((file_path,))[0]
        #open(activity, file_path)
        activity._area.d.loadImage(file_path)
    elif response == gtk.RESPONSE_CANCEL:
        print 'Closed, no files selected'
    dialog.destroy()

  
