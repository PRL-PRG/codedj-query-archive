# -*- coding: utf-8 -*-
"""
toolbox.py

Create Oficina Toolbar in Sugar


Copyright 2007, NATE-LSI-EPUSP

Oficina is developed in Brazil at Escola Politécnica of 
Universidade de São Paulo. NATE is part of LSI (Integrable
Systems Laboratory) and stands for Learning, Work and Entertainment
Research Group. Visit our web page: 
www.lsi.usp.br/nate
Suggestions, bugs and doubts, please email oficina@lsi.usp.br

Oficina is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License 
as published by the Free Software Foundation version 2 of 
the License.

Oficina is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public
License along with Oficina; if not, write to the
Free Software Foundation, Inc., 51 Franklin St, Fifth Floor, 
Boston, MA  02110-1301  USA.
The copy of the GNU General Public License is found in the 
COPYING file included in the source distribution.


Authors:

Joyce Alessandra Saul               (joycealess@gmail.com)
Andre Mossinato                     (andremossinato@gmail.com)
Nathalia Sautchuk Patrício          (nathalia.sautchuk@gmail.com)
Pedro Kayatt                        (pekayatt@gmail.com)
Rafael Barbolo Lopes                (barbolo@gmail.com)
Alexandre A. Gonçalves Martinazzo   (alexandremartinazzo@gmail.com)

Colaborators:
Bruno Gola                          (brunogola@gmail.com)

Group Manager:
Irene Karaguilla Ficheman           (irene@lsi.usp.br)

Cientific Coordinator:
Roseli de Deus Lopes                (roseli@lsi.usp.br)

"""

from gettext import gettext as _

import gtk, logging

from sugar.activity.activity import ActivityToolbox, EditToolbar
from sugar.graphics.toolcombobox import ToolComboBox
from sugar.graphics.toolbutton import ToolButton
from sugar.graphics.toggletoolbutton import ToggleToolButton
from sugar.graphics.combobox import ComboBox
from sugar.graphics.palette import Palette

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
        
        self._activity = activity
        
        self.undo.set_tooltip(_('Undo'))
        self.redo.set_tooltip(_('Redo'))
        self.copy.set_tooltip(_('Copy'))
        self.paste.set_tooltip(_('Paste'))
        
        self.undo.connect('clicked', self._undo_cb)
        self.redo.connect('clicked', self._redo_cb)

        self.copy.connect('clicked', self._copy_cb)
        self.paste.connect('clicked', self._paste_cb)
        
        self._activity._area.connect('undo', self._on_signal_undo_cb)
        self._activity._area.connect('redo', self._on_signal_redo_cb)
        self._activity._area.connect('action-saved', self._on_signal_action_saved_cb)
        
    def _undo_cb(self, widget, data=None):
        self._activity._area.undo()
        
    def _redo_cb(self, widget, data=None):
        self._activity._area.redo()
        
    def _copy_cb(self, widget, data=None):
        self._activity._area.copy()
        
    def _paste_cb(self, widget, data=None):
        self._activity._area.past()
        
    def _on_signal_undo_cb(self, widget, data=None):
        self._verify_sensitive_buttons()
        
    def _on_signal_redo_cb(self, widget, data=None):
        self._verify_sensitive_buttons()
        
    def _on_signal_action_saved_cb(self, widget, data=None):
        self._verify_sensitive_buttons()
        
    def _verify_sensitive_buttons(self):
        self.undo.set_sensitive( self._activity._area.can_undo() )
        self.redo.set_sensitive( self._activity._area.can_redo() )
        #TODO: it is not possible to verify these yet.
        #self.copy.set_sensitive( self._activity._area.can_copy() )
        #self.paste.set_sensitive( self._activity._area.can_paste() )

