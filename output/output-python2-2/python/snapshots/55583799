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
from sugar.graphics.menuitem import MenuItem

##Create toolbars for the activity
class Toolbox(ActivityToolbox):
    ## The Constructor
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
        
        self.set_current_toolbar(2)
        
    
##Make the Edit Toolbar
class DrawEditToolbar(EditToolbar):
    ## The Constructor
    def __init__(self, activity):
        EditToolbar.__init__(self)
        
        self._activity = activity
        
        self.undo.set_tooltip(_('Undo'))
        self.redo.set_tooltip(_('Redo'))
        self.copy.set_tooltip(_('Copy'))
        self.paste.set_tooltip(_('Paste'))
        
        separator = gtk.SeparatorToolItem()
        separator.set_draw(True)
        self.insert(separator, -1)
        separator.show()
        
        self._clear_all = ToolButton('edit-clear')
        self.insert(self._clear_all, -1)
        self._clear_all.show()
        self._clear_all.set_tooltip(_('Clear'))
        
        self.undo.connect('clicked', self._undo_cb)
        self.redo.connect('clicked', self._redo_cb)

        self.copy.connect('clicked', self._copy_cb)
        self.paste.connect('clicked', self._paste_cb)
        self._clear_all.connect('clicked', self._clear_all_cb)
        
        
        self._activity.area.connect('undo', self._on_signal_undo_cb)
        self._activity.area.connect('redo', self._on_signal_redo_cb)
        self._activity.area.connect('selected', self._on_signal_copy_cb)
        self._activity.area.connect('action-saved', self._on_signal_action_saved_cb)

        
    def _undo_cb(self, widget, data=None):
        self._activity.area.undo()
        
    def _redo_cb(self, widget, data=None):
        self._activity.area.redo()
        
    def _copy_cb(self, widget, data=None):
        self._activity.area.copy()
        
    def _paste_cb(self, widget, data=None):
        self._activity.area.past()
        
    def _on_signal_undo_cb(self, widget, data=None):
        self._verify_sensitive_buttons()
        
    def _on_signal_redo_cb(self, widget, data=None):
        self._verify_sensitive_buttons()
           
    def _on_signal_copy_cb(self, widget, data=None):
        self._verify_sensitive_buttons()
        
    def _on_signal_action_saved_cb(self, widget, data=None):
        self._verify_sensitive_buttons()
        
    ##define when a button is active    
    def _verify_sensitive_buttons(self):
        self.undo.set_sensitive( self._activity.area.can_undo() )
        self.redo.set_sensitive( self._activity.area.can_redo() )
        self.copy.set_sensitive( self._activity.area.is_selected() )
        #TODO: it is not possible to verify this yet.
        #self.paste.set_sensitive( self._activity.area.can_paste() )
        
    def _clear_all_cb(self, widget, data=None):
        self._activity.area.clear()
        
##Determine Tools of the Toolbar
class ToolsToolbar(gtk.Toolbar):
        
    #Tool default definitions
    _TOOL_PENCIL = {
        'name'          : 'pencil',
        'line size'     : 2,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : None
    }
    _TOOL_BRUSH = {
        'name'          : 'brush',
        'line size'     : 10,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : None
    }
    _TOOL_ERASER = {
        'name'          : 'eraser',
        'line size'     : 20,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : None
    }
    
    _TOOL_POLYGON = {
        'name'          : 'polygon',
        'line size'     : 2,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : None
    }
    
    _TOOL_BUCKET = {
        'name'          : 'bucket',
        'line size'     : None,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : None,
        'fill'          : None,
        'vertices'      : None
    }
    
    _TOOL_MARQUEE_ELLIPTICAL = {
        'name'          : 'marquee-elliptical',
        'line size'     : None,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : None,
        'fill'          : None,
        'vertices'      : None
    }
    
    _TOOL_MARQUEE_FREEFORM = {
        'name'          : 'marquee-freeform',
        'line size'     : None,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : None,
        'fill'          : None,
        'vertices'      : None
    }
    
    _TOOL_MARQUEE_RECTANGULAR = {
        'name'          : 'marquee-rectangular',
        'line size'     : None,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : None,
        'fill'          : None,
        'vertices'      : None
    }
    
    _TOOL_MARQUEE_SMART = {
        'name'          : 'marquee-smart',
        'line size'     : None,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : None,
        'fill'          : None,
        'vertices'      : None
    }
    
    ##The Constructor
    def __init__(self, activity):
        gtk.Toolbar.__init__(self)

         # FIXME: This should be a file picker instead of a combobox
         
        self._activity = activity

        self._icon_stroke = ToolButton('icon-stroke')
        self.insert(self._icon_stroke, -1)
        self._icon_stroke.show()
        self._icon_stroke.set_tooltip(_('Tool Color'))
        
        self._stroke_color = ButtonStrokeColor(activity)
        self._stroke_color.show()
