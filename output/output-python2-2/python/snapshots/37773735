import gtk, gtk.gdk, gobject, gtk.keysyms

from lib.consts import SELECT_SQUARE_COLOR, SELECT_SQUARE_SIZE

from common import CWidget, event
from lib.Drawing import CDrawingArea, CElement, CConnection

from lib.Elements import CElementObject
from lib.Connections import CConnectionObject

from lib.Drawing.Canvas import GtkCanvas

targets = [('document/uml', 0, gtk.TARGET_SAME_WIDGET)]

class CpicDrawingArea(CWidget):
    name = 'picDrawingArea'
    widgets = ('picDrawingArea', 'picEventBox', 'picVBar', 'picHBar', 'tbDrawingArea', 'vbAll', 'nbTabs')
    
    __gsignals__ = {
        'get-selected':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_PYOBJECT, 
            ()), 
        'set-selected':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, 
            (gobject.TYPE_PYOBJECT, )), 
        'selected-item':  (gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, 
            (gobject.TYPE_PYOBJECT, )), 
        'add-element':(gobject.SIGNAL_RUN_LAST, gobject.TYPE_NONE, (gobject.TYPE_PYOBJECT,gobject.TYPE_PYOBJECT,)), 
    }
    
    def __init__(self, app, wTree):
        CWidget.__init__(self, app, wTree)
        
        self.__NewConnection = None
        self.dnd = None
        self.selecting = None
        
        self.Buffer = gtk.gdk.Pixmap(self.picDrawingArea.window, 1000, 1000)
        self.DrawingArea = CDrawingArea(None,"Start page")
        self.canvas = GtkCanvas(self.picDrawingArea, self.Buffer)
        
        cmap = self.picDrawingArea.window.get_colormap()
        self.DragGC = self.picDrawingArea.window.new_gc(foreground = cmap.alloc_color(SELECT_SQUARE_COLOR), 
            function = gtk.gdk.XOR, line_width = SELECT_SQUARE_SIZE)
            
        self.AdjustScrollBars()
        self.Hide()
        self.Paint()
        
    def Hide(self):
        self.vbAll.set_child_packing(self.nbTabs, True, True, 0, gtk.PACK_START)
        self.tbDrawingArea.hide()
    
    def Show(self):
        self.vbAll.set_child_packing(self.nbTabs, False, True, 0, gtk.PACK_START)
        self.tbDrawingArea.show()
    
    def GetDrawingArea(self):
        return self.picDrawingArea
    
    def SetDrawingArea(self, drawingArea):
        self.DrawingArea = drawingArea
        self.Paint()
        
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
        if self.dnd == 'rect':
            self.__DrawDragRect(0,0, True, False)
        elif self.dnd == 'point':
            self.__DrawDragPoint(None, None, True, False)
        
        if self.__NewConnection is not None:
            self.__DrawNewConnection(None, None, False)
        
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
        self.picDrawingArea.grab_focus()
        if event.button == 1:
            toolBtnSel =  self.emit('get-selected')
            if toolBtnSel is not None:
                self.__AddItem(toolBtnSel, event)
                return
                
            pos = self.GetAbsolutePos(event.x, event.y)
            itemSel = self.DrawingArea.GetElementAtPosition(self.canvas, pos)
            if itemSel is not None:
                if itemSel in self.DrawingArea.GetSelected():
                    if (event.state & gtk.gdk.CONTROL_MASK):
                        self.DrawingArea.RemoveFromSelection(itemSel)
                        self.Paint()
                    elif isinstance(itemSel, CConnection):
                        i = itemSel.GetPointAtPosition(pos)
                        if i is not None:
                            itemSel.SelectPoint(i)
                            self.__BeginDragPoint(event, itemSel, i)
                            #self.Paint()
                        else:
                            itemSel.DeselectPoint()
                            i = itemSel.WhatPartOfYouIsAtPosition(self.canvas, pos)
                            self.__BeginDragLine(event, itemSel, i)
                            #self.Paint()
                    else:
                        self.__BeginDragRect(event)
                elif not (event.state & gtk.gdk.CONTROL_MASK):
                    self.DrawingArea.DeselectAll()
                    self.DrawingArea.AddToSelection(itemSel)
                    self.emit('selected-item', itemSel)
                    if isinstance(itemSel, CConnection):
                        i = itemSel.GetPointAtPosition(pos)
                        if i is not None:
                            itemSel.SelectPoint(i)
                            self.__BeginDragPoint(event, itemSel, i)
                            #self.Paint()
                        else:
                            itemSel.DeselectPoint()
                            i = itemSel.WhatPartOfYouIsAtPosition(self.canvas, pos)
                            self.__BeginDragLine(event, itemSel, i)
                            #self.Paint()
                    else:
                        self.__BeginDragRect(event)
                    #self.Paint()
                else:
                    self.DrawingArea.AddToSelection(itemSel)
                    self.emit('selected-item', None)
                    self.Paint()
                
            elif self.DrawingArea.SelectedCount() > 0:
                if not (event.state & gtk.gdk.CONTROL_MASK):
                    self.DrawingArea.DeselectAll()
                    self.emit('selected-item', None)
                    self.Paint()
        
    def __AddItem(self, toolBtnSel, event):
        pos = self.GetAbsolutePos(event.x, event.y)
        if toolBtnSel[0] == 'Element':
            ElementType = self.application.ElementFactory.GetElement(toolBtnSel[1])
            ElementObject = CElementObject(ElementType)
            CElement(self.DrawingArea, ElementObject).SetPosition(pos)
            self.emit('set-selected', None)
            self.emit('add-element', ElementObject, self.DrawingArea)
            self.Paint()
        
        elif toolBtnSel[0] == 'Connection':
            itemSel = self.DrawingArea.GetElementAtPosition(self.canvas, pos)
            
            if itemSel is None:
                if self.__NewConnection is not None:
                    pass
            elif isinstance(itemSel, CConnection):
                return
            elif self.__NewConnection is None:
                ConnectionType = self.application.ConnectionFactory.GetConnection(toolBtnSel[1])
                center = itemSel.GetCenter(self.canvas)
                self.__NewConnection = (CConnectionObject(ConnectionType), [center], itemSel)
                self.__NewConnection[0].SetSource(itemSel.GetObject())
                self.__DrawNewConnection( center[0], center[1], False )
            else:
                pass
        
    @event("picEventBox", "key-press-event")
    def on_key_press_event(self, widget, event):
        if event.keyval == gtk.keysyms.Delete:
            for sel in self.DrawingArea.GetSelected():
                if isinstance(sel, CConnection):
                    index = sel.GetSelectedPoint()
                    if index is not None and (sel.GetSource() != sel.GetDestination() or len(tuple(sel.GetMiddlePoints())) > 2):
                        sel.RemovePoint(self.canvas, index)
                        self.Paint()
                        return
            for sel in self.DrawingArea.GetSelected():
                self.DrawingArea.DeleteItem(sel)
            self.Paint()
        elif event.keyval == gtk.keysyms.Escape:
            self.ResetAction()
            self.emit('set-selected', None)
            
        
    @event("picEventBox", "button-release-event")
    def on_button_release_event(self, widget, event):
        pos = self.GetAbsolutePos(event.x, event.y)
        if self.dnd == 'rect':
            delta = self.__GetDelta(event.x, event.y)
            self.DrawingArea.MoveSelection(delta)
            self.dnd = None
            self.Paint()
        elif self.dnd == 'point':            
            point = self.GetAbsolutePos(event.x, event.y)
            connection, index = self.DragPoint
            connection.MovePoint(self.canvas, point, index)
            self.dnd = None
            self.Paint()
        elif self.dnd == 'line':
            point = self.GetAbsolutePos(event.x, event.y)
            connection, index = self.DragPoint
            connection.AddPoint(self.canvas, point, index)
            self.dnd = None
            self.Paint()
        elif self.__NewConnection is not None:
            itemSel = self.DrawingArea.GetElementAtPosition(self.canvas, pos)
            if itemSel is None:
                self.__NewConnection[1].append(pos)
                self.__DrawNewConnection( None, None )
            elif itemSel is not self.__NewConnection[2] or len(self.__NewConnection[1]) > 2:
                self.__NewConnection[0].SetDestination(itemSel.GetObject())
                (obj, points, source), destination = self.__NewConnection, itemSel
                x = CConnection(self.DrawingArea, obj, source, destination, points[1:])
                self.__NewConnection = None
                self.emit('set-selected', None)
                self.Paint()
            else:
                pass
        
    @event("picEventBox", "motion-notify-event")
    def on_motion_notify_event(self, widget, event):
        if self.dnd == 'rect':
            self.__DrawDragRect(event.x, event.y)
        elif self.dnd == 'point':
            self.__DrawDragPoint(event.x, event.y)
        elif self.dnd == 'line':
            self.__DrawDragLine(event.x, event.y)
        elif self.__NewConnection is not None:
            self.__DrawNewConnection(event.x, event.y)
        
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
        
    @event("picDrawingArea", "focus-out-event")
    def on_picDrawingArea_foucus_out_event(self, widget, event):
        self.emit('set-selected', None)
        self.ResetAction()
        
    def __Scroll(self, scrollbar, direction):
        tmp = scrollbar.get_adjustment()
        if direction == gtk.gdk.SCROLL_UP:
            tmp.value = max(tmp.lower, tmp.value - 20)
        elif direction == gtk.gdk.SCROLL_DOWN:
            tmp.value = min(tmp.upper - tmp.page_size, tmp.value + 20)
        scrollbar.set_adjustment(tmp)
        
    def __BeginDragRect(self, event):
        self.DragStartPos = self.GetAbsolutePos(event.x, event.y)
        self.DragRect = self.DrawingArea.GetSelectSquare(self.canvas)
        self.__DrawDragRect(event.x, event.y, False)
        self.dnd = 'rect'
        
    def __BeginDragPoint(self, event, connection, point):
        self.DragStartPos = self.GetAbsolutePos(event.x, event.y)
        self.DragPoint = (connection, point)
        self.__DrawDragPoint(event.x, event.y, False)
        self.dnd = 'point'
        
    def __BeginDragLine(self, event, connection, point):
        self.DragStartPos = self.GetAbsolutePos(event.x, event.y)
        self.DragPoint = (connection, point)
        self.__DrawDragLine(event.x, event.y, False)
        self.dnd = 'line'
        
    def __GetDelta(self, x, y):
        sizx, sizy = self.GetDrawingAreaSize()
        selx, sely = self.DragRect[1]
        sizx, sizy = sizx - selx, sizy - sely
        tmpx, tmpy = self.GetAbsolutePos(x,y)
        dx, dy = tmpx - self.DragStartPos[0], tmpy - self.DragStartPos[1]
        posx, posy = self.DragRect[0]
        tmpx, tmpy = posx + dx, posy + dy
        tmpx = min(max(0, tmpx), sizx)
        tmpy = min(max(0, tmpy), sizy)
        return int(tmpx - posx), int(tmpy - posy)
        
    
    def __DrawDragRect(self, x, y, erase = True, draw = True):
        if erase:
            self.picDrawingArea.window.draw_rectangle(self.DragGC, False, self.__oldx, self.__oldy, *self.DragRect[1])
        if draw:
            tmpx, tmpy = self.GetRelativePos(*self.DragRect[0])
            dx, dy = self.__GetDelta(x, y)
            self.picDrawingArea.window.draw_rectangle(self.DragGC, False, tmpx + dx, tmpy + dy, *self.DragRect[1])
            self.__oldx, self.__oldy = tmpx + dx, tmpy + dy
            
    def __DrawDragPoint(self, x, y, erase = True, draw = True):
        if x is None:
            x, y = self.__oldPoints2
        connection, index = self.DragPoint
        prev, next = connection.GetNeighbours(index, self.canvas)
        points = [self.GetRelativePos(*prev), (int(x), int(y)), self.GetRelativePos(*next)]
        if erase:
            self.picDrawingArea.window.draw_lines(self.DragGC, self.__oldPoints)
        if draw:
            self.__oldPoints = points
            self.__oldPoints2 = self.GetAbsolutePos(x, y)
            self.picDrawingArea.window.draw_lines(self.DragGC, self.__oldPoints)
    
    def __DrawDragLine(self, x, y, erase = True, draw = True):
        if x is None:
            x, y = self.__oldPoints2
        connection, index = self.DragPoint
        all = tuple(connection.GetPoints(self.canvas))
        prev, next = all[index], all[index + 1]
        points = [self.GetRelativePos(*prev), (int(x), int(y)), self.GetRelativePos(*next)]
        if erase:
            self.picDrawingArea.window.draw_lines(self.DragGC, self.__oldPoints)
        if draw:
            self.__oldPoints = points
            self.__oldPoints2 = self.GetAbsolutePos(x, y)
            self.picDrawingArea.window.draw_lines(self.DragGC, self.__oldPoints)
    
    def __DrawNewConnection(self, x, y, erase = True, draw = True):
        if x is None:
            points = self.__NewConnection[1][:]
        else:
            points = self.__NewConnection[1]
        points = [self.GetRelativePos(*point) for point in points]
        if x is not None:
            points += [(int(x), int(y))]
        if erase:
            self.picDrawingArea.window.draw_lines(self.DragGC, self.__oldNewConnection)
        if draw:
            self.__oldNewConnection = points
            self.picDrawingArea.window.draw_lines(self.DragGC, self.__oldNewConnection)
    
    def ResetAction(self):
        self.dnd = None
        if self.__NewConnection is not None:
            self.__NewConnection = None
        self.Paint()
        