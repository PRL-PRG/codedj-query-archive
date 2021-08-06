import lib.consts

class CElement:
    def __init__(self, drwngArea, obj):
        self.drawArea = drwngArea
        self.drawArea.AddElement(self)
        self.objct = obj
        self.position = (0,0)
        self.selected = False
        self.squares = []
    
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

    def AreYouAtPosition(self, canvas, x, y):
        width = self.objct.GetType().GetAppearance().GetWidth(canvas, self)
        height = self.objct.GetType().GetAppearance().GetHeight(canvas, self)
        
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
    
    def GetSize(self, canvas):
        return self.objct.GetWidth(canvas, self), self.objct.GetHeight(canvas, self)

    def Paint(self, canvas):
        self.objct.Paint(canvas, self)
        if self.selected:
            x, y = self.position
            w = self.objct.GetType().GetAppearance().GetWidth(canvas, self)
            h = self.objct.GetType().GetAppearance().GetHeight(canvas, self)
            
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

    def SetPosition(self, x, y):
        self.position = (x, y)
        
    def GetDrawingArea(self):
        return self.drawArea