#         self._stroke_color.set_tooltip(_('Stroke Color'))
        item = gtk.ToolItem()
        item.add(self._stroke_color)
        self.insert(item, -1)
        item.show()
        
        separator = gtk.SeparatorToolItem()
        separator.set_draw(True)
        self.insert(separator, -1)
        separator.show()

        self._tool_pencil = ToolButton('tool-pencil')
        self.insert(self._tool_pencil, -1)
        self._tool_pencil.show()
        self._tool_pencil.set_tooltip(_('Pencil'))
        try:
            self._configure_palette(self._tool_pencil, self._TOOL_PENCIL)
        except:
            logging.debug('Could not create palette for tool Pencil')
        
        self._tool_brush = ToolButton('tool-brush')
        self.insert(self._tool_brush, -1)
        self._tool_brush.show()
        self._tool_brush.set_tooltip(_('Brush'))
        try:
            self._configure_palette(self._tool_brush, self._TOOL_BRUSH)
        except:
            logging.debug('Could not create palette for tool Brush')
        
        self._tool_eraser = ToolButton('tool-eraser')
        self.insert(self._tool_eraser, -1)
        self._tool_eraser.show()
        self._tool_eraser.set_tooltip(_('Eraser'))
        try:
            self._configure_palette(self._tool_eraser, self._TOOL_ERASER)
        except:
            logging.debug('Could not create palette for tool Eraser')
        
        self._tool_polygon = ToolButton('tool-polygon')
        self.insert(self._tool_polygon, -1)
        self._tool_polygon.show()
        self._tool_polygon.set_tooltip(_('Polygon'))
        try:
            self._configure_palette(self._tool_polygon, self._TOOL_POLYGON)
        except:
            logging.debug('Could not create palette for tool Polygon')
        
        self._tool_bucket = ToolButton('tool-bucket')
        self.insert(self._tool_bucket, -1)
        self._tool_bucket.show()
        self._tool_bucket.set_tooltip(_('Bucket'))
        
        separator = gtk.SeparatorToolItem()
        separator.set_draw(True)
        self.insert(separator, -1)
        separator.show()
        
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
        # Using dictionnaries to control tool's properties
        self._tool_polygon.connect('clicked', self.set_tool, self._TOOL_POLYGON)
        self._tool_pencil.connect('clicked', self.set_tool, self._TOOL_PENCIL)
        self._tool_brush.connect('clicked', self.set_tool, self._TOOL_BRUSH)
        self._tool_eraser.connect('clicked', self.set_tool, self._TOOL_ERASER)
        self._tool_bucket.connect('clicked', self.set_tool, self._TOOL_BUCKET)
        #self._tool_marquee_elliptical.connect('clicked', self.set_tool, self._TOOL_MARQUEE_ELLIPTICAL)
        #self._tool_marquee_freeform.connect('clicked', self.set_tool, self._TOOL_MARQUEE_FREEFORM)
        self._tool_marquee_rectangular.connect('clicked', self.set_tool, self._TOOL_MARQUEE_RECTANGULAR)
        #self._tool_marquee_smart.connect('clicked', self.set_tool, self._TOOL_MARQUEE_SMART)
        
        

    def _configure_palette(self, widget, tool=None):
        """Set palette for a tool
            @param self -- gtk.Toolbar
            @param widget  - the widget which Palette will be set, a ToolButton object
            @param tool    - the reference tool for Palette creation. Its values are restricted to Class constants
        """
        
        logging.debug('setting a palette for %s', tool['name'])
        
        palette = widget.get_palette()
        
        if tool is None:
            raise TypeError
            
        # We can set size when using either Pencil, Free Polygon, Brush or Eraser
        if tool['name'] is self._TOOL_PENCIL['name'] or \
             tool['name'] is self._TOOL_POLYGON['name'] or \
             tool['name'] is self._TOOL_BRUSH['name'] or \
             tool['name'] is self._TOOL_ERASER['name']:
            
            size_spinbutton = gtk.SpinButton()
            size_spinbutton.show()
            
            black = gtk.gdk.Color(0,0,0)
            size_spinbutton.modify_text(gtk.STATE_NORMAL, black)
            
            # This is where we set restrictions for size:
            # Initial value, minimum value, maximum value, step
            adj = gtk.Adjustment(tool['line size'], 1.0, 100.0, 1.0)
            size_spinbutton.set_adjustment(adj)
            
            size_spinbutton.set_numeric(True)
            
            label = gtk.Label(_('Size: '))
            label.show()
            
            palette.action_bar.pack_start(label)
            palette.action_bar.pack_start(size_spinbutton)
            
            size_spinbutton.connect('value-changed', self._on_value_changed, tool)
            
        # User is able to choose Shapes for 'Brush' and 'Eraser'
        if tool['name'] is self._TOOL_BRUSH['name'] or \
            tool['name'] is self._TOOL_ERASER['name']:
            
            # Changing to gtk.RadioButton
            item1 = gtk.RadioButton(None, _('Circle'))
            item1.show()
            item1.set_active(True)
            
            image1 = gtk.Image()
            image1.set_from_file('./icons/tool-shape-ellipse.svg')
            image1.show()
            item1.set_image(image1)
            
            item2 = gtk.RadioButton(item1, _('Square'))
            item2.show()
            
            image2 = gtk.Image()
            image2.set_from_file('./icons/tool-shape-rectangle.svg')
            image2.show()
            item2.set_image(image2)
            
            item1.connect('toggled', self._on_toggled, tool, 'circle')
            item2.connect('toggled', self._on_toggled, tool, 'square')
            
            label = gtk.Label(_('Shape'))
            label.show()
            
            vbox = gtk.VBox()
            vbox.show()
            
            vbox.pack_start(label)
            vbox.pack_start(item1)
            vbox.pack_start(item2)
            
            palette.set_content(vbox)
            
            separator = gtk.HSeparator()
            vbox.pack_end(separator)
            separator.show()

        # User is able to fill or not a polygon, and its fill color
        if tool['name'] is self._TOOL_POLYGON['name']:
            # Creating a CheckButton named "Fill".
            fill_checkbutton = gtk.CheckButton(_('Fill'))
            fill_checkbutton.show()
            fill_checkbutton.set_active(self._TOOL_POLYGON['fill'])
            
            fill_checkbutton.connect('toggled', self._on_fill_checkbutton_toggled, widget, self._TOOL_POLYGON)
            
            palette.set_content(fill_checkbutton)
            
            # Creating Fill Color Button
            hbox = gtk.HBox()
            hbox.show()
            
            label = gtk.Label(_('Fill Color: '))
            label.show()
            
            colorbutton = ButtonFillColor(self._activity)
            colorbutton.show()
            
            hbox.pack_start(label)
            hbox.pack_start(colorbutton)
            
            #palette.action_bar.pack_start(label)
            #palette.action_bar.pack_start(colorbutton)
            palette.set_content(hbox)
            
            colorbutton.connect_after('color-set', self._on_color_set, self._TOOL_POLYGON)
    

    def set_shape(self, widget=None, tool=None, shape=None):
        """
        Set a tool shape according to user choice at Tool Palette
        
            @param self -- gtk.Toolbar
            @param widget -- The connected widget, if any; necessary in case this method is used in a connect()
            @param tool -- A dictionnary to determine which tool is been using
            @param shape -- Determine which shape Brush and Erase will use
        """
        
        tool['line shape'] = shape
        self.set_tool(tool=tool)
            
    def set_tool(self, widget=None, tool=None):
        """
        Set tool to the Area object. Configures tool's color and size.
        
            @param self -- gtk.Toolbar
            @param widget -- The connected widget, if any; necessary in case this method is used in a connect()
            @param tool -- A dictionnary to determine which tool is been using
        """
        
        # New method to set tools; using dict
        
        # Color must be allocated; if not, it will be displayed as black
        new_color = self._stroke_color.get_color()
        tool['stroke color'] = self._stroke_color.alloc_color(new_color)
        
        self._activity.area.set_tool(tool)
        
        #setting cursor: Moved to Area
        
    def _on_icon_stroke_clicked(self, widget, data=None):
        self._stroke_color.clicked()
        
    def _on_fill_checkbutton_toggled(self, checkbutton, button=None, tool=None):
        logging.debug('Checkbutton is Active: %s', checkbutton.get_active() )
        
        # New method for setting tools
        #self._activity.area.fill = checkbutton.get_active()
        tool['fill'] = checkbutton.get_active()
        self.set_tool(tool=tool)
        
