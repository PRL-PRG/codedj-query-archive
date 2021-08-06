from lib.lib import UMLException
from lib.config import config
import Connection, Element
import lib.Math2D
from lib.Math2D import CRectangle
from lib.Math2D import CPoint


class CDrawingArea:
    def __init__(self, type, name = None): #  name = "untitled"
        self.elements = []
        self.connections = []
        self.selected = set()
        self.path = None
        self.typeDiagram = type
        self.size = None
        self.viewport = ((0, 0), (0, 0))
        if name is None:
            name = "New " + type.GetId()
        self.name = name
        
        
    # Cesta v strome kde sa nachadza drawing area
    def HasElementObject(self, object):
        for i in self.elements:
            if i.GetObject() is object:
                return i
        return None
        
    def GetPath(self):
        return self.path
    
    def SetPath(self, Path):
        self.path = Path
    
    def GetType(self):
        return self.typeDiagram
    
    def GetName(self):
        return self.name
    
    def SetName(self, name):
        self.name = name
    
    def AddElement(self, element):
        self.size = None
        if element not in self.elements:
            if element.GetObject().GetType().GetId() not in self.typeDiagram.GetElements():
                    raise UMLException("DiagramHaveNotThisElement")
            for i in self.elements:
                if i.GetObject() is element.GetObject():
                    raise UMLException("ElementAlreadyExists")
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
        self.size = None
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
        x1, y1 = self.GetSize(canvas)
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
    
    def MoveSelection(self, delta, canvas = None):
        self.size = None
        deltax, deltay = delta
        movedCon = set()
        for el in self.GetSelectedElements():
            x, y = el.GetPosition()
            el.SetPosition((x + deltax, y + deltay))
            for con in el.GetConnections():
                if (con.GetSource() in self.selected) and (con.GetDestination() in self.selected):
                    if con not in movedCon:
                        con.MoveAll(delta)
                        movedCon.add(con) 
        if canvas is not None:
            for conn in self.connections:
                conn.ValidatePoints(canvas)
                conn.RecalculateLabels()
    
    def DeleteObject(self, object):
        self.size = None
        for i in self.elements:
            if i.GetObject() is object:
                self.DeleteItem(i)
                return
    
    def DeleteItem(self, item):
        self.size = None
        if isinstance(item, Connection.CConnection):
            self.DeleteConnection(item)
        elif isinstance(item, Element.CElement):
            self.DeleteElement(item)
        else:
            raise UMLException("UnknownItemClass")
        
        
    def DeleteElement(self, element):
        self.size = None
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
        self.size = None
        if connection in self.connections:
            self.connections.remove(connection)
            if connection in self.selected:
                self.selected.remove(connection)
        else:
            raise UMLException("ConnectionDoesNotExists")

    def GetSize(self, canvas):
        if self.size is not None:
            return self.size
        else:
            result = (0, 0)
            for connection in self.connections:
                for point in connection.GetMiddlePoints():
                    result = tuple(max(x) for x in zip(result, point))
            for element in self.elements:
                    point = tuple(sum(x) for x in zip(element.GetPosition(), element.GetSize(canvas)))
                    result = tuple(max(x) for x in zip(result, point))
            page = (config['/Page/Width'], config['/Page/Height'])
            result = (page[0] * (result[0]//page[0] + 1), page[1] * (result[1]//page[1] + 1))
            self.size = result
        return result
        
    def GetDrawable(self):
        return self.drawable        
        
    def GetElementAtPosition(self, canvas, pos):
        for c in self.connections:
            if c.AreYouAtPosition(canvas, pos):
                return c
                
        for e in self.elements[::-1]:
            if e.AreYouAtPosition(canvas, pos):
                return e
            
        return None
    
    def SetViewPort(self, view):
        self.viewport = view
        
    def GetViewPort(self):
        return self.viewport
        
    #view = ((x, y), (w, h)
    def Paint(self, canvas):
        ((x, y), (w, h)) = self.viewport
        canvas.Clear()
        for e in self.elements:
            ((ex1, ey1), (ex2, ey2)) = e.GetSquare(canvas)
            if not (ex2 < x or x + w < ex1 or ey2 < y or y + w < ey1):
                e.Paint(canvas, delta = (-x, -y))
        for c in self.connections:
            ((ex1, ey1), (ex2, ey2)) = c.GetSquare(canvas)
            if not (ex2 < x or x + w < ex1 or ey2 < y or y + w < ey1):
                c.Paint(canvas, delta = (-x, -y))
            
    def GetElements(self):
        for e in self.elements:
            yield e
    
    def GetConnections(self):
        for c in self.connections:
            yield c
            
    # Automaticke generovanie mena diagramu 
    # pomocou cprojNode zisti mena diagramov na rovnakej urovni
    # ak meno uz existuje (a je rovnaky typ), area sa premenuje
    def Assign(self, cprojNode):
        if cprojNode.drawingareas != []:
            id = 1
            # zisti nazvy / typy diagramov, porovnaj a pripadne sa premenuj
            checkNames = True
            while checkNames :
                checkNames = False
                nName = self.GetName()
                for drArea in cprojNode.drawingareas:
                    if drArea.GetName() == self.GetName() and drArea.GetType() is self.GetType():
                        while nName[-1].isdigit(): # useknem cisla
                            nName = nName[:-1]
                        if nName.endswith(' '):
                            nName = nName + str(id)
                        else:
                            nName = nName + ' ' + str(id)
                        self.SetName(nName)
                        id = id + 1
                        checkNames = True #znovu prekontroluj nazvy
                        
    # Presunutie elementov uplne dopredu
    def ShiftElementsToTop(self):
        for selectedElement in self.GetSelectedElements():
            selectedIdx = self.elements.index(selectedElement)
            del self.elements[selectedIdx]
            self.elements.append(selectedElement) 

    # Presunutie elementov uplne dozadu
    def ShiftElementsToBottom(self):
        for selectedElement in self.GetSelectedElements():
            selectedIdx = self.elements.index(selectedElement)
            del self.elements[selectedIdx]
            self.elements.insert(0, selectedElement);
            
    # Presunutie elementov o 1 dopredu
    def ShiftElementsForward(self, canvas):
        for selectedElement in self.GetSelectedElements():
            selectedIdx = self.elements.index(selectedElement)
            selSq = selectedElement.GetSquare(canvas);
            selRect = CRectangle(CPoint(selSq[0]), CPoint(selSq[1]))
            selectedShifted = False
            otherElementIdx = selectedIdx + 1
            while otherElementIdx < len(self.elements) and selectedShifted == False:
                othSq = self.elements[otherElementIdx].GetSquare(canvas)
                othRect = CRectangle(CPoint(othSq[0]), CPoint(othSq[1]))
                prienik = selRect*othRect 
                if len(prienik) > 0:
                    del self.elements[selectedIdx]
                    self.elements.insert(otherElementIdx, selectedElement);
                    selectedShifted = True # uz je posunuty -> koncim a presuvam dalsi selecnuty
                otherElementIdx += 1
                
    # Presunutie elementov o 1 dozadu
    def ShiftElementsBack(self, canvas):
        for selectedElement in self.GetSelectedElements():
            selectedIdx = self.elements.index(selectedElement)
            selSq = selectedElement.GetSquare(canvas);
            selRect = CRectangle(CPoint(selSq[0]), CPoint(selSq[1]))
            selectedShifted = False
            otherElementIdx = selectedIdx - 1
            while otherElementIdx >= 0 and selectedShifted == False:
                othSq = self.elements[otherElementIdx].GetSquare(canvas)
                othRect = CRectangle(CPoint(othSq[0]), CPoint(othSq[1]))
                prienik = selRect*othRect
                if len(prienik) > 0:
                    del self.elements[selectedIdx]
                    self.elements.insert(otherElementIdx, selectedElement);
                    selectedShifted = True # uz je posunuty -> koncim a presuvam dalsi selecnuty
                otherElementIdx -= 1
