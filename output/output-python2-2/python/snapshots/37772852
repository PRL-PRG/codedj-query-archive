from common import CWidget
import gtk
import gtk.gdk
import gobject

from lib.consts import STARTPAGE_IMAGE
from common import  event

gtk.rc_parse_string("""
    style "test"
    {
        bg_pixmap[NORMAL] = "<none>"
    } widget "*.ebStartPage" style "test"
""")
import os.path


class CtabStartPage(CWidget):
    name = 'tabStartPage'
    widgets = ('ebStartPage', 'fixStartPage', 'lblCreateProject', 'lblOpenProject', 'lblAbout', 'ebAbout', 'ebCreate', 'ebOpen',
                'ebRecent1', 'ebRecent2', 'ebRecent3', 'ebRecent4', 'ebRecent5', 'lblRecent1', 'lblRecent2', 'lblRecent3', 'lblRecent4', 'lblRecent5',)
    
    __gsignals__ = {
        'open-about-dialog':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, ()), 
        'open-project': (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_PYOBJECT,)),
        'open-file':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_PYOBJECT,)), 
        }
    
    def __init__(self, app, wTree):
        CWidget.__init__(self, app, wTree)
        
        style = self.ebStartPage.get_style().copy()
        pixbuf = gtk.gdk.pixbuf_new_from_file(self.GetRelativeFile(STARTPAGE_IMAGE))
        pixmap = gtk.gdk.Pixmap(self.ebStartPage.window, 2000, 2000)
        cmap = self.ebStartPage.get_colormap()
        gc = self.ebStartPage.window.new_gc(foreground = cmap.alloc_color("#CDCDFF"))
        pixmap.draw_rectangle(gc, True, 0, 0, 2000, 2000)
        pixbuf.render_to_drawable(pixmap, gc, 0, 0, 0, 0, -1, -1, 0, 0, 0)
        style.bg_pixmap[gtk.STATE_NORMAL] = pixmap
        self.ebStartPage.set_style(style)
        
        self.ebAbout.set_visible_window(False)
        self.ebCreate.set_visible_window(False)
        self.ebOpen.set_visible_window(False)
        self.hand = gtk.gdk.Cursor(gtk.gdk.HAND2)
        self.default = None
        
        for lbl in (self.lblOpenProject, self.lblCreateProject, self.lblAbout):
            lbl.set_label(u"<span font_desc=\"Arial bold single 10\"><u>%s</u></span>"%lbl.get_label())
        
        self.ebRecent1.set_visible_window(False)
        self.ebRecent2.set_visible_window(False)
        self.ebRecent3.set_visible_window(False)
        self.ebRecent4.set_visible_window(False)
        self.ebRecent5.set_visible_window(False)
        self.Labels = [self.lblRecent1, self.lblRecent2, self.lblRecent3, self.lblRecent4, self.lblRecent5]
        self.Paths = []
        self.Fill()
        
    def Fill(self):
        self.Paths = []
        for lbl in self.Labels:
            lbl.set_label("")
        for id, i in enumerate(self.application.GetRecentFiles().GetRecentFiles()):
            if id <5:
                self.Labels[id].set_label("<span font_desc=\"Arial bold single 10\"><u>%s</u></span>"%os.path.basename(i[0]))
                self.Paths.append(i[0])
    
    @event("ebRecent1", "button_press_event", 0)
    @event("ebRecent2", "button_press_event", 1)
    @event("ebRecent3", "button_press_event", 2)
    @event("ebRecent4", "button_press_event", 3)
    @event("ebRecent5", "button_press_event", 4)
    def on_ebRecent_button_press_event(self, widget, event, index):
        if index < len(self.Paths):
            self.application.GetRecentFiles().AddFile(self.Paths[index])
            self.emit('open-file',self.Paths[index])
            self.Fill()
    
    @event("ebRecent1", "enter-notify-event", 0)
    @event("ebRecent2", "enter-notify-event", 1)
    @event("ebRecent3", "enter-notify-event", 2)
    @event("ebRecent4", "enter-notify-event", 3)
    @event("ebRecent5", "enter-notify-event", 4)
    def on_ebRecent_enter_notify_event(self, widget, event, index):
        if index < len(self.Paths):
            widget.window.set_cursor(self.hand)
    
    @event("ebRecent1", "leave-notify-event", 0)
    @event("ebRecent2", "leave-notify-event", 1)
    @event("ebRecent3", "leave-notify-event", 2)
    @event("ebRecent4", "leave-notify-event", 3)
    @event("ebRecent5", "leave-notify-event", 4)
    def on_ebRecent_leave_notify_event(self, widget, event, index):
        if index < len(self.Paths):
            widget.window.set_cursor(self.default)
    
    @event("ebOpen", "button-press-event")
    def on_ebOpen_button_press_event(self, widget, event):
        self.emit('open-project',1)
        
    @event("ebAbout", "button-press-event")
    def on_ebAbout_button_press_event(self, widget, event):
        self.emit('open-about-dialog')
    
    @event("ebCreate", "button-press-event")
    def on_ebCreate_button_press_event(self, widget, event):
        self.emit('open-project',0)
    
    @event("ebAbout","enter-notify-event")
    def on_ebAbout_mouse_enter(self, widget, event):
        self.ebAbout.window.set_cursor(self.hand)
        
    @event("ebCreate","enter-notify-event")
    def on_ebCreate_mouse_enter(self, widget, event):
        self.ebCreate.window.set_cursor(self.hand)
    
    @event("ebOpen","enter-notify-event")
    def on_ebOpen_mouse_enter(self, widget, event):
        self.ebOpen.window.set_cursor(self.hand)
        
    @event("ebOpen","leave-notify-event")
    def on_ebOpen_mouse_leave(self, widget, event):
        self.ebOpen.window.set_cursor(self.default)
    
    @event("ebAbout","leave-notify-event")
    def on_ebAbout_mouse_leave(self, widget, event):
        self.ebAbout.window.set_cursor(self.default)
        
    @event("ebCreate","leave-notify-event")
    def on_ebCreate_mouse_leave(self, widget, event):
        self.ebCreate.window.set_cursor(self.default)
    