#     def _on_fill_checkbutton_map(self, checkbutton, data=None):
#         """
#         Update checkbutton condition to agree with Area.Area object; this prevents tools to have fill checked but be drawed not filled.
#         
#             @param self -- gtk.Toolbar
#             @param checkbutton
#             @param data
#         """
#         self._activity.area.fill = checkbutton.get_active()
        
    def _on_color_set(self, colorbutton, tool):
        logging.debug('toolbox.ToolsToolbar._on_color_set')
        
        # Color must be allocated; if not, it will be displayed as black
        new_color = colorbutton.get_color()
        tool['fill color'] = colorbutton.alloc_color(new_color)
        self.set_tool(tool=tool)

    def _on_value_changed(self, spinbutton, tool):
        size = spinbutton.get_value_as_int()
        tool['line size'] = size
        self.set_tool(tool=tool)

    def _on_toggled(self, radiobutton, tool, shape):
        if radiobutton.get_active():
            self.set_shape(tool=tool, shape=shape)


##Class to manage the Fill Color of a Button
class ButtonFillColor(gtk.ColorButton):
    ##The Constructor
    def __init__(self, activity):
        gtk.ColorButton.__init__(self)
        self._activity = activity
        
        self.connect('color-set', self._color_button_cb)
        
    def _color_button_cb(self, widget):
        color = self.get_color()
        self.set_fill_color(color)
        
    def alloc_color(self, color):
        colormap = self._activity.area.get_colormap()
        return colormap.alloc_color(color.red, color.green, color.blue)
        
    def set_fill_color(self, color):
        new_color = self.alloc_color(color)
        self._activity.area._set_fill_color(new_color)