class ToolsToolbar(gtk.Toolbar):

    _TOOL_PENCIL = 'pencil'
    _TOOL_BRUSH = 'brush'
    _TOOL_ERASER = 'eraser'
    _TOOL_POLYGON = 'polygon'
    _TOOL_BUCKET = 'bucket'
    _TOOL_MARQUEE_ELLIPTICAL = 'marquee-elliptical'
    _TOOL_MARQUEE_FREEFORM = 'marquee-freeform'
    _TOOL_MARQUEE_RECTANGULAR = 'marquee-rectangular'
    _TOOL_MARQUEE_SMART = 'marquee-smart'

    
    def __init__(self, activity):
        gtk.Toolbar.__init__(self)

         # FIXME: This should be a file picker instead of a combobox
         
        self._activity = activity

        self._icon_stroke = ToolButton('icon-stroke')
        self.insert(self._icon_stroke, -1)
        self._icon_stroke.show()
        self._icon_stroke.set_tooltip(_('Tool Color'))
        
        # Changing widget: using toolbox.ButtonStrokeColor instead of toolbox.ComboStrokeColors
        '''
        self._stroke_color = ComboStrokeColors(activity)
        self.insert(self._stroke_color, -1)
        self._stroke_color.show()
        '''
        self._stroke_color = ButtonStrokeColor(activity)
        self._stroke_color.show()
#         self._stroke_color.set_tooltip(_('Stroke Color'))
        item = gtk.ToolItem()
        item.add(self._stroke_color)
        self.insert(item, -1)
        item.show()
        
        
        self._stroke_size = ComboStrokeSize(activity)
        self.insert(self._stroke_size, -1)
        self._stroke_size.show()
        
        separator = gtk.SeparatorToolItem()
        separator.set_draw(True)
        self.insert(separator, -1)
        separator.show()

        self._tool_pencil = ToolButton('tool-pencil')
        self.insert(self._tool_pencil, -1)
        self._tool_pencil.show()
        self._tool_pencil.set_tooltip(_('Pencil'))

        self._tool_brush = ToolButton('tool-brush')
        self.insert(self._tool_brush, -1)
        self._tool_brush.show()
        #self._tool_brush.set_tooltip(_('Brush'))
        self._brush_palette = self.create_palette(_('Brush'))
        self._tool_brush.set_palette(self._brush_palette)

        self._tool_eraser = ToolButton('tool-eraser')
        self.insert(self._tool_eraser, -1)
        self._tool_eraser.show()
        #self._tool_eraser.set_tooltip(_('Eraser'))
        self._eraser_palette = self.create_palette(_('Eraser'))
        self._tool_eraser.set_palette(self._eraser_palette)

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
        
        self._icon_stroke.connect('clicked', self._on_icon_stroke_clicked)
        
        # New connect method
        self._tool_polygon.connect('clicked', self.set_tool, self._TOOL_POLYGON)
        self._tool_pencil.connect('clicked', self.set_tool, self._TOOL_PENCIL)
        self._tool_brush.connect('clicked', self.set_tool, self._TOOL_BRUSH)
        self._tool_eraser.connect('clicked', self.set_tool, self._TOOL_ERASER)
        self._tool_bucket.connect('clicked', self.set_tool, self._TOOL_BUCKET)
        #self._tool_marquee_elliptical.connect('clicked', self.set_tool, self._TOOL_MARQUEE_ELLIPTICAL)
        #self._tool_marquee_freeform.connect('clicked', self.set_tool, self._TOOL_MARQUEE_FREEFORM)
        self._tool_marquee_rectangular.connect('clicked', self.set_tool, self._TOOL_MARQUEE_RECTANGULAR)
        #self._tool_marquee_smart.connect('clicked', self.set_tool, self._TOOL_MARQUEE_SMART)

    def create_palette(self, tool=None):
    
        #TODO: create palettes for other tools.
        if tool == None:
            return None
        elif (tool == 'Brush') or (tool == 'Eraser'):
            palette = Palette(_(tool))
            item_1 = gtk.MenuItem(_('Square'))
            item_2 = gtk.MenuItem(_('Circle'))

            palette.append_menu_item(item_1)
            palette.append_menu_item(item_2)
            item_1.show()
            item_2.show()
            item_1.connect('activate', self.set_shape, tool, 'square')
            item_2.connect('activate', self.set_shape, tool,'circle')
            
            return palette

    def set_shape(self, button, tool, shape):
        '''
        Set a tool shape according to user choice at Tool Palette
        '''
        
        if tool == 'Brush':
            self._activity._area.brush_shape = shape
        elif tool == 'Eraser':
            self._activity._area.eraser_shape = shape
            
    def set_tool(self, widget, tool):
        '''
        Set tool to the Area object. Configures tool's color and size.
        '''
        
        # setting tool
        self._activity._area.tool = tool
        
        # setting size and color
        size = self._stroke_size.get_size()
        self._stroke_size.set_stroke_size(size)
        
        color = self._stroke_color.get_color()
        self._stroke_color.set_stroke_color(color)
        
        #setting cursor
        try:
            pixbuf = gtk.gdk.pixbuf_new_from_file('./images/' + tool + '.png')
            cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pixbuf, 6, 21)
        except:
            cursor = None
        self._activity._area.window.set_cursor(cursor)
        
    def _on_icon_stroke_clicked(self, widget, data=None):
        self._stroke_color.clicked()
        
        

