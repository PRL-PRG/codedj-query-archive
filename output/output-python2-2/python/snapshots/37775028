from lib.Depend.gtk2 import gtk
from lib.Depend.gtk2 import gobject
from lib.Depend.gtk2 import pango

class CellRendererButton(gtk.GenericCellRenderer):
    __gproperties__ = {
        "text": (gobject.TYPE_STRING, None, "Text",
        "Displayed text", gobject.PARAM_READWRITE),
    }
    
    __gsignals__ = {
        'click' :        (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE,
                           (gobject.TYPE_STRING, )),
    }

    def __init__(self):
        self.__gobject_init__()
        
        self.text = ""
        self.width = 30
        
        self.set_property('mode', gtk.CELL_RENDERER_MODE_EDITABLE)
    
    def do_set_property(self, pspec, value):
        setattr(self, pspec.name, value)
    
    def do_get_property(self, pspec):
        return getattr(self, pspec.name)

    def on_render(self, window, widget, background_area, cell_area, expose_area, flags):
        tid = 0
        if flags & gtk.CELL_RENDERER_SELECTED:
            tid = 3
        layout = widget.create_pango_layout(self.text)
        layout.set_font_description(widget.style.font_desc)
        w, h = layout.get_size()
        self.width = w / pango.SCALE + 6
        x = cell_area.x + 3
        y = int(cell_area.y + (cell_area.height - h / pango.SCALE) / 2)
        widget.style.paint_box(window, gtk.STATE_NORMAL, gtk.SHADOW_OUT, None, widget, "button",
                cell_area.x, cell_area.y, self.width, cell_area.height)
        window.draw_layout(widget.style.text_gc[tid], x, y, layout)

    def on_get_size(self, widget, cell_area=None):
        if cell_area is None:
            return (0, 0, self.width, 18)
        else:
            return (cell_area.x, cell_area.y, cell_area.width, cell_area.height)
    
    def on_start_editing(self, event, widget, path, background_area, cell_area, flags):
        if event is not None and event.type == gtk.gdk.BUTTON_PRESS and \
            cell_area.x < event.x < cell_area.x + self.width and \
            cell_area.y < event.y < cell_area.y + cell_area.height:
            self.emit("click", path)

gobject.type_register(CellRendererButton)