##Class to manage the Stroke Color of a Button
class ButtonStrokeColor(gtk.ColorButton):
    ##The Constructor
    def __init__(self, activity):
        gtk.ColorButton.__init__(self)
        self._activity = activity
        
        self.connect('color-set', self._color_button_cb)
        
    def _color_button_cb(self, widget):
        color = self.get_color()
        self.set_stroke_color(color)
        
    def alloc_color(self, color):
        colormap = self._activity.area.get_colormap()
        return colormap.alloc_color(color.red, color.green, color.blue)
        
    def set_stroke_color(self, color):
        new_color = self.alloc_color(color)
        self._activity.area._set_stroke_color(new_color)

##Make the Shapes Toolbar
class ShapesToolbar(gtk.Toolbar):

    _SHAPE_ARROW = {
        'name'          : 'arrow',
        'line size'     : 5,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : 5
    }
    _SHAPE_CURVE = {
        'name'          : 'curve',
        'line size'     : 2,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : None
    }
    _SHAPE_ELLIPSE = {
        'name'          : 'ellipse',
        'line size'     : 2,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : None
    }
    _SHAPE_FREEFORM = {
        'name'          : 'freeform',
        'line size'     : 2,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : None
    }
    _SHAPE_HEART = {
        'name'          : 'heart',
        'line size'     : 2,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : None
    }
    _SHAPE_LINE = {
        'name'          : 'line',
        'line size'     : 2,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : None
    }
    _SHAPE_PARALLELOGRAM = {
        'name'          : 'parallelogram',
        'line size'     : 2,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : None
    }
    _SHAPE_POLYGON = {
        'name'          : 'polygon_regular',
        'line size'     : 2,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : 5
    }
    _SHAPE_RECTANGLE = {
        'name'          : 'rectangle',
        'line size'     : 2,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : None
    }
    _SHAPE_STAR = {
        'name'          : 'star',
        'line size'     : 2,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : 5
    }
    _SHAPE_TRAPEZOID = {
        'name'          : 'trapezoid',
        'line size'     : 2,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : None
    }
    _SHAPE_TRIANGLE = {
        'name'          : 'triangle',
        'line size'     : 2,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : None
    }
    
    ##The Constructor
    def __init__(self, activity):
        gtk.Toolbar.__init__(self)

        self._activity = activity
        
        self._icon_fill = ToolButton('icon-fill')
        self.insert(self._icon_fill, -1)
        self._icon_fill.show()
        self._icon_fill.set_tooltip(_('Fill Color'))
        
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
        
        
        self._stroke_color = ButtonStrokeColor(activity)
        self._stroke_color.show()
        item = gtk.ToolItem()
        item.add(self._stroke_color)
        self.insert(item, -1)
        item.show()
        
        separator = gtk.SeparatorToolItem()
        separator.set_draw(True)
        self.insert(separator, -1)
        separator.show()

        self._shape_ellipse = ToolButton('tool-shape-ellipse')
        self.insert(self._shape_ellipse, -1)
        self._shape_ellipse.show()
        self._shape_ellipse.set_tooltip(_('Ellipse'))
        try:
            self._configure_palette_shape_ellipse()
        except:
            logging.debug('Could not create palette for Shape Ellipse')

        self._shape_rectangle = ToolButton('tool-shape-rectangle')
        self.insert(self._shape_rectangle, -1)
        self._shape_rectangle.show()
        self._shape_rectangle.set_tooltip(_('Rectangle'))
        try:
            self._configure_palette_shape_rectangle()
        except:
            logging.debug('Could not create palette for Shape Ellipse')
        
        self._shape_line = ToolButton('tool-shape-line')
        self.insert(self._shape_line, -1)
        self._shape_line.show()
        self._shape_line.set_tooltip(_('Line'))
        try:
            self._configure_palette_shape_line()
        except:
            logging.debug('Could not create palette for Shape Line')
        
        self._shape_polygon = ToolButton('tool-shape-polygon')
        self.insert(self._shape_polygon, -1)
        self._shape_polygon.show()
        self._shape_polygon.set_tooltip(_('Polygon'))
        try:
            self._configure_palette_shape_polygon()
        except:
            logging.debug('Could not create palette for Regular Polygon')
        
        """
        
        self._shape_freeform = ToolButton('tool-shape-freeform')
        self.insert(self._shape_freeform, -1)
        self._shape_freeform.show()
        self._shape_freeform.set_tooltip(_('Freeform'))

        """

        self._shape_heart = ToolButton('tool-shape-heart')
        self.insert(self._shape_heart, -1)
        self._shape_heart.show()
        self._shape_heart.set_tooltip(_('Heart'))
        try:
            self._configure_palette_shape_heart()
        except:
            logging.debug('Could not create palette for Shape Heart')
        

        self._shape_parallelogram = ToolButton('tool-shape-parallelogram')
        self.insert(self._shape_parallelogram, -1)
        self._shape_parallelogram.show()
        self._shape_parallelogram.set_tooltip(_('Parallelogram'))
        try:
            self._configure_palette_shape_parallelogram()
        except:
            logging.debug('Could not create palette for Shape Parallelogram')

        self._shape_arrow = ToolButton('tool-shape-arrow')
        self.insert(self._shape_arrow, -1)
        self._shape_arrow.show()
        self._shape_arrow.set_tooltip(_('Arrow'))
        try:
            self._configure_palette_shape_arrow()
        except:
            logging.debug('Could not create palette for Shape Arrow')

        self._shape_star = ToolButton('tool-shape-star')
        self.insert(self._shape_star, -1)
        self._shape_star.show()
        self._shape_star.set_tooltip(_('Star'))
        try:
            self._configure_palette_shape_star()
        except:
            logging.debug('Could not create palette for Shape Star')

        self._shape_trapezoid = ToolButton('tool-shape-trapezoid')
        self.insert(self._shape_trapezoid, -1)
        self._shape_trapezoid.show()
        self._shape_trapezoid.set_tooltip(_('Trapezoid'))
        try:
            self._configure_palette_shape_trapezoid()
        except:
            logging.debug('Could not create palette for Shape Trapezoid')

        self._shape_triangle = ToolButton('tool-shape-triangle')
        self.insert(self._shape_triangle, -1)
        self._shape_triangle.show()
        self._shape_triangle.set_tooltip(_('Triangle'))
        try:
            self._configure_palette_shape_triangle()
        except:
            logging.debug('Could not create palette for Shape Triangle')
        
        
        self._icon_stroke.connect('clicked', self._on_icon_stroke_clicked)
        self._icon_fill.connect('clicked', self._on_icon_fill_clicked)

        self._shape_arrow.connect('clicked', self.set_tool, self._SHAPE_ARROW)
        self._shape_ellipse.connect('clicked', self.set_tool, self._SHAPE_ELLIPSE)
        #self._shape_freeform.connect('clicked', self.set_tool, self._SHAPE_FREEFORM)
        self._shape_heart.connect('clicked', self.set_tool, self._SHAPE_HEART)
        self._shape_line.connect('clicked', self.set_tool, self._SHAPE_LINE)
        self._shape_parallelogram.connect('clicked', self.set_tool, self._SHAPE_PARALLELOGRAM)
        self._shape_polygon.connect('clicked', self.set_tool, self._SHAPE_POLYGON)
        self._shape_rectangle.connect('clicked', self.set_tool, self._SHAPE_RECTANGLE)
        self._shape_star.connect('clicked', self.set_tool, self._SHAPE_STAR)
        self._shape_trapezoid.connect('clicked', self.set_tool, self._SHAPE_TRAPEZOID)
        self._shape_triangle.connect('clicked', self.set_tool, self._SHAPE_TRIANGLE)
    
    def set_tool(self, widget=None, tool=None):
        # New method to set tools; using dict
        
        # Color must be allocated; if not, it will be displayed as black
        stroke_color = self._stroke_color.get_color()
        tool['stroke color'] = self._stroke_color.alloc_color(stroke_color)
        
        fill_color = self._fill_color.get_color()
        tool['fill color'] = self._fill_color.alloc_color(fill_color)
        
        self._activity.area.set_tool(tool)
        
        #setting cursor: moved to Area
        
        
    def _on_icon_stroke_clicked(self, widget, data=None):
        self._stroke_color.clicked()
        
    def _on_icon_fill_clicked(self, widget, data=None):
        self._fill_color.clicked()
        
            
    def _on_vertices_value_changed(self, spinbutton, tool):
        #self._activity.area.polygon_sides = spinbutton.get_value_as_int()
        tool['vertices'] = spinbutton.get_value_as_int()
        self.set_tool(tool=tool)
        
    def _on_line_size_value_changed(self, spinbutton, tool):
        tool['line size'] = spinbutton.get_value_as_int()
        self.set_tool(tool=tool)
        
    def _on_fill_checkbutton_toggled(self, checkbutton, tool):
        logging.debug('Checkbutton is Active: %s', checkbutton.get_active() )
        
        #self._activity.area.fill = checkbutton.get_active()
        tool['fill'] = checkbutton.get_active()
        self.set_tool(tool=tool)
            
    def _configure_palette_shape_ellipse(self):
        logging.debug('Creating palette to shape ellipse')
        self._create_simple_palette(self._shape_ellipse, self._SHAPE_ELLIPSE)
    
    def _configure_palette_shape_rectangle(self):
        logging.debug('Creating palette to shape rectangle')
        self._create_simple_palette(self._shape_rectangle, self._SHAPE_RECTANGLE)
    
    def _configure_palette_shape_polygon(self):
        logging.debug('Creating palette to shape polygon')
        
        # Enable 'Size' and 'Fill' option
        self._create_simple_palette(self._shape_polygon, self._SHAPE_POLYGON)
        
        # We want choose the number of sides to our polygon
        palette = self._shape_polygon.get_palette()
                
        spin = gtk.SpinButton()
        spin.show()
        
        # When inserted in a Palette, a spinbutton does not display text in black
        black = gtk.gdk.Color(0,0,0)
        spin.modify_text(gtk.STATE_NORMAL, black)
        
        # This is where we set restrictions for sides in Regular Polygon:
        # Initial value, minimum value, maximum value, step
        adj = gtk.Adjustment(self._SHAPE_POLYGON['vertices'], 3.0, 50.0, 1.0)
        spin.set_adjustment(adj)
        spin.set_numeric(True)
        
        label = gtk.Label(_('Sides: '))
        label.show()
        
        hbox = gtk.HBox()
        hbox.show()
        hbox.pack_start(label)
        hbox.pack_start(spin)
        
        #palette.action_bar.pack_start(label)
        #palette.action_bar.pack_start(spin)
        palette.set_content(hbox)
        
        spin.connect('value-changed', self._on_vertices_value_changed, self._SHAPE_POLYGON)
        
        
    def _configure_palette_shape_heart(self):
        logging.debug('Creating palette to shape heart')
        self._create_simple_palette(self._shape_heart, self._SHAPE_HEART)
    
    def _configure_palette_shape_parallelogram(self):
        logging.debug('Creating palette to shape parallelogram')
        self._create_simple_palette(self._shape_parallelogram, self._SHAPE_PARALLELOGRAM)
    
    def _configure_palette_shape_arrow(self):
        logging.debug('Creating palette to shape arrow')
        self._create_simple_palette(self._shape_arrow, self._SHAPE_ARROW)
    
    def _configure_palette_shape_star(self):
        logging.debug('Creating palette to shape star')
        
        # Enable 'Size' and 'Fill' option
        self._create_simple_palette(self._shape_star, self._SHAPE_STAR)
        
        # We want choose the number of sides to our star
        palette = self._shape_star.get_palette()
        
        spin = gtk.SpinButton()
        spin.show()
        
        # When inserted in a Palette, a spinbutton does not display text in black
        black = gtk.gdk.Color(0,0,0)
        spin.modify_text(gtk.STATE_NORMAL, black)
        
        # This is where we set restrictions for Star:
        # Initial value, minimum value, maximum value, step
        adj = gtk.Adjustment(self._SHAPE_STAR['vertices'], 3.0, 50.0, 1.0)
        spin.set_adjustment(adj)
        spin.set_numeric(True)
        
        label = gtk.Label(_('Points: '))
        label.show()
        
        hbox = gtk.HBox()
        hbox.show()
        hbox.pack_start(label)
        hbox.pack_start(spin)
        
        #palette.action_bar.pack_start(label)
        #palette.action_bar.pack_start(spin)
        palette.set_content(hbox)
        
        spin.connect('value-changed', self._on_vertices_value_changed, self._SHAPE_STAR)
    
    def _configure_palette_shape_trapezoid(self):
        logging.debug('Creating palette to shape trapezoid')
        self._create_simple_palette(self._shape_trapezoid, self._SHAPE_TRAPEZOID)
    
    def _configure_palette_shape_triangle(self):
        logging.debug('Creating palette to shape triangle')
        self._create_simple_palette(self._shape_triangle, self._SHAPE_TRIANGLE)
    
    def _create_simple_palette(self, button, tool, line_size_only=False):
        """
        Create a simple palette with an CheckButton named "Fill" and a SpinButton to represent the line size. Most tools use only this.
            @param self -- toolbox.ShapesToolbar
            @param button -- a ToolButton to associate the palette.
            @param tool -- a dictionnary describing tool's properties.
            @param line_size_only -- indicates if palette should only display Line Size option. Default value is False.
        """
        palette = button.get_palette()
        
        # Fill option
        if not line_size_only:
            fill_checkbutton = gtk.CheckButton(_('Fill'))
            fill_checkbutton.show()
            fill_checkbutton.set_active(tool['fill'])
            
            fill_checkbutton.connect('toggled', self._on_fill_checkbutton_toggled, tool)
            
            palette.set_content(fill_checkbutton)
            
        size_spinbutton = gtk.SpinButton()
        size_spinbutton.show()
        
        # When inserted in a Palette, a spinbutton does not display text in black
        black = gtk.gdk.Color(0,0,0)
        size_spinbutton.modify_text(gtk.STATE_NORMAL, black)
        
        # This is where we set restrictions for size:
        # Initial value, minimum value, maximum value, step
        adj = gtk.Adjustment(tool['line size'], 1.0, 100.0, 1.0)
        size_spinbutton.set_adjustment(adj)
        
        size_spinbutton.set_numeric(True)
        
        label = gtk.Label(_('Size: '))
        label.show()
        
        palette.action_bar.pack_start(label)
        palette.action_bar.pack_start(size_spinbutton)
        
        size_spinbutton.connect('value-changed', self._on_line_size_value_changed, tool)
    
