from lib.config import config
from lib.Drawing import CConnection

class CElement:
    def __init__(self, drawingArea, obj, isLoad = False):
        self.isLoad = isLoad
        self.objct = obj
        self.position = (0,0)
        self.deltaSize = (0,0)
        self.selected = False
        self.squares = []
        self.drawingArea = drawingArea
        self.drawingArea.AddElement(self)
        self.__AddExistingConnections()
    
    def __AddExistingConnections(self):
        if not self.isLoad:
            for i in self.objct.GetConnections():
                if i.GetSource() is not self.objct:
                    if self.drawingArea.HasElementObject(i.GetSource()) is not None:
                        CConnection(self.drawingArea,i,self.drawingArea.HasElementObject(i.GetSource()),self)
                elif i.GetDestination is not self.objct:
                    if self.drawingArea.HasElementObject(i.GetDestination()) is not None:
                        CConnection(self.drawingArea,i,self,self.drawingArea.HasElementObject(i.GetDestination()))
                    
    def __AddSquare(self, index, x, y, posx, posy):
        size = config['/Styles/Selection/PointsSize']
        if posx == 0:
            x = x - size // 2
            x1 = x + size
        else:
            x1 = x + posx * size
        if posy == 0:
            y = y - size // 2
            y1 = y + size
        else:
            y1 = y + posy * size
        if x1 < x:
            x1, x = x, x1
        if y1 < y:
            y1, y = y, y1
        self.squares.append((index, (x, y), (x1 - x, y1 - y)))

    def AreYouAtPosition(self, canvas, pos):
        x, y = pos
        width, height = self.GetSize(canvas)
        
        if  (self.position[0] <= x <= self.position[0] + width) and (self.position[1] <= y <= self.position[1] + height):
            return True
        else:
            return False
    
    def Select(self):
        self.selected = True
    
    def Deselect(self):
        self.selected = False
        self.squares = []
    
    def GetSelected(self):
        return self.selected
        
    def GetObject(self):
        return self.objct

    def GetPosition(self):
        return self.position
        
    def GetCenter(self, canvas):
        w, h = self.GetSize(canvas)
        return w / 2 + self.position[0], h / 2 + self.position[1]
    
    def GetSize(self, canvas):
        w, h = self.objct.GetSize(canvas, self)
        w, h = w + self.deltaSize[0], h + self.deltaSize[1]
        return w, h 
        
    def GetMinimalSize(self, canvas):
        w, h = self.objct.GetSize(canvas, self)
        return w, h
        
    def GetSquare(self, canvas):
        x, y = self.GetPosition()
        w, h = self.GetSize(canvas)
        return ((x, y), (x + w, y + h))

    def Paint(self, canvas, delta = (0, 0)):
        self.objct.Paint(canvas, self, delta)
        if self.selected:
            x, y = self.position
            dx, dy = delta
            x += dx
            y += dy
            w, h = self.GetSize(canvas)
            rx, ry = self.objct.GetType().GetResizable()
            
            self.squares = []
            
            if rx and ry:
                self.__AddSquare(0, x       , y       ,  1,  1)
                self.__AddSquare(2, x + w   , y       , -1,  1)
                self.__AddSquare(5, x       , y + h   ,  1, -1)
                self.__AddSquare(7, x + w   , y + h   , -1, -1)
            if ry:
                self.__AddSquare(1, x + w//2, y       ,  0,  1)
                self.__AddSquare(6, x + w//2, y + h   ,  0, -1)
            if rx:
                self.__AddSquare(3, x       , y + h//2,  1,  0)
                self.__AddSquare(4, x + w   , y + h//2, -1,  0)
            
            for i in self.squares:
                canvas.DrawRectangle(i[1], i[2], None, config['/Styles/Selection/PointsColor'])
            
            canvas.DrawRectangle((x, y), (w, h), fg = config['/Styles/Selection/RectangleColor'], line_width = config['/Styles/Selection/RectangleWidth'])

    def SetPosition(self, pos):
        self.position = pos
        
    def GetDrawingArea(self):
        return self.drawingArea
        
    def GetConnections(self):
        for c1 in self.GetObject().GetConnections(): #ConnectionObject
            for c2 in self.drawingArea.GetConnections(): # Connection
                if c2.GetObject() is c1:
                    yield c2

    
    # Vrati poziciu obvodoveho(resizing) stvorceka na pozicii
    def GetSquareAtPosition(self, pos):
        x, y = pos
        for sq in self.squares:
            sqbx = sq[1][0]
            sqby = sq[1][1]
            sqex = sqbx + sq[2][0]
            sqey = sqby + sq[2][1]
            if (x >= sqbx and x <= sqex and y >= sqby and y <= sqey):
                return sq[0]
    
    # Zmena velkosti(pripadne pozicie) elementu
    def Resize(self, canvas, delta, selSquareIdx):
        resRect = self.GetResizedRect(canvas, delta, selSquareIdx)
        minSize = self.GetMinimalSize(canvas)
        self.position = resRect[0]
        self.deltaSize = (resRect[1][0]-minSize[0], resRect[1][1]-minSize[1])
            
    def GetSizeRelative(self):
        return self.deltaSize
        
    def SetSizeRelative(self, relatSize):
        self.deltaSize = relatSize
    
    # Zistenie novej polohy a velkosti pri resizingu
        # delta = relativna zmena velkosti
        # selSquareIdx = index uchytavacieho-resizovacieho bodu
    def GetResizedRect(self, canvas, delta, selSquareIdx):
        oldPos = self.GetPosition()
        position = oldPos
        oldSize = self.GetSize(canvas)
        size = oldSize
        minSize = self.GetMinimalSize(canvas)
        
        if (selSquareIdx not in [3,4]):
        # changing vertically (height)
            if (selSquareIdx in [0, 1, 2]):
                if (delta[1] <= 0): #zmensujem
                    size = (size[0], size[1] + delta[1])
                    if (size[1] < minSize[1]): #zmensujem pod minimum
                        position = (position[0], position[1]+oldSize[1]-minSize[1])
                        size = (size[0], minSize[1])
                    else:
                        position = (position[0], position[1] - delta[1])
                else: #zvacsujem
                    if (position[1] - delta[1] < 0):
                        delta = (delta[0], position[1])
                    position = (position[0], position[1] - delta[1])
                    size = (size[0], size[1] + delta[1])
            else: # [5,6,7]:  #position je nemenne
                size = (size[0], size[1] - delta[1])
                if (size[1] < minSize[1]):
                    size = (size[0], minSize[1])
                
        if (selSquareIdx not in [1, 6]):
        # changing horisontally (width)
            if (selSquareIdx in [0, 3, 5]):
                if (delta[0] <= 0): #zmensujem
                    size = (size[0] + delta[0], size[1])
                    if (size[0] < minSize[0]): #zmensujem pod minimum
                        position = (position[0]+oldSize[0]-minSize[0], position[1])
                        size = (minSize[0], size[1])
                    else:
                        position = (position[0] - delta[0], position[1])
                else: #zvacsujem
                    if (position[0] - delta[0] < 0):
                        delta = (position[0], delta[1])
                    position = (position[0] - delta[0], position[1])
                    size = (size[0] + delta[0], size[1])
            else: # [2,4,7] #position je nemenne
                size = (size[0] - delta[0], size[1])
                if (size[0] < minSize[0]):
                    size = (minSize[0], size[1])
                    
        return ((position), (size))
        
    def CopyFromElement(self, element):
        self.deltaSize = element.deltaSize
        self.position = element.position