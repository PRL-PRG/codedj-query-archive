from lib.lib import UMLException
import lib.consts
import Connection, Element

class CDrawingArea:
    def __init__(self):
        self.elements = []
        self.connections = []
        self.selected = set()
        
    def AddElement(self, element):
        if element not in self.elements:
            self.elements.append(element)
        else:
            raise UMLException("ElementAlreadyExists")
     
    def GetSelected(self):
        selected = tuple(self.selected)
        for i in selected:
            if i in self.selected:
                yield i
            
    def GetSelectedElements(self):
        for i in self.selected:
            if isinstance(i, Element.CElement):
                yield i
            
    def GetSelectedConnections(self):
        for i in self.selected:
            if isinstance(i, Connection.CConnection):
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
    
    def GetSelectSquare(self, canvas):
        x1, y1 = self.GetSize()
        x2, y2 = 0, 0
        
        for el in self.GetSelectedElements():
            x, y = el.GetPosition()
            w, h = el.GetSize(canvas)
            if x < x1:
                x1 = x
            if y < y1:
                y1 = y
            if x + w > x2:
                x2 = x + w
            if y + h > y2:
                y2 = y + h
        return (int(x1), int(y1)), (int(x2 - x1), int(y2 - y1))
    
    def MoveSelection(self, deltax, deltay):
        #selected = set([el.GetObject() for el in self.selected if isinstance(el, Element.CElement)])
        movedCon = set()
        #~ print "selected: ", selected, "\n"
        for el in self.GetSelectedElements():
            x, y = el.GetPosition()
            el.SetPosition(x + deltax, y + deltay)
            for con in el.GetConnections():
                if (con.GetSource() in self.selected) and (con.GetDestination() in self.selected):
                    if con not in movedCon:
                        con.MoveAll(deltax , deltay )
                        movedCon.add(con)
        
        
    def DeleteItem(self, item):
        if isinstance(item, Connection.CConnection):
            self.DeleteConnection(item)
        elif isinstance(item, Element.CElement):
            self.DeleteElement(item)
        else:
            raise UMLException("UnknownItemClass")
        
        
    def DeleteElement(self, element):
        if element in self.elements:
            deleted = []
            self.elements.remove(element)
            if element in self.selected:
                self.selected.remove(element)
            for con in self.connections:
                if (con.GetSource() is element) or \
                    (con.GetDestination() is element):
                    deleted.append(con)
            for con in deleted:
                self.DeleteConnection(con)
        else:
            raise UMLException("ElementDoesNotExists")
        
    def DeleteConnection(self, connection):
        if connection in self.connections:
            self.connections.remove(connection)
            if connection in self.selected:
                self.selected.remove(connection)
        else:
            raise UMLException("ConnectionDoesNotExists")

    def GetSize(self):
        return (1000, 1000)
        
    def GetDrawable(self):
        return self.drawable        
        
    def GetElementAtPosition(self, canvas, x, y):
        for c in self.connections:
            if c.AreYouAtPosition(canvas, x, y):
                return c
                
        for e in self.elements[::-1]:
            if e.AreYouAtPosition(canvas, x, y):
                return e
            
        return None
        
    def Paint(self, canvas):
        canvas.Clear()
        for e in self.elements:
            e.Paint(canvas)
        
        for c in self.connections:
            c.Paint(canvas)
            
    def GetConnections(self):
        for c in self.connections:
            yield c