#     def _on_fill_checkbutton_map(self, checkbutton, data=None):
#         """
#         Update checkbutton condition to agree with Area.Area object; this prevents tools to have fill checked but be drawed not filled.
#         
#             @param self -- gtk.Toolbar
#             @param checkbutton
#             @param data
#         """
#         self._activity.area.fill = checkbutton.get_active()
    
    def _configure_palette_shape_line(self):
        logging.debug('Creating palette to shape line')
        self._create_simple_palette(self._shape_line, self._SHAPE_LINE, True)
    
##Make the Text Toolbar
class TextToolbar(gtk.Toolbar):

    _ACTION_TEXT = {
        'name'          : 'text',
        'line size'     : None,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : None,
        'fill'          : True,
        'vertices'      : None
    }
    ##The Constructor
    def __init__(self, activity):
        gtk.Toolbar.__init__(self)

        self._activity = activity

        self._text = ToolButton('text')
        self.insert(self._text, -1)
        self._text.show()
        self._text.set_tooltip(_('Type'))
        self._text.connect('clicked', self.set_tool, self._ACTION_TEXT)
        
        self._text_color = ButtonFillColor(activity)
        self._text_color.show()
        item = gtk.ToolItem()
        item.add(self._text_color)
        self.insert(item, -1)
        item.show()
        
        separator = gtk.SeparatorToolItem()
        separator.set_draw(True)
        self.insert(separator, -1)
        separator.show()
        
        
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
        new_color = self._text_color.get_color()
        tool['fill color'] = self._text_color.alloc_color(new_color)
        self._activity.area.set_tool(tool)
        
        