class ComboFillColors(ToolComboBox):
    """Class to manage Fill colors """   
            
    def __init__(self, activity):
        """Initialize the object

        Keyword arguments:
        activity -- the OficinaActivity object
        """
    
        ToolComboBox.__init__(self)
        self._activity = activity

        self._fill_color = self.combo
        self._fill_color.append_item(self.alloc_color('#000000'), _('Black'))
        self._fill_color.append_item(self.alloc_color('#ffffff'), _('White'))
        self._fill_color.append_item(self.alloc_color('#800000'), _('Maroon'))
        self._fill_color.append_item(self.alloc_color('#ff0000'), _('Red'))        
        self._fill_color.append_item(self.alloc_color('#808000'), _('Olive'))
        self._fill_color.append_item(self.alloc_color('#ffff00'), _('Yellow')) 
        self._fill_color.append_item(self.alloc_color('#008000'), _('Green'))
        self._fill_color.append_item(self.alloc_color('#00ff00'), _('Lime'))
        self._fill_color.append_item(self.alloc_color('#008080'), _('Teal'))
        self._fill_color.append_item(self.alloc_color('#00ffff'), _('Aqua'))
        self._fill_color.append_item(self.alloc_color('#000080'), _('Navy'))
        self._fill_color.append_item(self.alloc_color('#0000ff'), _('Blue'))
        self._fill_color.append_item(self.alloc_color('#800080'), _('Purple'))
        self._fill_color.append_item(self.alloc_color('#ff00ff'), _('Fuchsia'))

        self._fill_color.set_active(0)
        self._fill_color.connect('changed', self._combo_changed_cb)

    def alloc_color(self, color):
        """Alloc new color.

        Keyword arguments:
        color -- hexadecimal number
        
        Return:
        a gdk.Color object

        """
        colormap = self.get_colormap()
        _COLOR_ = colormap.alloc_color(color, True, True) 
        return _COLOR_  

    def _combo_changed_cb(self, combo):
        color = self.get_color()
        self.set_fill_color(color)

    def set_fill_color(self, color):
        """Set the fill color in Area

        Keyword arguments:
        color -- a gdk.Color object

        """
        self._activity._area._set_fill_color(color)
    
    def get_color(self):  
        """Get the fill color from combobox

        Return:
        a gdk.Color object

        """      
        model = self.combo.get_model()
        active = self.combo.get_active()
        return model[active][0]


