import lib.consts

class CElement:
    def __init__(self, drawingArea, obj):
        self.objct = obj
        self.position = (0,0)
        self.deltaSize = (0,0)
        self.selected = False
        self.squares = []
        self.drawingArea = drawingArea
        self.drawingArea.AddElement(self)
    
    def __AddSquare(self, x, y, posx, posy):
        if posx == 0:
            x = x - lib.consts.SELECT_SQUARES_SIZE // 2
            x1 = x + lib.consts.SELECT_SQUARES_SIZE
        else:
            x1 = x + posx * lib.consts.SELECT_SQUARES_SIZE
        if posy == 0:
            y = y - lib.consts.SELECT_SQUARES_SIZE // 2
            y1 = y + lib.consts.SELECT_SQUARES_SIZE
        else:
            y1 = y + posy * lib.consts.SELECT_SQUARES_SIZE
        if x1 < x:
            x1, x = x, x1
        if y1 < y:
            y1, y = y, y1
        self.squares.append(((x, y), (x1 - x, y1 - y)))

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
        return self.objct.GetWidth(canvas, self) / 2 + self.position[0], self.objct.GetHeight(canvas, self) / 2 + self.position[1]
    
    def GetSize(self, canvas):
        w = self.objct.GetWidth(canvas, self) + self.deltaSize[0]
        h = self.objct.GetHeight(canvas, self) + self.deltaSize[1]
        return w, h 
        
    def GetSquare(self, canvas):
        x, y = self.GetPosition()
        w, h = self.GetSize(canvas)
        return (x, y), (x + w, y + h)

    def Paint(self, canvas):
        self.objct.Paint(canvas, self)
        if self.selected:
            x, y = self.position
            w, h = self.GetSize(canvas)
            
            self.squares = []
            
            self.__AddSquare(x       , y       ,  1,  1)
            self.__AddSquare(x + w//2, y       ,  0,  1)
            self.__AddSquare(x + w   , y       , -1,  1)
            self.__AddSquare(x       , y + h//2,  1,  0)
            self.__AddSquare(x + w   , y + h//2, -1,  0)
            self.__AddSquare(x       , y + h   ,  1, -1)
            self.__AddSquare(x + w//2, y + h   ,  0, -1)
            self.__AddSquare(x + w   , y + h   , -1, -1)
            
            for i in self.squares:
                canvas.DrawRectangle(i[0], i[1], None, lib.consts.SELECT_SQUARES_COLOR)
            
            canvas.DrawRectangle((x, y), (w, h), fg = lib.consts.SELECT_SQUARE_COLOR, line_width = lib.consts.SELECT_SQUARE_SIZE)

    def SetPosition(self, pos):
        self.position = pos
        
    def GetDrawingArea(self):
        return self.drawingArea
        
    def GetConnections(self):
        for c1 in self.GetObject().GetConnections(): #ConnectionObject
            for c2 in self.drawingArea.GetConnections(): # Connection
                if c2.GetObject() is c1:
                    yield c2

    
    # Vrati poziciu obvodoveho stvorceka na pozicii
    def GetSquareAtPosition(self, pos):
        x, y = pos
        for sq in self.squares:
            sqbx = sq[0][0]
            sqby = sq[0][1]
            sqex = sqbx + sq[1][0]
            sqey = sqby + sq[1][1]
            if (x >= sqbx and x <= sqex and y >= sqby and y <= sqey):
                return self.squares.index(sq)
    
    # Zmena velkosti - zmenenie nejde pod minimum
    def Resize(self, delta, selSquareIdx):
        # Zmenim velkost bez ohladu na minim. velkost:
        if (selSquareIdx not in [3,4]):
            # changing horisontally
            if (selSquareIdx in [0, 1, 2]):
                self.deltaSize = (self.deltaSize[0], self.deltaSize[1] - delta[1])
                self.position = (self.position[0], self.position[1] + delta[1])
            else:
                self.deltaSize = (self.deltaSize[0], self.deltaSize[1] + delta[1])
        if (selSquareIdx not in [1, 6]):
            if (selSquareIdx in [0, 3, 5]):
                # changing vertically
                self.deltaSize = (self.deltaSize[0] - delta[0], self.deltaSize[1])
                self.position = (self.position[0] + delta[0], self.position[1])
            else:
                self.deltaSize = (self.deltaSize[0] + delta[0], self.deltaSize[1])
            
        # Pripadne nastavenie minimalnej velkosti:
        if (self.deltaSize[0] < 0):
            self.deltaSize = (0, self.deltaSize[1]) 
        if (self.deltaSize[1] < 0):
            self.deltaSize = (self.deltaSize[0], 0)
            
    def GetDelta(self):
        return self.deltaSize
       
    def CopyFromElement(self, element):
        self.deltaSize = element.deltaSize
        self.position = element.position