##Make the Images Toolbar
class ImageToolbar(gtk.Toolbar):

    _OBJECT_HEIGHT = 'height'
    _OBJECT_INSERT = 'insert'
    _OBJECT_ROTATE_LEFT = 'rotate-left'
    _OBJECT_ROTATE_RIGHT = 'rotate-right'
    _OBJECT_WIDTH = 'width'
    ##The Constructor
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
        
        self.width_percent = 1.
        self.height_percent = 1.

        """
        self._object_rotate_left = ToolButton('object-rotate-left')
        self.insert(self._object_rotate_left, -1)
        self._object_rotate_left.show()
        self._object_rotate_left.set_tooltip(_('Rotate Left'))
        
        self._object_rotate_right = ToolButton('object-rotate-right')
        self.insert(self._object_rotate_right, -1)
        self._object_rotate_right.show()
        self._object_rotate_right.set_tooltip(_('Rotate Right'))

        """
        
        self._object_height = ToolButton('object-height')
        self.insert(self._object_height, -1)
        self._object_height.show()
        self._object_height.set_tooltip(_('Height'))           

        height_spinButton = self._create_spinButton(self._object_height, 'object-height', activity)

        item = gtk.ToolItem()
        item.add(height_spinButton)
        self.insert(item, -1)
        item.show()

        self._object_width = ToolButton('object-width')
        self.insert(self._object_width, -1)
        self._object_width.show()
        self._object_width.set_tooltip(_('Width'))

        width_spinButton = self._create_spinButton(self._object_width, 'object-width', activity)
        
        item = gtk.ToolItem()
        item.add(width_spinButton)
        self.insert(item, -1)
        item.show()

