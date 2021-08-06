import hippo
import pango
from sugar.graphics import style

class InfoPanel(hippo.CanvasBox):
    def __init__(self):
        hippo.CanvasBox.__init__(self, spacing=4, padding=5,
                orientation=hippo.ORIENTATION_VERTICAL)
        self.status_box = hippo.CanvasBox(spacing=4, padding=5,
                orientation=hippo.ORIENTATION_VERTICAL)
        self.append(self.status_box)

    def show(self, text):
        textwidget = hippo.CanvasText(text=text,
            font_desc=pango.FontDescription('Sans 10'),
            color=style.COLOR_WHITE.get_int(),
            xalign=hippo.ALIGNMENT_CENTER)
        self.status_box.remove_all()
        self.status_box.append(textwidget)

