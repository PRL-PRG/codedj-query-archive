from lib.lib import UMLException
import pango
import lib.consts

class CDrawingArea:
    def __init__(self, widget, drawable = None):
        self.widget = widget
        self.font = pango.FontDescription(lib.consts.FONT_TYPE)
        self.pango = (self.widget.create_pango_context(), self.widget.create_pango_layout(""))
        self.pango[1].set_font_description(self.font)
        self.elements = []
        self.elementsreverse = []
        self.connections = []
        self.selected = set()
        
        if drawable is None:
            self.drawable = self.widget.window
        else:
            self.drawable = drawable
        
    def AddElement(self, element):
        if element not in self.elements:
            self.elements.append(element)
            self.elementsreverse.insert(0, element)
        else:
            raise UMLException("ElementAlreadyExists")
     
    def GetSelected(self):
        for i in self.selected:
            yield i     
            
    def AddConnection(self, connection):
        if connection not in self.connections:
            self.connections.append(connection)
        else:
            raise UMLException("ConnectionAlreadyExists")
            
    def SelectedCount(self):
        return len(self.selected)
    
    def AddToSelection(self, element):
        self.selected.add(element)
        element.Select()
    
    def RemoveFromSelection(self, element):
        self.selected.remove(element)
        element.Deselect()
    
    def DeselectAll(self):
        for e in self.selected:
            e.Deselect()
        self.selected = set()
            
    def SelectAll(self):
        for e in self.elements:
            self.selected.add(e)
            e.Select()
        
        for c in self.connections:
            self.selected.add(c)
            c.Select()
    
    def GetSelectSquare(self):
        x1, y1 = self.GetSize()
        x2, y2 = 0, 0
        
        for el in self.selected:
            x, y = el.GetPosition()
            w, h = el.GetSize()
            if x < x1:
                x1 = x
            if y < y1:
                y1 = y
            if x + w > x2:
                x2 = x + w
            if y + h > y2:
                y2 = y + h
        return (x1, y1), (x2 - x1, y2 - y1)
    
    def MoveSelection(self, deltax, deltay):
        for el in self.selected:
            x, y = el.GetPosition()
            el.SetPosition(x + deltax, y + deltay)
        
    def DeleteElement(self, element):
        if element in self.elements:
            self.elements.remove(element)
            self.elementsreverse.remove(element)
        else:
            raise UMLException("ElementDoesNotExists")
        
    def DeleteConnection(self, connection):
        if connection in self.connections:
            self.connections.remove(connection)
        else:
            raise UMLException("ConnectionDoesNotExists")

    def GetSize(self):
        return (1000, 1000)
        
    def GetDrawable(self):
        return self.drawable        
        
    def GetElementAtPosition(self, x, y):
        for c in self.connections:
            if c.AreYouAtPosition(x, y):
                return c
                
        for e in self.elementsreverse:
            if e.AreYouAtPosition(x, y):
                return e
            
        return None
        
    def GetWidget(self):
        return self.widget
    
    def GetPango(self):
        return self.pango

    def Paint(self):
        gc = self.widget.get_style().white_gc
        self.drawable.draw_rectangle(gc, True, 0, 0, *self.GetSize())
        for e in self.elements:
            e.Paint()
        
        for c in self.connections:
            c.Paint()