#        self._object_height.connect('clicked', self.resize, activity, 'object-height', self._OBJECT_HEIGHT)

        self._object_insert.connect('clicked', self.insertImage, activity)
        #self._object_rotate_left.connect('clicked', self.rotate_left, activity)
        #self._object_rotate_right.connect('clicked', set_tool, activity, 'object-rotate-right', self._OBJECT_ROTATE_RIGHT)
#        self._object_width.connect('clicked', self.resize, activity, 'object-width', self._OBJECT_WIDTH)

    def _selected(self, widget, spin, activity):
        if not activity.area.is_selected():
            spin.set_value(100)
            self.width_percent = 1.
            self.height_percent = 1.
        try:
            del(activity.area.d.resize_pixbuf)
            del(activity.area.d.resized)
        except: pass

    def rotate_left(self, widget, activity):
        #activity.area._rotate_left(widget)
        pass

    def resize(self, spinButton, tool, activity):
        if activity.area.tool['name'] == 'marquee-rectangular' and activity.area.selmove:
            if tool == "object-height":
                self.height_percent = spinButton.get_value_as_int()/100.
                activity.area.d.resizeSelection(activity.area, self.width_percent, self.height_percent)
            elif tool == "object-width":
                self.width_percent = spinButton.get_value_as_int()/100.
                activity.area.d.resizeSelection(activity.area, self.width_percent, self.height_percent)
                
    def _create_spinButton(self, widget, tool, activity):
        """Set palette for a tool - width or height

            @param self -- gtk.Toolbar
            @param widget  - the widget which Palette will be set, a ToolButton object
            @param tool
            @param activity
        """
        logging.debug('setting a spinButton for %s', tool)
        
        spin = gtk.SpinButton()
        spin.show()
        
        # This is where we set restrictions for Resizing:
        # Initial value, minimum value, maximum value, step
        initial = float(100)
        adj = gtk.Adjustment(initial, 10.0, 500.0, 1.0)
        spin.set_adjustment(adj)
        spin.set_numeric(True)
        
        spin.connect('value-changed', self.resize, tool, activity)
        activity.area.connect('selected', self._selected, spin, activity)

        return spin
        
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
            activity.area.loadImage(file_path,widget,True)
        elif response == gtk.RESPONSE_CANCEL:
            logging.debug('Closed, no files selected')
            
        dialog.destroy()


