from CairoBase import CCairoBaseCanvas

class CCairoCanvas(CCairoBaseCanvas):
    def __init__(self, widget, window = None, storage = None):
        self.widget = widget
        if window is None:
            self.window = widget.window
        else:
            self.window = window
        CCairoBaseCanvas.__init__(self, self.window.cairo_create(), storage)

    def Clear(self):
        gc = self.widget.get_style().white_gc
        self.window.draw_rectangle(gc, True, 0, 0, *self.window.get_size())