class ComboStrokeColors(ToolComboBox):
    """Class to manage Stroke colors """   
    
    def __init__(self, activity):
        """Initialize the object

        Keyword arguments:
        activity -- the OficinaActivity object
        """
        
        ToolComboBox.__init__(self)
        self._activity = activity
        
        self._stroke_color = self.combo
        self._stroke_color.append_item(self.alloc_color('#000000'), _('Black'))
        self._stroke_color.append_item(self.alloc_color('#ffffff'), _('White'))
        self._stroke_color.append_item(self.alloc_color('#800000'), _('Maroon'))
        self._stroke_color.append_item(self.alloc_color('#ff0000'), _('Red'))        
        self._stroke_color.append_item(self.alloc_color('#808000'), _('Olive'))
        self._stroke_color.append_item(self.alloc_color('#ffff00'), _('Yellow')) 
        self._stroke_color.append_item(self.alloc_color('#008000'), _('Green'))
        self._stroke_color.append_item(self.alloc_color('#00ff00'), _('Lime'))
        self._stroke_color.append_item(self.alloc_color('#008080'), _('Teal'))
        self._stroke_color.append_item(self.alloc_color('#00ffff'), _('Aqua'))
        self._stroke_color.append_item(self.alloc_color('#000080'), _('Navy'))
        self._stroke_color.append_item(self.alloc_color('#0000ff'), _('Blue'))
        self._stroke_color.append_item(self.alloc_color('#800080'), _('Purple'))
        self._stroke_color.append_item(self.alloc_color('#ff00ff'), _('Fuchsia'))

        self._stroke_color.set_active(0)
        self._stroke_color.connect('changed', self._combo_changed_cb)

    def alloc_color(self, color):
        """Alloc new color.

        Keyword arguments:
        color -- hexadecimal number
        
        Return:
        a gdk.Color object

        """
        colormap = self.get_colormap()
        _COLOR_ = colormap.alloc_color(color, True, True) 
        return _COLOR_
    
    def _combo_changed_cb(self, combo):
        color = self.get_color()
        self.set_stroke_color(color)

    def get_color(self):
        """Get the fill color from combobox

        Return:
        a gdk.Color object

        """        
        model = self.combo.get_model()
        active = self.combo.get_active()
        return model[active][0]

    def set_stroke_color(self, color):
        """Set the fill color in Area

        Keyword arguments:
        color -- a gdk.Color object

        """
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
    """
    _ACTION_500 = 500
    _ACTION_1000 = 1000
    _ACTION_5000 = 5000
    _ACTION_10000 = 10000
    _ACTION_100000 = 100000
    """

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
        """
        self._stroke_size.append_item(self._ACTION_500, _('500'))
        self._stroke_size.append_item(self._ACTION_1000, _('1000'))
        self._stroke_size.append_item(self._ACTION_5000, _('5000'))
        self._stroke_size.append_item(self._ACTION_10000, _('10000'))
        self._stroke_size.append_item(self._ACTION_100000, _('100000'))
        """

        self._stroke_size.set_active(1)
        self._stroke_size.connect('changed', self._combo_changed_cb)

    def _combo_changed_cb(self, combo):
#         model = combo.get_model()
#         active = combo.get_active()
#         self.set_stroke_size(model[active][0])
        size = self.get_size()
        self.set_stroke_size(size)
        
    def set_stroke_size(self, size):
        self._activity._area.configure_line(size)
        
    def get_size(self):
        model = self.combo.get_model()
        active = self.combo.get_active()
        return model[active][0]


class ButtonFillColor(gtk.ColorButton):

    def __init__(self, activity):
        gtk.ColorButton.__init__(self)
        self._activity = activity
        
        self.connect('color-set', self._color_button_cb)
        
    def _color_button_cb(self, widget):
        color = self.get_color()
        self.set_fill_color(color)
        
    def alloc_color(self, color):
        colormap = self._activity._area.get_colormap()
        return colormap.alloc_color(color.red, color.green, color.blue)
        
    def set_fill_color(self, color):
        new_color = self.alloc_color(color)
        self._activity._area._set_fill_color(new_color)


class ButtonStrokeColor(gtk.ColorButton):

    def __init__(self, activity):
        gtk.ColorButton.__init__(self)
        self._activity = activity
        
        self.connect('color-set', self._color_button_cb)
        
    def _color_button_cb(self, widget):
        color = self.get_color()
        self.set_stroke_color(color)
        
    def alloc_color(self, color):
        colormap = self._activity._area.get_colormap()
        return colormap.alloc_color(color.red, color.green, color.blue)
        
    def set_stroke_color(self, color):
        new_color = self.alloc_color(color)
        self._activity._area._set_stroke_color(new_color)