##Make the Effects Tools Toolbar
class EffectsToolbar(gtk.Toolbar):

    _EFFECT_GRAYSCALE = 'grayscale'
    # Rainbow acts as a tool in Area, and it has to be described as a dict
    _EFFECT_RAINBOW = {
        'name'          : 'rainbow',
        'line size'     : 10,
        'fill color'    : None,
        'stroke color'  : None,
        'line shape'    : 'circle',
        'fill'          : True,
        'vertices'      : None
    }
    
    
    ##The Constructor
    def __init__(self, activity):
        gtk.Toolbar.__init__(self)
        
        self._activity = activity
	    

        self._effect_grayscale = ToolButton('effect-grayscale')
        self.insert(self._effect_grayscale, -1)
        self._effect_grayscale.show()
        self._effect_grayscale.set_tooltip(_('Grayscale'))
        
        self._effect_rainbow = ToolButton('effect-raindow')
        self.insert(self._effect_rainbow, -1)
        self._effect_rainbow.show()
        self._effect_rainbow.set_tooltip(_('Rainbow'))
        self._configure_palette(self._effect_rainbow, self._EFFECT_RAINBOW)
	
        separator = gtk.SeparatorToolItem()
        self.insert(separator, -1)
        separator.show()
        
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
        self._effect_grayscale.connect('clicked', self.grayscale)
        self._effect_rainbow.connect('clicked', self.rainbow)
        
    ##Make the colors be in grayscale
    def grayscale(self, widget):	
        self._activity.area.grayscale(widget)
        
    ##Like the brush, but change it color when painting
    def rainbow(self, widget):
        self._activity.area.set_tool(self._EFFECT_RAINBOW)
        
        # setting cursor: moved to Area
        
    def _configure_palette(self, button, tool=None):
        """Set palette for a tool
            @param self -- toolbox.EffectsToolbar
            @param widget  - the ToolButton which Palette will be set
            @param tool -- a dictionnary describing tool's properties.
        """
        
        logging.debug('setting a palette for %s', tool)
        
        palette = button.get_palette()
        
        if tool is None:
            return
        elif tool is self._EFFECT_RAINBOW:
            # We can adjust 'Line Size' and 'Line Shape' here
            
            # Line Size
            size_spinbutton = gtk.SpinButton()
            size_spinbutton.show()
            
            black = gtk.gdk.Color(0,0,0)
            size_spinbutton.modify_text(gtk.STATE_NORMAL, black)
            
            # This is where we set restrictions for Rainbow:
            # Initial value, minimum value, maximum value, step
            adj = gtk.Adjustment(tool['line size'], 1.0, 100.0, 1.0)
            size_spinbutton.set_adjustment(adj)
            
            size_spinbutton.set_numeric(True)
            
            label = gtk.Label(_('Size: '))
            label.show()
            
            palette.action_bar.pack_start(label)
            palette.action_bar.pack_start(size_spinbutton)
            
            size_spinbutton.connect('value-changed', self._on_size_value_changed, tool)
            
            # Line Shape
            item1 = gtk.RadioButton(None, _('Circle'))
            item1.show()
            item1.set_active(True)
            
            image1 = gtk.Image()
            image1.set_from_file('./icons/tool-shape-ellipse.svg')
            image1.show()
            item1.set_image(image1)
            
            item2 = gtk.RadioButton(item1, _('Square'))
            item2.show()
            
            image2 = gtk.Image()
            image2.set_from_file('./icons/tool-shape-rectangle.svg')
            image2.show()
            item2.set_image(image2)
            
            item1.connect('toggled', self._on_radio_toggled, tool, 'circle')
            item2.connect('toggled', self._on_radio_toggled, tool, 'square')
            
            label = gtk.Label(_('Shape'))
            label.show()
            
            vbox = gtk.VBox()
            vbox.show()
            
            vbox.pack_start(label)
            vbox.pack_start(item1)
            vbox.pack_start(item2)
            
            separator = gtk.HSeparator()
            vbox.pack_end(separator)
            separator.show()
            
            palette.set_content(vbox)
            
    def _on_size_value_changed(self, spinbutton, tool):
#         size = spinbutton.get_value_as_int()
#         self._activity.area.configure_line(size)
        
        tool['line size'] = spinbutton.get_value_as_int()
        self.rainbow(self._effect_rainbow)
        
    def _on_radio_toggled(self, radiobutton, tool, shape):
        if radiobutton.get_active():
            tool['line shape'] = shape
            self.rainbow(self._effect_rainbow)
        
##Make the View Toolbar
class ViewToolbar(gtk.Toolbar):

    _ACTION_1000 = 0
    _ACTION_500 = 1
    _ACTION_200 = 2
    _ACTION_150 = 3
    _ACTION_100 = 4
    _ACTION_50 = 5
    _ACTION_25 = 6
    _ACTION_10 = 7

    ##The Constructor
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



