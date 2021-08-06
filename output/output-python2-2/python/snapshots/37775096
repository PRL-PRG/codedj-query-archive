from lib.Exceptions.UserException import *
from lib.Exceptions.UMLException import UMLException
from lib.config import config
import Connection, Element, ConLabelInfo
import lib.Math2D
from lib.Math2D import CRectangle
from lib.Math2D import CPoint


class CDiagram:
    def __init__(self, type, name = None): #  name = "untitled"
        self.elements = []
        self.connections = []
        self.selected = set()
        self.path = None
        self.typeDiagram = type
        self.size = None
        self.viewport = ((0, 0), (0, 0))
        self.scrollingPos = [0, 0]                  #position on diagram (needed for scrollBars)
        if name is None:
            name = "New " + type.GetId()
        self.name = name
        
    def GetHScrollingPos(self):
        return self.scrollingPos[0]
    
    def GetVScrollingPos(self):
        return self.scrollingPos[1]
    
    def SetHScrollingPos(self, value):
        self.scrollingPos[0] = value
    
    def SetVScrollingPos(self, value):
        self.scrollingPos[1] = value
        
    # Cesta v strome kde sa nachadza diagram
    def HasElementObject(self, object):
        for i in self.elements:
            if i.GetObject() is object:
                return i
        return None
    
    def GetConnection(self, conObject):
        for c in self.connections:
            if c.GetObject() is conObject:
                return c
        return None
    
    def HasConnection(self,conObject):
        for c in self.connections:
            if c.GetObject() is conObject:
                return True
        return False
        
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
                raise DrawingError("DiagramHaveNotThisElement", element)
            for i in self.elements:
                if i.GetObject() is element.GetObject():
                    raise DrawingError("ElementAlreadyExists", element)
            self.elements.append(element)
        else:
            raise DrawingError("ElementAlreadyExists", element)
     
    def GetSelected(self):
        selected = tuple(self.selected)
        for i in selected:
            if i in self.selected:
                yield i
            
    def GetSelectedElements(self, nolabels = False):
        for i in self.selected:
            if nolabels:
                if isinstance(i, Element.CElement):
                    yield i
            else:
                if isinstance(i, (Element.CElement, ConLabelInfo.CConLabelInfo)):
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
            raise DrawingError("ConnectionAlreadyExists")
            
    def SelectedCount(self):
        return len(self.selected)
    
    def AddToSelection(self, element):
        self.selected.add(element)
        element.Select()
    
    def AddRangeToSelection(self, canvas, topleft, rightbottom):
        for el in self.GetElementsInRange(canvas, topleft, rightbottom, False):
            self.selected.add(el)
            el.Select()
    
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
            x, y = el.GetPosition(canvas)
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
        elements = set()
        if canvas is not None:
            for el in self.GetSelectedElements():
                if not isinstance(el, ConLabelInfo.CConLabelInfo):
                    pos1, pos2 = el.GetSquare(canvas)
                    zorder = self.elements.index(el)
                    for el2 in self.GetElementsInRange(canvas, pos1, pos2, True):
                        if not isinstance(el2, ConLabelInfo.CConLabelInfo):
                            if self.elements.index(el2) > zorder:
                                elements.add(el2)
        elements |= set(self.GetSelectedElements())
        for el in elements:
            x, y = el.GetPosition(canvas)
            el.SetPosition((x + deltax, y + deltay), canvas)
            if not isinstance(el, ConLabelInfo.CConLabelInfo):
                for con in el.GetConnections():
                    if (con.GetSource() in elements) and (con.GetDestination() in elements):
                        if con not in movedCon:
                            con.MoveAll(delta, canvas)
                            movedCon.add(con)
        if canvas is not None:
            for conn in self.connections:
                conn.ValidatePoints(canvas)
    
    def DeleteObject(self, object):
        self.size = None
        for o in self.elements:
            if o.GetObject() is object:
                for c in o.GetConnections():
                    self.ShiftDeleteConnection(c)
                self.DeleteItem(o)
                return
    
    def DeleteItem(self, item):
        self.size = None
        if isinstance(item, Connection.CConnection):
            self.DeleteConnection(item)
        elif isinstance(item, Element.CElement):
            self.DeleteElement(item)
        else:
            raise DrawingError("UnknownItemClass")
        
        
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
            raise DrawingError("ElementDoesNotExists")
        
    def DeleteConnection(self, connection):
        self.size = None
        if connection in self.connections:
            self.connections.remove(connection)
            if connection in self.selected:
                self.selected.remove(connection)
        else:
            raise DrawingError("ConnectionDoesNotExists")
    
    def DeleteConnectionObject(self, object):
        for i in self.connections:
            if i.GetObject() is object:
                self.connections.remove(i)
                return
    
    def ShiftDeleteConnection(self, connection):
        self.size = None
        if connection in self.connections:
            obj = connection.GetObject()
            for a in obj.GetAppears():
                a.DeleteConnectionObject(obj)
                
            obj.GetSource().RemoveConnection(obj)
            if obj.GetSource() is not obj.GetDestination():
                obj.GetDestination().RemoveConnection(obj)
            #self.connections.remove(connection)
            if connection in self.selected:
                self.selected.remove(connection)
        else:
            raise DrawingError("ConnectionDoesNotExists")
    
    def GetSize(self, canvas):
        if self.size is not None:
            return self.size
        else:
            result = (0, 0)
            for connection in self.connections:
                for point in connection.GetMiddlePoints():
                    result = tuple(max(x) for x in zip(result, point))
            for element in self.elements:
                    point = tuple(sum(x) for x in zip(element.GetPosition(canvas), element.GetSize(canvas)))
                    result = tuple(max(x) for x in zip(result, point))
            page = (config['/Page/Width'], config['/Page/Height'])
            result = (page[0] * (result[0]//page[0] + 1), page[1] * (result[1]//page[1] + 1))
            self.size = result
        return result
        
    def GetDrawable(self):
        return self.drawable        
        
    def GetElementAtPosition(self, canvas, pos):
        for c in self.connections:
            r = c.WhatPartOfYouIsAtPosition(canvas, pos)
            if isinstance(r, (int, long)):
                return c
            elif r is not None:
                return r
                
        for e in self.elements[::-1]:
            if e.AreYouAtPosition(canvas, pos):
                return e
            
        return None
    
    def GetElementsInRange(self, canvas, topleft, bottomright, includeall = True):
        for e in self.elements:
            if e.AreYouInRange(canvas, topleft, bottomright, includeall):
                yield e
    
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
            
    def PaintFull(self, canvas):
        canvas.Clear()
        for e in self.elements:
            e.Paint(canvas)
        for c in self.connections:
            c.Paint(canvas)
    
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
        if cprojNode.diagrams != []:
            id = 1
            # zisti nazvy / typy diagramov, porovnaj a pripadne sa premenuj
            checkNames = True
            while checkNames :
                checkNames = False
                nName = self.GetName()
                for drArea in cprojNode.diagrams:
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
            if not isinstance(selectedElement, ConLabelInfo.CConLabelInfo):
                selectedIdx = self.elements.index(selectedElement)
                del self.elements[selectedIdx]
                self.elements.append(selectedElement) 

    # Presunutie elementov uplne dozadu
    def ShiftElementsToBottom(self):
        for selectedElement in self.GetSelectedElements():
            if not isinstance(selectedElement, ConLabelInfo.CConLabelInfo):
                selectedIdx = self.elements.index(selectedElement)
                del self.elements[selectedIdx]
                self.elements.insert(0, selectedElement);
            
    # Presunutie elementov o 1 dopredu
    def ShiftElementsForward(self, canvas):
        for selectedElement in self.GetSelectedElements():
            if not isinstance(selectedElement, ConLabelInfo.CConLabelInfo):
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
            if not isinstance(selectedElement, ConLabelInfo.CConLabelInfo):
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
    
    def CutSelection(self, clipboard):
        if self.selected:
            clipboard.SetContent((el for el in self.selected if isinstance(el, Element.CElement)))
            for el in list(self.selected):
                if isinstance(el, Element.CElement):
                    self.DeleteElement(el)
    
    def CopySelection(self, clipboard):
        if self.selected:
            clipboard.SetContent((el for el in self.selected if isinstance(el, Element.CElement)))
    
    def PasteSelection(self, clipboard):
        pasted = set()
        for i in clipboard.GetContent():
            try:
                el = Element.CElement(self, i.GetObject())
            except UMLException, e:
                for el in pasted:
                    self.DeleteElement(el)
                raise
            self.AddToSelection(el)
            el.CopyFromElement(i)
            pasted.add(el)

    def GetExpSquare(self, canvas):
        #square for export, the minimal size is measured so the exported diagram has the same edges - looks better
        x_max, y_max,x_min, y_min,  = 0, 0,  101, 101
        for el in self.elements:
            posX, posY = el.GetPosition(canvas)
            w, h = el.GetSize(canvas)
            if posX + w > x_max:
                x_max = posX + w
            if posY + h > y_max:
                y_max = posY + h
            if posX < x_min:
                x_min = posX
            if posY < y_min:
                y_min = posY

        for connection in self.connections:
            for point in connection.GetMiddlePoints():
                posX, posY = point
                if posX > x_max:
                    x_max = posX
                if posY > y_max:
                    y_max = posY
                if posX < x_min:
                    x_min = posX
                if posY < y_min:
                    y_min = posY
        if x_min > 100 :
            x_min = 100
        if y_min > 100 :
            y_min = 100
        return (x_max +x_min, y_max + y_min)

    def GetSizeSquare(self, canvas):
        x_max, y_max,x_min, y_min,  = 0, 0,  9999, 9999
        for el in self.elements:
            posX, posY = el.GetPosition(canvas)
            w, h = el.GetSize(canvas)
            if posX + w > x_max:
                x_max = posX + w
            if posY + h > y_max:
                y_max = posY + h
            if posX < x_min:
                x_min = posX
            if posY < y_min:
                y_min = posY
        for connection in self.connections:
            #posX, posY = connection.GetSquare(canvas, True)[1]
            #if posX > x_max:
                #x_max = posX
            #if posY > y_max:
                #y_max = posY
            #if posX < x_min:
                #x_min = posX
            #if posY < y_min:
                #y_min = posY
                    
            for point in connection.GetMiddlePoints():
                posX, posY = point
                if posX > x_max:
                    x_max = posX
                if posY > y_max:
                    y_max = posY
                if posX < x_min:
                    x_min = posX
                if posY < y_min:
                    y_min = posY
                  
        return ((int(x_min),int(y_min)),(int(x_max), int(y_max)))