class ShapesToolbar(gtk.Toolbar):

    _TOOL_SHAPE_ARROW = 'arrow'
    _TOOL_SHAPE_CURVE = 'curve'
    _TOOL_SHAPE_ELLIPSE = 'ellipse'
    _TOOL_SHAPE_FREEFORM = 'freeform'
    _TOOL_SHAPE_HEART = 'heart'
    _TOOL_SHAPE_LINE = 'line'
    _TOOL_SHAPE_PARALLELOGRAM = 'parallelogram'
    _TOOL_SHAPE_POLYGON = 'polygon_regular'
    _TOOL_SHAPE_RECTANGLE = 'rectangle'
    _TOOL_SHAPE_STAR = 'star'
    _TOOL_SHAPE_TRAPEZOID = 'trapezoid'
    _TOOL_SHAPE_TRIANGLE = 'triangle'

    def __init__(self, activity):
        gtk.Toolbar.__init__(self)

        self._activity = activity
        
        self._icon_fill = ToolButton('icon-fill')
        self.insert(self._icon_fill, -1)
        self._icon_fill.show()
        self._icon_fill.set_tooltip(_('Fill Color'))
        
        # Changing widget: using toolbox.ButtonFillColor instead of toolbox.ComboFillColors
        '''
        self._fill_color = ComboFillColors(activity)
        self.insert(self._fill_color, -1)
        self._fill_color.show()
        '''
        self._fill_color = ButtonFillColor(activity)
        self._fill_color.show()
        item = gtk.ToolItem()
        item.add(self._fill_color)
        self.insert(item, -1)
        item.show()
        
        self._icon_stroke = ToolButton('icon-stroke')
        self.insert(self._icon_stroke, -1)
        self._icon_stroke.show()
        self._icon_stroke.set_tooltip(_('Stroke Color'))
        
        # Changing widget: using toolbox.ButtonStrokeColor instead of toolbox.ComboStrokeColors
        '''
        self._stroke_color = ComboStrokeColors(activity)
        self.insert(self._stroke_color, -1)
        self._stroke_color.show()
        '''
        self._stroke_color = ButtonStrokeColor(activity)
        self._stroke_color.show()
        item = gtk.ToolItem()
        item.add(self._stroke_color)
        self.insert(item, -1)
        item.show()
        
        
        self._stroke_size = ComboStrokeSize(activity)
        self.insert(self._stroke_size, -1)
        self._stroke_size.show()
        
        separator = gtk.SeparatorToolItem()
        separator.set_draw(True)
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
        
        self._tool_shape_polygon = ToolButton('tool-shape-polygon')
        self.insert(self._tool_shape_polygon, -1)
        self._tool_shape_polygon.show()
        self._tool_shape_polygon.set_tooltip(_('Polygon'))

        """
        
        self._tool_shape_freeform = ToolButton('tool-shape-freeform')
        self.insert(self._tool_shape_freeform, -1)
        self._tool_shape_freeform.show()
        self._tool_shape_freeform.set_tooltip(_('Freeform'))

        self._tool_shape_heart = ToolButton('tool-shape-heart')
        self.insert(self._tool_shape_heart, -1)
        self._tool_shape_heart.show()
        self._tool_shape_heart.set_tooltip(_('Heart'))

        """

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

        self._tool_shape_trapezoid = ToolButton('tool-shape-trapezoid')
        self.insert(self._tool_shape_trapezoid, -1)
        self._tool_shape_trapezoid.show()
        self._tool_shape_trapezoid.set_tooltip(_('Trapezoid'))

        self._tool_shape_triangle = ToolButton('tool-shape-triangle')
        self.insert(self._tool_shape_triangle, -1)
        self._tool_shape_triangle.show()
        self._tool_shape_triangle.set_tooltip(_('Triangle'))

        
        
        self._icon_stroke.connect('clicked', self._on_icon_stroke_clicked)
        self._icon_fill.connect('clicked', self._on_icon_fill_clicked)

        self._tool_shape_arrow.connect('clicked', self.set_tool, self._TOOL_SHAPE_ARROW)
        self._tool_shape_ellipse.connect('clicked', self.set_tool, self._TOOL_SHAPE_ELLIPSE)
        #self._tool_shape_freeform.connect('clicked', self.set_tool, self._TOOL_SHAPE_FREEFORM)
        #self._tool_shape_heart.connect('clicked', self.set_tool, self._TOOL_SHAPE_HEART)
        self._tool_shape_line.connect('clicked', self.set_tool, self._TOOL_SHAPE_LINE)
        self._tool_shape_parallelogram.connect('clicked', self.set_tool, self._TOOL_SHAPE_PARALLELOGRAM)
        self._tool_shape_polygon.connect('clicked', self.set_tool, self._TOOL_SHAPE_POLYGON)
        self._tool_shape_rectangle.connect('clicked', self.set_tool, self._TOOL_SHAPE_RECTANGLE)
        self._tool_shape_star.connect('clicked', self.set_tool, self._TOOL_SHAPE_STAR)
        self._tool_shape_trapezoid.connect('clicked', self.set_tool, self._TOOL_SHAPE_TRAPEZOID)
        self._tool_shape_triangle.connect('clicked', self.set_tool, self._TOOL_SHAPE_TRIANGLE)
    
    def set_tool(self, widget, tool):
        
        # setting tool
        self._activity._area.tool = tool
        
        # setting size and color
        size = self._stroke_size.get_size()
        self._stroke_size.set_stroke_size(size)
        
        stroke_color = self._stroke_color.get_color()
        self._stroke_color.set_stroke_color(stroke_color)
        
        fill_color = self._fill_color.get_color()
        self._fill_color.set_fill_color(fill_color)
        
        #setting cursor
        try:
            pixbuf = gtk.gdk.pixbuf_new_from_file('./images/' + tool + '.png')
            cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pixbuf, 6, 21)
        except:
            cursor = None
        
        self._activity._area.window.set_cursor(cursor)
        
    def _on_icon_stroke_clicked(self, widget, data=None):
        self._stroke_color.clicked()
        
    def _on_icon_fill_clicked(self, widget, data=None):
        self._fill_color.clicked()

