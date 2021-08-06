import gtk, gtk.gdk, gobject

from lib.consts import SELECT_SQUARE_COLOR, SELECT_SQUARE_SIZE

from common import CWidget, event
from lib.Drawing import CDrawingArea, CElement, CConnection

from lib.Drawing import CDrawingArea, CElement, CConnection
from lib.Elements import CElementObject
from lib.Connections import CConnectionObject

from lib.Drawing.Canvas import GtkCanvas

targets = [('document/uml', 0, gtk.TARGET_SAME_WIDGET)]

class CpicDrawingArea(CWidget):
    name = 'picDrawingArea'
    widgets = ('picDrawingArea', 'picEventBox', 'picVBar', 'picHBar')
    
    __gsignals__ = {
        'get-selected':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_PYOBJECT, 
            ()), 
        'set-selected':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, 
            (gobject.TYPE_PYOBJECT, )), 
        'selected-item':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, 
            (gobject.TYPE_PYOBJECT, )), 
        

    }
    dnd = False
    
    def __init__(self, app, wTree):
        CWidget.__init__(self, app, wTree)
        
        self.NewConnection = None
        #self.picEventBox.drag_dest_set(gtk.DEST_DEFAULT_ALL, targets, gtk.gdk.ACTION_MOVE)
        self.Buffer = gtk.gdk.Pixmap(self.picDrawingArea.window, 1000, 1000)
        #self.DrawingArea = CDrawingArea(self.picDrawingArea, self.Buffer)
        self.DrawingArea = CDrawingArea()
        self.canvas = GtkCanvas(self.picDrawingArea, self.Buffer)
        
        self.AdjustScrollBars()
        self.Paint()
        
        
    def GetDrawingArea(self):
        return self.picDrawingArea
        
    def GetWindowSize(self):
        tmpx, tmpy =  self.picDrawingArea.window.get_size()
        return (int(tmpx), (tmpy))
        
    def GetDrawingAreaSize(self):
        tmp = [int(max(i)) for i in zip(self.DrawingArea.GetSize(), self.picDrawingArea.window.get_size())]
        return tuple(tmp)
        
    def GetAbsolutePos(self, posx, posy):
        return int(self.picHBar.get_value() + posx), int(self.picVBar.get_value() + posy)
        
    def GetRelativePos(self, posx, posy):
        return int(-self.picHBar.get_value() + posx), int(-self.picVBar.get_value() + posy)
        
    def Paint(self):
        self.DrawingArea.Paint(self.canvas)
        self.Repaint()
        
    def Repaint(self):
        posx, posy = int(self.picHBar.get_value()), int(self.picVBar.get_value())
        sizx, sizy = self.GetWindowSize()
        wgt = self.picDrawingArea.window
        gc = wgt.new_gc()
        wgt.draw_drawable(gc, self.Buffer, posx, posy, 0, 0, sizx, sizy)
        if self.dnd:
            self.__DrawDragRect(0,0, True, False)
        
    def AdjustScrollBars(self):
        dasx, dasy = self.GetDrawingAreaSize()
        wisx, wisy = self.GetWindowSize()
        
        tmp = self.picHBar.get_adjustment()
        tmp.upper = dasx 
        tmp.page_size = wisx
        self.picHBar.set_adjustment(tmp)
        
        tmp = self.picVBar.get_adjustment()
        tmp.upper = dasy 
        tmp.page_size = wisy
        self.picVBar.set_adjustment(tmp)
        
    @event("picEventBox", "button-press-event")
    def on_picEventBox_button_press_event(self, widget, event):
        toolBtnSel =  self.emit('get-selected')
        posx, posy = self.GetAbsolutePos(event.x, event.y)
        if toolBtnSel is None:
            itemSel = self.DrawingArea.GetElementAtPosition(self.canvas, posx, posy)
            if itemSel is not None:
                if itemSel in self.DrawingArea.GetSelected():
                    if (event.state & gtk.gdk.CONTROL_MASK):
                        self.DrawingArea.RemoveFromSelection(itemSel)
                        self.Paint()
                    elif event.button == 1:
                        self.__DragBegin(event)
                    else:
                        self.Paint()
                elif not (event.state & gtk.gdk.CONTROL_MASK):
                    self.DrawingArea.DeselectAll()
                    self.DrawingArea.AddToSelection(itemSel)
                    self.emit('selected-item', itemSel)
                    if event.button == 1:
                        self.__DragBegin(event)
                    self.Paint()
                else:
                    self.DrawingArea.AddToSelection(itemSel)
                    self.emit('selected-item', None)
                    self.Paint()
            elif self.DrawingArea.SelectedCount() > 0:
                if not (event.state & gtk.gdk.CONTROL_MASK):
                    self.DrawingArea.DeselectAll()
                    self.emit('selected_item', None)
                    self.Paint()
        
        elif toolBtnSel[0] == 'Element':
            ElementType = self.application.ElementFactory.GetElement(toolBtnSel[1])
            ElementObject = CElementObject(ElementType)
            CElement(self.DrawingArea, ElementObject).SetPosition(posx, posy)
            self.emit('set-selected', None)
            self.Paint()
            
        elif toolBtnSel[0] == 'Connection':
            itemSel = self.DrawingArea.GetElementAtPosition(self.canvas, posx, posy)
            if itemSel is None:
                if self.NewConnection is not None:
                    self.NewConnection[1].append((posx, posy))
                    print self.NewConnection[1]
            elif self.NewConnection is None:
                ConnectionType = self.application.ConnectionFactory.GetConnection(toolBtnSel[1])
                self.NewConnection = (CConnectionObject(ConnectionType), [(posx, posy)])
                self.NewConnection[0].SetSource(itemSel)
                print "selected 1"
            else:
                self.NewConnection[0].SetDestination(itemSel)
                self.NewConnection[1].append((posx, posy))
                CConnection(self.DrawingArea, *self.NewConnection)
                self.NewConnection = None
                self.emit('set-selected', None)
                self.Paint()
                print "selected 2"
        
    @event("picEventBox", "button_release_event")
    def on_button_release_event(self, widget, event):
        if self.dnd:
            dx, dy = self.__GetDelta(event.x, event.y)
            self.DrawingArea.MoveSelection(dx, dy)
            self.dnd = False
            self.Paint()
        
    @event("picEventBox", "motion_notify_event")
    def on_motion_notify_event(self, widget, event):
        if self.dnd:
            self.__DrawDragRect(event.x, event.y)
        
    @event("picDrawingArea", "expose-event")
    def on_picDrawingArea_configure_event(self, widget, tmp):
        self.Repaint()
        
    @event("picDrawingArea", "expose-event")
    def on_picDrawingArea_expose_event(self, widget, tmp):
        self.Repaint()
        
    @event("picVBar", "value-changed")
    def on_picVBar_value_changed(self, widget):
        self.Repaint()
        
    @event("picHBar", "value-changed")
    def on_picHBar_value_changed(self, widget):
        self.Repaint()
        
    @event("picDrawingArea", "size-allocate")
    def on_picDrawingArea_size_allocate(self, widget, tmp):
        self.AdjustScrollBars()
        self.Repaint()
        
    @event("picEventBox", "scroll-event")
    def on_picEventBox_scroll_event(self, widget, event):
        if  event.state & gtk.gdk.SHIFT_MASK :
            self.__Scroll(self.picHBar, event.direction)
        else:
            self.__Scroll(self.picVBar, event.direction)
        self.Repaint()
        
    
    def __Scroll(self, scrollbar, direction):
        tmp = scrollbar.get_adjustment()
        if direction == gtk.gdk.SCROLL_UP:
            tmp.value = max(tmp.lower, tmp.value - 20)
        elif direction == gtk.gdk.SCROLL_DOWN:
            tmp.value = min(tmp.upper - tmp.page_size, tmp.value + 20)
        scrollbar.set_adjustment(tmp)
        
    def __DragBegin(self, event):
        #self.picEventBox.drag_begin(targets, gtk.gdk.ACTION_MOVE, event.button, event)
        self.DragStartPos = self.GetAbsolutePos(event.x, event.y)
        self.DragRect = self.DrawingArea.GetSelectSquare(self.canvas)
        cmap = self.picDrawingArea.window.get_colormap()
        self.DragGC = self.picDrawingArea.window.new_gc(foreground = cmap.alloc_color(SELECT_SQUARE_COLOR), 
            function = gtk.gdk.XOR, line_width = SELECT_SQUARE_SIZE)
        self.__DrawDragRect(event.x, event.y, False)
        self.dnd = True
        
    def  __GetDelta(self, x, y):
        sizx, sizy = self.GetDrawingAreaSize()
        selx, sely = self.DragRect[1]
        sizx, sizy = sizx - selx, sizy - sely
        tmpx, tmpy = self.GetAbsolutePos(x,y)
        dx, dy = tmpx - self.DragStartPos[0], tmpy - self.DragStartPos[1]
        posx, posy = self.DragRect[0]
        tmpx, tmpy = posx + dx, posy + dy
        tmpx = min(max(0, tmpx), sizx)
        tmpy = min(max(0, tmpy), sizy)
        return tmpx - posx, tmpy - posy
        
    
    def __DrawDragRect(self, x, y, erase = True, draw = True):
        if erase:
            self.picDrawingArea.window.draw_rectangle(self.DragGC, False, self.__oldx, self.__oldy, *self.DragRect[1])
        if draw:
            tmpx, tmpy = self.GetRelativePos(*self.DragRect[0])
            dx, dy = self.__GetDelta(x, y)
            self.picDrawingArea.window.draw_rectangle(self.DragGC, False, int(tmpx + dx), int(tmpy + dy), *self.DragRect[1])
            self.__oldx, self.__oldy = int(tmpx + dx), int(tmpy + dy)