class TextToolbar(gtk.Toolbar):

    _ACTION_TEXT = 'text'

    def __init__(self, activity):
        gtk.Toolbar.__init__(self)

        self._activity = activity

        self._text = ToolButton('text')
        self.insert(self._text, -1)
        self._text.show()
        self._text.set_tooltip(_('Type'))
        self._text.connect('clicked', self.set_tool, self._ACTION_TEXT)
        
        separator = gtk.SeparatorToolItem()
        separator.set_draw(True)
        self.insert(separator, -1)
        separator.show()
        
        self._text_color = ButtonFillColor(activity)
        self._text_color.show()
        item = gtk.ToolItem()
        item.add(self._text_color)
        self.insert(item, -1)
        item.show()
        
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
        
    def set_tool(self, widget, tool):
        #FIXME: this callback must change as others buttons get enabled
        self._activity._area.tool = tool
        
        color = self._text_color.get_color()
        self._text_color.set_fill_color(color)
        
        # setting cursor
        pixbuf = gtk.gdk.pixbuf_new_from_file('./images/text.png')
        cursor = gtk.gdk.Cursor(gtk.gdk.display_get_default() , pixbuf, 6, 21)
        self._activity._area.window.set_cursor(cursor)


class ImageToolbar(gtk.Toolbar):

    _OBJECT_HEIGHT = 'height'
    _OBJECT_INSERT = 'insert'
    _OBJECT_ROTATE_LEFT = 'rotate-left'
    _OBJECT_ROTATE_RIGHT = 'rotate-right'
    _OBJECT_WIDTH = 'width'

    def __init__(self, activity):
        gtk.Toolbar.__init__(self)
        
        self._object_insert = ToolButton('object-insert')
        self.insert(self._object_insert, -1)
        self._object_insert.show()
        self._object_insert.set_tooltip(_('Insert Image'))
        
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
        # TODO: add a filter to display images only.
        dialog = gtk.FileChooserDialog(title=(_('Open File...')),   
                     action=gtk.FILE_CHOOSER_ACTION_OPEN,   
                     buttons=(gtk.STOCK_CANCEL, gtk.RESPONSE_CANCEL,   
                     gtk.STOCK_OK, gtk.RESPONSE_OK)) 
        dialog.show_all()
        
        logging.debug('Importing image from file')
        response = dialog.run()
        
        if response == gtk.RESPONSE_OK:
            file_path = dialog.get_filename()
            logging.debug('file selected')
            logging.debug(file_path)
            #file_path = decode_path((file_path,))[0]
            #open(activity, file_path)
            activity._area.d.loadImage(file_path,widget)
        elif response == gtk.RESPONSE_CANCEL:
            logging.debug('Closed, no files selected')
            
        dialog.destroy()



class EffectsToolbar(gtk.Toolbar):

    _ACTION_GRAYSCALE = 'grayscale'

    def __init__(self, activity):
        gtk.Toolbar.__init__(self)
	
        separator = gtk.SeparatorToolItem()
        self.insert(separator, -1)
        separator.show()

        self._effect_grayscale = ToolButton('effect-grayscale')
        self.insert(self._effect_grayscale, -1)
        self._effect_grayscale.show()
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

        '''
        # FIXME: these callbacks are not implemented
        self._zoom_plus.connect('clicked', test_connect, activity, 'zoom_plus')
        self._zoom_minus.connect('clicked', test_connect, activity, 'zoom_minus')
        '''
        
    def _combo_changed_cb(self, combo):
        if combo == self._view_percent:
            print 'treeeter'



